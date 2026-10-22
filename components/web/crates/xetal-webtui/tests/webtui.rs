//! The browser's keys become the editor's, and the editor's screen
//! becomes rows of styled runs: the same screen the terminal shows.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use xetal_edit::{Editor, StoreLibraries};
use xetal_keys::{Code, Key};
use xetal_webtui::{key_of, rows};

#[test]
fn browser_key_names_become_the_editors_keys() {
    assert_eq!(
        key_of("a", false, false, false),
        Some(Key::plain(Code::Char('a')))
    );
    assert_eq!(key_of("l", true, false, false), Some(Key::ctrl('l')));
    assert_eq!(
        key_of("ArrowDown", false, false, false),
        Some(Key::plain(Code::Down))
    );
    assert_eq!(
        key_of("Tab", false, false, true).map(|k| k.code),
        Some(Code::BackTab)
    );
    assert_eq!(
        key_of("Escape", false, false, false).map(|k| k.code),
        Some(Code::Esc)
    );
    assert_eq!(
        key_of("Shift", false, false, true),
        None,
        "a modifier alone"
    );
    assert_eq!(
        key_of("F5", false, false, false).map(|k| k.code),
        Some(Code::F(5))
    );
    assert_eq!(
        key_of("F12", false, false, false).map(|k| k.code),
        Some(Code::F(12))
    );
    assert_eq!(key_of("Fn", false, false, false), None);
}

/// The text typed through browser keys shows in the ASCII pane's rows,
/// drawn by the same editor the terminal runs; the cursor is marked.
#[test]
fn the_editor_draws_into_rows_with_the_cursor_marked() {
    let mut editor = Editor::with_text(std::path::Path::new("t.xtl"), "", Box::new(StoreLibraries));
    for name in ["1", " ", "+", " ", "2"] {
        editor.handle(key_of(name, false, false, false).unwrap());
    }
    let mut terminal = Terminal::new(TestBackend::new(60, 12)).unwrap();
    terminal.draw(|f| editor.draw(f)).unwrap();
    let at = terminal.backend().cursor_position();
    let grid = rows(terminal.backend().buffer(), Some((at.x, at.y)));
    assert_eq!(grid.len(), 12);
    let line1: String = grid[1].iter().map(|r| r.text.as_str()).collect();
    assert!(line1.contains("1 + 2"), "{line1:?}");
    assert!(grid.iter().flatten().any(|r| r.class.contains("cursor")));
    let bottom: String = grid.iter().flatten().map(|r| r.text.as_str()).collect();
    assert!(
        bottom.contains("Types") && bottom.contains("Int"),
        "{bottom}"
    );
}
