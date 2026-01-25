use super::{Distrib, QueryResult, count_filter_versions};
use crate::{data::caniuse::get_browser_stat, error::Error, opts::Opts};

pub(super) fn last_n_x_browsers(count: usize, name: &str, opts: &Opts) -> QueryResult {
    let (name, stat) = get_browser_stat(name, opts.mobile_to_desktop)
        .ok_or_else(|| Error::BrowserNotFound(name.to_string()))?;
    let count = count_filter_versions(name, opts.mobile_to_desktop, count);

    let distribs = stat
        .version_list
        .iter()
        .filter(|version| version.release_date().is_some())
        .rev()
        .take(count)
        .map(|version| Distrib::new(name, version.version()))
        .collect();
    Ok(distribs)
}
