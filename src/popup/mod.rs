use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

use crate::words::{languages, downloaded};

#[derive(Debug, Clone, PartialEq)]
pub enum PopupAction {
    None,
    Close,
    SelectWordList(String),
    SelectColorScheme(usize),
    SelectCursorStyle(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Section {
    WordList,
    ColorScheme,
    CursorStyle,
}

trait Filterable {
    fn filter(&self, query: &str) -> Vec<String>;
    fn len(&self) -> usize;
}

impl Filterable for Vec<String> {
    fn filter(&self, query: &str) -> Vec<String> {
        if query.is_empty() { return self.clone(); }
        self.iter().filter(|item| item.to_lowercase().contains(&query.to_lowercase())).cloned().collect()
    }
    fn len(&self) -> usize { self.len() }
}

impl Section {
    const ALL: [Section; 3] = [Section::WordList, Section::ColorScheme, Section::CursorStyle];
}

#[derive(Debug)]
pub struct PopupManager {
    is_open: bool,
    current_section: Section,
    word_list_selected: usize,
    word_list_visible_start: usize,
    color_scheme_selected: usize,
    cursor_style_selected: usize,
    word_lists: Vec<String>,
    color_schemes: Vec<String>,
    cursor_styles: Vec<String>,
    filter: String,
}

impl Default for PopupManager {
    fn default() -> Self {
        let downloaded_langs = downloaded();
        let mut available = languages();
        
        // Remove downloaded languages from available list
        available.retain(|lang| !downloaded_langs.contains(lang));
        
        // Combine default word lists with downloaded first, then available
        let mut word_lists = vec!["english.json".to_string(), "english_10k.json".to_string()];
        word_lists.extend(downloaded_langs);
        word_lists.extend(available);
        
        Self {
            is_open: false,
            current_section: Section::WordList,
            word_list_selected: 0,
            word_list_visible_start: 0,
            color_scheme_selected: 0,
            cursor_style_selected: 0,
            word_lists,
            color_schemes: vec!["gruvbox".to_string(), "dracula".to_string(), "nord".to_string(), "solarized".to_string()],
            cursor_styles: vec!["underline".to_string(), "block".to_string(), "default".to_string()],
            filter: String::new(),
        }
    }
}

impl PopupManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
    
    pub fn refresh_languages(&mut self) {
        let downloaded_langs = downloaded();
        let mut available = languages();
        
        // Remove downloaded languages from available list
        available.retain(|lang| !downloaded_langs.contains(lang));
        
        // Combine default word lists with downloaded first, then available
        let mut word_lists = vec!["english.json".to_string(), "english_10k.json".to_string()];
        word_lists.extend(downloaded_langs);
        word_lists.extend(available);
        
        self.word_lists = word_lists;
    }

    pub fn handle_key(&mut self, key: KeyCode) -> PopupAction {
        if !self.is_open {
            return PopupAction::None;
        }

        match key {
            KeyCode::Esc => {
                self.close();
                PopupAction::Close
            }
            KeyCode::Right => {
                self.next_section();
                self.filter.clear();
                PopupAction::None
            }
            KeyCode::Left => {
                self.prev_section();
                self.filter.clear();
                PopupAction::None
            }
            KeyCode::Up => {
                self.move_up();
                PopupAction::None
            }
            KeyCode::Down => {
                self.move_down();
                PopupAction::None
            }
            KeyCode::Enter => {
                let filtered = self.current_filtered();
                let action = match self.current_section {
                    Section::WordList => PopupAction::SelectWordList(filtered[self.word_list_selected].clone()),
                    Section::ColorScheme => PopupAction::SelectColorScheme(self.color_schemes.iter().position(|x| x == &filtered[self.color_scheme_selected]).unwrap_or(0)),
                    Section::CursorStyle => PopupAction::SelectCursorStyle(self.cursor_styles.iter().position(|x| x == &filtered[self.cursor_style_selected]).unwrap_or(0)),
                };
                self.close();
                action
            }
            KeyCode::Backspace => {
                self.filter.pop();
                self.reset_selection();
                PopupAction::None
            }
            KeyCode::Char(c) => {
                self.filter.push(c);
                self.reset_selection();
                PopupAction::None
            }
            _ => PopupAction::None,
        }
    }

    fn next_section(&mut self) {
        let current_index = Section::ALL.iter().position(|&t| t == self.current_section).unwrap();
        self.current_section = Section::ALL[(current_index + 1) % Section::ALL.len()];
    }

    fn prev_section(&mut self) {
        let current_index = Section::ALL.iter().position(|&t| t == self.current_section).unwrap();
        self.current_section = Section::ALL[(current_index + Section::ALL.len() - 1) % Section::ALL.len()];
    }

    fn move_up(&mut self) {
        match self.current_section {
            Section::WordList => {
                if self.word_list_selected > 0 {
                    self.word_list_selected -= 1;
                    self.update_word_list_scroll();
                }
            }
            Section::ColorScheme => {
                if self.color_scheme_selected > 0 {
                    self.color_scheme_selected -= 1;
                }
            }
            Section::CursorStyle => {
                if self.cursor_style_selected > 0 {
                    self.cursor_style_selected -= 1;
                }
            }
        }
    }

    fn reset_selection(&mut self) {
        match self.current_section {
            Section::WordList => {
                self.word_list_selected = 0;
                self.word_list_visible_start = 0;
            }
            Section::ColorScheme => self.color_scheme_selected = 0,
            Section::CursorStyle => self.cursor_style_selected = 0,
        }
    }

    fn move_down(&mut self) {
        let filtered_len = self.current_filtered().len();
        match self.current_section {
            Section::WordList => {
                if self.word_list_selected < filtered_len - 1 {
                    self.word_list_selected += 1;
                    self.update_word_list_scroll();
                }
            }
            Section::ColorScheme => {
                if self.color_scheme_selected < filtered_len - 1 {
                    self.color_scheme_selected += 1;
                }
            }
            Section::CursorStyle => {
                if self.cursor_style_selected < filtered_len - 1 {
                    self.cursor_style_selected += 1;
                }
            }
        }
    }

    fn current_filtered(&self) -> Vec<String> {
        match self.current_section {
            Section::WordList => self.word_lists.filter(&self.filter),
            Section::ColorScheme => self.color_schemes.filter(&self.filter),
            Section::CursorStyle => self.cursor_styles.filter(&self.filter),
        }
    }

    fn update_word_list_scroll(&mut self) {
        const VISIBLE_COUNT: usize = 5;
        if self.word_list_selected >= self.word_list_visible_start + VISIBLE_COUNT {
            self.word_list_visible_start = self.word_list_selected - VISIBLE_COUNT + 1;
        } else if self.word_list_selected < self.word_list_visible_start {
            self.word_list_visible_start = self.word_list_selected;
        }
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        if !self.is_open {
            return;
        }

        let popup_area = centered_rect(50, 12, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .title("Settings (←/→ to switch sections, ↑/↓ to navigate, Enter to select, Esc to close)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue));

        frame.render_widget(block, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
            .split(popup_area.inner(&Margin::new(1, 1)));

        self.render_word_list(frame, chunks[0]);
        self.render_color_scheme_list(frame, chunks[1]);
        self.render_cursor_style_list(frame, chunks[2]);
    }

    fn render_word_list<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let filtered = if matches!(self.current_section, Section::WordList) {
            self.word_lists.filter(&self.filter)
        } else {
            self.word_lists.clone()
        };
        let visible_items = &filtered[self.word_list_visible_start..(self.word_list_visible_start + 5).min(filtered.len())];
        let downloaded_langs = downloaded();
        let is_selected = matches!(self.current_section, Section::WordList);

        let items: Vec<ListItem> = visible_items.iter().enumerate().map(|(i, item)| {
            let actual_index = self.word_list_visible_start + i;
            let is_downloaded = downloaded_langs.contains(item);
            let style = if actual_index == self.word_list_selected && is_selected {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else if is_downloaded { Style::default().fg(Color::Green) } else { Style::default() };

            let display_name = item.trim_end_matches(".json");
            let text = if is_downloaded { format!("✓ {}", display_name) } else { display_name.to_string() };
            ListItem::new(Line::from(Span::styled(text, style)))
        }).collect();

        let title = if is_selected && !self.filter.is_empty() { format!("Word Lists ({}/{}) [{}]", self.word_list_selected + 1, filtered.len(), self.filter) } else { format!("Word Lists ({}/{})", self.word_list_selected + 1, filtered.len()) };
        frame.render_widget(List::new(items).block(Block::default().borders(Borders::ALL).title(title).border_style(if is_selected { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) })), area);
    }


    fn render_color_scheme_list<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let is_selected = matches!(self.current_section, Section::ColorScheme);
        let filtered = if is_selected {
            self.color_schemes.filter(&self.filter)
        } else {
            self.color_schemes.clone()
        };
        let items: Vec<ListItem> = filtered.iter().enumerate().map(|(i, scheme)| {
            let style = if i == self.color_scheme_selected && is_selected {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else { Style::default() };
            ListItem::new(Line::from(Span::styled(scheme, style)))
        }).collect();

        let title = if is_selected && !self.filter.is_empty() { format!("Color Schemes [{}]", self.filter) } else { "Color Schemes".to_string() };
        frame.render_widget(List::new(items).block(Block::default().borders(Borders::ALL).title(title).border_style(if is_selected { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) })), area);
    }

    fn render_cursor_style_list<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let is_selected = matches!(self.current_section, Section::CursorStyle);
        let filtered = if is_selected {
            self.cursor_styles.filter(&self.filter)
        } else {
            self.cursor_styles.clone()
        };
        let items: Vec<ListItem> = filtered.iter().enumerate().map(|(i, style)| {
            let style_config = if i == self.cursor_style_selected && is_selected {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else { Style::default() };
            ListItem::new(Line::from(Span::styled(style, style_config)))
        }).collect();

        let title = if is_selected && !self.filter.is_empty() { format!("Cursor Styles [{}]", self.filter) } else { "Cursor Styles".to_string() };
        frame.render_widget(List::new(items).block(Block::default().borders(Borders::ALL).title(title).border_style(if is_selected { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) })), area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}