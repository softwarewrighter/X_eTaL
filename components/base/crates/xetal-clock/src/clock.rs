//! What a clock is, and the system's.

/// A local time stamp: year, month, day, hour, minute, second and
/// millisecond, as APL's quad TS gives it.
pub type Stamp = [i64; 7];

/// A clock: the local time, and waiting. Either may be refused.
pub trait Clock: Send + Sync {
    /// The local time now.
    fn now(&self) -> Result<Stamp, String> {
        Err("this host has no clock".into())
    }

    /// Wait `seconds` (finite and not negative, checked by the caller);
    /// the seconds actually waited.
    fn delay(&self, _seconds: f64) -> Result<f64, String> {
        Err("this host cannot wait".into())
    }
}

/// The operating system's clock, in its local time zone.
pub struct System;

#[cfg(not(target_arch = "wasm32"))]
impl Clock for System {
    fn now(&self) -> Result<Stamp, String> {
        use chrono::{Datelike, Timelike};
        let t = chrono::Local::now();
        let milli = i64::from(t.timestamp_subsec_millis().min(999));
        let (h, m, s) = (t.hour(), t.minute(), t.second());
        let day = [t.year().into(), t.month().into(), t.day().into()];
        Ok([day[0], day[1], day[2], h.into(), m.into(), s.into(), milli])
    }

    fn delay(&self, seconds: f64) -> Result<f64, String> {
        let start = std::time::Instant::now();
        std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
        Ok(start.elapsed().as_secs_f64())
    }
}

/// In the browser the system's clock is not reachable from Rust: a
/// host installs its own (the worker does).
#[cfg(target_arch = "wasm32")]
impl Clock for System {}
