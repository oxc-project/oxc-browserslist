use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{caniuse_browsers, get_browser_stat},
    date::now_unix_timestamp,
    error::Error,
    opts::Opts,
};

const ONE_YEAR_IN_SECONDS: f64 = 365.259_641 * 24.0 * 60.0 * 60.0;

pub(super) fn years(count: f64, opts: &Opts) -> QueryResult {
    let duration_secs = count * ONE_YEAR_IN_SECONDS;
    if !duration_secs.is_finite() || duration_secs < 0.0 || duration_secs > i64::MAX as f64 {
        return Err(Error::YearOverflow);
    }
    let time = now_unix_timestamp() - duration_secs as i64;

    let distribs = caniuse_browsers()
        .keys()
        .filter_map(|name| get_browser_stat(name, opts.mobile_to_desktop))
        .flat_map(|(name, stat)| {
            stat.version_list
                .iter()
                .filter(
                    |version| matches!(version.release_date(), Some(date) if date.get() >= time),
                )
                .map(move |version| Distrib::new(name, version.version()))
        })
        .collect();
    Ok(distribs)
}
