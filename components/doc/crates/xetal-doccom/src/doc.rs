//! A doc comment's prose and examples.

/// One `## >> code` example and the output shown under it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Example {
    pub code: String,
    pub output: String,
}

/// A doc comment: its prose (paragraphs split by a blank line), its
/// examples in order, and a macro's contract lines (MC31): the names
/// it binds for the text it is given on purpose (`## binds: it`), the
/// shape of call it takes (`## takes: @ text`) and whether it copies
/// or parses its arguments (`## uses: parsed`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Doc {
    pub text: String,
    pub examples: Vec<Example>,
    pub binds: Vec<String>,
    pub takes: Option<String>,
    pub uses: Option<String>,
}

impl Doc {
    /// The doc read from a block's lines, each with its `##` removed.
    pub(crate) fn from_lines(lines: &[&str]) -> Doc {
        let mut doc = Doc::default();
        let mut prose: Vec<&str> = Vec::new();
        let mut example: Option<(String, Vec<&str>)> = None;
        for line in lines {
            if let Some(names) = line.strip_prefix("binds:") {
                doc.binds.extend(names.split_whitespace().map(String::from));
            } else if let Some(shape) = line.strip_prefix("takes:") {
                doc.takes = Some(shape.split_whitespace().collect::<Vec<_>>().join(" "));
            } else if let Some(how) = line.strip_prefix("uses:") {
                doc.uses = Some(how.trim().to_string());
            } else if let Some(code) = line.strip_prefix(">> ") {
                doc.close(example.take());
                example = Some((code.trim().to_string(), Vec::new()));
            } else if let (Some((_, out)), false) = (example.as_mut(), line.is_empty()) {
                out.push(line);
            } else {
                doc.close(example.take());
                prose.push(line);
            }
        }
        doc.close(example);
        doc.text = prose.join("\n").trim_matches('\n').to_string();
        doc
    }

    fn close(&mut self, example: Option<(String, Vec<&str>)>) {
        if let Some((code, out)) = example {
            let output = out.join("\n");
            self.examples.push(Example { code, output });
        }
    }
}
