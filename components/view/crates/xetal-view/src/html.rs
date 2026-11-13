//! Segments as HTML spans (`xetal render --html`): each run of one
//! class is a `<span class="c-...">`, named as the live demo's
//! Rendered pane names them, so one stylesheet colors both; plain text
//! (names, punctuation, spaces) is left bare. Text is escaped.

use xetal_lex::Bracket;

use crate::{Class, Segment};

/// The CSS class of a class of token; none for plain text.
fn css(class: Class) -> Option<String> {
    match class {
        Class::Variable | Class::Punct | Class::Unit | Class::Space => None,
        other => Some(format!("c-{other:?}").to_lowercase()),
    }
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// The segments as HTML, for inside a `<pre>`.
pub fn html(segments: &[Segment]) -> String {
    spans(segments, false)
}

/// As [`html`], with rainbow delimiters: a bracket's span also has
/// `c-depth0` to `c-depth3` (its depth, cycling), or `c-unmatched`.
pub fn html_rainbow(segments: &[Segment]) -> String {
    spans(segments, true)
}

/// The rainbow class of a bracket's segment (D-12); none for others.
pub fn rainbow_class(s: &Segment) -> Option<String> {
    s.bracket.map(|b| match b {
        Bracket::Depth(d) => format!("c-depth{}", d % 4),
        Bracket::Unmatched => "c-unmatched".to_string(),
    })
}

fn spans(segments: &[Segment], rainbow: bool) -> String {
    let classes = |s: &Segment| {
        let mine = css(s.class);
        let depth = rainbow.then(|| rainbow_class(s)).flatten();
        match (mine, depth) {
            (Some(a), Some(b)) => Some(format!("{a} {b}")),
            (a, b) => a.or(b),
        }
    };
    let mut out = String::new();
    let mut rest = segments;
    while let Some(first) = rest.first() {
        let c = classes(first);
        let same = |s: &&Segment| s.class == first.class && classes(s) == c;
        let n = rest.iter().take_while(same).count();
        let run: String = rest[..n].iter().map(|s| s.text.as_str()).collect();
        match c {
            Some(c) => out.push_str(&format!("<span class=\"{c}\">{}</span>", escape(&run))),
            None => out.push_str(&escape(&run)),
        }
        rest = &rest[n..];
    }
    out
}
