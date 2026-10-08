//! The frame of every page: head, the side bar (the files, and what is
//! on this page), the theme switch.

use xetal_base::LANG_NAME;
use xetal_doc::DocFile;
use xetal_dochtml::{escape, page, relative};

use crate::context::Ctx;

/// A whole page: `title`, the side bar's `toc` (this page's own
/// entries, HTML list items) and the `body`.
pub(crate) fn shell(cx: &Ctx, title: &str, toc: &str, body: &str) -> String {
    let side = sidebar(cx, toc);
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{} - {LANG_NAME} doc</title>\n<link rel=\"stylesheet\" href=\"style.css\">\n\
         <script src=\"theme.js\"></script>\n<script defer src=\"search-index.js\"></script>\n\
         <script defer src=\"search.js\"></script>\n</head>\n<body>\n<nav class=\"side\">\n{side}</nav>\n\
         <main>\n<button class=\"theme\" title=\"Light or dark\">theme</button>\n\
         <section id=\"results\"></section>\n{body}</main>\n\
         </body>\n</html>\n",
        escape(title)
    )
}

/// The search box: a name, or a type such as `Num a => a -> a`.
const SEARCH: &str = "<input id=\"search\" type=\"search\" autocomplete=\"off\" \
     placeholder=\"Search: a name, or a type\" title=\"A name (mean finds m_ean), or a type: \
     Num a =&gt; a -&gt; a -&gt; a finds +, m_ax ...\">\n";

/// The side bar: home, every file by kind, then this page's entries.
fn sidebar(cx: &Ctx, toc: &str) -> String {
    let mut out = format!("<a class=\"home\" href=\"index.html\">{LANG_NAME} doc</a>\n");
    out.push_str(SEARCH);
    out.push_str("<h2>Files</h2>\n");
    for (dir, files) in by_directory(cx) {
        if !dir.is_empty() {
            out.push_str(&format!("<h3>{}</h3>\n", escape(&dir)));
        }
        out.push_str("<ul>\n");
        for f in files {
            let name = escape(short(&f.name));
            out.push_str(&format!(
                "<li><a href=\"{}.html\">{name}</a></li>\n",
                page(&f.name)
            ));
        }
        out.push_str("</ul>\n");
    }
    out.push_str("<ul>\n<li><a href=\"builtins.html\">built-ins</a></li>\n</ul>\n");
    if !toc.is_empty() {
        out.push_str(&format!("<h2>On this page</h2>\n<ul>\n{toc}</ul>\n"));
    }
    out
}

/// The files grouped by their directory relative to the site's root
/// (with its `/`; empty for files in the root, listed first), the
/// directories and the files in each in alphabetical order, case aside: same-named
/// files in different directories (eleven play.xtl) stay apart.
fn by_directory<'a>(cx: &Ctx<'a>) -> Vec<(String, Vec<&'a DocFile>)> {
    let mut groups: Vec<(String, Vec<&DocFile>)> = Vec::new();
    for f in cx.files {
        let rel = relative(&cx.root, &f.name);
        let dir = rel
            .rsplit_once('/')
            .map_or(String::new(), |(d, _)| format!("{d}/"));
        match groups.iter_mut().find(|(d, _)| *d == dir) {
            Some((_, list)) => list.push(f),
            None => groups.push((dir, vec![f])),
        }
    }
    // Alphabetical as a reader expects: case aside, then as written.
    let key = |s: &str| (s.to_lowercase(), s.to_string());
    groups.sort_by_key(|(dir, _)| key(dir));
    for (_, list) in &mut groups {
        list.sort_by_key(|f| key(short(&f.name)));
    }
    groups
}

/// A file's name without its directories.
fn short(name: &str) -> &str {
    name.rsplit('/').next().unwrap_or(name)
}
