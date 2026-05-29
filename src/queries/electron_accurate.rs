use super::{Distrib, QueryResult};
use crate::{
    data::{
        electron::{ELECTRON_CHROMIUM_VERSIONS, ELECTRON_VERSIONS, parse_version},
        unpack_str,
    },
    error::Error,
};

pub(super) fn electron_accurate(version: &str) -> QueryResult {
    let version_str = version;
    let version = parse_version(version)?;

    let distribs = ELECTRON_VERSIONS
        .iter()
        .find(|(electron_version, _)| *electron_version == version)
        .map(|(_, chromium_version)| {
            vec![Distrib::new("chrome", unpack_str(ELECTRON_CHROMIUM_VERSIONS, *chromium_version))]
        })
        .ok_or_else(|| Error::UnknownElectronVersion(version_str.to_string()))?;
    Ok(distribs)
}
