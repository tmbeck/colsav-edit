use colonization_sav::{ControlType, Difficulty, Season, SaveFile};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use super::theme;

fn enum_name<T>(value: u8) -> String
where
    T: TryFrom<u8, Error = u8> + ToString,
{
    T::try_from(value)
        .map(|v| v.to_string())
        .unwrap_or_else(|v| format!("Unknown(0x{v:02X})"))
}

pub fn render(frame: &mut Frame, area: Rect, save: &SaveFile) {
    let header = &save.header;
    let difficulty = enum_name::<Difficulty>(header.difficulty);
    let season = enum_name::<Season>(header.season as u8);

    let mut lines = vec![
        Line::from(vec![Span::styled("COLONIZATION SAVE HEADER", theme::header())]),
        Line::from(""),
        Line::from(format!("Year: {}  Season: {}", header.year, season)),
        Line::from(format!("Turn: {}  Difficulty: {}", header.turn, difficulty)),
        Line::from(format!(
            "Map: {} x {}  Units: {}  Colonies: {}  Tribes: {}",
            header.map_size_x,
            header.map_size_y,
            header.unit_count,
            header.colony_count,
            header.tribe_count
        )),
        Line::from(""),
        Line::from(vec![Span::styled("Players", theme::header())]),
    ];

    for (idx, player) in save.players.iter().enumerate() {
        let control = enum_name::<ControlType>(player.control);
        lines.push(Line::from(format!(
            "{}: {} ({})  Control: {}  Founded Colonies: {}",
            idx + 1,
            player.name(),
            player.country_name(),
            control,
            player.founded_colonies
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled("Game Options", theme::header())]));
    lines.push(Line::from(format!(
        "Tutorial: {}  Autosave: {}  Combat Analysis: {}  Cheats: {}",
        yes_no(header.game_options.tutorial_hints),
        yes_no(header.game_options.autosave),
        yes_no(header.game_options.combat_analysis),
        yes_no(header.game_options.cheats_enabled)
    )));
    lines.push(Line::from(format!(
        "Show Foreign Moves: {}  Show Indian Moves: {}  End Turn Prompt: {}",
        yes_no(header.game_options.show_foreign_moves),
        yes_no(header.game_options.show_indian_moves),
        yes_no(header.game_options.end_of_turn)
    )));

    let paragraph = Paragraph::new(lines)
        .style(theme::base())
        .block(
            Block::bordered()
                .title(" Header ")
                .border_style(theme::border()),
        )
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn yes_no(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}
