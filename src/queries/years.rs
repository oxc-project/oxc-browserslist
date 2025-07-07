use time::{Duration, OffsetDateTime};

use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{caniuse_browsers, get_browser_stat},
    error::Error,
    opts::Opts,
};

const ONE_YEAR_IN_SECONDS: f64 = 365.259_641 * 24.0 * 60.0 * 60.0;

pub(super) fn years(count: f64, opts: &Opts) -> QueryResult {
    let duration =
        Duration::checked_seconds_f64(count * ONE_YEAR_IN_SECONDS).ok_or(Error::YearOverflow)?;
    let time = (OffsetDateTime::now_utc() - duration).unix_timestamp();

    let distribs = caniuse_browsers()
        .keys()
        .filter_map(|name| get_browser_stat(name, opts.mobile_to_desktop))
        .flat_map(|(name, stat)| {
            stat.version_list
                .iter()
                .filter(|version| matches!(version.release_date(), Some(date) if date >= time))
                .map(|version| Distrib::new(name, version.version()))
        })
        .collect();
    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::test::run_compare;

    #[test_case("last 2 years"; "basic")]
    #[test_case("last 1 year"; "one year")]
    #[test_case("last 1.4 years"; "year fraction")]
    #[test_case("Last 5 Years"; "case insensitive")]
    #[test_case("last    2     years"; "more spaces")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
