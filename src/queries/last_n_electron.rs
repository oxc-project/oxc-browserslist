use super::{Distrib, QueryResult};
use crate::data::electron::ELECTRON_VERSIONS;

pub(super) fn last_n_electron(count: usize) -> QueryResult {
    let distribs = ELECTRON_VERSIONS
        .iter()
        .rev()
        .take(count)
        .map(|(_, version)| Distrib::new("chrome", *version))
        .collect();
    Ok(distribs)
}
