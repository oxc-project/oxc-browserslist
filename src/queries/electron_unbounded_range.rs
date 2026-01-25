use super::{Distrib, QueryResult};
use crate::{
    data::electron::{ELECTRON_VERSIONS, parse_version},
    parser::Comparator,
};

pub(super) fn electron_unbounded_range(comparator: Comparator, version: &str) -> QueryResult {
    let version = parse_version(version)?;

    let distribs = ELECTRON_VERSIONS
        .iter()
        .filter(|(electron_version, _)| match comparator {
            Comparator::Greater => *electron_version > version,
            Comparator::Less => *electron_version < version,
            Comparator::GreaterOrEqual => *electron_version >= version,
            Comparator::LessOrEqual => *electron_version <= version,
        })
        .map(|(_, chromium_version)| Distrib::new("chrome", *chromium_version))
        .collect();
    Ok(distribs)
}
