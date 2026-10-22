//! A key as the editors see it, from any host (a terminal through
//! crossterm, the browser through its events), and key to command.
//! Control keys come from [`KEYMAP`]: nano's file keys and Emacs
//! motions (Ctrl-A/E line start and end, Ctrl-B/F back and forward a
//! character, Ctrl-P/N previous and next line), Ctrl-T to zoom the
//! focused pane (for once not transpose; Ctrl-. too, for a browser
//! that keeps Ctrl-T), Ctrl-L to lint (again to
//! make the fixes); a printable character without Control inserts
//! itself.

/// What was pressed, without its modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    Char(char),
    Enter,
    Backspace,
    Delete,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    /// Shift-Tab.
    BackTab,
    Esc,
    /// A key the editors do not use.
    Other,
}

/// A key with its modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    pub code: Code,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Key {
    /// `code` with no modifier.
    pub fn plain(code: Code) -> Key {
        Key {
            code,
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    /// Control and a letter.
    pub fn ctrl(c: char) -> Key {
        Key {
            ctrl: true,
            ..Key::plain(Code::Char(c))
        }
    }
}

/// A terminal's key event (crossterm) as a key.
#[cfg(feature = "crossterm")]
impl From<ratatui::crossterm::event::KeyEvent> for Key {
    fn from(key: ratatui::crossterm::event::KeyEvent) -> Key {
        use ratatui::crossterm::event::{KeyCode, KeyModifiers};
        let code = match key.code {
            KeyCode::Char(c) => Code::Char(c),
            KeyCode::Enter => Code::Enter,
            KeyCode::Backspace => Code::Backspace,
            KeyCode::Delete => Code::Delete,
            KeyCode::Left => Code::Left,
            KeyCode::Right => Code::Right,
            KeyCode::Up => Code::Up,
            KeyCode::Down => Code::Down,
            KeyCode::Home => Code::Home,
            KeyCode::End => Code::End,
            KeyCode::PageUp => Code::PageUp,
            KeyCode::PageDown => Code::PageDown,
            KeyCode::Tab => Code::Tab,
            KeyCode::BackTab => Code::BackTab,
            KeyCode::Esc => Code::Esc,
            _ => Code::Other,
        };
        Key {
            code,
            ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
            alt: key.modifiers.contains(KeyModifiers::ALT),
            shift: key.modifiers.contains(KeyModifiers::SHIFT),
        }
    }
}

/// What the application (not the buffer) does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Save,
    Quit,
    Run,
    /// Lint the text: the findings in the bottom pane; again while they
    /// carry fixes makes them.
    Lint,
    /// Move the focus to the next (or previous) pane.
    NextPane,
    PrevPane,
    /// Toggle between all panes and the focused pane full screen.
    Zoom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Insert(char),
    Newline,
    Backspace,
    Delete,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    App(Action),
}

/// Keys with a fixed meaning, as (code, with Control, command).
pub const KEYMAP: &[(Code, bool, Command)] = &[
    (Code::Char('s'), true, Command::App(Action::Save)),
    (Code::Char('o'), true, Command::App(Action::Save)),
    (Code::Char('q'), true, Command::App(Action::Quit)),
    (Code::Char('x'), true, Command::App(Action::Quit)),
    (Code::Char('r'), true, Command::App(Action::Run)),
    (Code::Char('t'), true, Command::App(Action::Zoom)),
    (Code::Char('.'), true, Command::App(Action::Zoom)),
    (Code::Char('l'), true, Command::App(Action::Lint)),
    (Code::Char('a'), true, Command::Home),
    (Code::Char('e'), true, Command::End),
    (Code::Char('b'), true, Command::Left),
    (Code::Char('f'), true, Command::Right),
    (Code::Char('n'), true, Command::Down),
    (Code::Char('p'), true, Command::Up),
    (Code::Enter, false, Command::Newline),
    (Code::Backspace, false, Command::Backspace),
    (Code::Delete, false, Command::Delete),
    (Code::Left, false, Command::Left),
    (Code::Right, false, Command::Right),
    (Code::Up, false, Command::Up),
    (Code::Down, false, Command::Down),
    (Code::Home, false, Command::Home),
    (Code::End, false, Command::End),
    (Code::PageUp, false, Command::PageUp),
    (Code::PageDown, false, Command::PageDown),
    (Code::Tab, false, Command::App(Action::NextPane)),
    (Code::BackTab, false, Command::App(Action::PrevPane)),
];

/// The command for a key, if it has one (Shift does not matter; Alt
/// means none).
pub fn command(key: impl Into<Key>) -> Option<Command> {
    let key = key.into();
    if key.alt {
        return None;
    }
    let fixed = KEYMAP
        .iter()
        .find(|(code, ctrl, _)| *code == key.code && *ctrl == key.ctrl);
    match (fixed, key.code) {
        (Some((_, _, c)), _) => Some(*c),
        (None, Code::Char(c)) if !key.ctrl => Some(Command::Insert(c)),
        _ => None,
    }
}
