//! The system clock (the default on the command line): local time and
//! waiting.

use xetal_clock::{delay, now};

#[test]
fn the_stamp_is_seven_plausible_numbers() {
    let [year, month, day, hour, minute, second, milli] = now().expect("a clock");
    assert!(year >= 2026, "{year}");
    assert!((1..=12).contains(&month) && (1..=31).contains(&day));
    assert!((0..24).contains(&hour) && (0..60).contains(&minute));
    assert!((0..=60).contains(&second) && (0..1000).contains(&milli));
}

#[test]
fn a_delay_waits_at_least_as_long_as_asked() {
    let waited = delay(0.02).expect("waits");
    assert!(waited >= 0.02, "{waited}");
    assert!(waited < 2.0, "{waited}");
}

#[test]
fn a_negative_or_unknown_delay_is_refused() {
    assert!(delay(-1.0).is_err());
    assert!(delay(f64::NAN).is_err());
    assert!(delay(f64::INFINITY).is_err());
}
