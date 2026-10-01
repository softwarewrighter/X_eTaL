//! What Open offers: the demos and the tour, built in; the standard
//! libraries, named as `u_se<` finds them (`Stats.xtl`); and the files
//! saved in the store (local storage, in the browser).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Demo {
    pub name: &'static str,
    pub text: &'static str,
}

macro_rules! demos {
    ($($name:literal),* $(,)?) => {
        &[
            Demo { name: "tour.xtl", text: include_str!("../../../../../demos/tour.xtl") },
            Demo { name: "(empty)", text: "" },
            $(Demo {
                name: $name,
                text: include_str!(concat!("../../../../../demos/", $name)),
            }),*
        ]
    };
}

/// The first is shown when the page opens; the second is an empty
/// editor, to type into as at a REPL.
pub const DEMOS: &[Demo] = demos![
    "life.xtl",
    "hello-library.xtl",
    "combinators.xtl",
    "monads.xtl",
    "stats.xtl",
    "keys.xtl",
    "tttml.xtl",
    "tttml-train.xtl",
    "tttml-play.xtl",
    "factorial.xtl",
    "higher-order.xtl",
    "arrays.xtl",
    "classics/pascal.xtl",
    "classics/life-drawn.xtl",
    "classics/turtle.xtl",
];

/// The choices, as (group, value, label); a value is `demo:N`, `lib:Name`
/// or `file:path`, and [`open`] reads it.
pub fn choices(saved: &[String]) -> Vec<(&'static str, String, String)> {
    let demos = DEMOS
        .iter()
        .enumerate()
        .map(|(i, d)| ("Demos", format!("demo:{i}"), d.name.to_string()));
    let libs = xetal_libs::LIBRARIES
        .iter()
        .map(|(n, _)| ("Libraries", format!("lib:{n}"), format!("{n}.xtl")));
    let files = saved
        .iter()
        .map(|p| ("Your files", format!("file:{p}"), p.clone()));
    demos.chain(libs).chain(files).collect()
}

/// The name and text of a choice.
pub fn open(value: &str) -> Option<(String, String)> {
    match value.split_once(':')? {
        ("demo", i) => DEMOS
            .get(i.parse::<usize>().ok()?)
            .map(|d| (d.name.into(), d.text.into())),
        ("lib", n) => xetal_libs::standard(n).map(|t| (format!("{n}.xtl"), t.into())),
        ("file", p) => xetal_store::read(p).ok().map(|t| (p.to_string(), t)),
        _ => None,
    }
}

/// Put the demos' own libraries among your files, unless they are there
/// already (so your edits are kept): `hello-library.xtl` imports
/// `Hello.xtl` from there, as it would from beside it on disk.
pub fn seed() {
    for (path, text) in OWN_LIBRARIES {
        if xetal_store::read(path).is_err() {
            let _ = xetal_store::write(path, text);
        }
    }
}

/// The libraries the demos import that are not standard ones.
const OWN_LIBRARIES: &[(&str, &str)] = &[
    (
        "Hello.xtl",
        include_str!("../../../../../userlibs/Hello.xtl"),
    ),
    (
        "Greetings.xtl",
        include_str!("../../../../../userlibs/Greetings.xtl"),
    ),
];
