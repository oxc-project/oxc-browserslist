use super::{Distrib, QueryResult};
use crate::data::electron::{electron_versions, unpack_chromium, unpack_version};

pub(super) fn last_n_electron_major(count: usize) -> QueryResult {
    // `count` can be 0 ("last 0 electron major versions"); like browserslist-js, a minimum
    // below every version selects them all.
    let minimum = count
        .checked_sub(1)
        .and_then(|n| electron_versions().iter().rev().nth(n))
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
