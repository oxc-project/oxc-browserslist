use super::{Distrib, QueryResult};
use crate::data::electron::{ELECTRON_VERSIONS, packed_chromium, packed_version};

pub(super) fn last_n_electron_major(count: usize) -> QueryResult {
    let minimum = ELECTRON_VERSIONS
        .iter()
        .rev()
        .nth(count - 1)
        .map(|&packed| packed_version(packed))
        .unwrap_or_default();

    let distribs = ELECTRON_VERSIONS
        .iter()
        .filter(|&&packed| packed_version(packed) >= minimum)
        .rev()
        .map(|&packed| Distrib::new("chrome", packed_chromium(packed)))
        .collect();

    Ok(distribs)
}
