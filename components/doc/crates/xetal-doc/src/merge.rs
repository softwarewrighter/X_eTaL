//! One model of several programs and libraries (one site): each file
//! once, even when found under two names (`lib/Stats.xtl` given, the
//! built-in `std/Stats.xtl` imported), kept under the first name, and
//! every reference to a dropped name moved to the kept one.

use xetal_base::Diagnostic;

use crate::files::model;
use crate::model::DocFile;

/// The documented files of every (name, text) of `inputs`, in order.
pub fn models(inputs: &[(String, String)]) -> Result<Vec<DocFile>, Diagnostic> {
    let (mut out, mut moved): (Vec<DocFile>, Vec<(String, String)>) = (Vec::new(), Vec::new());
    for (name, text) in inputs {
        for f in model(name, text)? {
            match out.iter().find(|k| same(k, &f)) {
                Some(k) if k.name != f.name => moved.push((f.name, k.name.clone())),
                Some(_) => {}
                None => out.push(f),
            }
        }
    }
    for f in &mut out {
        rename(f, &moved);
    }
    Ok(out)
}

/// The same file: the same name, or the same file name and text.
fn same(a: &DocFile, b: &DocFile) -> bool {
    let base = |f: &DocFile| f.name.rsplit('/').next().unwrap_or("").to_string();
    a.name == b.name || (base(a) == base(b) && a.text == b.text)
}

/// `f`'s references to a dropped file name moved to the kept one.
fn rename(f: &mut DocFile, moved: &[(String, String)]) {
    let kept = |n: &mut String| {
        if let Some((_, k)) = moved.iter().find(|(d, _)| d == n) {
            *n = k.clone();
        }
    };
    f.imports
        .iter_mut()
        .flat_map(|i| i.files.iter_mut())
        .for_each(kept);
    f.items
        .iter_mut()
        .flat_map(|i| i.uses.iter_mut())
        .for_each(|u| kept(&mut u.file));
}
