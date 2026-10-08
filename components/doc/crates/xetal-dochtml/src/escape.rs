//! Escaping text, and the anchors and page names of the site.

/// `text` safe inside HTML text and attribute values.
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// An element id for `name`: letters, digits, `_` and `-` as they are,
/// `:` as `.`, anything else as `-` and its code in hex (`s:i_f<` is
/// `s.i_f-3c`), so it needs no escaping in a URL.
pub fn anchor(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            c if c.is_ascii_alphanumeric() || c == '_' => c.to_string(),
            ':' => ".".to_string(),
            c => format!("-{:x}", c as u32),
        })
        .collect()
}

/// The page (without `.html`) for the file reported as `name`: its
/// path without leading `./` and `../`, `/` as `-`; text given on the
/// command line (`-e`) is `program`.
pub fn page(name: &str) -> String {
    if name == "-e" {
        return "program".into();
    }
    let parts: Vec<&str> = name
        .split('/')
        .skip_while(|p| matches!(*p, "" | "." | ".."))
        .collect();
    parts.join("-")
}

/// The directory every one of `names` is in, with its trailing `/`
/// (empty when they share none): what file names are shown relative to.
pub fn root(names: &[&str]) -> String {
    let dirs: Vec<&str> = names
        .iter()
        .map(|n| n.rsplit_once('/').map_or("", |(d, _)| d))
        .collect();
    let mut common: Vec<&str> = dirs.first().map_or(Vec::new(), |d| d.split('/').collect());
    for d in &dirs[1.min(dirs.len())..] {
        let parts: Vec<&str> = d.split('/').collect();
        let same = common
            .iter()
            .zip(&parts)
            .take_while(|(a, b)| a == b)
            .count();
        common.truncate(same);
    }
    let joined = common.join("/");
    if joined.is_empty() {
        joined
    } else {
        format!("{joined}/")
    }
}

/// `name` relative to `root` (from [`root`]).
pub fn relative<'n>(root: &str, name: &'n str) -> &'n str {
    name.strip_prefix(root).unwrap_or(name)
}
