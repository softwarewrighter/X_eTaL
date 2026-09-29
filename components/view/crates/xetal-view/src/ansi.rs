//! Segments as ANSI-colored terminal text (`xetal render --color`).

use crate::{Class, Segment};

const RESET: &str = "\u{1b}[0m";

/// The SGR color of a class; whitespace is left plain.
fn color(class: Class) -> Option<&'static str> {
    Some(match class {
        Class::Builtin => "34",
        Class::UserFunc => "32",
        Class::LibFunc => "36",
        Class::LambdaArg => "35",
        Class::Number | Class::Exponent => "33",
        Class::String => "93",
        Class::Symbol | Class::Quote => "1",
        Class::Comment => "2",
        Class::Error => "31;4",
        Class::Variable | Class::Punct | Class::Unit | Class::Space => return None,
    })
}

/// The segments with each class colored, ending in a reset.
pub fn ansi(segments: &[Segment]) -> String {
    let mut out = String::new();
    for s in segments {
        match color(s.class) {
            Some(c) => out.push_str(&format!("\u{1b}[{c}m{}{RESET}", s.text)),
            None => out.push_str(&s.text),
        }
    }
    if !out.ends_with(RESET) {
        out.push_str(RESET);
    }
    out
}
