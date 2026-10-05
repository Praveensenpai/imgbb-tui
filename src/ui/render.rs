use crate::cli::app::App;
use crate::domain::model::UploadState;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;

// Catppuccin Mocha - soft, cohesive palette
const ACCENT: Color = Color::Rgb(0xb4, 0xbe, 0xfe); // lavender
const ACCENT_DIM: Color = Color::Rgb(0x72, 0x87, 0xfd); // periwinkle
const SUCCESS: Color = Color::Rgb(0xa6, 0xe3, 0xa1); // mint
const ERROR: Color = Color::Rgb(0xf3, 0x8b, 0xa8); // rose
const TEXT: Color = Color::Rgb(0xf5, 0xe0, 0xdc); // rosewater
const SUBTEXT: Color = Color::Rgb(0xba, 0xc2, 0xde); // subtext
const MUTED: Color = Color::Rgb(0x7f, 0x84, 0x9c); // overlay
const SURFACE: Color = Color::Rgb(0x31, 0x32, 0x44); // surface
const SPINNER: [&str; 10] = [
    "\u{280b}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283c}", "\u{2834}", "\u{2826}", "\u{2827}",
    "\u{2807}", "\u{280f}",
];

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(6),
            Constraint::Length(2),
        ])
        .split(frame.area());

    draw_header(frame, chunks[0]);
    if app.needs_key_setup() {
        draw_key_setup(frame, app, chunks[1]);
    } else {
        draw_body(frame, app, chunks[1]);
    }
    draw_footer(frame, app, chunks[2]);
}

fn panel(title: &str, color: Color) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(color))
        .title(Line::from(vec![
            Span::styled(" ", Style::default().fg(color)),
            Span::styled(
                title.to_string(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default().fg(color)),
        ]))
}

fn draw_header(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled("\u{25c8} ", Style::default().fg(ACCENT)),
        Span::styled(
            "imgbb",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" \u{2022} ", Style::default().fg(SURFACE)),
        Span::styled(
            "tui",
            Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "   paste \u{2192} upload \u{2192} copy",
            Style::default().fg(MUTED),
        ),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

fn draw_key_setup(frame: &mut Frame, app: &App, area: Rect) {
    let block = panel("welcome \u{2022} api key", ACCENT);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = vec![
        Line::from(Span::styled(
            "Get a free key at ",
            Style::default().fg(SUBTEXT),
        )),
        Line::from(Span::styled(
            "https://api.imgbb.com/",
            Style::default().fg(ACCENT_DIM),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("\u{276f} ", Style::default().fg(SUCCESS)),
            Span::styled(app.input().to_string(), Style::default().fg(TEXT)),
            Span::styled("\u{2588}", Style::default().fg(ACCENT)),
        ]),
    ];
    frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), inner);
}

fn draw_body(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(4)])
        .split(area);

    draw_input(frame, app, rows[0]);
    draw_result(frame, app, rows[1]);
}

fn draw_input(frame: &mut Frame, app: &App, area: Rect) {
    let active = matches!(app.state(), UploadState::Idle | UploadState::Failed);
    let border = if active { ACCENT } else { SURFACE };
    let block = panel("image path", border);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let line = if app.input().is_empty() {
        Line::from(vec![
            Span::styled("\u{276f} ", Style::default().fg(MUTED)),
            Span::styled(
                "Ctrl+V paste an image  \u{2022}  or type a path and press Enter",
                Style::default().fg(MUTED),
            ),
        ])
    } else {
        let prompt_width = 2;
        let cursor_width = 1;
        let available = inner.width.saturating_sub(prompt_width + cursor_width) as usize;
        let input = app.input();
        let total = input.chars().count();
        let visible = if total > available {
            let skip = total - available;
            input.chars().skip(skip).collect::<String>()
        } else {
            input.to_string()
        };
        Line::from(vec![
            Span::styled("\u{276f} ", Style::default().fg(SUCCESS)),
            Span::styled(visible, Style::default().fg(TEXT)),
            Span::styled("\u{2588}", Style::default().fg(ACCENT)),
        ])
    };
    frame.render_widget(Paragraph::new(line), inner);
}

fn draw_result(frame: &mut Frame, app: &App, area: Rect) {
    let block = panel("result", status_color(app.state()));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    match app.state() {
        UploadState::Idle => render_idle(frame, inner),
        UploadState::Uploading => render_uploading(frame, app, inner),
        UploadState::Success => render_success(frame, app, inner),
        UploadState::Failed => render_failed(frame, app, inner),
    }
}

fn render_idle(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("\u{25cb} ", Style::default().fg(SUBTEXT)),
            Span::styled("ready", Style::default().fg(SUBTEXT)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Uploaded URLs are copied to your clipboard automatically.",
            Style::default().fg(MUTED),
        )),
    ];
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
}

fn render_uploading(frame: &mut Frame, app: &App, area: Rect) {
    let frame_char = SPINNER[app.spinner_frame() % SPINNER.len()];
    let lines = vec![
        Line::from(vec![
            Span::styled(format!("{frame_char} "), Style::default().fg(ACCENT)),
            Span::styled("uploading", Style::default().fg(TEXT)),
            Span::styled(" to imgbb", Style::default().fg(MUTED)),
            Span::styled("\u{2026}", Style::default().fg(MUTED)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            app.status().unwrap_or("please wait"),
            Style::default().fg(SUBTEXT),
        )),
    ];
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
}

fn render_success(frame: &mut Frame, app: &App, area: Rect) {
    let Some(result) = app.result() else {
        return;
    };
    let mut lines = vec![
        Line::from(vec![
            Span::styled("\u{2713} ", Style::default().fg(SUCCESS)),
            Span::styled(
                "upload complete",
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
            ),
        ]),
        divider(),
        link_row("\u{25cd}", "direct", &result.url, ACCENT),
        link_row("\u{25c8}", "viewer", &result.viewer_url, SUBTEXT),
        link_row("\u{25c7}", "thumb ", &result.thumb_url, MUTED),
    ];
    if let Some(status) = app.status() {
        lines.push(Line::from(""));
        lines.push(copied_banner(status));
    }
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
}

fn copied_banner(status: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled("  ", Style::default().bg(SUCCESS)),
        Span::styled(
            " \u{2713} ",
            Style::default()
                .fg(Color::Rgb(0x1e, 0x1e, 0x2e))
                .bg(SUCCESS)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {status} "),
            Style::default()
                .fg(Color::Rgb(0x1e, 0x1e, 0x2e))
                .bg(SUCCESS)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default().bg(SUCCESS)),
    ])
}

fn render_failed(frame: &mut Frame, app: &App, area: Rect) {
    let message = app.error().unwrap_or("unknown error");
    let lines = vec![
        Line::from(vec![
            Span::styled("\u{2717} ", Style::default().fg(ERROR)),
            Span::styled(
                "upload failed",
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
            ),
        ]),
        divider(),
        Line::from(Span::styled(
            message.to_string(),
            Style::default().fg(ERROR),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press Ctrl+V to retry from clipboard, or edit the path above.",
            Style::default().fg(MUTED),
        )),
    ];
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
}

fn link_row(icon: &str, label: &str, url: &str, color: Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{icon} "), Style::default().fg(color)),
        Span::styled(
            format!("{label:<7}"),
            Style::default().fg(MUTED).add_modifier(Modifier::BOLD),
        ),
        Span::styled(url.to_string(), Style::default().fg(color)),
    ])
}

fn divider() -> Line<'static> {
    Line::from(Span::styled(
        "\u{2500}".repeat(48),
        Style::default().fg(SURFACE),
    ))
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let hint = if app.needs_key_setup() {
        "Enter save  \u{2022}  Ctrl+C quit"
    } else {
        "Ctrl+V paste  \u{2022}  Enter upload  \u{2022}  Ctrl+C quit"
    };
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(hint, Style::default().fg(MUTED))))
            .alignment(Alignment::Center),
        area,
    );
}

fn status_color(state: UploadState) -> Color {
    match state {
        UploadState::Success => SUCCESS,
        UploadState::Failed => ERROR,
        _ => ACCENT,
    }
}
