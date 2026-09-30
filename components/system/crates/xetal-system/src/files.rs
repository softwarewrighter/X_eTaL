//! Text files and the keyboard. These are effects, so a program runs
//! them only when it is run (never on a keystroke in the editor).

use std::io::BufRead;

use xetal_base::Diagnostic;
use xetal_value::Value;

use crate::text::{chars, text};

fn io(e: impl std::fmt::Display, what: &str) -> Diagnostic {
    Diagnostic::new("io", format!("{what}: {e}"))
}

/// `t []N_PUT path`: write text t to the file (made, with its
/// directories, if missing; replaced if there); how many characters.
pub(crate) fn put<'a>(t: &Value<'a>, path: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let (t, path) = (chars(t)?, chars(path)?);
    if let Some(dir) = std::path::Path::new(&path)
        .parent()
        .filter(|d| !d.as_os_str().is_empty())
    {
        std::fs::create_dir_all(dir).map_err(|e| io(e, &path))?;
    }
    std::fs::write(&path, &t).map_err(|e| io(e, &path))?;
    Ok(Value::Int(t.chars().count() as i64))
}

/// `[]N_GET path`: the file's text.
pub(crate) fn get<'a>(path: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let path = chars(path)?;
    std::fs::read_to_string(&path)
        .map(|s| text(&s))
        .map_err(|e| io(e, &path))
}

/// `[]R_EAD @`: a line typed at the keyboard, without its newline.
pub(crate) fn read<'a>() -> Result<Value<'a>, Diagnostic> {
    let mut line = String::new();
    match std::io::stdin().lock().read_line(&mut line) {
        Ok(0) => Err(Diagnostic::new("io", "[]R_EAD: no more input")),
        Ok(_) => Ok(text(line.trim_end_matches(['\n', '\r']))),
        Err(e) => Err(io(e, "[]R_EAD")),
    }
}
