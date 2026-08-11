use super::QueryResult;
use crate::{
    data::baseline::{BaselineOptions, get_compatible_versions},
    date::{add_months, date_to_unix_timestamp, now_unix_timestamp},
    error::Error,
    opts::Opts,
    parser::BaselineAvailability,
    resolve,
};

/// bbm long name -> caniuse name, mirroring browserslist's `bbmTransform`. bbm browsers
/// without a caniuse equivalent (ya_android, facebook_android, instagram_android) are
/// dropped.
const BROWSERS: &[(&str, &str)] = &[
    ("chrome", "chrome"),
    ("chrome_android", "and_chr"),
    ("edge", "edge"),
    ("firefox", "firefox"),
    ("firefox_android", "and_ff"),
    ("safari", "safari"),
    ("safari_ios", "ios_saf"),
    ("webview_android", "android"),
    ("samsunginternet_android", "samsung"),
    ("opera_android", "op_mob"),
    ("opera", "opera"),
    ("qq_android", "and_qq"),
    ("uc_android", "and_uc"),
    ("kai_os", "kaios"),
];

/// The browserslist `baseline` select: pick per-browser minimum versions with the ported
/// bbm `getCompatibleVersions`, then resolve them as `<name> >= <version>` sub-queries.
/// JS computes the now-based targets with local-time `setMonth`; here it is UTC.
pub(super) fn baseline(
    year: Option<&str>,
    availability: Option<BaselineAvailability>,
    date: Option<(u16, u8, u8)>,
    downstream: bool,
    kaios: bool,
    opts: &Opts,
) -> QueryResult {
    if matches!(availability, Some(BaselineAvailability::Newly)) && date.is_some() {
        return Err(Error::BaselineNewlyWithDate);
    }

    let mut options = BaselineOptions {
        target_year: None,
        widely_available_on_date: None,
        include_downstream_browsers: downstream,
        include_kaios: kaios,
    };
    if year.is_some() {
        options.target_year = year;
    } else if let Some((year, month, day)) = date {
        // `new Date("YYYY-MM-DD")` is a UTC midnight; a V8-invalid date maps to `None`.
        options.widely_available_on_date =
            Some(date_to_unix_timestamp(i32::from(year), u32::from(month), u32::from(day)));
    } else if matches!(availability, Some(BaselineAvailability::Newly)) {
        options.widely_available_on_date = Some(add_months(now_unix_timestamp(), 30));
    } else {
        // Pin "now" here rather than defaulting inside `get_compatible_versions`, so the
        // two calls of the KaiOS path below see the same instant.
        options.widely_available_on_date = Some(Some(now_unix_timestamp()));
    }

    // bbm counts KaiOS as a downstream browser and refuses to return it alone, so take
    // KaiOS from the downstream list and everything else from the core one.
    let baseline_versions = if options.include_kaios && !options.include_downstream_browsers {
        options.include_downstream_browsers = true;
        let downstream_versions = get_compatible_versions(&options);
        options.include_downstream_browsers = false;
        options.include_kaios = false;
        let mut versions = get_compatible_versions(&options);
        versions
            .extend(downstream_versions.into_iter().filter(|(browser, _)| *browser == "kai_os"));
        versions
    } else {
        get_compatible_versions(&options)
    };

    resolve(&bbm_transform(&baseline_versions), opts)
}

/// browserslist's `bbmTransform`.
fn bbm_transform(versions: &[(&'static str, &'static str)]) -> Vec<String> {
    versions
        .iter()
        .filter_map(|(bbm_name, version)| {
            BROWSERS
                .iter()
                .find(|(long_name, _)| long_name == bbm_name)
                .map(|(_, caniuse_name)| format!("{caniuse_name} >= {version}"))
        })
        .collect()
}

// Ports of the browserslist test/baseline.test.js cases that need crate internals; the
// rest live in tests/queries/baseline.rs. Gated from Miri like every other test that runs
// full query resolution.
#[cfg(all(test, not(miri)))]
mod tests {
    use super::*;
    use crate::date::{julian_day_to_date, now_julian_day, unix_timestamp_to_julian_day};

    /// Rerun `body` until it runs entirely within one UTC day, so its independent "now"
    /// reads cannot straddle a midnight rollover. Callers assert on the returned values,
    /// not inside `body`.
    fn same_utc_day<T>(body: impl Fn() -> T) -> T {
        loop {
            let day = now_julian_day();
            let result = body();
            if now_julian_day() == day {
                return result;
            }
        }
    }

    /// `Widely available versions from baseline-browser-mapping appear in browserslist
    /// output`: every bbm minimum version — excluding chrome_android/firefox_android, for
    /// which caniuse only ships the latest release — must appear in `baseline widely
    /// available` results, either exactly or as a bound of a joined caniuse range.
    #[test]
    fn widely_available_versions_from_bbm_appear_in_output() {
        let (output, bbm_widely) = same_utc_day(|| {
            (
                resolve(&["baseline widely available"], &Opts::default()).unwrap(),
                get_compatible_versions(&BaselineOptions {
                    target_year: None,
                    widely_available_on_date: None,
                    include_downstream_browsers: false,
                    include_kaios: false,
                }),
            )
        });
        for (bbm_name, version) in bbm_widely {
            if matches!(bbm_name, "chrome_android" | "firefox_android") {
                continue;
            }
            let (_, name) = BROWSERS.iter().find(|(long_name, _)| *long_name == bbm_name).unwrap();
            let found = output.iter().any(|distrib| {
                distrib.name() == *name
                    && distrib.version().split('-').any(|part| {
                        part == version
                            || part.strip_prefix(version).is_some_and(|rest| rest.starts_with('.'))
                    })
            });
            assert!(found, "{name} {version} not found in `baseline widely available`");
        }
    }

    /// `Newly available today and Widely available 30 months from now match`.
    #[test]
    fn newly_available_matches_widely_available_in_30_months() {
        let (newly, widely) = same_utc_day(|| {
            let future = add_months(now_unix_timestamp(), 30).unwrap();
            let (year, month, day) = julian_day_to_date(unix_timestamp_to_julian_day(future));
            let widely = format!("baseline widely available on {year:04}-{month:02}-{day:02}");
            (
                resolve(&["baseline newly available"], &Opts::default()).unwrap(),
                resolve(&[widely], &Opts::default()).unwrap(),
            )
        });
        assert_eq!(newly, widely);
    }
}
