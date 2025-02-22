use super::{Distrib, QueryResult, count_filter_versions};
use crate::{
    data::caniuse::{caniuse_browsers, get_browser_stat},
    opts::Opts,
};

pub(super) fn last_n_browsers(count: usize, opts: &Opts) -> QueryResult {
    let distribs = caniuse_browsers()
        .keys()
        .filter_map(|name| get_browser_stat(name, opts.mobile_to_desktop))
        .flat_map(|(name, stat)| {
            let count = count_filter_versions(name, opts.mobile_to_desktop, count);

            stat.version_list
                .iter()
                .filter(|version| version.release_date.is_some())
                .rev()
                .take(count)
                .map(move |version| Distrib::new(name, version.version))
        })
        .collect();

    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::test::run_compare;

    #[test_case("last 2 versions"; "basic")]
    #[test_case("last 31 versions"; "android")]
    #[test_case("last 1 version"; "support pluralization")]
    #[test_case("Last 02 Versions"; "case insensitive")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
