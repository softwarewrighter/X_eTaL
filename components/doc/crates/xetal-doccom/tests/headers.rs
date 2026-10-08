//! A file's header is its first `##` block, with or without a blank
//! line after it (X_eTaL-libraries had to add one to 37 files): it is
//! never given to the import, definition or section that follows.

use xetal_doccom::{doc_above, file_doc, stray_blocks};

fn text(doc: Option<xetal_doccom::Doc>) -> Option<String> {
    doc.map(|d| d.text)
}

#[test]
fn a_header_then_an_import_is_the_files() {
    let src = "## Stats helpers.\n\"s:\" u_se< \"Stats\"\nl:t_wice := { 2 * _r }\n";
    assert_eq!(text(file_doc(src)).as_deref(), Some("Stats helpers."));
}

#[test]
fn a_header_then_a_definition_is_the_files_not_the_definitions() {
    let src = "## Doubling.\nl:t_wice := { 2 * _r }\n";
    assert_eq!(text(file_doc(src)).as_deref(), Some("Doubling."));
    assert_eq!(text(doc_above(src, 2)), None);
}

#[test]
fn a_header_then_a_section_is_the_files() {
    let src = "## File doc.\n### Sizes\n## Twice.\nl:t_wice := { 2 * _r }\n";
    assert_eq!(text(file_doc(src)).as_deref(), Some("File doc."));
    assert_eq!(text(doc_above(src, 4)).as_deref(), Some("Twice."));
}

#[test]
fn after_a_shebang_and_with_a_blank_line_as_before() {
    let src = "#!/usr/bin/env xetal\n## A demo.\n\n## Twice.\nu:t_wice := { 2 * _r }\n";
    assert_eq!(text(file_doc(src)).as_deref(), Some("A demo."));
    assert_eq!(text(doc_above(src, 5)).as_deref(), Some("Twice."));
}

#[test]
fn a_block_that_documents_nothing_is_found() {
    let src = "## Header.\n\n## Lost: above an import.\n\"s:\" u_se< \"Stats\"\n## Twice.\nl:t_wice := { 2 * _r }\n";
    assert_eq!(stray_blocks(src, &[6]), [3]);
}
