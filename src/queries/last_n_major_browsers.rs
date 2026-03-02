use super::{Distrib, QueryResult, count_filter_versions, find_minimum_major};
use crate::{
    data::caniuse::{caniuse_browsers, get_browser_stat},
    opts::Opts,
};

pub(super) fn last_n_major_browsers(count: usize, opts: &Opts) -> QueryResult {
    let distribs = caniuse_browsers()
        .keys()
        .filter_map(|name| get_browser_stat(name, opts.mobile_to_desktop))
        .flat_map(|(name, stat)| {
            let count = count_filter_versions(name, opts.mobile_to_desktop, count);
            let minimum = find_minimum_major(stat, count);

            stat.version_list
                .iter()
                .filter(|version| version.release_date().is_some())
                .map(|version| version.version())
                .filter(move |version| {
                    version.split('.').next().unwrap().parse().unwrap_or(0) >= minimum
                })
                .rev()
                .map(move |version| Distrib::new(name, version))
        })
        .collect();

    Ok(distribs)
}
