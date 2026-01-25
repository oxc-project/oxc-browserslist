use super::{Distrib, QueryResult, count_filter_versions};
use crate::{data::caniuse::get_browser_stat, error::Error, opts::Opts};

pub(super) fn last_n_x_major_browsers(count: usize, name: &str, opts: &Opts) -> QueryResult {
    let (name, stat) = get_browser_stat(name, opts.mobile_to_desktop)
        .ok_or_else(|| Error::BrowserNotFound(name.to_string()))?;
    let count = count_filter_versions(name, opts.mobile_to_desktop, count);
    let mut vec = stat
        .version_list
        .iter()
        .filter(|version| version.release_date().is_some())
        .map(|version| version.version())
        .rev()
        .map(|version| version.split('.').next().unwrap())
        .collect::<Vec<_>>();
    vec.dedup();
    let minimum = vec.get(count - 1).and_then(|minimum| minimum.parse().ok()).unwrap_or(0);

    let distribs = stat
        .version_list
        .iter()
        .filter(|version| version.release_date().is_some())
        .map(|version| version.version())
        .filter(move |version| version.split('.').next().unwrap().parse().unwrap_or(0) >= minimum)
        .rev()
        .map(move |version| Distrib::new(name, version))
        .collect();

    Ok(distribs)
}
