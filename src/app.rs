use std::time::{Duration, Instant};

use crate::words::generate_words;
use crate::popup::WordListPopup;

pub struct App {
    target: String,
    input: String,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    time_limit: Option<Duration>,
    pub scheme_index: usize,
    pub cursor_style_index: usize,
    pub word_list_popup: Option<WordListPopup>,
}

impl App {
    pub fn new(word_count: usize, time_limit_seconds: Option<usize>) -> Self {
        let target = generate_words(word_count, Some("english.json"));

        Self {
            target: target.join(" "),
            input: String::new(),
            start_time: None,
            end_time: None,
            time_limit: time_limit_seconds.map(|s| Duration::from_secs(s as u64)),
            scheme_index: 0,
            cursor_style_index: 0,
            word_list_popup: None,
        }
    }

    pub fn restart(&mut self) {
        let word_count = self.target.split_whitespace().count();
        self.target = generate_words(word_count, Some("english.json")).join(" ");
        self.input.clear();
        self.start_time = None;
        self.end_time = None;
    }

    pub fn is_done(&self) -> bool {
        let length_complete = self.input.len() >= self.target.len();
        let time_complete = self
            .time_limit
            .and_then(|limit| self.start_time.map(|start| start.elapsed() >= limit))
            .unwrap_or(false);

        length_complete || time_complete
    }

    pub fn handle_char(&mut self, ch: char) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        if self.is_done() || self.input.len() >= self.target.len() {
            return;
        }

        if ch == ' ' {
            self.handle_space();
        } else {
            self.input.push(ch);
        }

        // Set end time when test is completed
        if self.is_done() && self.end_time.is_none() {
            self.end_time = Some(Instant::now());
        }
    }

    pub fn handle_backspace(&mut self) {
        if !self.input.is_empty() {
            self.input.pop();
        }
    }

    pub fn handle_ctrl_backspace(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let mut chars: Vec<char> = self.input.chars().collect();

        // Remove trailing spaces
        while chars.last() == Some(&' ') {
            chars.pop();
        }

        // Remove characters until we hit a space or beginning (delete whole word)
        while !chars.is_empty() && chars.last() != Some(&' ') {
            chars.pop();
        }

        self.input = chars.into_iter().collect();
    }

    pub fn cycle_color_scheme(&mut self) {
        self.scheme_index = self.scheme_index.wrapping_add(1);
    }

    pub fn cycle_cursor_style(&mut self) {
        self.cursor_style_index = self.cursor_style_index.wrapping_add(1);
    }

    pub fn wpm(&self) -> f64 {
        if let Some(start) = self.start_time {
            let elapsed = if let Some(end) = self.end_time {
                // Use the time when test completed, not current time
                end.duration_since(start).as_secs_f64()
            } else {
                // Test still in progress, use current time
                start.elapsed().as_secs_f64()
            };

            if elapsed > 0.0 {
                (self.input.len() as f64 / 5.0) / (elapsed / 60.0)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.input.is_empty() {
            100.0
        } else {
            // Count only characters that were actually typed (not '#')
            let mut typed_count = 0;
            let mut correct_count = 0;

            for (i, input_ch) in self.input.chars().enumerate() {
                if input_ch != '#' {
                    typed_count += 1;
                    if let Some(target_ch) = self.target.chars().nth(i) {
                        if input_ch == target_ch {
                            correct_count += 1;
                        }
                    }
                }
            }

            if typed_count == 0 {
                100.0
            } else {
                (correct_count as f64 / typed_count as f64) * 100.0
            }
        }
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn toggle_word_list_popup(&mut self) {
        if self.word_list_popup.is_some() {
            self.word_list_popup = None;
        } else {
            let word_lists = vec!["english.json".to_string(), "english_10k.json".to_string()];
            self.word_list_popup = Some(WordListPopup::new(word_lists));
        }
    }

    pub fn handle_popup_key(&mut self, key_code: crossterm::event::KeyCode) -> bool {
        if let Some(popup) = &mut self.word_list_popup {
            match key_code {
                crossterm::event::KeyCode::Up => popup.previous(),
                crossterm::event::KeyCode::Down => popup.next(),
                crossterm::event::KeyCode::Enter => {
                    let selected_list = &popup.word_lists[popup.selected];
                    let word_count = self.target.split_whitespace().count();
                    self.target = generate_words(word_count, Some(selected_list)).join(" ");
                    self.input.clear();
                    self.start_time = None;
                    self.end_time = None;
                    self.word_list_popup = None;
                }
                crossterm::event::KeyCode::Esc => {
                    self.word_list_popup = None;
                }
                _ => return false,
            }
            true
        } else {
            false
        }
    }

    fn handle_space(&mut self) {
        let pos = self.input.len();

        if let Some(next_space_offset) = self.target[pos..].find(' ') {
            // Fill missing characters with '#' and add space
            for _ in 0..next_space_offset {
                self.input.push('#');
            }
            self.input.push(' ');
        } else {
            // Fill remaining characters with '#'
            let remaining = self.target.len() - pos;
            for _ in 0..remaining {
                self.input.push('#');
            }
        }
    }
}
