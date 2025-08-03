use time::{Date, Month, OffsetDateTime, Time};

use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{caniuse_browsers, get_browser_stat},
    error::Error,
    opts::Opts,
};

pub(super) fn since(year: i32, month: u32, day: u32, opts: &Opts) -> QueryResult {
    let month = Month::try_from(month as u8)
        .map_err(|_| Error::InvalidDate(format!("{year}-{month}-{day}")))?;
    let date = Date::from_calendar_date(year, month, day as u8)
        .map_err(|_| Error::InvalidDate(format!("{year}-{month}-{day}")))?;
    let time = OffsetDateTime::new_utc(date, Time::MIDNIGHT).unix_timestamp();

    let distribs = caniuse_browsers()
        .keys()
        .filter_map(|name| get_browser_stat(name, opts.mobile_to_desktop))
        .flat_map(|(name, stat)| {
            stat.version_list
                .iter()
                .filter(
                    |version| matches!(version.release_date(), Some(date) if date.get() >= time),
                )
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

    #[test_case("since 2017"; "year only")]
    #[test_case("Since 2017"; "case insensitive")]
    #[test_case("since 2017-02"; "with month")]
    #[test_case("since 2017-02-15"; "with day")]
    #[test_case("since 1970"; "unix timestamp zero")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
