mod app;
mod cli;
mod ui;
mod words;

use std::io;

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal, ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or_default();
    let word_count = command.word_count();
    let time_limit = command.time_limit();

    // Setup terminal
    terminal::enable_raw_mode()?;
    io::stdout().execute(terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let result = run_typing_test(&mut terminal, word_count, time_limit);

    // Cleanup terminal
    io::stdout().execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}

fn run_typing_test(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    word_count: usize,
    time_limit: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(word_count, time_limit);
    let mut restart_timer: Option<std::time::Instant> = None;

    loop {
        // Calculate countdown for display
        let countdown = if let Some(start_time) = restart_timer {
            let elapsed = start_time.elapsed().as_secs();
            if elapsed < 3 {
                Some(3 - elapsed)
            } else {
                None
            }
        } else {
            None
        };

        terminal.draw(|frame| {
            ui::render_typing_test(
                frame,
                app.target(),
                app.input(),
                app.wpm(),
                app.accuracy(),
                app.scheme_index,
                app.cursor_style_index,
                app.is_done(),
                countdown,
            );
        })?;

        if app.is_done() && restart_timer.is_none() {
            restart_timer = Some(std::time::Instant::now());
        }

        if let Some(start_time) = restart_timer {
            if start_time.elapsed() >= std::time::Duration::from_secs(3) {
                app.restart();
                restart_timer = None;
            }
        }

        let timeout = std::time::Duration::from_millis(100);

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) => break,
                    (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                        app.restart();
                        restart_timer = None;
                    }
                    (KeyCode::BackTab, _) => app.cycle_color_scheme(),
                    (KeyCode::Char('`'), KeyModifiers::SHIFT) => app.cycle_cursor_style(),
                    (KeyCode::Char(ch), KeyModifiers::NONE) => {
                        if app.is_done() {
                            restart_timer = None;
                        }
                        app.handle_char(ch);
                    }
                    (KeyCode::Backspace, KeyModifiers::CONTROL) => app.handle_ctrl_backspace(),
                    (KeyCode::Char('w'), KeyModifiers::CONTROL) => app.handle_ctrl_backspace(),
                    (KeyCode::Backspace, _) => app.handle_backspace(),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
