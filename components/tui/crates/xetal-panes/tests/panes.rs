//! The source and rendered panes, drawn into a test terminal buffer.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer as Screen;
use ratatui::layout::Rect;
use xetal_buffer::Buffer;
use xetal_panes::Panes;

/// The screen as text, one string per row (combining marks kept).
fn rows(screen: &Screen) -> Vec<String> {
    let area = screen.area;
    (0..area.height)
        .map(|y| {
            (0..area.width)
                .map(|x| screen[(x, y)].symbol())
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

fn draw(buffer: &Buffer, width: u16, height: u16) -> (Vec<String>, Panes) {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    let panes = Panes::new(buffer, 0);
    terminal
        .draw(|f| f.render_widget(&panes, f.area()))
        .unwrap();
    (rows(terminal.backend().buffer()), panes)
}

#[test]
fn ascii_on_the_left_decorated_on_the_right() {
    let (screen, _) = draw(&Buffer::new("u:s_quare := { _r * _r }\nu:s_quare 7"), 64, 4);
    assert!(
        screen[1].starts_with("\u{2502}u:s_quare := { _r * _r }"),
        "{screen:?}"
    );
    assert!(
        screen[1].contains("\u{1d58}s\u{332}quare \u{2190} { \u{1d63} \u{d7} \u{1d63} }"),
        "{screen:?}"
    );
    assert!(screen[2].contains("\u{1d58}s\u{332}quare 7"), "{screen:?}");
}

#[test]
fn the_cursor_maps_into_both_panes() {
    let mut b = Buffer::new("x^2 + y");
    for _ in 0..4 {
        b.right();
    }
    let (_, panes) = draw(&b, 40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let (left, right) = panes.cursors(area);
    assert_eq!((left.x, left.y), (1 + 4, 1));
    assert_eq!((right.x, right.y), (21 + 3, 1));
}

#[test]
fn invalid_text_still_draws() {
    let (screen, _) = draw(&Buffer::new("3-1 r_ev x"), 40, 3);
    assert!(screen[1].contains("3-1 r\u{332}ev x"), "{screen:?}");
}

#[test]
fn scrolling_keeps_the_cursor_row_visible() {
    let mut b = Buffer::new("a\nb\nc\nd\ne");
    for _ in 0..4 {
        b.down();
    }
    assert_eq!(Panes::scroll_for(&b, 0, 2), 3);
    assert_eq!(Panes::scroll_for(&b, 4, 2), 4);
    let panes = Panes::new(&b, 3);
    let (screen, _) = (draw_with(&panes, 20, 4), ());
    assert!(screen[1].starts_with("\u{2502}d"), "{screen:?}");
}

fn draw_with(panes: &Panes, width: u16, height: u16) -> Vec<String> {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal.draw(|f| f.render_widget(panes, f.area())).unwrap();
    rows(terminal.backend().buffer())
}
