//! The search index: every item of the documented files and every
//! built-in, by name and by type (types compared Hoogle-like, up to
//! the names of their type variables and without constraints).

use std::path::PathBuf;

use xetal_doc::model;
use xetal_docsearch::{document, entries, normalize};

fn app() -> Vec<xetal_doc::DocFile> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../doc/fixtures/app.xtl");
    let text = std::fs::read_to_string(&path).expect("fixture");
    model(&path.to_string_lossy(), &text).expect("model")
}

#[test]
fn types_are_compared_up_to_variable_names_and_constraints() {
    assert_eq!(normalize("Num a => a -> a -> a"), "a -> a -> a");
    assert_eq!(normalize("Num b => b -> b -> b"), "a -> a -> a");
    assert_eq!(normalize("(x -> y) -> [x]"), "( a -> b ) -> [ a ]");
    assert_eq!(normalize("Char -> Char"), "Char -> Char");
}

#[test]
fn the_index_holds_the_items_and_the_builtins() {
    let all = entries(&app());
    let plus = all.iter().find(|e| e.name == "+").expect("plus");
    assert_eq!(
        (plus.kind.as_str(), plus.norm.as_str()),
        ("built-in", "a -> a -> a")
    );
    assert_eq!(plus.href, "builtins.html#-2b");
    let hello = all.iter().find(|e| e.name == "l:h_ello").expect("hello");
    assert_eq!(hello.norm, "Char -> Char");
    assert!(hello.href.ends_with(".html#l.h_ello"), "{}", hello.href);
    assert_eq!(hello.about, "The greeting for a name.");
}

#[test]
fn the_site_is_written_with_its_search() {
    let dir = std::env::temp_dir().join(format!("xetal-docsearch-{}-site", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../doc/fixtures/app.xtl");
    let text = std::fs::read_to_string(&path).expect("fixture");
    let inputs = [(path.to_string_lossy().into_owned(), text)];
    let written = document(&inputs, &dir).expect("written");
    assert!(written.contains("search-index.js"), "{written}");
    let index = std::fs::read_to_string(dir.join("search-index.js")).expect("index");
    assert!(index.starts_with("window.XETAL_DOC_INDEX = ["), "{index}");
    assert!(dir.join("search.js").is_file());
    std::fs::remove_dir_all(&dir).expect("cleaned");
}

#[test]
fn a_file_is_named_by_its_path_relative_to_the_common_root() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../doc/fixtures/games");
    let files: Vec<_> = ["chess", "go"]
        .iter()
        .flat_map(|game| {
            let path = root.join(game).join("play.xtl");
            let text = std::fs::read_to_string(&path).expect("fixture");
            model(&path.to_string_lossy(), &text).expect("model")
        })
        .collect();
    let files_named: Vec<String> = entries(&files)
        .into_iter()
        .map(|e| e.file)
        .filter(|f| !f.is_empty())
        .collect();
    assert_eq!(files_named, ["chess/play.xtl", "go/play.xtl"]);
}
