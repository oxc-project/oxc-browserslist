//! Port of `getCompatibleVersions` from the `baseline-browser-mapping` (bbm) npm package,
//! the engine behind browserslist's `baseline` queries. Only the code paths browserslist can
//! reach are ported: `listAllCompatibleVersions` is never set, and `suppressWarnings` is
//! always true, so every bbm warning/throw path is unreachable.

use crate::{
    data::unpack_str,
    date::{add_months, date_to_unix_timestamp, now_unix_timestamp, unix_timestamp_to_julian_day},
    generated::baseline_timeline::{BASELINE_BROWSERS, BASELINE_TIMELINE, BASELINE_VERSIONS},
};

/// The `getCompatibleVersions` options browserslist can pass.
pub(crate) struct BaselineOptions<'a> {
    /// `baseline <year>` digits; kept as a string like the JS regex capture, because V8 date
    /// parsing distinguishes year digit counts.
    pub target_year: Option<&'a str>,
    /// Target instant in unix seconds; `Some(None)` models a JS Invalid Date.
    pub widely_available_on_date: Option<Option<i64>>,
    pub include_downstream_browsers: bool,
    pub include_kaios: bool,
}

/// `2015-07-29T00:00:00Z`, the first timeline section. Targets strictly before it predate
/// Baseline entirely and select version "0" (all versions) of every browser.
const PRE_BASELINE_TS: i64 = 1_438_128_000;

/// bbm long browser name and minimum compatible version for the requested Baseline target,
/// in bbm's `nameMappings` order. (bbm sorts its result afterwards, but the order is
/// immaterial here: browserslist re-sorts after resolving the per-browser sub-queries.)
pub(crate) fn get_compatible_versions(
    options: &BaselineOptions,
) -> Vec<(&'static str, &'static str)> {
    debug_assert_eq!(date_to_unix_timestamp(2015, 7, 29), Some(PRE_BASELINE_TS));
    // The regex makes year and date mutually exclusive; bbm throws when both are set.
    debug_assert!(options.target_year.is_none() || options.widely_available_on_date.is_none());

    let mut target_date =
        if options.widely_available_on_date.is_none() && options.target_year.is_none() {
            Some(now_unix_timestamp())
        } else if let Some(date) = options.widely_available_on_date {
            date
        } else {
            // `new Date(`${targetYear}-12-31`)`: four-digit years use V8's ISO parser and
            // other lengths its legacy parser; both mean the literal year, valid up to
            // JavaScript's Date instant range (December 31st of 275760 already exceeds
            // it). Exception: one- and two-digit years get the legacy month-day-year
            // reading (`12-12-31` is 2031-12-12) — deliberately not emulated, they parse
            // numerically here.
            let year = options.target_year.unwrap();
            year.parse::<i32>()
                .ok()
                .filter(|&year| year <= 275_759)
                .and_then(|year| date_to_unix_timestamp(year, 12, 31))
        };

    // Sets a cutoff date for feature interoperability 30 months before the stated date
    if options.widely_available_on_date.is_some() || options.target_year.is_none() {
        target_date = target_date.and_then(|ts| add_months(ts, -30));
    }

    // JS `targetDate < new Date("2015-07-29")`: false for an Invalid Date, like every NaN
    // comparison below.
    let is_pre2015 = target_date.is_some_and(|ts| ts < PRE_BASELINE_TS);

    // Find the active minimum version of each browser at targetDate: walk the timeline in
    // order, last write per browser wins. Sections are `<= targetDate` by instant; section
    // dates are UTC midnights, so comparing Julian days is equivalent.
    let mut min_versions = vec![None::<u32>; BASELINE_BROWSERS.len()];
    if let Some(target_day) = target_date.map(unix_timestamp_to_julian_day) {
        for &(day, browser, version) in BASELINE_TIMELINE {
            if day <= target_day {
                min_versions[browser as usize] = Some(version);
            }
        }
    }

    let mut result = Vec::new();
    for (index, &(long_name, is_core)) in BASELINE_BROWSERS.iter().enumerate() {
        if !options.include_kaios && long_name == "kai_os" {
            continue;
        }
        if !options.include_downstream_browsers && !is_core {
            continue;
        }
        if is_pre2015 {
            result.push((long_name, "0"));
        } else if let Some(packed) = min_versions[index] {
            result.push((long_name, unpack_str(BASELINE_VERSIONS, packed >> 8, packed & 0xff)));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options(target_year: Option<&str>, date: Option<Option<i64>>) -> BaselineOptions<'_> {
        BaselineOptions {
            target_year,
            widely_available_on_date: date,
            include_downstream_browsers: false,
            include_kaios: false,
        }
    }

    // Expected values come from running the real thing:
    // `bbm.getCompatibleVersions({...})` with baseline-browser-mapping 2.11.12. Timeline
    // history is immutable, so these stay valid as the data updates.

    #[test]
    fn test_target_year() {
        let versions = get_compatible_versions(&BaselineOptions {
            include_downstream_browsers: true,
            include_kaios: true,
            ..options(Some("2020"), None)
        });
        #[rustfmt::skip]
        assert_eq!(versions, vec![
            ("chrome", "87"), ("chrome_android", "87"), ("edge", "87"), ("firefox", "83"),
            ("firefox_android", "83"), ("safari", "14"), ("safari_ios", "14"),
            ("opera", "73"), ("opera_android", "62"), ("samsunginternet_android", "14.0"),
            ("webview_android", "87"), ("ya_android", "20.12"), ("uc_android", "15.3"),
            ("qq_android", "11.7"), ("kai_os", "3.0"), ("facebook_android", "348"),
            ("instagram_android", "163"),
        ]);
    }

    #[test]
    fn test_widely_available_on_date() {
        let date = date_to_unix_timestamp(2022, 7, 1);
        let versions = get_compatible_versions(&options(None, Some(date)));
        #[rustfmt::skip]
        assert_eq!(versions, vec![
            ("chrome", "66"), ("chrome_android", "66"), ("edge", "18"), ("firefox", "65"),
            ("firefox_android", "65"), ("safari", "13"), ("safari_ios", "13"),
        ]);
    }

    #[test]
    fn test_widely_available_on_baseline_start() {
        // 2018-01-29 minus 30 months is exactly 2015-07-29: the first timeline section
        // applies (`<=`), not the pre-2015 "0" branch (strict `<`).
        let date = date_to_unix_timestamp(2018, 1, 29);
        let versions = get_compatible_versions(&options(None, Some(date)));
        #[rustfmt::skip]
        assert_eq!(versions, vec![
            ("chrome", "38"), ("chrome_android", "38"), ("edge", "12"), ("firefox", "38"),
            ("firefox_android", "38"), ("safari", "11"), ("safari_ios", "11"),
        ]);
    }

    #[test]
    fn test_pre2015_target_year() {
        let versions = get_compatible_versions(&options(Some("2013"), None));
        #[rustfmt::skip]
        assert_eq!(versions, vec![
            ("chrome", "0"), ("chrome_android", "0"), ("edge", "0"), ("firefox", "0"),
            ("firefox_android", "0"), ("safari", "0"), ("safari_ios", "0"),
        ]);
    }

    #[test]
    fn test_invalid_dates_are_empty() {
        // A year past JavaScript's Date range and an invalid calendar date are JS Invalid
        // Dates: every comparison is false, so no browser gets a minimum version.
        assert_eq!(get_compatible_versions(&options(Some("275760"), None)), vec![]);
        assert_eq!(get_compatible_versions(&options(Some("9999999999"), None)), vec![]);
        assert_eq!(get_compatible_versions(&options(None, Some(None))), vec![]);
    }

    #[test]
    fn test_far_future_year_selects_latest_minimums() {
        // Long years within JavaScript's Date range are literal years (V8's legacy parser),
        // saturating at the newest timeline snapshot.
        let versions = get_compatible_versions(&options(Some("99999"), None));
        assert!(!versions.is_empty());
        assert_eq!(versions, get_compatible_versions(&options(Some("275759"), None)));
    }
}
