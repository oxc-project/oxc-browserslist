use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{caniuse_browsers, get_browser_stat},
    opts::Opts,
};

pub(super) fn unreleased_browsers(opts: &Opts) -> QueryResult {
    let distribs = caniuse_browsers()
        .keys()
        .filter_map(|name| get_browser_stat(name, opts.mobile_to_desktop))
        .flat_map(|(name, stat)| {
            stat.version_list
                .iter()
                .filter(|version| version.release_date().is_none())
                .map(move |version| Distrib::new(name, version.version()))
        })
        .collect();
    Ok(distribs)
}

#[cfg(all(test, not(miri)))]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::test::run_compare;

    #[test_case("unreleased versions"; "basic")]
    #[test_case("Unreleased Versions"; "case insensitive")]
    #[test_case("unreleased        versions"; "more spaces")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
