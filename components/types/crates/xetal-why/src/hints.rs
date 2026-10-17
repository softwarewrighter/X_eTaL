//! Hints for the common type mistakes (Saga 27 step 3, D164): read
//! from the mismatch's own words (`expected E, found F`), each a
//! `help:` saying what to write instead.

use xetal_base::Diagnostic;

/// `d` with a hint, when it is a type mismatch a hint is known for.
pub fn hinted(d: Diagnostic) -> Diagnostic {
    let hint = match d.code.as_str() {
        "type-mismatch" => d.message.split_once(", found ").and_then(|(e, f)| {
            let expected = e.strip_prefix("expected ")?;
            hint(expected, f)
        }),
        _ => None,
    };
    match hint {
        Some(h) => d.with_help(h),
        None => d,
    }
}

/// The hint for a value of type `found` where `expected` is wanted.
fn hint(expected: &str, found: &str) -> Option<&'static str> {
    let function = |t: &str| t.contains("->");
    let boxed = |t: &str| t.starts_with("Box ");
    Some(match (expected, found) {
        ("Float", "Int") | ("Int", "Float") => {
            "Int and Float do not mix: `f_loat n` makes a Float of an Int, `f_loor x` or `c_eiling x` an Int of a Float"
        }
        ("Bool or Int", "Float") => {
            "a Float is not a condition or a count: `f_loor x` makes an Int of it, a comparison (`x > 0`) a Bool"
        }
        ("a number", "Char") => "a string is text, not a number: write a number without quotes",
        (e, f) if boxed(f) && !boxed(e) => "`d_isclose b` opens a box to get the value inside",
        (e, f) if boxed(e) && !boxed(f) => "`e_nclose x` puts a value in a box",
        (e, f) if function(f) && !function(e) => {
            "this is a function still waiting for an argument: give it one, or quote it ('f_) to pass the function itself"
        }
        (e, f) if function(e) && !function(f) => {
            "a value is applied as if it were a function: is there an argument too many, or a function passed without its quote?"
        }
        _ => return None,
    })
}
