//! The lint report an editor shows (the live demo's Lint button,
//! `xetal edit`'s Ctrl-L): the findings of a text as `xetal lint`
//! prints them, each line with its level so a pane can color it, the
//! count always, and the text with every sure fix made, for a second
//! click to apply. The driver is `xetal-linter`; this crate only
//! shapes its result for a screen. See docs/lint.md.

mod report;

pub use report::{Kind, Line, Report, report, report_with};
