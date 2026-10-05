//! The built-in nominal types' values: `Color` (the eight colors) and
//! `Key` (the named keys, then a printing key as `PRINTING` plus its
//! code point) by index (QD6), and `Event`, what `[]E_VENT` gives
//! (RS1): one line of text parsed into its kind, its numbers and its
//! key. The lines: `tick 0.016` (the seconds since the last tick),
//! `down 120 80`, `move 120 80`, `up 120 80`, `click 120 80` (a
//! pointer, x then y), `key Up` (a key as `[]K_EY` names it) and `end`
//! (no more events).

use std::fmt;

/// The colors, index 0 to 7 (ANSI color order).
pub const COLORS: [&str; 8] = [
    "BLACK", "RED", "GREEN", "YELLOW", "BLUE", "MAGENTA", "CYAN", "WHITE",
];

/// The named keys, index 0 to 10.
pub const KEYS: [&str; 11] = [
    "UP",
    "DOWN",
    "LEFT",
    "RIGHT",
    "ENTER",
    "ESCAPE",
    "BACKSPACE",
    "TAB",
    "DELETE",
    "HOME",
    "END",
];

/// A printing key's index: this plus its character's code point.
const PRINTING: u32 = 0x100;

/// The name a value of `ty` with index `i` prints as: its name, or a
/// printing key's character.
pub fn name(ty: &str, i: u32) -> String {
    let table: &[&str] = if ty == "Color" { &COLORS } else { &KEYS };
    match table.get(i as usize) {
        Some(n) => (*n).to_string(),
        None if ty == "Key" => {
            char::from_u32(i.saturating_sub(PRINTING)).map_or_else(String::new, String::from)
        }
        None => format!("{ty}?{i}"),
    }
}

/// The Key index for a key as a terminal names it (`Up`, `Enter`, or
/// one character), if it is one.
pub fn key_named(name: &str) -> Option<u32> {
    if let Some(i) = KEYS.iter().position(|k| k.eq_ignore_ascii_case(name)) {
        return Some(i as u32);
    }
    let mut chars = name.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some(PRINTING + c as u32),
        _ => None,
    }
}

/// The kinds, as `[]E_KIND` names them.
pub const KINDS: [&str; 7] = ["tick", "down", "move", "up", "click", "key", "end"];

/// One event: its kind, its numbers (`x y`, or the seconds of a tick,
/// or none) and its key (a Key index, for a key event).
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub kind: &'static str,
    pub at: Vec<f64>,
    pub key: Option<u32>,
}

impl Event {
    /// The event a line describes, if it is one.
    pub fn parse(line: &str) -> Option<Event> {
        let mut words = line.split_whitespace();
        let first = words.next()?;
        let kind = *KINDS.iter().find(|k| **k == first)?;
        let rest: Vec<&str> = words.collect();
        let numbers = |n: usize| -> Option<Vec<f64>> {
            (rest.len() == n)
                .then(|| rest.iter().map(|w| w.parse().ok()).collect())
                .flatten()
        };
        let (at, key) = match kind {
            "tick" => (numbers(1)?, None),
            "key" if rest.len() == 1 => (Vec::new(), Some(key_named(rest[0])?)),
            "key" => return None,
            "end" => (numbers(0)?, None),
            _ => (numbers(2)?, None),
        };
        Some(Event { kind, at, key })
    }

    /// The event that ends a program's events.
    pub fn end() -> Event {
        Event {
            kind: "end",
            at: Vec::new(),
            key: None,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.kind)?;
        for x in &self.at {
            write!(f, " {x}")?;
        }
        if let Some(k) = self.key {
            write!(f, " {}", name("Key", k))?;
        }
        Ok(())
    }
}
