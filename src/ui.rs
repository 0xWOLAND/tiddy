use ratatui::{prelude::*, widgets::*};

pub trait ThemeColors {
    fn text(self) -> Color;
    fn done(self) -> Color;
    fn skipped(self) -> Color;
    fn error(self) -> Color;
    fn accent(self) -> Color;
}

#[derive(Clone, Copy, Debug)]
pub enum ColorScheme {
    Gruvbox,
    Dracula,
    Nord,
    Solarized,
}

impl ThemeColors for ColorScheme {
    fn text(self) -> Color {
        match self {
            Self::Gruvbox | Self::Dracula | Self::Nord => Color::White,
            Self::Solarized => Color::Gray,
        }
    }

    fn done(self) -> Color {
        match self {
            Self::Gruvbox | Self::Solarized => Color::Green,
            Self::Dracula => Color::Blue,
            Self::Nord => Color::Cyan,
        }
    }

    fn skipped(self) -> Color {
        Color::DarkGray
    }

    fn error(self) -> Color {
        Color::Red
    }

    fn accent(self) -> Color {
        match self {
            Self::Gruvbox => Color::Yellow,
            Self::Dracula => Color::Cyan,
            Self::Nord | Self::Solarized => Color::Blue,
        }
    }
}

impl ColorScheme {
    pub fn get(index: usize) -> Self {
        const SCHEMES: [ColorScheme; 4] = [
            ColorScheme::Gruvbox,
            ColorScheme::Dracula,
            ColorScheme::Nord,
            ColorScheme::Solarized,
        ];
        SCHEMES[index % SCHEMES.len()]
    }
}

impl std::fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{self:?}").to_lowercase();
        write!(f, "{name}")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CursorStyle {
    Underline,
    Block,
    Default,
}

impl CursorStyle {
    pub fn apply(self, style: Style) -> Style {
        match self {
            CursorStyle::Underline => style.add_modifier(Modifier::UNDERLINED),
            CursorStyle::Block => style.bg(style.fg.unwrap_or(Color::White)).fg(Color::Black),
            CursorStyle::Default => style,
        }
    }

    pub fn cycle(index: usize) -> Self {
        match index % 3 {
            0 => CursorStyle::Underline,
            1 => CursorStyle::Block,
            _ => CursorStyle::Default,
        }
    }
}

impl std::fmt::Display for CursorStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{self:?}").to_lowercase();
        write!(f, "{name}")
    }
}

pub struct RenderConfig<'a> {
    pub target: &'a str,
    pub input: &'a str,
    pub wpm: f64,
    pub accuracy: f64,
    pub scheme_index: usize,
    pub cursor_style_index: usize,
    pub is_done: bool,
    pub restart_countdown: Option<u64>,
}

pub fn render_typing_test<B: Backend>(frame: &mut Frame<B>, config: RenderConfig) {
    let scheme = ColorScheme::get(config.scheme_index);

    // Create centered layout
    let area = frame.size();
    let content_width = area.width.clamp(40, 80); // Max 80 chars, min 40 chars
    let horizontal_margin = (area.width.saturating_sub(content_width)) / 2;

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(2), // Spacing
            Constraint::Min(5),    // Main content
            Constraint::Length(2), // Spacing
            Constraint::Length(1), // Help
        ])
        .split(area);

    let centered_chunks: Vec<Rect> = vertical_chunks
        .iter()
        .map(|&chunk| Rect {
            x: chunk.x + horizontal_margin,
            y: chunk.y,
            width: content_width,
            height: chunk.height,
        })
        .collect();

    // Title
    let title = format!(
        "tiddy ({}) | wpm: {:.0} | acc: {:.0}%",
        scheme, config.wpm, config.accuracy
    );
    frame.render_widget(
        Paragraph::new(title)
            .fg(scheme.accent())
            .alignment(Alignment::Center),
        centered_chunks[0],
    );

    // Main text content (skip spacing chunks at index 1 and 3)
    let spans = create_text_spans(
        config.target,
        config.input,
        scheme,
        config.cursor_style_index,
    );
    frame.render_widget(
        Paragraph::new(Line::from(spans))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left),
        centered_chunks[2],
    );

    // Help
    let help = if let Some(countdown) = config.restart_countdown {
        format!("Auto-restart in {countdown}s (any key to cancel) | Ctrl+R restart | Esc quit")
    } else if config.is_done {
        "Test complete | Ctrl+R restart | Esc quit".to_string()
    } else {
        "Ctrl+R restart | Esc quit".to_string()
    };
    frame.render_widget(
        Paragraph::new(help)
            .fg(scheme.text())
            .alignment(Alignment::Center),
        centered_chunks[4],
    );
}

fn create_text_spans<'a>(
    target: &'a str,
    input: &'a str,
    scheme: ColorScheme,
    cursor_style_index: usize,
) -> Vec<Span<'a>> {
    let mut spans = Vec::new();

    // Render typed characters
    for (i, ch) in input.chars().enumerate() {
        if let Some(target_ch) = target.chars().nth(i) {
            if ch == '#' {
                // Show skipped characters with dedicated skipped color
                spans.push(Span::styled(
                    target_ch.to_string(),
                    Style::default().fg(scheme.skipped()),
                ));
            } else if ch == target_ch {
                // Correctly typed character
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(scheme.done()),
                ));
            } else {
                // Incorrectly typed character
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(scheme.error()),
                ));
            }
        } else {
            // Input is longer than target
            spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(scheme.error()),
            ));
        }
    }

    // Render remaining target characters
    for (i, ch) in target.chars().enumerate().skip(input.len()) {
        let style = if i == input.len() {
            // Cursor position
            let base_style = Style::default().fg(scheme.accent());
            CursorStyle::cycle(cursor_style_index).apply(base_style)
        } else {
            Style::default().fg(scheme.text())
        };
        spans.push(Span::styled(ch.to_string(), style));
    }

    spans
}
