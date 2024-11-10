use super::{Distrib, QueryResult};
use crate::{data::caniuse::get_browser_stat, error::Error, opts::Opts};

pub(super) fn unreleased_x_browsers(name: &str, opts: &Opts) -> QueryResult {
    let (name, stat) = get_browser_stat(name, opts.mobile_to_desktop)
        .ok_or_else(|| Error::BrowserNotFound(name.to_string()))?;
    let distribs = stat
        .version_list
        .iter()
        .filter(|version| version.release_date.is_none())
        .map(|version| Distrib::new(name, version.version.as_str()))
        .collect();
    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::test::run_compare;

    #[test_case("unreleased edge versions"; "basic")]
    #[test_case("Unreleased Chrome Versions"; "case insensitive")]
    #[test_case("unreleased firefox version"; "support pluralization")]
    #[test_case("unreleased    safari     versions"; "more spaces")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
