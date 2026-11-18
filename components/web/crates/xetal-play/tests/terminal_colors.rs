//! The interactive demos on the terminal (Saga 25, step
//! terminal-retrofit): where the terminal draws colors, as the live
//! demo's pane does, Mastermind's pegs come in color; piped, the text
//! is plain (the goldens).

use std::io::Write;
use std::sync::{Arc, Mutex, OnceLock};

use xetal_play::{Interactive, Step};
use xetal_store::{Memory, install};
use xetal_tty::Forward;

fn store() {
    static STORE: OnceLock<Arc<Memory>> = OnceLock::new();
    STORE.get_or_init(|| {
        let store = Arc::new(Memory::default());
        install(store.clone());
        store
    });
}

#[derive(Clone, Default)]
struct Shared(Arc<Mutex<Vec<u8>>>);

impl Write for Shared {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| std::io::Error::other("poisoned"))?
            .extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Run `src` with `seed`, feeding `lines` where it waits; its output.
fn play(src: &str, seed: u64, lines: &[&str]) -> String {
    store();
    let out = Shared::default();
    let mut run = Interactive::start(src, seed, Box::new(out.clone())).expect("starts");
    let mut fed = lines.iter();
    loop {
        match run.step(100_000) {
            Step::Running => {}
            Step::Done(_) => break,
            _ => match fed.next() {
                Some(line) => run.feed(line.to_string()),
                None => break,
            },
        }
    }
    let bytes = out.0.lock().unwrap().clone();
    String::from_utf8_lossy(&bytes).into_owned()
}

#[test]
fn mastermind_pegs_are_in_color_on_the_pane() {
    xetal_tty::install(Arc::new(Forward {
        error: |_| {},
        facts: [24, 80, 1, 1],
    }));
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../demos/classics/mastermind-play.xtl"
    ))
    .expect("demo");
    let out = play(&src, 3, &["1122", "3344"]);
    assert!(out.contains("3 3 4 4   \u{1b}["), "{out:?}");
    assert!(out.contains('*') && out.contains('o'), "{out:?}");
}
