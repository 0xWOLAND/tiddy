use ratatui::{
    prelude::*,
    widgets::*,
};

pub struct ColorScheme {
    pub text: Color,
    pub done: Color,
    pub error: Color,
    pub accent: Color,
}

impl ColorScheme {
    const SCHEMES: [ColorScheme; 4] = [
        // Gruvbox
        ColorScheme {
            text: Color::Rgb(235, 219, 178),
            done: Color::Rgb(146, 131, 116),
            error: Color::Red,
            accent: Color::Yellow,
        },
        // Dracula
        ColorScheme {
            text: Color::Rgb(248, 248, 242),
            done: Color::Rgb(98, 114, 164),
            error: Color::Red,
            accent: Color::Cyan,
        },
        // Nord
        ColorScheme {
            text: Color::Rgb(216, 222, 233),
            done: Color::Rgb(143, 188, 187),
            error: Color::Red,
            accent: Color::Blue,
        },
        // Solarized
        ColorScheme {
            text: Color::Rgb(131, 148, 150),
            done: Color::Rgb(88, 110, 117),
            error: Color::Red,
            accent: Color::Blue,
        },
    ];

    const SCHEME_NAMES: [&'static str; 4] = ["gruvbox", "dracula", "nord", "solarized"];

    pub fn get(index: usize) -> &'static ColorScheme {
        &Self::SCHEMES[index % Self::SCHEMES.len()]
    }

    pub fn name(index: usize) -> &'static str {
        Self::SCHEME_NAMES[index % Self::SCHEME_NAMES.len()]
    }
}

#[derive(Clone, Copy)]
pub enum CursorStyle {
    Underline,
    Block,
    Reverse,
}

impl CursorStyle {
    pub fn apply(self, style: Style) -> Style {
        match self {
            CursorStyle::Underline => style.add_modifier(Modifier::UNDERLINED),
            CursorStyle::Block => style.bg(style.fg.unwrap_or(Color::White)).fg(Color::Black),
            CursorStyle::Reverse => style.add_modifier(Modifier::REVERSED),
        }
    }

    pub fn cycle(index: usize) -> Self {
        match index % 3 {
            0 => CursorStyle::Underline,
            1 => CursorStyle::Block,
            _ => CursorStyle::Reverse,
        }
    }
}

pub fn render_typing_test<B: Backend>(
    frame: &mut Frame<B>,
    target: &str,
    input: &str,
    wpm: f64,
    accuracy: f64,
    scheme_index: usize,
    cursor_style_index: usize,
    is_done: bool,
    restart_countdown: Option<u64>,
) {
    let scheme = ColorScheme::get(scheme_index);
    
    // Create centered layout
    let area = frame.size();
    let content_width = (area.width.min(80)).max(40); // Max 80 chars, min 40 chars
    let horizontal_margin = (area.width.saturating_sub(content_width)) / 2;
    
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),  // Title
            Constraint::Length(2),  // Spacing
            Constraint::Min(5),     // Main content
            Constraint::Length(2),  // Spacing
            Constraint::Length(1),  // Help
        ])
        .split(area);

    let centered_chunks: Vec<Rect> = vertical_chunks
        .iter()
        .map(|&chunk| {
            Rect {
                x: chunk.x + horizontal_margin,
                y: chunk.y,
                width: content_width,
                height: chunk.height,
            }
        })
        .collect();

    // Title
    let title = format!(
        "tiddy ({}) | wpm: {:.0} | acc: {:.0}%",
        ColorScheme::name(scheme_index),
        wpm,
        accuracy
    );
    frame.render_widget(
        Paragraph::new(title)
            .fg(scheme.accent)
            .alignment(Alignment::Center),
        centered_chunks[0],
    );

    // Main text content (skip spacing chunks at index 1 and 3)
    let spans = create_text_spans(target, input, scheme, cursor_style_index);
    frame.render_widget(
        Paragraph::new(Line::from(spans))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left),
        centered_chunks[2],
    );

    // Help
    let help = if let Some(countdown) = restart_countdown {
        format!("Auto-restart in {}s (any key to cancel) | Ctrl+R restart | Esc quit", countdown)
    } else if is_done {
        "Test complete | Ctrl+R restart | Esc quit".to_string()
    } else {
        "Ctrl+R restart | Shift+Tab colors | Shift+CapsLock cursor | Esc quit".to_string()
    };
    frame.render_widget(
        Paragraph::new(help)
            .fg(scheme.text)
            .alignment(Alignment::Center),
        centered_chunks[4],
    );
}

fn create_text_spans<'a>(
    target: &'a str,
    input: &'a str,
    scheme: &ColorScheme,
    cursor_style_index: usize,
) -> Vec<Span<'a>> {
    let mut spans = Vec::new();

    // Render typed characters
    for (i, ch) in input.chars().enumerate() {
        if let Some(target_ch) = target.chars().nth(i) {
            let color = if ch == '#' {
                // Show skipped characters as dimmed target characters
                spans.push(Span::styled(
                    target_ch.to_string(),
                    Style::default().fg(scheme.text).add_modifier(Modifier::DIM),
                ));
                continue;
            } else if ch == target_ch {
                scheme.done
            } else {
                scheme.error
            };
            spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        } else {
            // Input is longer than target
            spans.push(Span::styled(ch.to_string(), Style::default().fg(scheme.error)));
        }
    }

    // Render remaining target characters
    for (i, ch) in target.chars().enumerate().skip(input.len()) {
        let style = if i == input.len() {
            // Cursor position
            let base_style = Style::default().fg(scheme.accent);
            CursorStyle::cycle(cursor_style_index).apply(base_style)
        } else {
            Style::default().fg(scheme.text)
        };
        spans.push(Span::styled(ch.to_string(), style));
    }

    spans
}