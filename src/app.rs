use std::time::{Duration, Instant};

use crate::words::{generate_words, words_to_text};

pub struct App {
    target: String,
    input: String,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    time_limit: Option<Duration>,
    pub scheme_index: usize,
    pub cursor_style_index: usize,
}

impl App {
    pub fn new(word_count: usize, time_limit_seconds: Option<usize>) -> Self {
        let target = words_to_text(generate_words(word_count));

        Self {
            target,
            input: String::new(),
            start_time: None,
            end_time: None,
            time_limit: time_limit_seconds.map(|s| Duration::from_secs(s as u64)),
            scheme_index: 0,
            cursor_style_index: 0,
        }
    }

    pub fn restart(&mut self) {
        let word_count = self.target.split_whitespace().count();
        self.target = words_to_text(generate_words(word_count));
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
