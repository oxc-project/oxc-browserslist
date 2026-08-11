//! Date utilities for converting between calendar dates, unix timestamps, and julian days.
//! Replaces the `time` crate dependency with minimal inline implementations.

/// Seconds per day
const SECONDS_PER_DAY: i64 = 86400;

/// Julian Day Number of Unix epoch (1970-01-01)
const UNIX_EPOCH_JDN: i64 = 2440588;

/// Convert a calendar date to Julian Day Number.
/// Uses the algorithm from <https://en.wikipedia.org/wiki/Julian_day#Converting_Gregorian_calendar_date_to_Julian_Day_Number>
/// Returns None if the calculation would overflow.
const fn date_to_julian_day(year: i32, month: u32, day: u32) -> Option<i32> {
    let a = (14 - month.cast_signed()) / 12;
    let Some(y) = year.checked_add(4800 - a) else { return None };
    let m = month.cast_signed() + 12 * a - 3;
    // Use checked arithmetic to prevent overflow
    let Some(term1) = 365_i32.checked_mul(y) else { return None };
    let Some(term2) = term1.checked_add((153 * m + 2) / 5) else { return None };
    let Some(term3) = term2.checked_add(day.cast_signed()) else { return None };
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

/// Convert a Julian Day Number back to a calendar date, the inverse of
/// [`date_to_julian_day`]. Uses the algorithm from
/// <https://en.wikipedia.org/wiki/Julian_day#Julian_or_Gregorian_calendar_from_Julian_day_number>.
/// Only valid for non-negative Julian days (every date from year 0 onwards).
pub(crate) const fn julian_day_to_date(jdn: i32) -> (i32, u32, u32) {
    let e = 4 * (jdn + 1401 + (4 * jdn + 274277) / 146097 * 3 / 4 - 38) + 3;
    let h = 5 * ((e % 1461) / 4) + 2;
    let day = (h % 153) / 5 + 1;
    let month = (h / 153 + 2) % 12 + 1;
    let year = e / 1461 - 4716 + (14 - month) / 12;
    (year, month.cast_unsigned(), day.cast_unsigned())
}

/// Floor a unix timestamp (seconds) to the Julian Day Number of its UTC day.
pub fn unix_timestamp_to_julian_day(ts: i64) -> i32 {
    (ts.div_euclid(SECONDS_PER_DAY) + UNIX_EPOCH_JDN) as i32
}

/// Shift a unix timestamp by calendar months with JavaScript
/// `date.setMonth(date.getMonth() + delta)` semantics: the day-of-month and time-of-day are
/// kept, and a day-of-month past the target month's end rolls into the following month
/// (e.g. `2026-08-31` plus 30 months lands on `2029-03-03` via "February 31st").
/// JS runs this in local time; here it is UTC.
pub fn add_months(ts: i64, delta: i32) -> Option<i64> {
    let seconds_of_day = ts.rem_euclid(SECONDS_PER_DAY);
    let (year, month, day) = julian_day_to_date(unix_timestamp_to_julian_day(ts));
    let months = year.checked_mul(12)?.checked_add(month.cast_signed() - 1)?.checked_add(delta)?;
    let (year, month) = (months.div_euclid(12), months.rem_euclid(12).cast_unsigned() + 1);
    // Rebuild as day 1 of the target month plus (day - 1) days so overflow rolls over.
    let jdn = i64::from(date_to_julian_day(year, month, 1)?) + i64::from(day) - 1;
    Some((jdn - UNIX_EPOCH_JDN) * SECONDS_PER_DAY + seconds_of_day)
}

/// Get current unix timestamp.
pub fn now_unix_timestamp() -> i64 {
    std::cfg_select! {
        not(target_arch = "wasm32") => {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().cast_signed())
                .unwrap_or(0)
        }
        // wasm version using js_sys
        feature = "wasm_bindgen" => {
            (js_sys::Date::now() / 1000.0) as i64
        }
        // Fallback for wasm without js_sys - returns 0 (tests will fail but compiles).
        _ => {
            0
        }
    }
}

/// Get current Julian Day Number.
pub fn now_julian_day() -> i32 {
    unix_timestamp_to_julian_day(now_unix_timestamp())
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

    #[test]
    fn test_julian_day_to_date() {
        assert_eq!(julian_day_to_date(2451545), (2000, 1, 1)); // J2000.0
        assert_eq!(julian_day_to_date(2440588), (1970, 1, 1)); // Unix epoch
        // Round-trips across leap years and month ends
        for (year, month, day) in
            [(2000, 2, 29), (2015, 7, 29), (2020, 12, 31), (2024, 2, 29), (2026, 8, 31), (1, 1, 1)]
        {
            let jdn = date_to_julian_day(year, month, day).unwrap();
            assert_eq!(julian_day_to_date(jdn), (year, month, day));
        }
    }

    #[test]
    fn test_add_months() {
        let ts = |year, month, day| date_to_unix_timestamp(year, month, day).unwrap();
        // JS `setMonth` semantics (node-verified vectors): day-of-month overflow rolls over,
        // so a +30/-30 round trip is not the identity near month ends.
        assert_eq!(add_months(ts(2026, 8, 31), 30), Some(ts(2029, 3, 3)));
        assert_eq!(add_months(ts(2029, 3, 3), -30), Some(ts(2026, 9, 3)));
        assert_eq!(add_months(ts(2024, 6, 1), -30), Some(ts(2021, 12, 1)));
        assert_eq!(add_months(ts(2024, 1, 31), 1), Some(ts(2024, 3, 2)));
        // Time-of-day is preserved
        assert_eq!(add_months(ts(2024, 6, 1) + 37_907, -30), Some(ts(2021, 12, 1) + 37_907));
    }
}
