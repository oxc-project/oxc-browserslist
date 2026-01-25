use super::{Distrib, QueryResult};
use crate::{
    data::electron::{ELECTRON_VERSIONS, parse_version},
    error::Error,
};

pub(super) fn electron_bounded_range(from: &str, to: &str) -> QueryResult {
    let from_str = from;
    let to_str = to;
    let from = parse_version(from)?;
    let to = parse_version(to)?;

    if ELECTRON_VERSIONS.iter().all(|(version, _)| *version != from) {
        return Err(Error::UnknownElectronVersion(from_str.to_string()));
    }
    if ELECTRON_VERSIONS.iter().all(|(version, _)| *version != to) {
        return Err(Error::UnknownElectronVersion(to_str.to_string()));
    }

    let distribs = ELECTRON_VERSIONS
        .iter()
        .filter(|(version, _)| from <= *version && *version <= to)
        .map(|(_, version)| Distrib::new("chrome", *version))
        .collect();
    Ok(distribs)
}
