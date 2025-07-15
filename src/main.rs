mod app;
mod cli;
mod popup;
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
                ui::RenderConfig {
                    target: app.target(),
                    input: app.input(),
                    wpm: app.wpm(),
                    accuracy: app.accuracy(),
                    scheme_index: app.scheme_index,
                    cursor_style_index: app.cursor_style_index,
                    is_done: app.is_done(),
                    restart_countdown: countdown,
                },
            );

            if let Some(popup) = &app.word_list_popup {
                popup.render(frame, frame.size());
            }
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
                if app.handle_popup_key(key.code) {
                    continue;
                }

                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) => break,
                    (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                        app.toggle_word_list_popup();
                    }
                    (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                        app.restart();
                        restart_timer = None;
                    }
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => app.handle_ctrl_backspace(), // Ctrl+Backspace in Ubuntu
                    (KeyCode::Char('w'), KeyModifiers::CONTROL) => app.handle_ctrl_backspace(),
                    (KeyCode::Backspace, KeyModifiers::CONTROL) => app.handle_ctrl_backspace(),
                    (KeyCode::Delete, KeyModifiers::CONTROL) => app.handle_ctrl_backspace(),
                    (KeyCode::BackTab, _) => app.cycle_color_scheme(),
                    (KeyCode::Char('i'), KeyModifiers::CONTROL) => app.cycle_cursor_style(),
                    (KeyCode::Backspace, _) => app.handle_backspace(),
                    (KeyCode::Char(ch), KeyModifiers::NONE) => {
                        if app.is_done() {
                            restart_timer = None;
                        }
                        app.handle_char(ch);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
