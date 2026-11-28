//! The transcript: what the pane shows, as text, for tests.

use xetal_runner::Output;

/// Output as a terminal shows it: plain text as printed, or, when the
/// program placed or styled its text, the grid's 24 rows of 80
/// characters with trailing spaces and empty last rows dropped.
pub fn screen_text(out: &str) -> String {
    if !out.contains('\x1b') {
        return out.to_string();
    }
    let mut grid = xetal_screen::Grid::new(24, 80);
    grid.write(out);
    let mut rows: Vec<String> = grid
        .rows()
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| c.ch)
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect();
    while rows.last().is_some_and(String::is_empty) {
        rows.pop();
    }
    rows.into_iter().map(|r| r + "\n").collect()
}

/// The run's transcript: its output (typed lines echoed, as shown),
/// then its errors, then a line per picture; empty before a run.
pub fn transcript(output: &Output) -> String {
    let Some(r) = &output.run else {
        return String::new();
    };
    let pictures = r.pictures.iter().map(|_| "[picture]\n");
    let mut text = screen_text(&r.out) + &r.err;
    text.extend(pictures);
    text
}
