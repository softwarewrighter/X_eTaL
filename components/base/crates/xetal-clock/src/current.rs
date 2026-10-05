//! The clock in use: the system's until a host installs another.

use std::sync::{Arc, RwLock};

use crate::{Clock, Stamp, System};

static CURRENT: RwLock<Option<Arc<dyn Clock>>> = RwLock::new(None);

/// Use `clock` from now on.
pub fn install(clock: Arc<dyn Clock>) {
    if let Ok(mut current) = CURRENT.write() {
        *current = Some(clock);
    }
}

fn current() -> Arc<dyn Clock> {
    let installed = CURRENT.read().ok().and_then(|c| c.clone());
    installed.unwrap_or_else(|| Arc::new(System))
}

/// The local time now, from the clock in use.
pub fn now() -> Result<Stamp, String> {
    current().now()
}

/// Wait `seconds` on the clock in use; a negative, infinite or unknown
/// number of seconds is refused before any waiting.
pub fn delay(seconds: f64) -> Result<f64, String> {
    if !(seconds.is_finite() && seconds >= 0.0) {
        return Err(format!(
            "a delay is a finite number of seconds, 0 or more, not {seconds}"
        ));
    }
    current().delay(seconds)
}
