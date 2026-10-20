//! Without KaTeX (every native build), a notation is its LaTeX, escaped,
//! so a page shows what was written rather than nothing.

use xetal_katex::{escaped, html};

#[test]
fn without_katex_the_latex_is_shown_escaped() {
    assert_eq!(html("a < b"), "a &lt; b");
    assert_eq!(escaped("\"x\" & <y>"), "&quot;x&quot; &amp; &lt;y&gt;");
}
