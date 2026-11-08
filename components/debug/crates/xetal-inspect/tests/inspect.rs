//! Summaries and children of the values a debugger shows: scalars,
//! vectors, matrices paged by rows, strings, tuples and boxes.

use std::rc::Rc;

use xetal_array::Array;
use xetal_inspect::{children, count, summary, truthy, type_name};
use xetal_value::Value;

fn floats(shape: Vec<usize>, xs: &[f64]) -> Value<'static> {
    let items = xs.iter().map(|x| Value::Float(*x)).collect();
    Value::Array(Rc::new(Array::new(shape, items).unwrap()))
}

#[test]
fn a_summary_gives_the_type_the_shape_and_the_first_items() {
    assert_eq!(summary(&Value::Int(42)), "42");
    let m = floats(vec![4, 3], &(1..=12).map(f64::from).collect::<Vec<_>>());
    assert_eq!(
        summary(&m),
        "Float 4 3: 1.0 2.0 3.0 4.0 5.0 6.0 7.0 8.0 ..."
    );
    assert_eq!(type_name(&m), "Float");
    let s = Value::Array(Rc::new(Array::vector(
        "hi".chars().map(Value::Char).collect(),
    )));
    assert_eq!(summary(&s), "Char 2: \"hi\"");
    let t = Value::Tuple(Rc::from(vec![Value::Int(7), s]));
    assert_eq!(summary(&t), "(7, Char 2: \"hi\")");
    assert_eq!(type_name(&t), "(Int, Char)");
}

#[test]
fn a_matrix_opens_a_page_of_rows_and_a_row_its_items() {
    let m = floats(vec![4, 3], &(1..=12).map(f64::from).collect::<Vec<_>>());
    assert_eq!(count(&m), 4);
    let page = children(&m, 1, 2);
    let names: Vec<&str> = page.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["[2]", "[3]"]);
    assert_eq!(summary(&page[0].1), "Float 3: 4.0 5.0 6.0");
    assert_eq!(count(&page[0].1), 3);
    assert_eq!(
        children(&m, 3, 10).len(),
        1,
        "a page past the end is clipped"
    );
}

#[test]
fn tuples_open_to_parts_and_boxes_to_their_content() {
    let t = Value::Tuple(Rc::from(vec![Value::Int(1), Value::Float(2.5)]));
    let parts: Vec<String> = children(&t, 0, 9)
        .into_iter()
        .map(|(n, v)| format!("{n}={}", summary(&v)))
        .collect();
    assert_eq!(parts, [".1=1", ".2=2.5"]);
    let b = Value::Boxed(Rc::new(Value::Int(3)));
    assert_eq!(summary(&b), "Box of 3");
    assert_eq!(children(&b, 0, 1)[0].0, ">");
}

#[test]
fn conditions_are_truthy_as_the_language_reads_them() {
    assert!(truthy(&Value::Bool(true)) && truthy(&Value::Int(2)));
    assert!(!truthy(&Value::Int(0)) && !truthy(&Value::Float(0.0)));
}
