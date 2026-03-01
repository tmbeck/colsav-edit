use colonization_sav::SaveFile;
use colonization_sav::goods::GOODS_NAMES;
use colonization_sav::raw::trade_route::TradeRoute;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, TableState};

use super::theme;

pub fn render(frame: &mut Frame, area: Rect, save: &SaveFile, state: &mut TableState) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).split(area);

    let route_indices: Vec<usize> = save
        .trade_routes
        .iter()
        .enumerate()
        .filter_map(|(idx, route)| {
            if route.name_raw.iter().all(|&b| b == 0) {
                None
            } else {
                Some(idx)
            }
        })
        .collect();

    if route_indices.is_empty() {
        state.select(None);
    } else if state.selected().is_none() {
        state.select(Some(0));
    } else if let Some(selected) = state.selected()
        && selected >= route_indices.len()
    {
        state.select(Some(route_indices.len() - 1));
    }

    let rows: Vec<Row> = route_indices
        .iter()
        .map(|&route_idx| {
            let route = &save.trade_routes[route_idx];
            Row::new(vec![
                Cell::from((route_idx + 1).to_string()),
                Cell::from(route.name().to_string()),
                Cell::from(route_type_name(route.land_or_sea)),
                Cell::from(route.stops_count.to_string()),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Length(26),
            Constraint::Length(8),
            Constraint::Length(7),
        ],
    )
    .header(Row::new(vec!["#", "Name", "Type", "Stops"]).style(theme::header()))
    .block(
        Block::bordered()
            .title(" Trade Routes ")
            .border_style(theme::border()),
    )
    .row_highlight_style(theme::highlight());
    frame.render_stateful_widget(table, chunks[0], state);

    let detail = state
        .selected()
        .and_then(|selected| route_indices.get(selected).copied())
        .and_then(|idx| save.trade_routes.get(idx))
        .map_or_else(no_routes_detail, route_detail);
    let detail_paragraph = Paragraph::new(detail).style(theme::base()).block(
        Block::bordered()
            .title(" Route Detail ")
            .border_style(theme::border()),
    );
    frame.render_widget(detail_paragraph, chunks[1]);
}

fn route_detail(route: &TradeRoute) -> Text<'static> {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            route.name().to_string(),
            theme::header(),
        )]),
        Line::from(format!("Type: {}", route_type_name(route.land_or_sea))),
        Line::from(format!("Stops: {}", route.stops_count)),
        Line::from(""),
        Line::from(vec![Span::styled("Stops", theme::header())]),
    ];

    let stops_len = usize::min(route.stops_count as usize, route.stops.len());
    if stops_len == 0 {
        lines.push(Line::from("None"));
    } else {
        for (idx, stop) in route.stops.iter().take(stops_len).enumerate() {
            lines.push(Line::from(format!(
                "{}: Colony {}",
                idx + 1,
                stop.colony_index
            )));
            lines.push(Line::from(format!(
                "  Load  : {}",
                cargo_list(stop.loads_cargo, stop.loads_count)
            )));
            lines.push(Line::from(format!(
                "  Unload: {}",
                cargo_list(stop.unloads_cargo, stop.unloads_count)
            )));
        }
    }

    Text::from(lines)
}

fn no_routes_detail() -> Text<'static> {
    Text::from(vec![
        Line::from(vec![Span::styled("Trade Routes", theme::header())]),
        Line::from(""),
        Line::from("No named trade routes in this save."),
    ])
}

fn route_type_name(value: u8) -> &'static str {
    match value {
        0 => "Land",
        1 => "Sea",
        _ => "Unknown",
    }
}

fn cargo_list(cargo: [u8; 6], count: u8) -> String {
    let used = usize::min(count as usize, cargo.len());
    if used == 0 {
        return "None".to_string();
    }

    cargo
        .iter()
        .take(used)
        .map(|&goods_idx| goods_name(goods_idx).to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn goods_name(goods_idx: u8) -> &'static str {
    GOODS_NAMES
        .get(goods_idx as usize)
        .copied()
        .unwrap_or("Unknown")
}
