use super::{Distrib, QueryResult};
use crate::{
    data::electron::{ELECTRON_VERSIONS, packed_chromium, packed_version, parse_version},
    parser::Comparator,
};

pub(super) fn electron_unbounded_range(comparator: Comparator, version: &str) -> QueryResult {
    let version = parse_version(version)?;

    let distribs = ELECTRON_VERSIONS
        .iter()
        .filter(|&&packed| {
            let electron_version = packed_version(packed);
            match comparator {
                Comparator::Greater => electron_version > version,
                Comparator::Less => electron_version < version,
                Comparator::GreaterOrEqual => electron_version >= version,
                Comparator::LessOrEqual => electron_version <= version,
            }
        })
        .map(|&packed| Distrib::new("chrome", packed_chromium(packed)))
        .collect();
    Ok(distribs)
}
