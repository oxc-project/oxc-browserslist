use super::{Distrib, QueryResult, count_filter_versions};
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

            let mut vec = stat
                .version_list
                .iter()
                .filter(|version| version.release_date().is_some())
                .rev()
                .map(|version| version.version().split('.').next().unwrap())
                .collect::<Vec<_>>();
            vec.dedup();
            let minimum = vec.get(count - 1).and_then(|minimum| minimum.parse().ok()).unwrap_or(0);

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
