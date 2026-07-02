use super::{Distrib, QueryResult};
use crate::{
    data::electron::{ELECTRON_VERSIONS, packed_chromium, packed_version, parse_version},
    error::Error,
};

pub(super) fn electron_bounded_range(from: &str, to: &str) -> QueryResult {
    let from_str = from;
    let to_str = to;
    let from = parse_version(from)?;
    let to = parse_version(to)?;

    if ELECTRON_VERSIONS.iter().all(|&packed| packed_version(packed) != from) {
        return Err(Error::UnknownElectronVersion(from_str.to_string()));
    }
    if ELECTRON_VERSIONS.iter().all(|&packed| packed_version(packed) != to) {
        return Err(Error::UnknownElectronVersion(to_str.to_string()));
    }

    let distribs = ELECTRON_VERSIONS
        .iter()
        .filter(|&&packed| {
            let version = packed_version(packed);
            from <= version && version <= to
        })
        .map(|&packed| Distrib::new("chrome", packed_chromium(packed)))
        .collect();
    Ok(distribs)
}
