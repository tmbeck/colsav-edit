use ratatui::style::{Color, Modifier, Style};

// VGA 4-bit palette — authentic Norton Commander / DOS editor colors.
// These are the exact RGB values from the standard VGA text-mode palette,
// not ANSI terminal colors which vary wildly across terminals.
pub const BG: Color = Color::Rgb(0x00, 0x00, 0xAA); // VGA dark blue (color 1)
pub const FG: Color = Color::Rgb(0xAA, 0xAA, 0xAA); // VGA light gray (color 7)
pub const FG_BRIGHT: Color = Color::Rgb(0xFF, 0xFF, 0xFF); // VGA bright white (color 15)
pub const BORDER: Color = Color::Rgb(0x00, 0xAA, 0xAA); // VGA cyan (color 3)
pub const HEADER: Color = Color::Rgb(0xFF, 0xFF, 0x55); // VGA yellow (color 14)
pub const HIGHLIGHT_BG: Color = Color::Rgb(0x00, 0xAA, 0xAA); // VGA cyan (color 3)
pub const HIGHLIGHT_FG: Color = Color::Rgb(0x00, 0x00, 0x00); // VGA black (color 0)
pub const STATUS_BG: Color = Color::Rgb(0x00, 0xAA, 0xAA); // VGA cyan (color 3)
pub const STATUS_FG: Color = Color::Rgb(0x00, 0x00, 0x00); // VGA black (color 0)
pub const MUTED: Color = Color::Rgb(0x55, 0x55, 0xFF); // VGA light blue (color 9)

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
    Style::default().fg(HIGHLIGHT_FG).bg(HIGHLIGHT_BG)
}

pub fn status() -> Style {
    Style::default().fg(STATUS_FG).bg(STATUS_BG)
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

pub fn muted() -> Style {
    Style::default().fg(MUTED).bg(BG)
}
