//! An expansion as JSON, for `xetal doc --json`.

use crate::Expansion;

impl Expansion {
    /// `{"line": .., "macro": .., "call": .., "text": .., "nested": [..]}`
    /// on one line.
    pub fn to_json(&self) -> String {
        let nested: Vec<String> = self.nested.iter().map(Expansion::to_json).collect();
        format!(
            "{{\"line\": {}, \"macro\": {}, \"call\": {}, \"text\": {}, \"nested\": [{}]}}",
            self.line,
            string(&self.name),
            string(&self.call),
            string(&self.text),
            nested.join(", ")
        )
    }
}

/// A JSON string literal.
fn string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
