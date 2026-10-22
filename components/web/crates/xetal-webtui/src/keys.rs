//! The page's key events as the editor's keys.

use xetal_keys::{Code, Key};

/// The key for a browser key name (`KeyboardEvent.key`) and its
/// modifiers; `None` for a key the editor does not use (a modifier
/// alone, a function key). Shift-Tab is BackTab; Command counts as
/// Control, as the live demo's other keys do.
pub fn key_of(name: &str, ctrl: bool, alt: bool, shift: bool) -> Option<Key> {
    let code = match name {
        "Enter" => Code::Enter,
        "Backspace" => Code::Backspace,
        "Delete" => Code::Delete,
        "ArrowLeft" => Code::Left,
        "ArrowRight" => Code::Right,
        "ArrowUp" => Code::Up,
        "ArrowDown" => Code::Down,
        "Home" => Code::Home,
        "End" => Code::End,
        "PageUp" => Code::PageUp,
        "PageDown" => Code::PageDown,
        "Tab" if shift => Code::BackTab,
        "Tab" => Code::Tab,
        "Escape" => Code::Esc,
        _ => {
            let mut chars = name.chars();
            let c = chars.next().filter(|_| chars.next().is_none())?;
            Code::Char(c)
        }
    };
    Some(Key {
        code,
        ctrl,
        alt,
        shift,
    })
}
