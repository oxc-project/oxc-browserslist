use std::str::FromStr;

use super::{Distrib, QueryResult};
use crate::{data::node::node_versions, error::Error, parser::Comparator, semver::Version};

pub(super) fn node_unbounded_range(comparator: Comparator, version: &str) -> QueryResult {
    let version =
        Version::from_str(version).map_err(|_| Error::UnknownNodejsVersion(version.to_string()))?;
    let distribs = node_versions()
        .iter()
        .filter(|(v, _)| comparator.compare(*v, version))
        .map(|(_, text)| Distrib::new("node", text.as_ref()))
        .collect();
    Ok(distribs)
}
