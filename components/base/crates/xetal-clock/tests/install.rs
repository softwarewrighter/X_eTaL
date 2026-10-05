//! A host installs its own clock (the browser's worker does); this
//! binary installs one, so it has a test of its own.

use std::sync::Arc;
use xetal_clock::{Clock, Stamp, delay, install, now};

struct Fixed;

impl Clock for Fixed {
    fn now(&self) -> Result<Stamp, String> {
        Ok([2026, 10, 4, 21, 50, 0, 125])
    }
}

#[test]
fn an_installed_clock_is_read_and_one_without_waiting_refuses() {
    install(Arc::new(Fixed));
    assert_eq!(now().unwrap(), [2026, 10, 4, 21, 50, 0, 125]);
    assert!(delay(0.0).unwrap_err().contains("cannot wait"));
}
