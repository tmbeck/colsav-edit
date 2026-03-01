use colonization_sav::{HillsRiver, SaveFile, TerrainType};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use super::theme;

pub fn render(frame: &mut Frame, area: Rect, save: &SaveFile, scroll: (u16, u16)) {
    let content_h = area.height.saturating_sub(2) as usize;
    let content_w = area.width.saturating_sub(2) as usize;
    let max_rows = save.tile_map.rows;
    let max_cols = save.tile_map.cols;
    let row_offset = usize::min(scroll.0 as usize, max_rows.saturating_sub(1));
    let col_offset = usize::min(scroll.1 as usize, max_cols.saturating_sub(1));

    let mut lines = Vec::with_capacity(content_h + 1);
    let end_row = usize::min(row_offset + content_h, max_rows);
    for row in row_offset..end_row {
        let mut spans = Vec::with_capacity(content_w);
        let end_col = usize::min(col_offset + content_w, max_cols);
        for col in col_offset..end_col {
            let tile = save.tile_map.get(row, col);
            let ch = terrain_char(tile);
            spans.push(Span::styled(ch.to_string(), tile_style(tile)));
        }
        lines.push(Line::from(spans));
    }

    lines.push(Line::from(format!(
        "Viewport row {row_offset} col {col_offset} (map {max_rows}x{max_cols})"
    )));

    let paragraph = Paragraph::new(Text::from(lines)).block(
        Block::bordered()
            .title(" Terrain Map ")
            .border_style(theme::border()),
    );
    frame.render_widget(paragraph, area);
}

fn terrain_base_char(terrain: TerrainType) -> char {
    match terrain {
        TerrainType::Ocean | TerrainType::SeaLane => ' ',
        TerrainType::Arctic => '#',
        TerrainType::Tundra => 't',
        TerrainType::Desert => 'd',
        TerrainType::Plains => 'p',
        TerrainType::Prairie => 'r',
        TerrainType::Grassland => 'g',
        TerrainType::Savannah => 's',
        TerrainType::Marsh => 'm',
        TerrainType::Swamp => 'w',
        TerrainType::TundraForest | TerrainType::TundraForestW => 'T',
        TerrainType::DesertForest | TerrainType::DesertForestW => 'D',
        TerrainType::PlainsForest | TerrainType::PlainsForestW => 'P',
        TerrainType::PrairieForest | TerrainType::PrairieForestW => 'R',
        TerrainType::GrasslandForest | TerrainType::GrasslandForestW => 'G',
        TerrainType::SavannahForest | TerrainType::SavannahForestW => 'S',
        TerrainType::MarshForest | TerrainType::MarshForestW => 'M',
        TerrainType::SwampForest | TerrainType::SwampForestW => 'W',
    }
}

fn terrain_char(tile: u8) -> char {
    let terrain_raw = (tile >> 3) & 0x1F;
    let hills_river_raw = tile & 0x07;

    let Ok(terrain) = TerrainType::try_from(terrain_raw) else {
        return '?';
    };

    if matches!(terrain, TerrainType::Ocean | TerrainType::SeaLane) {
        return ' ';
    }

    match HillsRiver::try_from(hills_river_raw) {
        Ok(HillsRiver::River | HillsRiver::MajorRiver | HillsRiver::RiverHills) => '~',
        Ok(HillsRiver::Mountains) => '^',
        Ok(HillsRiver::Hills) => 'h',
        _ => terrain_base_char(terrain),
    }
}

fn tile_style(tile: u8) -> Style {
    let terrain_raw = (tile >> 3) & 0x1F;
    let hills_river_raw = tile & 0x07;
    let terrain = TerrainType::try_from(terrain_raw).ok();
    let hills = HillsRiver::try_from(hills_river_raw).ok();

    if matches!(hills, Some(HillsRiver::Mountains)) {
        return Style::default().fg(Color::Gray).bg(theme::BG);
    }
    if matches!(
        hills,
        Some(HillsRiver::River | HillsRiver::MajorRiver | HillsRiver::RiverHills)
    ) {
        return Style::default().fg(Color::Blue).bg(theme::BG);
    }

    match terrain {
        Some(TerrainType::Ocean | TerrainType::SeaLane) => {
            Style::default().fg(Color::Blue).bg(theme::BG)
        }
        Some(TerrainType::Arctic) => Style::default().fg(Color::White).bg(theme::BG),
        Some(TerrainType::Tundra | TerrainType::TundraForest | TerrainType::TundraForestW) => {
            Style::default().fg(Color::LightBlue).bg(theme::BG)
        }
        Some(TerrainType::Desert | TerrainType::DesertForest | TerrainType::DesertForestW) => {
            Style::default().fg(Color::Yellow).bg(theme::BG)
        }
        Some(
            TerrainType::Plains
            | TerrainType::Prairie
            | TerrainType::Grassland
            | TerrainType::Savannah,
        ) => Style::default().fg(Color::LightYellow).bg(theme::BG),
        Some(
            TerrainType::Marsh
            | TerrainType::Swamp
            | TerrainType::PlainsForest
            | TerrainType::PrairieForest
            | TerrainType::GrasslandForest
            | TerrainType::SavannahForest
            | TerrainType::MarshForest
            | TerrainType::SwampForest
            | TerrainType::PlainsForestW
            | TerrainType::PrairieForestW
            | TerrainType::GrasslandForestW
            | TerrainType::SavannahForestW
            | TerrainType::MarshForestW
            | TerrainType::SwampForestW,
        ) => Style::default().fg(Color::Green).bg(theme::BG),
        None => Style::default().fg(Color::Red).bg(theme::BG),
    }
}
