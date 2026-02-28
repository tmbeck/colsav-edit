use colonization_sav::raw::Unit;
use colonization_sav::{CargoType, NationType, OrdersType, ProfessionType, SaveFile, UnitType};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, TableState};

use super::theme;

fn enum_name<T>(value: u8) -> String
where
    T: TryFrom<u8, Error = u8> + ToString,
{
    T::try_from(value)
        .map(|v| v.to_string())
        .unwrap_or_else(|v| format!("Unknown(0x{v:02X})"))
}

pub fn render(frame: &mut Frame, area: Rect, save: &SaveFile, state: &mut TableState) {
    let chunks = Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).split(area);

    let rows: Vec<Row> = save
        .units
        .iter()
        .enumerate()
        .map(|(idx, unit)| {
            Row::new(vec![
                Cell::from((idx + 1).to_string()),
                Cell::from(enum_name::<UnitType>(unit.unit_type)),
                Cell::from(enum_name::<NationType>(unit.nation_id)),
                Cell::from(format!("{},{}", unit.x, unit.y)),
                Cell::from(enum_name::<OrdersType>(unit.orders)),
            ])
        })
        .collect();

    if state.selected().is_none() && !rows.is_empty() {
        state.select(Some(0));
    }

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Length(16),
            Constraint::Length(14),
            Constraint::Length(9),
            Constraint::Length(13),
        ],
    )
    .header(Row::new(vec!["#", "Type", "Nation", "Location", "Orders"]).style(theme::header()))
    .block(
        Block::bordered()
            .title(" Units ")
            .border_style(theme::border()),
    )
    .row_highlight_style(theme::highlight());

    frame.render_stateful_widget(table, chunks[0], state);

    let selected = state.selected().unwrap_or(0);
    let detail = save.units.get(selected).map(unit_detail).unwrap_or_default();
    let detail_paragraph = Paragraph::new(detail)
        .style(theme::base())
        .block(
            Block::bordered()
                .title(" Unit Detail ")
                .border_style(theme::border()),
        );
    frame.render_widget(detail_paragraph, chunks[1]);
}

fn unit_detail(unit: &Unit) -> Text<'static> {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            enum_name::<UnitType>(unit.unit_type),
            theme::header(),
        )]),
        Line::from(format!("Nation: {}", enum_name::<NationType>(unit.nation_id))),
        Line::from(format!("Position: ({}, {})", unit.x, unit.y)),
        Line::from(format!("Orders: {}", enum_name::<OrdersType>(unit.orders))),
        Line::from(format!(
            "Profession: {}",
            enum_name::<ProfessionType>(unit.profession_or_treasure)
        )),
        Line::from(format!("Moves: {}", unit.moves)),
        Line::from(format!("Destination: ({}, {})", unit.goto_x, unit.goto_y)),
        Line::from(""),
        Line::from(vec![Span::styled("Cargo", theme::header())]),
    ];

    let cargo_count = usize::min(unit.holds_occupied as usize, unit.cargo_items.len());
    if cargo_count == 0 {
        lines.push(Line::from("None"));
    } else {
        for i in 0..cargo_count {
            lines.push(Line::from(format!(
                "Hold {}: {} {}",
                i + 1,
                enum_name::<CargoType>(unit.cargo_items[i]),
                unit.cargo_hold[i]
            )));
        }
    }

    Text::from(lines)
}
