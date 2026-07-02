use super::{Distrib, QueryResult};
use crate::data::electron::{ELECTRON_VERSIONS, packed_chromium};

pub(super) fn last_n_electron(count: usize) -> QueryResult {
    let distribs = ELECTRON_VERSIONS
        .iter()
        .rev()
        .take(count)
        .map(|&packed| Distrib::new("chrome", packed_chromium(packed)))
        .collect();
    Ok(distribs)
}
