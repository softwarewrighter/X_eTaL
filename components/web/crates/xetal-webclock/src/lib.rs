//! The browser's clock (QD2, QD3): `[]TS` reads JavaScript's `Date` in
//! the browser's time zone, and `[]D_L` waits by watching it. The live
//! demo's worker installs it (`xetal_clock::install`); a program runs in
//! the worker, so a wait never freezes the page.

use xetal_clock::{Clock, Stamp};

/// JavaScript's clock.
pub struct Browser;

impl Clock for Browser {
    fn now(&self) -> Result<Stamp, String> {
        let d = js_sys::Date::new_0();
        let date = [d.get_full_year(), d.get_month() + 1, d.get_date()];
        let time = [
            d.get_hours(),
            d.get_minutes(),
            d.get_seconds(),
            d.get_milliseconds(),
        ];
        let all: Vec<i64> = date.iter().chain(&time).map(|n| i64::from(*n)).collect();
        all.try_into()
            .map_err(|_| "the browser's date is incomplete".into())
    }

    fn delay(&self, seconds: f64) -> Result<f64, String> {
        let start = js_sys::Date::now();
        let end = start + seconds * 1000.0;
        while js_sys::Date::now() < end {}
        Ok((js_sys::Date::now() - start) / 1000.0)
    }
}
