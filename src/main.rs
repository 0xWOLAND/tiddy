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
    let word_count = cli.command.word_count();
    let time_limit = cli.command.time_limit();

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

    loop {
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
            );
        })?;

        let timeout = if app.is_done() {
            std::time::Duration::from_secs(10) // Long timeout when done
        } else {
            std::time::Duration::from_millis(100) // Fast updates during typing
        };

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) => break,
                    (KeyCode::Char('r'), KeyModifiers::CONTROL) => app.restart(),
                    (KeyCode::BackTab, _) => app.cycle_color_scheme(),
                    (KeyCode::CapsLock, KeyModifiers::SHIFT) => app.cycle_cursor_style(),
                    (KeyCode::Char(ch), KeyModifiers::NONE) => app.handle_char(ch),
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