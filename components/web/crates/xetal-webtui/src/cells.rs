//! The in-memory backend's cells as rows of styled runs for the page.

use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier};

/// A run of cells with one style: its text and its CSS classes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub text: String,
    pub class: String,
}

/// The buffer as rows of runs, adjacent cells of one style joined;
/// the cell at `cursor` gets the class `cursor`.
pub fn rows(buffer: &Buffer, cursor: Option<(u16, u16)>) -> Vec<Vec<Run>> {
    let area = buffer.area;
    (0..area.height)
        .map(|y| {
            let mut row: Vec<Run> = Vec::new();
            for x in 0..area.width {
                let cell = &buffer[(area.x + x, area.y + y)];
                let class = class_of(cell.fg, cell.modifier, cursor == Some((x, y)));
                match row.last_mut() {
                    Some(last) if last.class == class => last.text.push_str(cell.symbol()),
                    _ => row.push(Run {
                        text: cell.symbol().to_string(),
                        class,
                    }),
                }
            }
            row
        })
        .collect()
}

/// The CSS classes for a style: the color's name, the modifiers set.
fn class_of(fg: Color, modifier: Modifier, at_cursor: bool) -> String {
    let mut out: Vec<&str> = Vec::new();
    if let Some(name) = color_name(fg) {
        out.push(name);
    }
    for (flag, name) in [
        (Modifier::BOLD, "bold"),
        (Modifier::DIM, "dim"),
        (Modifier::ITALIC, "italic"),
        (Modifier::UNDERLINED, "underlined"),
        (Modifier::REVERSED, "reversed"),
    ] {
        if modifier.contains(flag) {
            out.push(name);
        }
    }
    if at_cursor {
        out.push("cursor");
    }
    out.join(" ")
}

/// The class for a named color the editor uses (the theme of
/// xetal-panes); the default color has none.
fn color_name(fg: Color) -> Option<&'static str> {
    Some(match fg {
        Color::Blue => "fg-blue",
        Color::LightBlue => "fg-lightblue",
        Color::Green => "fg-green",
        Color::Cyan => "fg-cyan",
        Color::Magenta => "fg-magenta",
        Color::Yellow => "fg-yellow",
        Color::LightYellow => "fg-lightyellow",
        Color::Red => "fg-red",
        Color::DarkGray => "fg-darkgray",
        Color::LightRed => "fg-lightred",
        Color::LightGreen => "fg-lightgreen",
        Color::LightMagenta => "fg-lightmagenta",
        Color::LightCyan => "fg-lightcyan",
        Color::Gray => "fg-gray",
        Color::White => "fg-white",
        _ => return None,
    })
}
