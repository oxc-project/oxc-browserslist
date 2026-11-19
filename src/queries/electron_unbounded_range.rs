use super::{Distrib, QueryResult};
use crate::{
    data::electron::{
        ElectronVersion, electron_versions, parse_version, unpack_chromium, unpack_version,
    },
    parser::Comparator,
};

pub(super) fn electron_unbounded_range(comparator: Comparator, version: &str) -> QueryResult {
    // Like browserslist, accept a major-only version (`electron >= 30`) as `30.0`.
    let version = match version.parse() {
        Ok(major) => ElectronVersion::new(major, 0),
        Err(_) => parse_version(version)?,
    };

    let distribs = electron_versions()
        .iter()
        .filter(|&&packed| comparator.compare(unpack_version(packed), version))
        .map(|&packed| Distrib::new("chrome", unpack_chromium(packed)))
        .collect();
    Ok(distribs)
}
