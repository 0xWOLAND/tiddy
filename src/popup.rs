use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

pub struct WordListPopup {
    pub selected: usize,
    pub word_lists: Vec<String>,
}

impl WordListPopup {
    pub fn new(word_lists: Vec<String>) -> Self {
        Self {
            selected: 0,
            word_lists,
        }
    }

    pub fn next(&mut self) {
        if self.selected < self.word_lists.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let popup_area = centered_rect(50, 30, area);

        frame.render_widget(Clear, popup_area);

        let items: Vec<ListItem> = self
            .word_lists
            .iter()
            .enumerate()
            .map(|(i, list)| {
                let style = if i == self.selected {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                };

                let name = list.trim_end_matches(".json");
                ListItem::new(Line::from(Span::styled(name, style)))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Select Word List (↑/↓ to navigate, Enter to select, Esc to cancel)")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)),
            )
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        frame.render_widget(list, popup_area);
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
