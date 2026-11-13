//! Segments as ANSI-colored terminal text (`xetal render --color`),
//! and with rainbow delimiters (D-12: each bracket colored by its depth).

use std::sync::OnceLock;

use xetal_lex::Bracket;

use crate::{Class, Segment};

const RESET: &str = "\u{1b}[0m";

/// The rainbow's four colors (256-color codes): gray, orange, pink and
/// white, apart from the classes' colors.
pub const RAINBOW: [u8; 4] = [245, 208, 218, 255];

/// The SGR color of a class; whitespace is left plain.
fn color(class: Class) -> Option<&'static str> {
    Some(match class {
        Class::Builtin => "34",
        Class::Macro => "1;33",
        Class::UserFunc => "32",
        Class::LibFunc => "36",
        Class::LambdaArg => "35",
        Class::Number | Class::Exponent => "33",
        Class::String => "93",
        Class::Symbol => "94",
        Class::Quote => "1",
        Class::Comment => "2",
        Class::Doc => "3;32",
        Class::Heading => "1;32",
        Class::Error => "31;4",
        Class::Notation => "3;93",
        Class::Applied => "4",
        Class::Variable | Class::Punct | Class::Unit | Class::Space => return None,
    })
}

/// A segment's SGR code: its class's, or with `rainbow` a bracket's
/// depth color (an applied group's parenthesis keeps its underline) or
/// the error color for a stray.
fn code(s: &Segment, rainbow: bool) -> Option<String> {
    match (rainbow, s.bracket) {
        (true, Some(Bracket::Depth(d))) => {
            let under = if s.class == Class::Applied { "4;" } else { "" };
            Some(format!("{under}38;5;{}", RAINBOW[d % RAINBOW.len()]))
        }
        (true, Some(Bracket::Unmatched)) => color(Class::Error).map(String::from),
        _ => color(s.class).map(String::from),
    }
}

/// The segments with each class colored, ending in a reset; runs of
/// one class and color share one code.
pub fn ansi(segments: &[Segment]) -> String {
    colored(segments, false)
}

/// As [`ansi`], with rainbow delimiters.
pub fn ansi_rainbow(segments: &[Segment]) -> String {
    colored(segments, true)
}

fn colored(segments: &[Segment], rainbow: bool) -> String {
    let mut out = String::new();
    let mut rest = segments;
    while let Some(first) = rest.first() {
        let c = code(first, rainbow);
        let same = |s: &&Segment| s.class == first.class && code(s, rainbow) == c;
        let n = rest.iter().take_while(same).count();
        let run: String = rest[..n].iter().map(|s| s.text.as_str()).collect();
        match c {
            Some(c) if !run.is_empty() => out.push_str(&format!("\u{1b}[{c}m{run}{RESET}")),
            _ => out.push_str(&run),
        }
        rest = &rest[n..];
    }
    if !out.ends_with(RESET) {
        out.push_str(RESET);
    }
    out
}

/// Whether the terminal views (the editor, the REPL's line) draw rainbow
/// delimiters: yes unless `XETAL_RAINBOW` is `off` or `0`.
pub fn rainbow_on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("XETAL_RAINBOW").map_or(true, |v| !matches!(v.as_str(), "off" | "0"))
    })
}
