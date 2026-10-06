//! An array remembers the kind of its items (T9): numbers unless told,
//! kept by map and zip, and part of equality only when empty.

use xetal_array::{Array, Kind, zip};

#[test]
fn numbers_unless_told_and_kept_by_map_and_zip() {
    let empty: Array<i64> = Array::vector(Vec::new());
    assert_eq!(empty.kind(), Kind::Number);
    let text = Array::vector(Vec::<char>::new()).with_kind(Kind::Char);
    assert_eq!(text.kind(), Kind::Char);
    let mapped = text.map(|c| Ok::<_, ()>(*c)).unwrap();
    assert_eq!(mapped.kind(), Kind::Char);
    let zipped = zip(&text, &text, |a, _| Ok::<_, xetal_array::ArrayError>(*a)).unwrap();
    assert_eq!(zipped.kind(), Kind::Char);
}

#[test]
fn empty_arrays_are_equal_only_in_one_kind() {
    let numbers: Array<char> = Array::vector(Vec::new());
    let text = Array::vector(Vec::new()).with_kind(Kind::Char);
    assert_ne!(numbers, text);
    assert_eq!(text, Array::vector(Vec::new()).with_kind(Kind::Char));
    let a = Array::vector(vec!['a']);
    assert_eq!(a, Array::vector(vec!['a']).with_kind(Kind::Char));
}
