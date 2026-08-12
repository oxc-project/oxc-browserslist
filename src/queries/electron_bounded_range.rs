use super::{Distrib, QueryResult};
use crate::{
    data::electron::{electron_versions, parse_version, unpack_chromium, unpack_version},
    error::Error,
};

pub(super) fn electron_bounded_range(from: &str, to: &str) -> QueryResult {
    let from_str = from;
    let to_str = to;
    let from = parse_version(from)?;
    let to = parse_version(to)?;

    if electron_versions().iter().all(|&packed| unpack_version(packed) != from) {
        return Err(Error::UnknownElectronVersion(from_str.to_string()));
    }
    if electron_versions().iter().all(|&packed| unpack_version(packed) != to) {
        return Err(Error::UnknownElectronVersion(to_str.to_string()));
    }

    let distribs = electron_versions()
        .iter()
        .filter(|&&packed| {
            let version = unpack_version(packed);
            from <= version && version <= to
        })
        .map(|&packed| Distrib::new("chrome", unpack_chromium(packed)))
        .collect();
    Ok(distribs)
}
