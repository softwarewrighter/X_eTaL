//! The live demo's engine, run natively against a store in memory.

use std::sync::{Arc, OnceLock};

use xetal_play::{check, run};
use xetal_store::{Memory, Store, install};

/// One store for every test here (the store in use is global).
fn store() -> &'static Arc<Memory> {
    static STORE: OnceLock<Arc<Memory>> = OnceLock::new();
    STORE.get_or_init(|| {
        let store = Arc::new(Memory::default());
        install(store.clone());
        store
    })
}

#[test]
fn a_program_runs_and_its_output_is_kept() {
    store();
    let r = run("1 + 2\n'+ r_/ 1 2 3", 1);
    assert_eq!((r.out.as_str(), r.err.as_str()), ("3\n6\n", ""));
}

#[test]
fn a_library_comes_from_the_store_then_the_standard_ones() {
    store().put("Sq.xtl", "l:s_q := { _r * _r }\n").unwrap();
    let r = run("\"q:\" u_se< \"Sq\"\nq:s_q 4", 1);
    assert_eq!(r.out, "16\n", "{}", r.err);
    let r = run("\"s:\" u_se< \"Stats\"\ns:m_ean 1 2 3", 1);
    assert_eq!(r.out, "2.0\n", "{}", r.err);
}

#[test]
fn files_are_written_to_and_read_from_the_store() {
    let r = run(
        "\"1 2\" []N_PUT \"work/m.txt\"\nn_umbers []N_GET \"work/m.txt\"",
        1,
    );
    assert_eq!(r.out, "3\n1.0 2.0\n", "{}", r.err);
    assert_eq!(store().get("work/m.txt").unwrap(), "1 2");
}

#[test]
fn checking_gives_the_types_or_the_first_error() {
    store();
    assert_eq!(
        check("u:s_q := { _r * _r }\nu:s_q 3"),
        ["u:s_q : Num a => a -> a", "Int"]
    );
    let bad = check("1 + \"a\"");
    assert!(bad[0].starts_with("error[type-mismatch]"), "{bad:?}");
    let r = run("1 + \"a\"", 1);
    assert!(r.err.starts_with("error[type-mismatch]"), "{}", r.err);
}

#[test]
fn a_library_shows_its_exports_and_their_types() {
    store();
    let lib = "l:t_wice := { 2 * _r }\nh_alf := { _r / 2 }\n";
    assert_eq!(check(lib), ["l:t_wice : Num a => a -> a"]);
    assert_eq!(run(lib, 1).out, "l:t_wice : Num a => a -> a\n");
}

#[test]
fn a_line_is_read_from_the_store_in_use() {
    store().push_line("hello");
    assert_eq!(run("[]R_EAD @", 1).out, "hello\n");
}
