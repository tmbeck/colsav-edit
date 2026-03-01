use colonization_sav::raw::Tribe;
use colonization_sav::{NationType, SaveFile, TechType};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, TableState};

use super::theme;

const INDIAN_NAMES: [&str; 8] = [
    "Inca", "Aztec", "Arawak", "Iroquois", "Cherokee", "Apache", "Sioux", "Tupi",
];

pub fn render(frame: &mut Frame, area: Rect, save: &SaveFile, state: &mut TableState) {
    let rows =
        Layout::vertical([Constraint::Percentage(42), Constraint::Percentage(58)]).split(area);

    let indian_panel = Paragraph::new(indian_nations_text(save))
        .style(theme::base())
        .block(
            Block::bordered()
                .title(" Indian Nations ")
                .border_style(theme::border()),
        );
    frame.render_widget(indian_panel, rows[0]);

    let lower =
        Layout::horizontal([Constraint::Percentage(62), Constraint::Percentage(38)]).split(rows[1]);

    let tribe_rows: Vec<Row> = save
        .tribes
        .iter()
        .enumerate()
        .map(|(idx, tribe)| {
            Row::new(vec![
                Cell::from((idx + 1).to_string()),
                Cell::from(tribe.x.to_string()),
                Cell::from(tribe.y.to_string()),
                Cell::from(enum_name::<NationType>(tribe.nation_id)),
                Cell::from(tribe.population.to_string()),
                Cell::from(yes_no(tribe.blcs.capital)),
                Cell::from(yes_no(tribe.blcs.scouted)),
            ])
        })
        .collect();

    if save.tribes.is_empty() {
        state.select(None);
    } else if state.selected().is_none() {
        state.select(Some(0));
    } else if let Some(selected) = state.selected()
        && selected >= save.tribes.len()
    {
        state.select(Some(save.tribes.len() - 1));
    }

    let table = Table::new(
        tribe_rows,
        [
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(12),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
        ],
    )
    .header(Row::new(vec!["#", "X", "Y", "Nation", "Pop", "Cap", "Scout"]).style(theme::header()))
    .block(
        Block::bordered()
            .title(" Tribe Dwellings ")
            .border_style(theme::border()),
    )
    .row_highlight_style(theme::highlight());
    frame.render_stateful_widget(table, lower[0], state);

    let detail = state
        .selected()
        .and_then(|selected| save.tribes.get(selected))
        .map_or_else(no_tribes_detail, tribe_detail);
    let detail_paragraph = Paragraph::new(detail).style(theme::base()).block(
        Block::bordered()
            .title(" Dwelling Detail ")
            .border_style(theme::border()),
    );
    frame.render_widget(detail_paragraph, lower[1]);
}

fn indian_nations_text(save: &SaveFile) -> Text<'static> {
    let mut lines = Vec::new();

    for (idx, indian) in save.indians.iter().enumerate() {
        let name = INDIAN_NAMES.get(idx).copied().unwrap_or("Unknown");
        let alarms = indian
            .alarm_by_player
            .iter()
            .map(u16::to_string)
            .collect::<Vec<_>>()
            .join("/");
        lines.push(Line::from(vec![Span::styled(
            name.to_string(),
            theme::header(),
        )]));
        lines.push(Line::from(format!(
            "Capitol: ({}, {})  Tech: {}  Extinct: {}",
            indian.capitol_x,
            indian.capitol_y,
            enum_name::<TechType>(indian.tech),
            yes_no(indian.tribe_flags.extinct)
        )));
        lines.push(Line::from(format!(
            "Muskets: {}  Horse Herds: {}  Alarm(E/F/S/N): {}",
            indian.muskets, indian.horse_herds, alarms
        )));
        if idx + 1 < save.indians.len() {
            lines.push(Line::from(""));
        }
    }

    Text::from(lines)
}

fn tribe_detail(tribe: &Tribe) -> Text<'static> {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            format!(
                "{} at ({}, {})",
                enum_name::<NationType>(tribe.nation_id),
                tribe.x,
                tribe.y
            ),
            theme::header(),
        )]),
        Line::from(format!("Population: {}", tribe.population)),
        Line::from(format!("Capital: {}", yes_no(tribe.blcs.capital))),
        Line::from(format!("Scouted: {}", yes_no(tribe.blcs.scouted))),
        Line::from(format!(
            "Mission: nation={} expert={}",
            tribe.mission.nation_id,
            yes_no(tribe.mission.expert)
        )),
        Line::from(format!("Growth Counter: {}", tribe.growth_counter)),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Alarm by European power",
            theme::header(),
        )]),
    ];

    for (idx, alarm) in tribe.alarm.iter().enumerate() {
        lines.push(Line::from(format!(
            "{}: friction={} attacks={}",
            idx + 1,
            alarm.friction,
            alarm.attacks
        )));
    }

    Text::from(lines)
}

fn no_tribes_detail() -> Text<'static> {
    Text::from(vec![
        Line::from(vec![Span::styled("Tribe Dwellings", theme::header())]),
        Line::from(""),
        Line::from("No tribe dwellings in this save."),
    ])
}

fn enum_name<T>(value: u8) -> String
where
    T: TryFrom<u8, Error = u8> + ToString,
{
    T::try_from(value).map_or_else(|v| format!("Unknown(0x{v:02X})"), |v| v.to_string())
}

fn yes_no(value: bool) -> &'static str {
    if value { "Yes" } else { "No" }
}
