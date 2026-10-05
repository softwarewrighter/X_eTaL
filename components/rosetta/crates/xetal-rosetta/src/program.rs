//! The program and its files, built into the page, and the request
//! that runs it: the worker finds `Stone.xtl` and `Comparison.xtl` in
//! the files it is given (as `u_se<` looks beside the program) and
//! reads `demos/rosetta/data.toml` through them too.

use xetal_runner::{Mode, Request};

const PROGRAM: &str = include_str!("../../../../../demos/rosetta/rosetta.xtl");
const STONE: &str = include_str!("../../../../../demos/rosetta/Stone.xtl");
const COMPARISON: &str = include_str!("../../../../../demos/rosetta/Comparison.xtl");
const DATA: &str = include_str!("../../../../../demos/rosetta/data.toml");

/// A list's items as (key, display name), in order.
pub type Named = Vec<(String, String)>;

/// The idioms and languages of the data, each as (key, name), in order,
/// read from the TOML's four lists by the page itself (the program
/// reads them through []L_IST; the page only needs them for its menus).
pub fn lists() -> (Named, Named) {
    let list = |key: &str| -> Vec<String> {
        DATA.lines()
            .find_map(|l| l.strip_prefix(&format!("{key} = [")))
            .map(|rest| {
                rest.trim_end_matches(']')
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    };
    let pair = |keys: Vec<String>, names: Vec<String>| keys.into_iter().zip(names).collect();
    (
        pair(list("idioms"), list("idiom_names")),
        pair(list("languages"), list("language_names")),
    )
}

/// The request that runs the stone: the program with its library and
/// data files, frames replacing one another.
pub fn request() -> Request {
    Request {
        src: PROGRAM.to_string(),
        seed: 1,
        mode: Mode::Run,
        boxed: false,
        frames: true,
        files: vec![
            ("Stone.xtl".into(), STONE.into()),
            ("Comparison.xtl".into(), COMPARISON.into()),
            ("demos/rosetta/data.toml".into(), DATA.into()),
        ],
    }
}

/// Where the program says it stands, from its output: the last
/// `at IDIOM TOP BOTTOM` line, as the three keys.
pub fn position(out: &str) -> Option<[String; 3]> {
    let line = out.lines().rev().find(|l| l.starts_with("at "))?;
    let mut words = line.split_whitespace().skip(1).map(String::from);
    Some([words.next()?, words.next()?, words.next()?])
}
