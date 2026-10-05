use crate::api::imgbb::ImgbbClient;
use crate::cli::events::{AppEvent, EventLoop};
use crate::domain::error::AppError;
use crate::domain::model::{HistoryEntry, ImagePayload, UploadResult, UploadState};
use crate::infra::clipboard;
use crate::infra::config::Config;
use crate::infra::history::HistoryStore;
use crate::ui::render;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::Stdout;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

pub struct App {
    input: String,
    state: UploadState,
    result: Option<UploadResult>,
    error: Option<String>,
    status: Option<String>,
    config: Config,
    history: HistoryStore,
    spinner_frame: usize,
    needs_key_setup: bool,
    tx: mpsc::Sender<UploadOutcome>,
    rx: mpsc::Receiver<UploadOutcome>,
}

enum UploadOutcome {
    Success(UploadResult, bool),
    Failure(String),
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load().unwrap_or_default();
        let history = HistoryStore::new()?;
        let (tx, rx) = mpsc::channel(4);
        Ok(Self {
            input: String::new(),
            state: UploadState::Idle,
            result: None,
            error: None,
            status: None,
            needs_key_setup: config.needs_api_key(),
            config,
            history,
            spinner_frame: 0,
            tx,
            rx,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut terminal = setup_terminal()?;
        let outcome = self.event_loop(&mut terminal).await;
        restore_terminal(&mut terminal)?;
        outcome
    }

    async fn event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<()> {
        let mut events = EventLoop::new();
        loop {
            if self.state == UploadState::Uploading {
                self.spinner_frame = self.spinner_frame.wrapping_add(1);
            }
            terminal.draw(|frame| render::draw(frame, self))?;

            tokio::select! {
                maybe = events.next() => match maybe {
                    Some(AppEvent::Key(key)) => {
                        if self.handle_key(key)? {
                            return Ok(());
                        }
                    }
                    Some(AppEvent::Paste(text)) => {
                        let cleaned = normalize_paste(&text);
                        self.input.push_str(&cleaned);
                    }
                    Some(AppEvent::Tick) => self.drain_outcomes(),
                    None => return Ok(()),
                },
                Some(outcome) = self.rx.recv() => self.apply_outcome(outcome),
            }
        }
    }

    fn drain_outcomes(&mut self) {
        while let Ok(outcome) = self.rx.try_recv() {
            self.apply_outcome(outcome);
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<bool> {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return Ok(true);
        }
        if self.needs_key_setup {
            self.handle_key_setup(key)?;
            return Ok(false);
        }
        match key.code {
            KeyCode::Esc => return Ok(true),
            KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.start_clipboard_upload();
            }
            KeyCode::Enter => self.start_input_upload(),
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Delete => self.input.clear(),
            KeyCode::Char(c) => self.input.push(c),
            _ => {}
        }
        Ok(false)
    }

    fn handle_key_setup(&mut self, key: KeyEvent) -> Result<(), AppError> {
        match key.code {
            KeyCode::Enter if !self.input.trim().is_empty() => {
                self.config.api_key = self.input.trim().to_string();
                self.config.auto_copy = true;
                self.config.keep_history = true;
                self.config.save()?;
                self.input.clear();
                self.needs_key_setup = false;
                self.status = Some("API key saved".to_string());
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Char(c) => self.input.push(c),
            _ => {}
        }
        Ok(())
    }

    fn start_clipboard_upload(&mut self) {
        match clipboard::read_clipboard_image() {
            Ok(bytes) => {
                let payload = ImagePayload {
                    bytes,
                    filename: "clipboard.png".to_string(),
                };
                self.spawn_upload(payload);
            }
            Err(err) => self.fail(err.to_string()),
        }
    }

    fn start_input_upload(&mut self) {
        let raw = self.input.trim().to_string();
        if raw.is_empty() {
            return;
        }
        match ImagePayload::from_file(&raw.into()) {
            Ok(payload) => self.spawn_upload(payload),
            Err(err) => self.fail(err.to_string()),
        }
    }

    fn spawn_upload(&mut self, payload: ImagePayload) {
        self.state = UploadState::Uploading;
        self.error = None;
        self.result = None;
        self.status = Some(format!("Uploading {} ...", payload.filename));

        let api_key = self.config.api_key.clone();
        let auto_copy = self.config.auto_copy;
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let outcome = match ImgbbClient::new(api_key) {
                Ok(client) => match client.upload(&payload).await {
                    Ok(result) => UploadOutcome::Success(result, auto_copy),
                    Err(err) => UploadOutcome::Failure(err.to_string()),
                },
                Err(err) => UploadOutcome::Failure(err.to_string()),
            };
            let _ = tx.send(outcome).await;
        });
    }

    fn apply_outcome(&mut self, outcome: UploadOutcome) {
        match outcome {
            UploadOutcome::Success(result, auto_copy) => {
                self.state = UploadState::Success;
                if auto_copy {
                    let _ = clipboard::copy_to_clipboard(&result.url);
                    self.status = Some("URL copied to clipboard".to_string());
                } else {
                    self.status = Some("Upload complete".to_string());
                }
                self.push_history(&result);
                self.result = Some(result);
            }
            UploadOutcome::Failure(message) => self.fail(message),
        }
    }

    fn push_history(&self, result: &UploadResult) {
        if !self.config.keep_history {
            return;
        }
        let entry = HistoryEntry {
            url: result.url.clone(),
            filename: result.filename.clone(),
            uploaded_at: now_epoch_string(),
        };
        let _ = self.history.push(entry);
    }

    fn fail(&mut self, message: String) {
        self.state = UploadState::Failed;
        self.status = None;
        self.error = Some(message);
    }

    pub fn input(&self) -> &str {
        &self.input
    }
    pub fn state(&self) -> UploadState {
        self.state
    }
    pub fn result(&self) -> Option<&UploadResult> {
        self.result.as_ref()
    }
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }
    pub fn needs_key_setup(&self) -> bool {
        self.needs_key_setup
    }
    pub fn spinner_frame(&self) -> usize {
        self.spinner_frame
    }
}

fn normalize_paste(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn now_epoch_string() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    secs.to_string()
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
