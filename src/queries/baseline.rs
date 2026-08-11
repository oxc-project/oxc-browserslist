use super::{QueryResult, browser_unbounded_range::browser_unbounded_range};
use crate::{
    data::baseline as data,
    date::{
        CivilDate, add_months_local, date_to_unix_timestamp, now_unix_timestamp,
        unix_timestamp_to_date,
    },
    error::Error,
    opts::Opts,
    parser::{BaselineKind, Comparator},
};

// December 31 of the next year is outside JavaScript's Date range.
const MAX_BASELINE_YEAR: u32 = 275_759;

pub(super) fn baseline(
    kind: BaselineKind,
    downstream: bool,
    kaios: bool,
    opts: &Opts,
) -> QueryResult {
    let cutoff = match kind {
        BaselineKind::Year(year) if year <= MAX_BASELINE_YEAR => u64::from(year) * 10_000 + 1_231,
        BaselineKind::Year(_) | BaselineKind::InvalidYear => return Ok(Vec::new()),
        BaselineKind::LegacyYear(year) => {
            let Some(date) = legacy_year_date(year) else { return Ok(Vec::new()) };
            date_key(date)
        }
        BaselineKind::NewlyAvailable => {
            let timestamp = now_unix_timestamp();
            let Some(timestamp) = add_months_local(timestamp, 30)
                .and_then(|timestamp| add_months_local(timestamp, -30))
            else {
                return Ok(Vec::new());
            };
            date_key(unix_timestamp_to_date(timestamp))
        }
        BaselineKind::NewlyAvailableOnDate => {
            return Err(Error::BaselineNewlyAvailableOnDate);
        }
        BaselineKind::WidelyAvailable => {
            let Some(timestamp) = add_months_local(now_unix_timestamp(), -30) else {
                return Ok(Vec::new());
            };
            date_key(unix_timestamp_to_date(timestamp))
        }
        BaselineKind::WidelyAvailableOnDate { year, month, day } => {
            let Ok(year) = i32::try_from(year) else { return Ok(Vec::new()) };
            let Some(timestamp) = date_to_unix_timestamp(year, month, day)
                .and_then(|timestamp| add_months_local(timestamp, -30))
            else {
                return Ok(Vec::new());
            };
            date_key(unix_timestamp_to_date(timestamp))
        }
    };

    // Ported from Browserslist's Baseline selection. KaiOS can be requested without the other
    // downstream browsers; when downstream is requested, KaiOS remains opt-in.
    let is_included = |browser: &str| {
        is_core_browser(browser)
            || (kaios && browser == "kaios")
            || (downstream && browser != "kaios")
    };

    let mut distribs = Vec::new();
    if let Some(versions) = data::min_versions_on(cutoff) {
        for (browser, version) in versions.filter(|(browser, _)| is_included(browser)) {
            distribs.append(&mut browser_unbounded_range(
                &browser,
                Comparator::GreaterOrEqual,
                version,
                opts,
            )?);
        }
    } else {
        // Before the first Baseline feature, baseline-browser-mapping returns version 0 for each
        // selected browser, which Browserslist resolves to every released version.
        for browser in data::browsers().filter(|browser| is_included(browser)) {
            distribs.append(&mut browser_unbounded_range(
                &browser,
                Comparator::GreaterOrEqual,
                "0",
                opts,
            )?);
        }
    }
    Ok(distribs)
}

fn is_core_browser(browser: &str) -> bool {
    matches!(browser, "and_chr" | "and_ff" | "chrome" | "edge" | "firefox" | "ios_saf" | "safari")
}

// V8's non-ISO date parser treats one- and two-digit years specially.
fn legacy_year_date(year: u32) -> Option<CivilDate> {
    match year {
        0 => Some((2000, 12, 31)),
        1..=12 => Some((2031, year, 12)),
        13..=31 => None,
        32..=49 => Some((i64::from(year) + 2000, 12, 31)),
        50..=99 => Some((i64::from(year) + 1900, 12, 31)),
        _ if year <= MAX_BASELINE_YEAR => Some((i64::from(year), 12, 31)),
        _ => None,
    }
}

fn date_key((year, month, day): CivilDate) -> u64 {
    if year < 0 { 0 } else { year as u64 * 10_000 + u64::from(month * 100 + day) }
}
