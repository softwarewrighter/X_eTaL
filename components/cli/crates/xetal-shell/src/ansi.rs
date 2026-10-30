//! Text with ANSI colors (what the commands print for a terminal) as a
//! ratatui line: the SGR codes the commands use become styles, and any
//! other escape sequence is dropped.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// One line of `text`, its colors as styles.
pub fn line(text: &str) -> Line<'static> {
    let (mut spans, mut style, mut run) = (Vec::new(), Style::default(), String::new());
    let mut rest = text;
    while let Some(at) = rest.find('\u{1b}') {
        run.push_str(&rest[..at]);
        let seq = &rest[at + 1..];
        let end = match seq.strip_prefix('[') {
            Some(body) => body
                .find(|c: char| c.is_ascii_alphabetic())
                .map(|i| (i + 2, &body[..i], body.as_bytes()[i])),
            None => None,
        };
        let Some((len, params, kind)) = end else {
            rest = seq;
            continue;
        };
        if kind == b'm' {
            if !run.is_empty() {
                spans.push(Span::styled(std::mem::take(&mut run), style));
            }
            style = sgr(style, params);
        }
        rest = &seq[len..];
    }
    run.push_str(rest);
    if !run.is_empty() {
        spans.push(Span::styled(run, style));
    }
    Line::from(spans)
}

/// `style` after the SGR parameters `params` (`1;33`, `0`, ...).
fn sgr(style: Style, params: &str) -> Style {
    let mut style = style;
    for p in params.split(';') {
        style = match p.parse::<u8>().unwrap_or(0) {
            0 => Style::default(),
            1 => style.add_modifier(Modifier::BOLD),
            2 => style.add_modifier(Modifier::DIM),
            3 => style.add_modifier(Modifier::ITALIC),
            4 => style.add_modifier(Modifier::UNDERLINED),
            22 => style.remove_modifier(Modifier::BOLD | Modifier::DIM),
            23 => style.remove_modifier(Modifier::ITALIC),
            24 => style.remove_modifier(Modifier::UNDERLINED),
            39 => style.fg(Color::Reset),
            n => match color(n) {
                Some(c) => style.fg(c),
                None => style,
            },
        };
    }
    style
}

/// The foreground color of an SGR code.
fn color(n: u8) -> Option<Color> {
    const NORMAL: [Color; 8] = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Gray,
    ];
    const BRIGHT: [Color; 8] = [
        Color::DarkGray,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
        Color::LightCyan,
        Color::White,
    ];
    match n {
        30..=37 => Some(NORMAL[usize::from(n - 30)]),
        90..=97 => Some(BRIGHT[usize::from(n - 90)]),
        _ => None,
    }
}

/// Error text in red, line by line.
pub(crate) fn red(text: &str) -> String {
    text.lines()
        .map(|l| format!("\u{1b}[31m{l}\u{1b}[0m\n"))
        .collect()
}
