use super::{Distrib, QueryResult};
use crate::data::electron::{electron_versions, unpack_chromium, unpack_version};

pub(super) fn last_n_electron_major(count: usize) -> QueryResult {
    let minimum = electron_versions()
        .iter()
        .rev()
        .nth(count - 1)
        .map(|&packed| unpack_version(packed))
        .unwrap_or_default();

    let distribs = electron_versions()
        .iter()
        .filter(|&&packed| unpack_version(packed) >= minimum)
        .rev()
        .map(|&packed| Distrib::new("chrome", unpack_chromium(packed)))
        .collect();

    Ok(distribs)
}
