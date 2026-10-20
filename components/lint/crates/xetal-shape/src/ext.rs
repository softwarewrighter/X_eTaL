//! Symbolic extents and shapes, and what the lint knows of a value.

/// One extent: a number, or a name bound to a single number the lint
/// cannot see, with the binding's number so a later binding of the
/// same name is a different extent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ext {
    Num(i64),
    Name(String, usize),
}

/// A shape whose rank is known: one extent per axis.
pub(crate) type Shape = Vec<Ext>;

/// What the lint knows of a value: its shape, its items when it is a
/// vector of extents (`2 3`, `N c_at 3`, `s_hape X`), and its parts
/// when it is a tuple (`(1, 3)`), each a fact of its own.
#[derive(Debug, Clone, Default)]
pub(crate) struct Fact {
    pub shape: Option<Shape>,
    pub value: Option<Vec<Ext>>,
    pub parts: Option<Vec<Fact>>,
}

impl Fact {
    pub fn shaped(shape: Option<Shape>) -> Fact {
        Fact {
            shape,
            value: None,
            parts: None,
        }
    }

    /// A tuple of `parts`.
    pub fn tuple(parts: Vec<Fact>) -> Fact {
        Fact {
            shape: None,
            value: None,
            parts: Some(parts),
        }
    }
}

/// Whether two shapes surely differ: another rank, or two numbers that
/// differ on some axis. Names never differ from anything.
pub(crate) fn differ(a: &Shape, b: &Shape) -> bool {
    let num = |x: &Ext, y: &Ext| matches!((x, y), (Ext::Num(m), Ext::Num(n)) if m != n);
    a.len() != b.len() || a.iter().zip(b).any(|(x, y)| num(x, y))
}

/// Whether two shapes are surely equal: extent by extent the same
/// number or the same binding.
pub(crate) fn same(a: &Shape, b: &Shape) -> bool {
    a == b
}

/// A shape as the run-time error writes it (`2 3`; names as written;
/// a scalar is `a scalar`).
pub(crate) fn show(s: &Shape) -> String {
    let words: Vec<String> = s
        .iter()
        .map(|e| match e {
            Ext::Num(n) => n.to_string(),
            Ext::Name(name, _) => name.clone(),
        })
        .collect();
    match words.is_empty() {
        true => "a scalar".to_string(),
        false => words.join(" "),
    }
}
