//! The programs the drop-down offers: the demos and the tour, built in.

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
    "combinators.xtl",
    "monads.xtl",
    "stats.xtl",
    "keys.xtl",
    "tttml.xtl",
    "factorial.xtl",
    "higher-order.xtl",
    "arrays.xtl",
];
