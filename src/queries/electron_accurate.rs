use super::{Distrib, QueryResult};
use crate::{
    data::electron::{ELECTRON_VERSIONS, parse_version, unpack_chromium, unpack_version},
    error::Error,
};

pub(super) fn electron_accurate(version: &str) -> QueryResult {
    let version_str = version;
    let version = parse_version(version)?;

    let distribs = ELECTRON_VERSIONS
        .iter()
        .find(|&&packed| unpack_version(packed) == version)
        .map(|&packed| vec![Distrib::new("chrome", unpack_chromium(packed))])
        .ok_or_else(|| Error::UnknownElectronVersion(version_str.to_string()))?;
    Ok(distribs)
}
