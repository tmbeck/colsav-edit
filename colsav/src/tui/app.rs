use anyhow::Result;
use colonization_sav::SaveFile;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Clear, Paragraph, TableState, Tabs};

use super::colonies_tab;
use super::header_tab;
use super::map_tab;
use super::nations_tab;
use super::tabs::Tab;
use super::theme;
use super::trade_routes_tab;
use super::tribes_tab;
use super::units_tab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditField {
    NationGold(usize),
    NationTax(usize),
}

pub struct App {
    pub save: SaveFile,
    pub file_path: String,
    pub should_quit: bool,
    pub active_tab: Tab,
    pub input_mode: InputMode,
    pub show_help: bool,
    pub colony_table_state: TableState,
    pub unit_table_state: TableState,
    pub trade_route_table_state: TableState,
    pub tribe_table_state: TableState,
    pub nation_selected: usize,
    pub map_scroll: (u16, u16),
    pub edit_field: Option<EditField>,
    pub edit_buffer: String,
    pub status_message: String,
    pub dirty: bool,
    pub nation_field_selected: usize,
    pending_quit_confirm: bool,
}

impl App {
    pub fn new(save: SaveFile, path: String) -> Self {
        let mut colony_table_state = TableState::default();
        if !save.colonies.is_empty() {
            colony_table_state.select(Some(0));
        }

        let mut unit_table_state = TableState::default();
        if !save.units.is_empty() {
            unit_table_state.select(Some(0));
        }

        let mut trade_route_table_state = TableState::default();
        if save
            .trade_routes
            .iter()
            .any(|route| !route.name().is_empty())
        {
            trade_route_table_state.select(Some(0));
        }

        let mut tribe_table_state = TableState::default();
        if !save.tribes.is_empty() {
            tribe_table_state.select(Some(0));
        }

        Self {
            save,
            file_path: path,
            should_quit: false,
            active_tab: Tab::Header,
            input_mode: InputMode::Normal,
            show_help: false,
            colony_table_state,
            unit_table_state,
            trade_route_table_state,
            tribe_table_state,
            nation_selected: 0,
            map_scroll: (0, 0),
            edit_field: None,
            edit_buffer: String::new(),
            status_message: "Loaded save file".to_string(),
            dirty: false,
            nation_field_selected: 0,
            pending_quit_confirm: false,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(Block::default().style(theme::base()), area);

        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area);

        let tab_titles: Vec<Line> = Tab::titles()
            .iter()
            .map(|t| Line::from(format!("[{t}]")))
            .collect();
        let tabs = Tabs::new(tab_titles)
            .select(self.active_tab.index())
            .style(theme::tab_inactive())
            .highlight_style(theme::tab_active());
        frame.render_widget(tabs, chunks[0]);

        match self.active_tab {
            Tab::Header => header_tab::render(frame, chunks[1], &self.save),
            Tab::Colonies => {
                colonies_tab::render(frame, chunks[1], &self.save, &mut self.colony_table_state);
            }
            Tab::Units => {
                units_tab::render(frame, chunks[1], &self.save, &mut self.unit_table_state);
            }
            Tab::Nations => nations_tab::render(frame, chunks[1], self),
            Tab::TradeRoutes => {
                trade_routes_tab::render(
                    frame,
                    chunks[1],
                    &self.save,
                    &mut self.trade_route_table_state,
                );
            }
            Tab::Tribes => {
                tribes_tab::render(frame, chunks[1], &self.save, &mut self.tribe_table_state);
            }
            Tab::Map => map_tab::render(frame, chunks[1], &self.save, self.map_scroll),
        }

        let dirty_mark = if self.dirty { "*DIRTY*" } else { "clean" };
        let mode = match self.input_mode {
            InputMode::Normal => "NORMAL",
            InputMode::Editing => "EDIT",
        };
        let status = format!(
            "{} | {} | {} | {} | Tab/Shift+Tab Switch | 1-7 Tabs | e Edit | s/Ctrl+S Save | ? Help | q Quit",
            self.file_path, dirty_mark, mode, self.status_message
        );
        let status_bar = Paragraph::new(status).style(theme::status());
        frame.render_widget(status_bar, chunks[2]);

        if self.show_help {
            let popup_area = centered_rect(72, 46, area);
            let help_text = Paragraph::new(help_text())
                .style(theme::base())
                .alignment(Alignment::Left)
                .block(
                    Block::bordered()
                        .title(" Help ")
                        .border_style(theme::border()),
                );
            frame.render_widget(Clear, popup_area);
            frame.render_widget(help_text, popup_area);
        }

        if matches!(self.active_tab, Tab::Nations)
            && matches!(self.input_mode, InputMode::Editing)
            && self.edit_field.is_some()
            && let Some((x, y)) = nations_tab::edit_cursor_position(chunks[1], self)
        {
            frame.set_cursor_position((x, y));
        }
    }

    pub fn handle_events(&mut self) -> Result<bool> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(self.should_quit);
            }
            match self.input_mode {
                InputMode::Normal => self.handle_key_normal(key)?,
                InputMode::Editing => self.handle_key_editing(key),
            }
        }
        Ok(self.should_quit)
    }

    fn handle_key_normal(&mut self, key: KeyEvent) -> Result<()> {
        if self.show_help {
            match key.code {
                KeyCode::Esc | KeyCode::Char('?') => {
                    self.show_help = false;
                    self.status_message = "Closed help".to_string();
                }
                _ => {}
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Tab => self.active_tab = self.active_tab.next(),
            KeyCode::BackTab => self.active_tab = self.active_tab.prev(),
            KeyCode::Char('1') | KeyCode::F(1) => self.active_tab = Tab::Header,
            KeyCode::Char('2') | KeyCode::F(2) => self.active_tab = Tab::Colonies,
            KeyCode::Char('3') | KeyCode::F(3) => self.active_tab = Tab::Units,
            KeyCode::Char('4') | KeyCode::F(4) => self.active_tab = Tab::Nations,
            KeyCode::Char('5') | KeyCode::F(5) => self.active_tab = Tab::TradeRoutes,
            KeyCode::Char('6') | KeyCode::F(6) => self.active_tab = Tab::Tribes,
            KeyCode::Char('7') | KeyCode::F(7) => self.active_tab = Tab::Map,
            KeyCode::Char('?') => {
                self.show_help = true;
                self.pending_quit_confirm = false;
                self.status_message = "Showing help (? or Esc to close)".to_string();
            }
            KeyCode::Esc | KeyCode::Char('q') => self.request_quit(),
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.save_file()?;
            }
            KeyCode::Char('s') => self.save_file()?,
            KeyCode::Char('e') => self.start_edit(),
            KeyCode::Up | KeyCode::Char('k') => self.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.navigate_down(),
            KeyCode::Left | KeyCode::Char('h') => self.navigate_left(),
            KeyCode::Right | KeyCode::Char('l') => self.navigate_right(),
            KeyCode::Enter => self.start_edit(),
            _ => {}
        }
        Ok(())
    }

    fn handle_key_editing(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.edit_field = None;
                self.edit_buffer.clear();
                self.status_message = "Edit canceled".to_string();
            }
            KeyCode::Enter => {
                self.commit_edit();
            }
            KeyCode::Backspace => {
                self.edit_buffer.pop();
            }
            KeyCode::Char(c) if c.is_ascii_digit() || (self.edit_buffer.is_empty() && c == '-') => {
                self.edit_buffer.push(c);
            }
            _ => {}
        }
    }

    fn request_quit(&mut self) {
        if self.dirty && !self.pending_quit_confirm {
            self.pending_quit_confirm = true;
            self.status_message = "Unsaved changes. Press q or Esc again to quit.".to_string();
            return;
        }
        self.should_quit = true;
    }

    fn save_file(&mut self) -> Result<()> {
        self.save.save(&self.file_path)?;
        self.dirty = false;
        self.pending_quit_confirm = false;
        self.status_message = "Saved!".to_string();
        Ok(())
    }

    fn navigate_up(&mut self) {
        self.pending_quit_confirm = false;
        match self.active_tab {
            Tab::Colonies => {
                move_selection_up(&mut self.colony_table_state, self.save.colonies.len());
            }
            Tab::Units => move_selection_up(&mut self.unit_table_state, self.save.units.len()),
            Tab::TradeRoutes => {
                let route_len = self.named_trade_routes_len();
                move_selection_up(&mut self.trade_route_table_state, route_len);
            }
            Tab::Tribes => move_selection_up(&mut self.tribe_table_state, self.save.tribes.len()),
            Tab::Nations => {
                if self.nation_field_selected > 0 {
                    self.nation_field_selected -= 1;
                } else if self.nation_selected > 0 {
                    self.nation_selected -= 1;
                    self.nation_field_selected = 1;
                }
            }
            Tab::Map => {
                self.map_scroll.0 = self.map_scroll.0.saturating_sub(1);
            }
            Tab::Header => {}
        }
    }

    fn navigate_down(&mut self) {
        self.pending_quit_confirm = false;
        match self.active_tab {
            Tab::Colonies => {
                move_selection_down(&mut self.colony_table_state, self.save.colonies.len());
            }
            Tab::Units => move_selection_down(&mut self.unit_table_state, self.save.units.len()),
            Tab::TradeRoutes => {
                let route_len = self.named_trade_routes_len();
                move_selection_down(&mut self.trade_route_table_state, route_len);
            }
            Tab::Tribes => move_selection_down(&mut self.tribe_table_state, self.save.tribes.len()),
            Tab::Nations => {
                if self.nation_field_selected < 1 {
                    self.nation_field_selected += 1;
                } else if self.nation_selected < 3 {
                    self.nation_selected += 1;
                    self.nation_field_selected = 0;
                }
            }
            Tab::Map => {
                let max_row = self.save.tile_map.rows.saturating_sub(1) as u16;
                self.map_scroll.0 = self.map_scroll.0.saturating_add(1).min(max_row);
            }
            Tab::Header => {}
        }
    }

    fn navigate_left(&mut self) {
        self.pending_quit_confirm = false;
        match self.active_tab {
            Tab::Nations => {
                if self.nation_selected % 2 == 1 {
                    self.nation_selected -= 1;
                }
            }
            Tab::Map => {
                self.map_scroll.1 = self.map_scroll.1.saturating_sub(1);
            }
            _ => {}
        }
    }

    fn navigate_right(&mut self) {
        self.pending_quit_confirm = false;
        match self.active_tab {
            Tab::Nations => {
                if self.nation_selected.is_multiple_of(2) {
                    self.nation_selected = usize::min(self.nation_selected + 1, 3);
                }
            }
            Tab::Map => {
                let max_col = self.save.tile_map.cols.saturating_sub(1) as u16;
                self.map_scroll.1 = self.map_scroll.1.saturating_add(1).min(max_col);
            }
            _ => {}
        }
    }

    fn start_edit(&mut self) {
        self.pending_quit_confirm = false;
        if !matches!(self.active_tab, Tab::Nations) {
            return;
        }

        let nation_idx = self.nation_selected;
        self.edit_field = if self.nation_field_selected == 0 {
            Some(EditField::NationGold(nation_idx))
        } else {
            Some(EditField::NationTax(nation_idx))
        };

        self.edit_buffer = match self.edit_field {
            Some(EditField::NationGold(i)) => self.save.nations[i].gold.to_string(),
            Some(EditField::NationTax(i)) => self.save.nations[i].tax_rate.to_string(),
            None => String::new(),
        };
        self.input_mode = InputMode::Editing;
        self.status_message = "Editing value... Enter to confirm, Esc to cancel".to_string();
    }

    fn commit_edit(&mut self) {
        let Some(field) = self.edit_field else {
            self.input_mode = InputMode::Normal;
            return;
        };

        match field {
            EditField::NationGold(i) => match self.edit_buffer.parse::<i32>() {
                Ok(value) => {
                    self.save.nations[i].gold = value;
                    self.dirty = true;
                    self.status_message = format!("Updated {} gold to {}", nation_name(i), value);
                }
                Err(_) => {
                    self.status_message = "Invalid number for gold".to_string();
                }
            },
            EditField::NationTax(i) => match self.edit_buffer.parse::<u8>() {
                Ok(value) => {
                    self.save.nations[i].tax_rate = value;
                    self.dirty = true;
                    self.status_message = format!("Updated {} tax to {}", nation_name(i), value);
                }
                Err(_) => {
                    self.status_message = "Invalid number for tax rate".to_string();
                }
            },
        }

        self.pending_quit_confirm = false;
        self.input_mode = InputMode::Normal;
        self.edit_field = None;
        self.edit_buffer.clear();
    }

    fn named_trade_routes_len(&self) -> usize {
        self.save
            .trade_routes
            .iter()
            .filter(|route| !route.name().is_empty())
            .count()
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100_u16.saturating_sub(percent_y)) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100_u16.saturating_sub(percent_y)) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100_u16.saturating_sub(percent_x)) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100_u16.saturating_sub(percent_x)) / 2),
    ])
    .split(vertical[1])[1]
}

fn help_text() -> Text<'static> {
    Text::from(vec![
        Line::from(vec![Span::styled("Keybindings", theme::header())]),
        Line::from(""),
        Line::from("Tab / Shift-Tab : Switch tabs"),
        Line::from("1-7             : Jump to tab"),
        Line::from("e / Enter       : Edit field"),
        Line::from("s / Ctrl+S      : Save"),
        Line::from("Arrow keys/hjkl : Navigate"),
        Line::from("q / Esc         : Quit"),
        Line::from("?               : Toggle help"),
        Line::from(""),
        Line::from("Press ? or Esc to close this popup."),
    ])
}

fn move_selection_up(state: &mut TableState, len: usize) {
    if len == 0 {
        state.select(None);
        return;
    }
    let selected = state.selected().unwrap_or(0);
    let next = if selected == 0 { 0 } else { selected - 1 };
    state.select(Some(next));
}

fn move_selection_down(state: &mut TableState, len: usize) {
    if len == 0 {
        state.select(None);
        return;
    }
    let selected = state.selected().unwrap_or(0);
    let next = usize::min(selected + 1, len - 1);
    state.select(Some(next));
}

fn nation_name(idx: usize) -> &'static str {
    match idx {
        0 => "England",
        1 => "France",
        2 => "Spain",
        3 => "Netherlands",
        _ => "Unknown",
    }
}
