//! Where each part of the running program was written (QD9): a data
//! file a program names with a relative path (`[]T_ABLE`, `[]L_IST`) is
//! found beside the file that names it, as `@ i_nclude<` finds its
//! file, so a program reads its data from any working directory.

use std::path::Path;
use std::sync::RwLock;

/// Each part of the combined program: where it starts, and the folder
/// of the file it was written in ("" for none: `-e` text, the browser).
static ORIGINS: RwLock<Vec<(usize, String)>> = RwLock::new(Vec::new());

/// The program now loaded: (offset, file name) for each part, in order.
pub fn set_origins(parts: Vec<(usize, String)>) {
    let folders = parts
        .into_iter()
        .map(|(at, name)| {
            let folder = Path::new(&name)
                .parent()
                .map(|p| p.to_string_lossy().into_owned());
            (at, folder.unwrap_or_default())
        })
        .collect();
    if let Ok(mut origins) = ORIGINS.write() {
        *origins = folders;
    }
}

/// `path` as named at offset `at` of the program: an absolute path as
/// written, a relative one joined to the naming file's folder.
pub fn beside(path: &str, at: usize) -> String {
    if Path::new(path).is_absolute() {
        return path.to_string();
    }
    let origins = ORIGINS.read().unwrap_or_else(|e| e.into_inner());
    let folder = origins.iter().rev().find(|(start, _)| *start <= at);
    match folder {
        Some((_, f)) if !f.is_empty() => Path::new(f).join(path).to_string_lossy().into_owned(),
        _ => path.to_string(),
    }
}
