//! Countdown calculation logic — days until a target date.

use chrono::{Local, NaiveDate};

/// Returns the number of days until the date represented by `date_key` (YYYYMMDD).
/// Returns `None` if the date is in the past or today.
/// Replicates the JS `+1` behavior: adds 1 to the raw day difference.
pub fn days_until(date_key: &str) -> Option<u64> {
    let target = NaiveDate::parse_from_str(date_key, "%Y%m%d").ok()?;
    let today = Local::now().date_naive();
    let diff = (target - today).num_days();
    if diff > 0 {
        // Match JS: Math.floor(diff / day) + 1
        Some((diff as u64) + 1)
    } else {
        None
    }
}
