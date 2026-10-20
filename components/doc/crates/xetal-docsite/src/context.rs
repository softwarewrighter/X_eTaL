//! What every page needs: the files, the resolver, where each item is
//! used, and how a target becomes a URL.

use std::collections::BTreeMap;

use xetal_doc::DocFile;
use xetal_dochtml::{Notation, anchor, code, code_with, page};
use xetal_doclink::{Resolver, Target, links, uses};

pub(crate) struct Ctx<'a> {
    pub files: &'a [DocFile],
    pub r: Resolver<'a>,
    /// Per item, the (file, line) places it is used.
    pub uses: BTreeMap<Target, Vec<(usize, usize)>>,
    /// The directory all the files are in: names are shown relative to it.
    pub root: String,
}

impl<'a> Ctx<'a> {
    pub(crate) fn new(files: &'a [DocFile]) -> Self {
        let r = Resolver::new(files);
        let uses = uses(&r);
        let names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
        let root = xetal_dochtml::root(&names);
        Ctx {
            files,
            r,
            uses,
            root,
        }
    }

    /// The URL of `target`, from any page (the site is flat).
    pub(crate) fn href(&self, target: Target) -> String {
        let file = |f: usize| page(&self.files[f].name);
        match target {
            Target::Item { file: f, item } => {
                format!(
                    "{}.html#{}",
                    file(f),
                    anchor(&self.files[f].items[item].name)
                )
            }
            Target::File(f) => format!("{}.html", file(f)),
            Target::Builtin(b) => format!("builtins.html#{}", anchor(b)),
        }
    }

    /// `text`, written in file `file`, drawn decorated and linked.
    pub(crate) fn draw(&self, file: usize, text: &str) -> String {
        code(text, &links(&self.r, file, text), &|t| self.href(t))
    }

    /// `text`, the lines of file `file` from line `line`, drawn as
    /// [`Ctx::draw`] does, with its notation calls drawn (MC32).
    pub(crate) fn draw_lines(&self, file: usize, line: usize, text: &str) -> String {
        let notations = self.notations(file, line, text.len());
        code_with(text, &notations, &links(&self.r, file, text), &|t| {
            self.href(t)
        })
    }

    /// The notations of file `file` within the `len` bytes from its
    /// line `line`, their spans relative to that text.
    pub(crate) fn notations(&self, file: usize, line: usize, len: usize) -> Vec<Notation> {
        let f = &self.files[file];
        let offset = f
            .text
            .lines()
            .take(line - 1)
            .map(|l| l.len() + 1)
            .sum::<usize>();
        let within = |span: &xetal_base::Span| span.start >= offset && span.end <= offset + len;
        let shift = |span: Option<xetal_base::Span>| {
            span.map(|s| xetal_base::Span::new(s.start - offset, s.end - offset))
        };
        f.notations
            .iter()
            .filter(|n| within(&n.call))
            .map(|n| Notation {
                left: shift(n.left),
                right: shift(n.right),
                latex: n.latex.clone(),
            })
            .collect()
    }

    /// `text` drawn as [`Ctx::draw`] does, its names resolved by `r`
    /// (an example's session).
    pub(crate) fn draw_with(&self, r: &Resolver, file: usize, text: &str) -> String {
        code(text, &links(r, file, text), &|t| self.href(t))
    }

    /// A name drawn decorated, not linked.
    pub(crate) fn name(&self, text: &str) -> String {
        code(text, &[], &|_| String::new())
    }
}
