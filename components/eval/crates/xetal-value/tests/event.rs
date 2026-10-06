//! Events parsed from their lines (RS1) and printed back.

use xetal_value::Event;

#[test]
fn each_kind_parses_with_its_numbers_and_prints_as_its_line() {
    for line in [
        "tick 0.016",
        "down 120 80",
        "move 1 2",
        "up 0 0",
        "click 3 4",
        "end",
    ] {
        let e = Event::parse(line).expect(line);
        assert_eq!(e.kind, line.split(' ').next().unwrap());
    }
    assert_eq!(Event::parse("down 120 80").unwrap().at, vec![120.0, 80.0]);
    assert_eq!(Event::parse("tick 0.5").unwrap().at, vec![0.5]);
    assert_eq!(
        Event::parse("down 120 80").unwrap().to_string(),
        "down 120 80"
    );
    assert_eq!(Event::parse("tick 0.5").unwrap().to_string(), "tick 0.5");
    assert_eq!(Event::end().to_string(), "end");
}

#[test]
fn a_key_event_carries_its_key() {
    let e = Event::parse("key Up").expect("a key");
    assert_eq!((e.kind, e.key), ("key", xetal_value::key_named("Up")));
    assert_eq!(e.to_string(), "key UP");
    assert_eq!(Event::parse("key a").unwrap().to_string(), "key a");
}

#[test]
fn the_space_key_is_a_key_written_either_way() {
    // The page writes the key as the browser names it, a space after
    // the space that follows `key`; a fixture may write Space.
    let typed = Event::parse("key  ").expect("a space key");
    let named = Event::parse("key Space").expect("a space key by name");
    assert_eq!(typed.key, xetal_value::key_named(" "));
    assert_eq!(named.key, typed.key);
    assert_eq!(typed.to_string(), "key  ");
    assert!(Event::parse("key   ").is_none(), "two spaces are not a key");
}

#[test]
fn a_line_that_is_not_an_event_is_none() {
    for line in [
        "",
        "bogus",
        "down 1",
        "tick",
        "tick x",
        "end 1",
        "key",
        "key Enter Up",
    ] {
        assert!(Event::parse(line).is_none(), "{line:?}");
    }
}
