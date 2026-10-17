//! How a string literal is spelled: its form, the delimiters each form
//! writes, and the fewest `#` a raw string needs. Another kind of string
//! (a code string `c"..."`, docs/code-strings.md) is one more variant.

/// The spelling of a string literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// `"..."`: single-line, with the escapes `\"` `\\` `\n` `\t`.
    Plain,
    /// `r"..."` with this many `#` around the quotes: verbatim (ST4).
    Raw(usize),
}

impl Form {
    /// The form of `token`, the text of a string literal; `None` for a
    /// spelling this crate does not know.
    pub fn of(token: &str) -> Option<Form> {
        match token.as_bytes().first()? {
            b'"' => Some(Form::Plain),
            b'r' => {
                let hashes = token.bytes().skip(1).take_while(|&b| b == b'#').count();
                Some(Form::Raw(hashes))
            }
            _ => None,
        }
    }

    /// What opens a literal of this form: `"` or `r`, its `#`, `"`.
    pub fn open(self) -> String {
        match self {
            Form::Plain => "\"".to_string(),
            Form::Raw(n) => format!("r{}\"", "#".repeat(n)),
        }
    }

    /// What closes a literal of this form: `"`, then any `#`.
    pub fn close(self) -> String {
        match self {
            Form::Plain => "\"".to_string(),
            Form::Raw(n) => format!("\"{}", "#".repeat(n)),
        }
    }
}

/// The fewest `#` a raw string holding `body` needs to end where it
/// should: none when it has no `"`, else one more than the most `#`
/// that follow a `"` in it.
pub fn fewest_hashes(body: &str) -> usize {
    let bytes = body.as_bytes();
    let after = |i: usize| bytes.iter().skip(i + 1).take_while(|&&b| b == b'#').count();
    (0..bytes.len())
        .filter(|&i| bytes[i] == b'"')
        .map(|i| after(i) + 1)
        .max()
        .unwrap_or(0)
}
