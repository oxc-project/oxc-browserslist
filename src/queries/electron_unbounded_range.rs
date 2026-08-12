use super::{Distrib, QueryResult};
use crate::{
    data::electron::{electron_versions, parse_version, unpack_chromium, unpack_version},
    parser::Comparator,
};

pub(super) fn electron_unbounded_range(comparator: Comparator, version: &str) -> QueryResult {
    let version = parse_version(version)?;

    let distribs = electron_versions()
        .iter()
        .filter(|&&packed| {
            let electron_version = unpack_version(packed);
            match comparator {
                Comparator::Greater => electron_version > version,
                Comparator::Less => electron_version < version,
                Comparator::GreaterOrEqual => electron_version >= version,
                Comparator::LessOrEqual => electron_version <= version,
            }
        })
        .map(|&packed| Distrib::new("chrome", unpack_chromium(packed)))
        .collect();
    Ok(distribs)
}
