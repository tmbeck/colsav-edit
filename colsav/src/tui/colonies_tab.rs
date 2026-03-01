use colonization_sav::goods::GOODS_NAMES;
use colonization_sav::raw::Colony;
use colonization_sav::{NationType, SaveFile};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, TableState};

use super::theme;

fn enum_name<T>(value: u8) -> String
where
    T: TryFrom<u8, Error = u8> + ToString,
{
    T::try_from(value).map_or_else(|v| format!("Unknown(0x{v:02X})"), |v| v.to_string())
}

pub fn render(frame: &mut Frame, area: Rect, save: &SaveFile, state: &mut TableState) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).split(area);

    let rows: Vec<Row> = save
        .colonies
        .iter()
        .enumerate()
        .map(|(idx, colony)| {
            Row::new(vec![
                Cell::from((idx + 1).to_string()),
                Cell::from(colony.name().to_string()),
                Cell::from(enum_name::<NationType>(colony.nation_id)),
                Cell::from(colony.population.to_string()),
                Cell::from(format!("{},{}", colony.x, colony.y)),
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
            Constraint::Length(20),
            Constraint::Length(14),
            Constraint::Length(5),
            Constraint::Length(9),
        ],
    )
    .header(Row::new(vec!["#", "Name", "Nation", "Pop", "Location"]).style(theme::header()))
    .block(
        Block::bordered()
            .title(" Colonies ")
            .border_style(theme::border()),
    )
    .row_highlight_style(theme::highlight());

    frame.render_stateful_widget(table, chunks[0], state);

    let selected = state.selected().unwrap_or(0);
    let detail = save
        .colonies
        .get(selected)
        .map(colony_detail)
        .unwrap_or_default();
    let detail_paragraph = Paragraph::new(detail).style(theme::base()).block(
        Block::bordered()
            .title(" Colony Detail ")
            .border_style(theme::border()),
    );
    frame.render_widget(detail_paragraph, chunks[1]);
}

fn colony_detail(colony: &Colony) -> Text<'static> {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            colony.name().to_string(),
            theme::header(),
        )]),
        Line::from(format!(
            "Nation: {}",
            enum_name::<NationType>(colony.nation_id)
        )),
        Line::from(format!("Position: ({}, {})", colony.x, colony.y)),
        Line::from(format!("Population: {}", colony.population)),
        Line::from(format!("Hammers: {}", colony.hammers)),
        Line::from(format!(
            "Constructing: {}",
            constructable_name(colony.building_in_production)
        )),
        Line::from(""),
        Line::from(vec![Span::styled("Storage", theme::header())]),
    ];

    for (idx, goods_name) in GOODS_NAMES.iter().enumerate() {
        lines.push(Line::from(format!(
            "{goods_name:<11}: {:>6}",
            colony.stock[idx]
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled("Buildings", theme::header())]));
    for name in building_names(&colony.buildings) {
        lines.push(Line::from(format!("- {name}")));
    }

    Text::from(lines)
}

fn building_names(buildings: &colonization_sav::raw::colony::Buildings) -> Vec<String> {
    let mut names = Vec::new();

    if buildings.fortification > 0 {
        names.push(
            match buildings.fortification {
                1 => "Stockade",
                2 => "Fort",
                3 => "Fortress",
                _ => "Fortification",
            }
            .to_string(),
        );
    }
    if buildings.armory > 0 {
        names.push(
            match buildings.armory {
                1 => "Armory",
                2 => "Magazine",
                3 => "Arsenal",
                _ => "Armory",
            }
            .to_string(),
        );
    }
    if buildings.docks > 0 {
        names.push(
            match buildings.docks {
                1 => "Docks",
                2 => "Drydock",
                3 => "Shipyard",
                _ => "Docks",
            }
            .to_string(),
        );
    }
    if buildings.town_hall > 0 {
        names.push(
            match buildings.town_hall {
                1 => "Town Hall",
                2 => "Town Hall",
                3 => "Town Hall",
                _ => "Town Hall",
            }
            .to_string(),
        );
    }
    if buildings.schoolhouse > 0 {
        names.push(
            match buildings.schoolhouse {
                1 => "Schoolhouse",
                2 => "College",
                3 => "University",
                _ => "Schoolhouse",
            }
            .to_string(),
        );
    }
    if buildings.warehouse {
        names.push("Warehouse".to_string());
    }
    if buildings.stables {
        names.push("Stables".to_string());
    }
    if buildings.custom_house {
        names.push("Custom House".to_string());
    }
    if buildings.printing_press > 0 {
        names.push(
            match buildings.printing_press {
                1 => "Printing Press",
                2 => "Newspaper",
                _ => "Printing Press",
            }
            .to_string(),
        );
    }
    if buildings.weavers_house > 0 {
        names.push(
            match buildings.weavers_house {
                1 => "Weaver's House",
                2 => "Weaver's Shop",
                3 => "Textile Mill",
                _ => "Weaver's House",
            }
            .to_string(),
        );
    }
    if buildings.tobacconists_house > 0 {
        names.push(
            match buildings.tobacconists_house {
                1 => "Tobacconist's House",
                2 => "Tobacconist's Shop",
                3 => "Cigar Factory",
                _ => "Tobacconist's House",
            }
            .to_string(),
        );
    }
    if buildings.rum_distillers_house > 0 {
        names.push(
            match buildings.rum_distillers_house {
                1 => "Rum Distiller's House",
                2 => "Rum Distiller's Shop",
                3 => "Rum Factory",
                _ => "Rum Distiller's House",
            }
            .to_string(),
        );
    }
    if buildings.fur_traders_house > 0 {
        names.push(
            match buildings.fur_traders_house {
                1 => "Fur Trader's House",
                2 => "Fur Trading Post",
                3 => "Fur Factory",
                _ => "Fur Trader's House",
            }
            .to_string(),
        );
    }
    if buildings.carpenters_shop > 0 {
        names.push(
            match buildings.carpenters_shop {
                1 => "Carpenter's Shop",
                2 => "Lumber Mill",
                _ => "Carpenter's Shop",
            }
            .to_string(),
        );
    }
    if buildings.church > 0 {
        names.push(
            match buildings.church {
                1 => "Church",
                2 => "Cathedral",
                _ => "Church",
            }
            .to_string(),
        );
    }
    if buildings.blacksmiths_house > 0 {
        names.push(
            match buildings.blacksmiths_house {
                1 => "Blacksmith's House",
                2 => "Blacksmith's Shop",
                3 => "Iron Works",
                _ => "Blacksmith's House",
            }
            .to_string(),
        );
    }

    if names.is_empty() {
        names.push("None".to_string());
    }

    names
}

fn constructable_name(value: u8) -> String {
    match value {
        0x00 => "Stockade".to_string(),
        0x01 => "Fort".to_string(),
        0x02 => "Fortress".to_string(),
        0x03 => "Armory".to_string(),
        0x04 => "Magazine".to_string(),
        0x05 => "Arsenal".to_string(),
        0x06 => "Docks".to_string(),
        0x07 => "Drydock".to_string(),
        0x08 => "Shipyard".to_string(),
        0x09 => "Town Hall".to_string(),
        0x0C => "Schoolhouse".to_string(),
        0x0D => "College".to_string(),
        0x0E => "University".to_string(),
        0x0F => "Warehouse".to_string(),
        0x10 => "Warehouse Expansion".to_string(),
        0x11 => "Stable".to_string(),
        0x12 => "Custom House".to_string(),
        0x13 => "Printing Press".to_string(),
        0x14 => "Newspaper".to_string(),
        0x15 => "Weaver's House".to_string(),
        0x16 => "Weaver's Shop".to_string(),
        0x17 => "Textile Mill".to_string(),
        0x18 => "Tobacconist's House".to_string(),
        0x19 => "Tobacconist's Shop".to_string(),
        0x1A => "Cigar Factory".to_string(),
        0x1B => "Rum Distiller's House".to_string(),
        0x1C => "Rum Distiller's Shop".to_string(),
        0x1D => "Rum Factory".to_string(),
        0x20 => "Fur Trader's House".to_string(),
        0x21 => "Fur Trading Post".to_string(),
        0x22 => "Fur Factory".to_string(),
        0x23 => "Carpenter's Shop".to_string(),
        0x24 => "Lumber Mill".to_string(),
        0x25 => "Church".to_string(),
        0x26 => "Cathedral".to_string(),
        0x27 => "Blacksmith's House".to_string(),
        0x28 => "Blacksmith's Shop".to_string(),
        0x29 => "Iron Works".to_string(),
        0x2A => "Artillery".to_string(),
        0x2B => "Wagon Train".to_string(),
        0x2C => "Caravel".to_string(),
        0x2D => "Merchantman".to_string(),
        0x2E => "Galleon".to_string(),
        0x2F => "Privateer".to_string(),
        0x30 => "Frigate".to_string(),
        0xFF => "Nothing".to_string(),
        v => format!("Unknown(0x{v:02X})"),
    }
}
