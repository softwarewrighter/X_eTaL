//! Keys written as text, in Vim's notation: a plain character types
//! itself, and a name in angle brackets is a key: `<Enter>`, `<Tab>`,
//! `<S-Tab>`, `<Esc>`, `<BS>`, `<Del>`, the arrows `<Up>` `<Down>`
//! `<Left>` `<Right>`, `<Home>`, `<End>`, `<PageUp>`, `<PageDown>`,
//! `<F1>` to `<F12>`, Control with a character `<C-r>`, and `<lt>` for
//! a `<` typed. `xetal edit --screen --keys` reads it, and the browser's
//! test types the same keys.

use crate::{Code, Key};

/// The keys `spec` names, in order.
pub fn keys(spec: &str) -> Result<Vec<Key>, String> {
    let mut out = Vec::new();
    let mut rest = spec;
    while let Some(c) = rest.chars().next() {
        if c != '<' {
            out.push(Key::plain(Code::Char(c)));
            rest = &rest[c.len_utf8()..];
            continue;
        }
        let end = rest
            .find('>')
            .ok_or_else(|| format!("unclosed key name in {rest:?} (write <lt> for <)"))?;
        out.push(named(&rest[1..end])?);
        rest = &rest[end + 1..];
    }
    Ok(out)
}

/// The key a name in angle brackets stands for.
fn named(name: &str) -> Result<Key, String> {
    if let Some(c) = name.strip_prefix("C-").filter(|c| c.chars().count() == 1) {
        let c = c.chars().next().unwrap_or(' ');
        return Ok(Key::ctrl(c));
    }
    let code = match name {
        "Enter" | "CR" => Code::Enter,
        "Tab" => Code::Tab,
        "S-Tab" => Code::BackTab,
        "Esc" => Code::Esc,
        "BS" => Code::Backspace,
        "Del" => Code::Delete,
        "Up" => Code::Up,
        "Down" => Code::Down,
        "Left" => Code::Left,
        "Right" => Code::Right,
        "Home" => Code::Home,
        "End" => Code::End,
        "PageUp" => Code::PageUp,
        "PageDown" => Code::PageDown,
        "lt" => Code::Char('<'),
        "Space" => Code::Char(' '),
        _ => match name.strip_prefix('F').and_then(|n| n.parse::<u8>().ok()) {
            Some(n @ 1..=12) => Code::F(n),
            _ => return Err(format!("unknown key name <{name}>")),
        },
    };
    Ok(Key::plain(code))
}
