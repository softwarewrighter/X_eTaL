//! A command line split into words as a POSIX shell splits it: blanks
//! separate words, single quotes keep everything, double quotes keep
//! all but `\"` and `\\`, and a backslash outside quotes keeps the next
//! character.

/// The words of `line`, or why it cannot be split (an open quote).
pub fn words(line: &str) -> Result<Vec<String>, String> {
    let (mut out, mut word, mut started) = (Vec::new(), String::new(), false);
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => {
                if started {
                    out.push(std::mem::take(&mut word));
                }
                started = false;
                continue;
            }
            '\'' => word.push_str(&quoted(&mut chars, '\'')?),
            '"' => word.push_str(&quoted(&mut chars, '"')?),
            '\\' => word.extend(chars.next()),
            c => word.push(c),
        }
        started = true;
    }
    if started {
        out.push(word);
    }
    Ok(out)
}

/// The text up to the closing `quote`; in double quotes `\"` and `\\`
/// are the character.
fn quoted(chars: &mut std::str::Chars, quote: char) -> Result<String, String> {
    let mut text = String::new();
    while let Some(c) = chars.next() {
        match c {
            c if c == quote => return Ok(text),
            '\\' if quote == '"' => match chars.next() {
                Some(n @ ('"' | '\\')) => text.push(n),
                Some(n) => {
                    text.push('\\');
                    text.push(n);
                }
                None => break,
            },
            c => text.push(c),
        }
    }
    Err(format!("the {quote} quote is not closed"))
}

/// A path as a word the shell splits back to itself.
pub(crate) fn quote(name: &str) -> String {
    match name.contains([' ', '\'', '"', '\\']) {
        true => format!("'{}'", name.replace('\'', "'\\''")),
        false => name.to_string(),
    }
}
