//! `# lint:` and `# lint-file:` comments: the level, then lints and
//! groups.

use crate::word::Candidate;

/// If `line` (up to the cursor) is a lint directive being written, the
/// start of the word at its end and the candidates: a level when none
/// is written yet, else the lints, the groups and `all`.
pub(crate) fn directive(line: &str) -> Option<(usize, Vec<Candidate>)> {
    let hash = line.rfind('#')?;
    let rest = line[hash + 1..].trim_start();
    let rest = rest
        .strip_prefix("lint-file:")
        .or_else(|| rest.strip_prefix("lint:"))?;
    let start = line.rfind(char::is_whitespace).map_or(0, |i| i + 1);
    let word = &line[start..];
    let words = rest.split_whitespace().count();
    let typing_level = words == 0 || (words == 1 && !rest.ends_with(char::is_whitespace));
    let candidates: Vec<Candidate> = match typing_level {
        true => levels(),
        false => lints(),
    }
    .into_iter()
    .filter(|c| c.label.starts_with(word) && c.label != word)
    .collect();
    Some((start, candidates))
}

fn levels() -> Vec<Candidate> {
    [
        ("allow", "not reported"),
        ("warn", "reported; the run goes on"),
        ("deny", "reported as an error; xetal lint fails"),
    ]
    .iter()
    .map(|(name, what)| Candidate {
        label: name.to_string(),
        insert: name.to_string(),
        detail: what.to_string(),
    })
    .collect()
}

/// Every lint with its summary, every group, and `all`.
fn lints() -> Vec<Candidate> {
    let one = |name: &str, what: &str| Candidate {
        label: name.to_string(),
        insert: name.to_string(),
        detail: what.to_string(),
    };
    let lints = xetal_lint::all().iter().map(|l| one(l.name, l.summary));
    let groups = xetal_lint::Group::ALL
        .iter()
        .map(|g| one(g.name(), "a group of lints"));
    lints
        .chain(groups)
        .chain(std::iter::once(one("all", "every lint")))
        .collect()
}
