//! A `##` block that documents nothing warns doc-unattached, but only at
//! the top level: inside a definition's own lines (above a local binding
//! in a body) it is a comment of that definition (X_eTaL-ML's Net.xtlm).

use xetal_doc::{model, warnings};

fn warned(src: &str) -> Vec<String> {
    let files = model("t.xtl", src).expect("model");
    warnings(&files).into_iter().map(|d| d.message).collect()
}

#[test]
fn blocks_inside_a_body_do_not_warn() {
    let src = "## Steps.\n\n## The step.\nu:s_tep := { x ->\n  ## Twice, first.\n  y := 2 * x\n  ## Then one more.\n  y + 1\n}\nu:s_tep 3\n";
    assert_eq!(warned(src), Vec::<String>::new());
}

#[test]
fn a_top_level_block_that_documents_nothing_still_warns() {
    let src = "## Steps.\n\n## Lost above an expression.\n1 + 2\n";
    let w = warned(src);
    assert_eq!(w.len(), 1, "{w:?}");
    assert!(w[0].ends_with("t.xtl:3"), "{w:?}");
}
