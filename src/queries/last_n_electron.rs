use super::{Distrib, QueryResult};
use crate::data::electron::{electron_versions, unpack_chromium};

pub(super) fn last_n_electron(count: usize) -> QueryResult {
    let distribs = electron_versions()
        .iter()
        .rev()
        .take(count)
        .map(|&packed| Distrib::new("chrome", unpack_chromium(packed)))
        .collect();
    Ok(distribs)
}
