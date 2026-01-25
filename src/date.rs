//! Date utilities for converting between calendar dates, unix timestamps, and julian days.
//! Replaces the `time` crate dependency with minimal inline implementations.

/// Seconds per day
const SECONDS_PER_DAY: i64 = 86400;

/// Julian Day Number of Unix epoch (1970-01-01)
const UNIX_EPOCH_JDN: i64 = 2440588;

/// Convert a calendar date to Julian Day Number.
/// Uses the algorithm from <https://en.wikipedia.org/wiki/Julian_day#Converting_Gregorian_calendar_date_to_Julian_Day_Number>
/// Returns None if the calculation would overflow.
#[allow(clippy::cast_possible_truncation)]
const fn date_to_julian_day(year: i32, month: u32, day: u32) -> Option<i32> {
    let a = (14 - month as i32) / 12;
    let Some(y) = year.checked_add(4800 - a) else { return None };
    let m = month as i32 + 12 * a - 3;
    // Use checked arithmetic to prevent overflow
    let Some(term1) = 365_i32.checked_mul(y) else { return None };
    let Some(term2) = term1.checked_add((153 * m + 2) / 5) else { return None };
    let Some(term3) = term2.checked_add(day as i32) else { return None };
    let Some(term4) = term3.checked_add(y / 4) else { return None };
    let Some(term5) = term4.checked_sub(y / 100) else { return None };
    let Some(term6) = term5.checked_add(y / 400) else { return None };
    term6.checked_sub(32045)
}

/// Convert a calendar date to Unix timestamp (seconds since 1970-01-01 00:00:00 UTC).
pub fn date_to_unix_timestamp(year: i32, month: u32, day: u32) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let jdn = i64::from(date_to_julian_day(year, month, day)?);
    Some((jdn - UNIX_EPOCH_JDN) * SECONDS_PER_DAY)
}

/// Get current unix timestamp.
#[cfg(not(target_arch = "wasm32"))]
pub fn now_unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Get current unix timestamp (wasm version using js_sys).
#[cfg(all(target_arch = "wasm32", feature = "wasm_bindgen"))]
pub fn now_unix_timestamp() -> i64 {
    (js_sys::Date::now() / 1000.0) as i64
}

/// Fallback for wasm without js_sys - returns 0 (tests will fail but compiles).
#[cfg(all(target_arch = "wasm32", not(feature = "wasm_bindgen")))]
pub fn now_unix_timestamp() -> i64 {
    0
}

/// Get current Julian Day Number.
pub fn now_julian_day() -> i32 {
    let ts = now_unix_timestamp();
    ((ts / SECONDS_PER_DAY) + UNIX_EPOCH_JDN) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_julian_day() {
        // Test known values
        assert_eq!(date_to_julian_day(2000, 1, 1), Some(2451545)); // J2000.0
        assert_eq!(date_to_julian_day(1970, 1, 1), Some(2440588)); // Unix epoch
        // Test overflow returns None
        assert_eq!(date_to_julian_day(i32::MAX, 1, 1), None);
    }

    #[test]
    fn test_date_to_unix_timestamp() {
        assert_eq!(date_to_unix_timestamp(1970, 1, 1), Some(0));
        assert_eq!(date_to_unix_timestamp(2000, 1, 1), Some(946684800));
        // Invalid dates
        assert_eq!(date_to_unix_timestamp(2000, 0, 1), None);
        assert_eq!(date_to_unix_timestamp(2000, 13, 1), None);
        assert_eq!(date_to_unix_timestamp(2000, 1, 0), None);
        assert_eq!(date_to_unix_timestamp(2000, 1, 32), None);
    }
}
