//! Date utilities for converting between calendar dates, unix timestamps, and julian days.
//! Replaces the `time` crate dependency with minimal inline implementations.

/// Seconds per day
const SECONDS_PER_DAY: i64 = 86400;

/// Julian Day Number of Unix epoch (1970-01-01)
const UNIX_EPOCH_JDN: i64 = 2440588;

pub(crate) type CivilDate = (i64, u32, u32);

#[cfg(windows)]
#[expect(unsafe_code, reason = "the C runtime provides the system's local calendar date")]
unsafe extern "C" {
    fn _mktime64(timeptr: *mut libc::tm) -> i64;
}

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

/// Get current unix timestamp.
pub fn now_unix_timestamp() -> i64 {
    std::cfg_select! {
        not(target_arch = "wasm32") => {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().cast_signed())
                .unwrap_or(0)
        }
        target_arch = "wasm32" => {
            (js_sys::Date::now() / 1000.0) as i64
        }
    }
}

/// Get current Julian Day Number.
pub fn now_julian_day() -> i32 {
    let ts = now_unix_timestamp();
    ((ts / SECONDS_PER_DAY) + UNIX_EPOCH_JDN) as i32
}

/// Convert a Unix timestamp to its UTC Gregorian calendar date.
pub(crate) fn unix_timestamp_to_date(timestamp: i64) -> CivilDate {
    // Howard Hinnant's civil-from-days algorithm, with day zero at 1970-01-01.
    let days = timestamp.div_euclid(SECONDS_PER_DAY);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month as u32, day as u32)
}

/// Add months to a timestamp using JavaScript's local-time `Date.setMonth` semantics.
pub(crate) fn add_months_local(timestamp: i64, offset: i32) -> Option<i64> {
    std::cfg_select! {
        all(not(miri), not(target_arch = "wasm32"), any(unix, windows)) => {
            add_months_local_native(timestamp, offset)
        }
        target_arch = "wasm32" => {
            let date = js_sys::Date::new_0();
            date.set_time(timestamp as f64 * 1000.0);
            let months = i64::from(date.get_full_year()) * 12
                + i64::from(date.get_month())
                + i64::from(offset);
            let year = months.div_euclid(12);
            if year < 0 {
                return Some(0);
            }
            let millis = date.set_full_year_with_month(
                u32::try_from(year).ok()?,
                months.rem_euclid(12) as i32,
            );
            millis.is_finite().then_some((millis / 1000.0) as i64)
        }
        _ => {
            add_months_utc(timestamp, offset)
        }
    }
}

#[cfg(all(not(miri), not(target_arch = "wasm32"), any(unix, windows)))]
#[expect(unsafe_code, reason = "the C runtime provides the system's local calendar date")]
fn add_months_local_native(timestamp: i64, offset: i32) -> Option<i64> {
    if std::mem::size_of::<libc::time_t>() == 4
        && !(i64::from(i32::MIN)..=i64::from(i32::MAX)).contains(&timestamp)
    {
        return add_months_utc(timestamp, offset);
    }
    let native_timestamp = timestamp as libc::time_t;
    let mut local = std::mem::MaybeUninit::<libc::tm>::uninit();
    #[cfg(unix)]
    let failed = unsafe { libc::localtime_r(&native_timestamp, local.as_mut_ptr()) }.is_null();
    #[cfg(windows)]
    let failed = unsafe { libc::localtime_s(local.as_mut_ptr(), &native_timestamp) } != 0;
    if failed {
        return None;
    }
    let mut local = unsafe { local.assume_init() };
    local.tm_mon = local.tm_mon.checked_add(offset)?;
    local.tm_isdst = -1;
    #[cfg(unix)]
    let shifted = unsafe { libc::mktime(&raw mut local) as i64 };
    #[cfg(windows)]
    let shifted = unsafe { _mktime64(&raw mut local) };
    mktime_or_fallback(timestamp, offset, shifted)
}

#[cfg(all(not(miri), not(target_arch = "wasm32"), any(unix, windows)))]
fn mktime_or_fallback(timestamp: i64, offset: i32, shifted: i64) -> Option<i64> {
    if shifted == -1 { add_months_utc(timestamp, offset) } else { Some(shifted) }
}

#[cfg(any(test, not(target_arch = "wasm32")))]
fn add_months_utc(timestamp: i64, offset: i32) -> Option<i64> {
    let (year, month, day) = add_months(unix_timestamp_to_date(timestamp), i64::from(offset));
    date_to_unix_timestamp(i32::try_from(year).ok()?, month, day)
}

/// Add months using JavaScript `Date.prototype.setMonth` overflow semantics.
#[cfg(any(test, not(target_arch = "wasm32")))]
fn add_months((year, month, mut day): CivilDate, offset: i64) -> CivilDate {
    let months = year * 12 + i64::from(month) - 1 + offset;
    let mut year = months.div_euclid(12);
    let mut month = months.rem_euclid(12) as u32 + 1;
    while day > days_in_month(year, month) {
        day -= days_in_month(year, month);
        month += 1;
        if month > 12 {
            month = 1;
            year += 1;
        }
    }
    (year, month, day)
}

#[cfg(any(test, not(target_arch = "wasm32")))]
fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        2 => 28,
        _ => 31,
    }
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
    fn baseline_date_helpers() {
        assert_eq!(unix_timestamp_to_date(0), (1970, 1, 1));
        assert_eq!(unix_timestamp_to_date(946684800), (2000, 1, 1));
        assert_eq!(unix_timestamp_to_date(-1), (1969, 12, 31));
        assert_eq!(add_months((2024, 5, 31), -30), (2021, 12, 1));
        assert_eq!(add_months(add_months((2024, 3, 31), 30), -30), (2024, 4, 1));
        let timestamp = date_to_unix_timestamp(2040, 1, 1).unwrap();
        assert_eq!(add_months_utc(timestamp, -30), date_to_unix_timestamp(2037, 7, 1));

        #[cfg(all(not(miri), not(target_arch = "wasm32"), any(unix, windows)))]
        {
            let timestamp = date_to_unix_timestamp(2035, 8, 1).unwrap();
            assert_eq!(mktime_or_fallback(timestamp, 30, -1), date_to_unix_timestamp(2038, 2, 1));
        }
    }

    #[cfg(all(not(miri), unix))]
    #[test]
    fn local_date_uses_timezone() {
        const CHILD: &str = "OXC_BROWSERSLIST_LOCAL_DATE_TEST";
        if std::env::var_os(CHILD).is_some() {
            let timestamp = date_to_unix_timestamp(2018, 1, 29).unwrap();
            let timestamp = add_months_local(timestamp, -30).unwrap();
            assert_eq!(unix_timestamp_to_date(timestamp), (2015, 7, 28));
            return;
        }

        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "date::tests::local_date_uses_timezone"])
            .env(CHILD, "1")
            .env("TZ", "PST8PDT")
            .status()
            .unwrap();
        assert!(status.success());
    }
}
