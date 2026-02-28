use ratatui::style::{Color, Modifier, Style};

pub const BG: Color = Color::Blue;
pub const FG: Color = Color::White;
pub const BORDER: Color = Color::Cyan;
pub const HEADER: Color = Color::Yellow;
pub const HIGHLIGHT: Color = Color::Cyan;
pub const STATUS_BG: Color = Color::DarkGray;

pub fn base() -> Style {
    Style::default().fg(FG).bg(BG)
}

pub fn border() -> Style {
    Style::default().fg(BORDER).bg(BG)
}

pub fn header() -> Style {
    Style::default()
        .fg(HEADER)
        .bg(BG)
        .add_modifier(Modifier::BOLD)
}

pub fn highlight() -> Style {
    Style::default().fg(Color::Black).bg(HIGHLIGHT)
}

pub fn status() -> Style {
    Style::default().fg(FG).bg(STATUS_BG)
}

pub fn tab_active() -> Style {
    Style::default()
        .fg(HEADER)
        .bg(BG)
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
}

pub fn tab_inactive() -> Style {
    Style::default().fg(FG).bg(BG)
}
