use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{caniuse_browsers, get_browser_stat},
    date::date_to_unix_timestamp,
    error::Error,
    opts::Opts,
};

pub(super) fn since(year: i32, month: u32, day: u32, opts: &Opts) -> QueryResult {
    let time = date_to_unix_timestamp(year, month, day)
        .ok_or_else(|| Error::InvalidDate(format!("{year}-{month}-{day}")))?;

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
