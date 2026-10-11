//! A notebook reads typed lines as a plain run does: it shows each
//! statement before it runs, and at []R_EAD it waits for a fed line
//! (the live demo's Notebook and Step on Mastermind failed instead).

use std::io::Write;
use std::sync::{Arc, Mutex, OnceLock};

use xetal_play::{Interactive, Step};
use xetal_store::{Memory, install};

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

/// Run `src` as a notebook, feeding `lines` as it waits; the cells shown,
/// the output, and the run's error.
fn notebook(src: &str, lines: &[&str]) -> (Vec<String>, String, String) {
    store();
    let cells = Arc::new(Mutex::new(Vec::new()));
    let seen = cells.clone();
    let cell = Box::new(move |c: &str| seen.lock().unwrap().push(c.to_string()));
    let out = Shared::default();
    let mut run = match Interactive::notebook(src, 1, None, cell, Box::new(out.clone())) {
        Ok(run) => run,
        Err(done) => return (Vec::new(), done.out, done.err),
    };
    let mut fed = lines.iter();
    let err = loop {
        match run.step(10_000) {
            Step::Running => {}
            Step::Waiting | Step::WaitingKey | Step::WaitingEvent => match fed.next() {
                Some(line) => run.feed(line.to_string()),
                None => break String::from("<still waiting>"),
            },
            Step::Done(done) => break done.err,
        }
    };
    let text = String::from_utf8_lossy(&out.0.lock().unwrap()).into_owned();
    let cells = cells.lock().unwrap().clone();
    (cells, text, err)
}

#[test]
fn a_notebook_waits_for_typed_lines() {
    let src = "# a greeting\nname := []R_EAD @\n\"hello \" c_at name\nage := []R_EAD @\nage\n";
    let (cells, out, err) = notebook(src, &["Ada", "36"]);
    assert_eq!(err, "");
    assert_eq!(out, "hello Ada\n36\n");
    assert_eq!(cells[0], "# a greeting\nname := []R_EAD @");
    assert_eq!(cells.len(), 4, "{cells:?}");
}

#[test]
fn mastermind_plays_in_a_notebook() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../demos/classics/mastermind-play.xtl"
    ))
    .expect("demo");
    let (cells, out, err) = notebook(&src, &["1122", "3344"]);
    assert_eq!(
        err, "<still waiting>",
        "it asks again after two guesses: {out}"
    );
    assert!(!cells.is_empty() && out.contains("guess 1"), "{out}");
}
