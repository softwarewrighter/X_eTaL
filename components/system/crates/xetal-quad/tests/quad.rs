//! The system values and character codes (QD2, QD7).

use xetal_base::Span;
use xetal_quad::call;
use xetal_value::{Value, printed};

fn run(name: &str, args: &[Value<'static>]) -> Result<String, String> {
    match call(name, args, Span::new(0, 1)) {
        Some(Ok(v)) => Ok(printed(&v)),
        Some(Err(d)) => Err(d.code.to_string()),
        None => Err("not a quad".into()),
    }
}

fn chars(text: &str) -> Value<'static> {
    xetal_value::to_value(xetal_array::Array::vector(
        text.chars().map(Value::Char).collect(),
    ))
}

#[test]
fn the_values_are_the_alphabet_digits_and_origin() {
    assert_eq!(run("[]A", &[]).unwrap(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(run("[]D", &[]).unwrap(), "0123456789");
    assert_eq!(run("[]IO", &[]).unwrap(), "1");
}

#[test]
fn the_atomic_vector_is_every_ascii_character() {
    let Some(Ok(Value::Array(av))) = call("[]AV", &[], Span::new(0, 1)) else {
        panic!("[]AV is a vector")
    };
    assert_eq!(av.data().len(), 128);
    assert!(matches!(av.data()[65], Value::Char('A')));
}

#[test]
fn codes_and_characters_go_both_ways() {
    assert_eq!(run("[]U_CS", &[chars("Hi")]).unwrap(), "72 105");
    let codes = xetal_value::to_value(xetal_array::Array::vector(vec![
        Value::Int(72),
        Value::Int(105),
    ]));
    assert_eq!(run("[]U_CHAR", &[codes]).unwrap(), "Hi");
}

#[test]
fn a_code_outside_ascii_is_a_domain_error() {
    assert_eq!(run("[]U_CHAR", &[Value::Int(200)]).unwrap_err(), "domain");
    assert_eq!(run("[]U_CHAR", &[Value::Int(-1)]).unwrap_err(), "domain");
}

#[test]
fn other_names_are_not_quads() {
    assert_eq!(run("[]N_GET", &[]).unwrap_err(), "not a quad");
}
