//! Countdown calculation logic — days until a target date.

use chrono::{Local, NaiveDate};

const DATE_FORMATS: &[&str] = &[
    "%Y-%m-%d",   // 2025-12-25
    "%Y/%m/%d",   // 2025/12/25
    "%m/%d/%Y",   // 12/25/2025
    "%m-%d-%Y",   // 12-25-2025
    "%d/%m/%Y",   // 25/12/2025
    "%d-%m-%Y",   // 25-12-2025
    "%Y%m%d",     // 20251225
];

/// Returns the number of days until the date represented by `date_key` (YYYYMMDD).
/// Returns `None` if the date is in the past or today.
/// Replicates the JS `+1` behavior: adds 1 to the raw day difference.
pub fn days_until(date_key: &str) -> Option<u64> {
    let target = NaiveDate::parse_from_str(date_key, "%Y%m%d").ok()?;
    days_until_naive(target)
}

/// Parse a date string in various formats and return days until that date.
/// Returns `None` if the string cannot be parsed or the date is in the past.
pub fn days_until_from_str(s: &str) -> Option<u64> {
    let target = parse_date(s)?;
    days_until_naive(target)
}

/// Try to parse a date string using common formats.
pub fn parse_date(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    for fmt in DATE_FORMATS {
        if let Ok(d) = NaiveDate::parse_from_str(s, fmt) {
            return Some(d);
        }
    }
    None
}

fn days_until_naive(target: NaiveDate) -> Option<u64> {
    let today = Local::now().date_naive();
    let diff = (target - today).num_days();
    if diff > 0 {
        Some((diff as u64) + 1)
    } else {
        None
    }
}
