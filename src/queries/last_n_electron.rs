use super::{Distrib, QueryResult};
use crate::data::{
    electron::{ELECTRON_CHROMIUM_VERSIONS, ELECTRON_VERSIONS},
    unpack_str,
};

pub(super) fn last_n_electron(count: usize) -> QueryResult {
    let distribs = ELECTRON_VERSIONS
        .iter()
        .rev()
        .take(count)
        .map(|(_, version)| {
            Distrib::new("chrome", unpack_str(ELECTRON_CHROMIUM_VERSIONS, *version))
        })
        .collect();
    Ok(distribs)
}
