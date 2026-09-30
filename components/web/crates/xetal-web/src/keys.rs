//! The editor's keys in the browser. Browsers keep Ctrl-T (a new tab)
//! for themselves, so Zoom is Ctrl-. here; Run is Ctrl-Enter (and
//! Ctrl-R where the browser lets a page have it).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Run,
    Zoom,
}

/// The action for a key pressed with Control (or Command) held.
pub fn action(key: &str, ctrl: bool) -> Option<Action> {
    match (ctrl, key) {
        (true, "Enter" | "r") => Some(Action::Run),
        (true, ".") => Some(Action::Zoom),
        _ => None,
    }
}
