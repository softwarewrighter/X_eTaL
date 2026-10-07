//! The type lines `xetal type` shows: a program's items, or a
//! library's exports.

use xetal_sources::Sources;

/// The type lines of a checked program, its interface (PN7): without
/// its libraries' own items (`LA:m_ean : ...`: hidden namespaces are
/// uppercase) or its own helpers (`h:`).
pub fn program_types(lines: Vec<String>) -> Vec<String> {
    let not_offered = |line: &String| {
        let name = line.split_once(" : ").map_or("", |(name, _)| name);
        let ns = name.split_once(':').map_or("", |(ns, _)| ns);
        ns == "h" || ns.starts_with(|c: char| c.is_ascii_uppercase())
    };
    lines.into_iter().filter(|l| !not_offered(l)).collect()
}

/// The type lines of a library checked on its own (file 0 of
/// `sources`): its exports only, written `l:` (`m:` in a macro library,
/// `s:` in the system macro library) as in the file.
pub fn library_types(sources: &Sources, lines: Vec<String>) -> Vec<String> {
    let name = sources.name(0);
    let own = match (name.ends_with(".xtlm"), name.ends_with("System.xtlm")) {
        (_, true) => "s:",
        (true, false) => "m:",
        _ => "l:",
    };
    lines
        .iter()
        .map(|line| sources.as_written(0, line))
        .filter(|line| line.starts_with(own))
        .collect()
}
