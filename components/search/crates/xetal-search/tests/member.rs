//! `m_ember?` (ask G10): each item's membership is what `=` says against
//! every item of the right side, whether the long-vector path hashes or
//! compares, and a long vector against a long vector is quick.

use std::rc::Rc;

use xetal_array::Array;
use xetal_base::Span;
use xetal_search::{call, equal};
use xetal_value::Value;

fn array(items: Vec<Value<'static>>) -> Value<'static> {
    Value::Array(Rc::new(Array::new(vec![items.len()], items).unwrap()))
}

fn member(x: &[Value<'static>], y: &[Value<'static>]) -> Vec<bool> {
    let got = call(
        "m_ember?",
        &[array(x.to_vec()), array(y.to_vec())],
        Span::default(),
    )
    .unwrap()
    .unwrap();
    let Value::Array(a) = got else {
        panic!("{got}")
    };
    a.data()
        .iter()
        .map(|v| matches!(v, Value::Bool(true)))
        .collect()
}

fn expected(x: &[Value<'static>], y: &[Value<'static>]) -> Vec<bool> {
    x.iter().map(|v| y.iter().any(|w| equal(v, w))).collect()
}

fn items() -> Vec<Value<'static>> {
    use Value::*;
    vec![
        Int(3),
        Float(3.0),
        Float(-0.0),
        Int(0),
        Bool(true),
        Int(1),
        Char('a'),
        Char('3'),
        Float(2.5),
        Int(-7),
        Float(1e300),
        Int(i64::MAX),
        Float(f64::NAN),
        Int(1 << 53),
        Float((1u64 << 53) as f64),
        Int((1 << 53) + 1),
    ]
}

#[test]
fn membership_agrees_with_equality_on_mixed_items() {
    let all = items();
    for split in 0..all.len() {
        for skip in 0..all.len() {
            let x: Vec<_> = all.iter().skip(skip).cloned().collect();
            let y: Vec<_> = all.iter().take(split).cloned().collect();
            assert_eq!(member(&x, &y), expected(&x, &y), "x {x:?} y {y:?}");
        }
    }
}

#[test]
fn a_long_vector_against_a_long_vector_is_quick() {
    let x: Vec<_> = (0..207_360).map(|i| Value::Int(i % 50_000)).collect();
    let y: Vec<_> = (0..20_000).map(Value::Int).collect();
    let start = std::time::Instant::now();
    let got = member(&x, &y).iter().filter(|b| **b).count();
    assert_eq!(got, 87_360);
    assert!(start.elapsed().as_secs_f64() < 1.0, "{:?}", start.elapsed());
}
