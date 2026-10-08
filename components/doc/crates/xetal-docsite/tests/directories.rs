//! Files with the same name in different directories (X_eTaL-games has
//! eleven play.xtl): the side bar groups files under their directory,
//! relative to the documented files' common root (the search index
//! names files by that relative path too: xetal-docsearch's tests).

use std::path::PathBuf;

use xetal_doc::{DocFile, model};
use xetal_docsite::site;

fn two_plays() -> Vec<DocFile> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/games");
    // Given out of order: the side bar sorts them.
    ["go", "chess"]
        .iter()
        .flat_map(|game| {
            let path = root.join(game).join("play.xtl");
            let text = std::fs::read_to_string(&path).expect("fixture");
            model(&path.to_string_lossy(), &text).expect("model")
        })
        .collect()
}

fn index(files: &[DocFile]) -> String {
    let pages = site(files);
    let found = pages.iter().find(|(p, _)| p == "index.html");
    found.expect("index").1.clone()
}

#[test]
fn the_side_bar_groups_files_under_their_directories() {
    let html = index(&two_plays());
    let chess = html.find("<h3>chess/</h3>").expect("a chess heading");
    let go = html.find("<h3>go/</h3>").expect("a go heading");
    assert!(chess < go, "directories in alphabetical order");
    let play = html[chess..go].find(">play.xtl</a>");
    assert!(play.is_some(), "chess's play.xtl listed under chess/");
    assert!(
        html[go..].contains(">play.xtl</a>"),
        "go's play.xtl under go/"
    );
}
