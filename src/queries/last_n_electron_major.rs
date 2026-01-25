use super::{Distrib, QueryResult};
use crate::data::electron::ELECTRON_VERSIONS;

pub(super) fn last_n_electron_major(count: usize) -> QueryResult {
    let minimum = ELECTRON_VERSIONS
        .iter()
        .rev()
        .nth(count - 1)
        .map(|(electron_version, _)| *electron_version)
        .unwrap_or_default();

    let distribs = ELECTRON_VERSIONS
        .iter()
        .filter(|(electron_version, _)| *electron_version >= minimum)
        .rev()
        .map(|(_, chromium_version)| Distrib::new("chrome", *chromium_version))
        .collect();

    Ok(distribs)
}
