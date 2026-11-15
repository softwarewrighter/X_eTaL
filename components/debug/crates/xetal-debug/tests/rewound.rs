//! Going back over a file write (its own test binary: the store's
//! counts are process-wide): the stop names the file, which stays as
//! written, and running forward again does not write it again.

use xetal_debug::Debugger;

#[test]
fn a_file_written_after_the_stop_is_named_and_stays_written() {
    let path = std::env::temp_dir().join(format!("xetal-rewound-{}.txt", std::process::id()));
    let p = path.display().to_string();
    let src = format!("x := 1\n\"one\" []N_PUT \"{p}\"\ny := 2\n\"two\" []N_PUT \"{p}\"\nz := 3");
    let mut loaded = xetal_program::load("t.xtl", &src).expect("loads");
    xetal_types::check_program(&mut loaded.program).expect("checks");
    let mut out = Vec::new();
    let mut dbg = Debugger::new(&loaded.program, &loaded.sources, &mut out, 1);
    dbg.start();
    for _ in 0..4 {
        dbg.step_over();
    }
    let read = || std::fs::read_to_string(&path).unwrap_or_default();
    assert_eq!(read(), "two");
    let back = dbg.run_to(1);
    assert_eq!(back.place.map(|s| s.line), Some(1));
    assert_eq!(back.rewound, vec![p.clone(), p.clone()]);
    dbg.step_over();
    dbg.step_over();
    assert_eq!(read(), "two", "the first write is not made again");
    let _ = std::fs::remove_file(&path);
}
