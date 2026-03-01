use colonization_sav::goods::GOODS_NAMES;
use colonization_sav::raw::Nation;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use super::app::{App, EditField, InputMode};
use super::theme;

const NATION_NAMES: [&str; 4] = ["England", "France", "Spain", "Netherlands"];

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let rows =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    let top =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);
    let bottom =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);

    let panels = [top[0], top[1], bottom[0], bottom[1]];
    for (idx, panel) in panels.iter().enumerate() {
        render_nation_panel(frame, *panel, app, idx);
    }
}

fn render_nation_panel(frame: &mut Frame, area: Rect, app: &App, idx: usize) {
    let nation = &app.save.nations[idx];
    let mut lines = Vec::new();

    let gold_label = editable_line(app, idx, 0, "Gold", nation.gold.to_string());
    let tax_label = editable_line(app, idx, 1, "Tax Rate", format!("{}%", nation.tax_rate));

    lines.push(gold_label);
    lines.push(tax_label);
    lines.push(Line::from(format!(
        "Liberty Bells: {}",
        nation.liberty_bells_total
    )));
    lines.push(Line::from(format!(
        "Founding Fathers: {}",
        nation.founding_father_count
    )));
    lines.push(Line::from(format!(
        "Rebel Sentiment: {}",
        nation.rebel_sentiment
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "Trade Prices",
        theme::header(),
    )]));
    for (gidx, goods) in GOODS_NAMES.iter().enumerate() {
        lines.push(Line::from(format!(
            "{goods:<11}: {}",
            nation.trade.euro_price[gidx]
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "Founding Fathers",
        theme::header(),
    )]));
    for father in founding_fathers(nation) {
        lines.push(Line::from(format!("- {father}")));
    }

    let title = format!(" {} {} ", idx + 1, NATION_NAMES[idx]);
    let style = if idx == app.nation_selected {
        theme::border().add_modifier(Modifier::BOLD)
    } else {
        theme::border()
    };
    let paragraph = Paragraph::new(Text::from(lines))
        .style(theme::base())
        .block(Block::bordered().title(title).border_style(style));
    frame.render_widget(paragraph, area);
}

fn editable_line(
    app: &App,
    nation_idx: usize,
    field_idx: usize,
    label: &str,
    value: String,
) -> Line<'static> {
    let selected = app.nation_selected == nation_idx && app.nation_field_selected == field_idx;
    let is_editing = match app.edit_field {
        Some(EditField::NationGold(i)) | Some(EditField::NationTax(i)) => i == nation_idx,
        None => false,
    };

    let text = if is_editing && matches!(app.input_mode, InputMode::Editing) {
        format!("{label}: {}_", app.edit_buffer)
    } else {
        format!("{label}: {value}")
    };

    if selected {
        Line::from(vec![Span::styled(text, theme::highlight())])
    } else {
        Line::from(text)
    }
}

pub fn edit_cursor_position(area: Rect, app: &App) -> Option<(u16, u16)> {
    if !matches!(app.input_mode, InputMode::Editing) {
        return None;
    }

    let rows =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    let top =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);
    let bottom =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);
    let panels = [top[0], top[1], bottom[0], bottom[1]];
    let panel = *panels.get(app.nation_selected)?;

    let field_line = u16::from(app.nation_field_selected != 0);
    let label_len: u16 = if app.nation_field_selected == 0 {
        6
    } else {
        10
    };
    let x = panel
        .x
        .saturating_add(1)
        .saturating_add(label_len)
        .saturating_add(app.edit_buffer.len() as u16);
    let y = panel.y.saturating_add(1).saturating_add(field_line);
    Some((x, y))
}

fn founding_fathers(nation: &Nation) -> Vec<&'static str> {
    let ff = nation.founding_fathers;
    let mut list = Vec::new();
    if ff.adam_smith {
        list.push("Adam Smith");
    }
    if ff.jakob_fugger {
        list.push("Jakob Fugger");
    }
    if ff.peter_minuit {
        list.push("Peter Minuit");
    }
    if ff.peter_stuyvesant {
        list.push("Peter Stuyvesant");
    }
    if ff.jan_de_witt {
        list.push("Jan de Witt");
    }
    if ff.ferdinand_magellan {
        list.push("Ferdinand Magellan");
    }
    if ff.francisco_coronado {
        list.push("Francisco Coronado");
    }
    if ff.hernando_de_soto {
        list.push("Hernando de Soto");
    }
    if ff.henry_hudson {
        list.push("Henry Hudson");
    }
    if ff.sieur_de_la_salle {
        list.push("Sieur de La Salle");
    }
    if ff.hernan_cortes {
        list.push("Hernan Cortes");
    }
    if ff.george_washington {
        list.push("George Washington");
    }
    if ff.paul_revere {
        list.push("Paul Revere");
    }
    if ff.francis_drake {
        list.push("Francis Drake");
    }
    if ff.john_paul_jones {
        list.push("John Paul Jones");
    }
    if ff.thomas_jefferson {
        list.push("Thomas Jefferson");
    }
    if ff.pocahontas {
        list.push("Pocahontas");
    }
    if ff.thomas_paine {
        list.push("Thomas Paine");
    }
    if ff.simon_bolivar {
        list.push("Simon Bolivar");
    }
    if ff.benjamin_franklin {
        list.push("Benjamin Franklin");
    }
    if ff.william_brewster {
        list.push("William Brewster");
    }
    if ff.william_penn {
        list.push("William Penn");
    }
    if ff.jean_de_brebeuf {
        list.push("Jean de Brebeuf");
    }
    if ff.juan_de_sepulveda {
        list.push("Juan de Sepulveda");
    }
    if ff.bartolme_de_las_casas {
        list.push("Bartolme de las Casas");
    }

    if list.is_empty() {
        list.push("None");
    }
    list
}
