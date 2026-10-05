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

#[test]
fn the_time_stamp_is_seven_numbers() {
    let Some(Ok(Value::Array(ts))) = call("[]TS", &[], Span::new(0, 1)) else {
        panic!("[]TS is a vector")
    };
    assert_eq!(ts.data().len(), 7);
}

#[test]
fn a_delay_gives_the_seconds_waited_and_refuses_negative_ones() {
    let Some(Ok(Value::Float(waited))) = call("[]D_L", &[Value::Int(0)], Span::new(0, 1)) else {
        panic!("[]D_L gives a Float")
    };
    assert!(waited >= 0.0);
    assert_eq!(run("[]D_L", &[Value::Float(-0.5)]).unwrap_err(), "domain");
}

#[test]
fn a_signal_is_an_error_with_the_given_code_and_message() {
    let err = match call(
        "[]S_IGNAL",
        &[chars("too-big"), chars("9 wide at most")],
        Span::new(0, 1),
    ) {
        Some(Err(d)) => d,
        other => panic!("{other:?}"),
    };
    assert_eq!(
        (err.code.as_str(), err.message.as_str()),
        ("too-big", "9 wide at most")
    );
}

#[test]
fn a_signal_code_is_lowercase_digits_and_hyphens() {
    for bad in ["", "Bad Code", "x_y", "-lead"] {
        assert_eq!(
            run("[]S_IGNAL", &[chars(bad), chars("m")]).unwrap_err(),
            "bad-code",
            "{bad:?}"
        );
    }
    assert_eq!(
        run("[]S_IGNAL", &[chars("a-1"), chars("m")]).unwrap_err(),
        "a-1"
    );
}
