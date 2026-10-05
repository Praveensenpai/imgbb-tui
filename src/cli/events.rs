use crossterm::event::{Event, EventStream, KeyEvent};
use futures::StreamExt;

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Paste(String),
}

pub struct EventLoop {
    stream: EventStream,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            stream: EventStream::new(),
        }
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        let mut ticker = tokio::time::interval(std::time::Duration::from_millis(250));
        loop {
            tokio::select! {
                maybe_event = self.stream.next() => match maybe_event {
                    Some(Ok(Event::Key(key))) => return Some(AppEvent::Key(key)),
                    Some(Ok(Event::Paste(text))) => return Some(AppEvent::Paste(text)),
                    Some(Ok(_)) => continue,
                    Some(Err(_)) | None => return None,
                },
                _ = ticker.tick() => return Some(AppEvent::Tick),
            }
        }
    }
}
