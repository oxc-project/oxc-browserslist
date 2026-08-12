use super::{Distrib, QueryResult};
use crate::{
    data::electron::{electron_versions, parse_version, unpack_chromium, unpack_version},
    error::Error,
};

pub(super) fn electron_accurate(version: &str) -> QueryResult {
    let version_str = version;
    let version = parse_version(version)?;

    let distribs = electron_versions()
        .iter()
        .find(|&&packed| unpack_version(packed) == version)
        .map(|&packed| vec![Distrib::new("chrome", unpack_chromium(packed))])
        .ok_or_else(|| Error::UnknownElectronVersion(version_str.to_string()))?;
    Ok(distribs)
}
