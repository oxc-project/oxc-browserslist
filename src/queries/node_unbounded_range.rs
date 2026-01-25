use std::{cmp::Ordering, str::FromStr};

use super::{Distrib, QueryResult};
use crate::{data::node::NODE_VERSIONS, error::Error, parser::Comparator, semver::Version};

pub(super) fn node_unbounded_range(comparator: Comparator, version: &str) -> QueryResult {
    let version = Version::from_str(version)
        .map_err(|_| Error::UnknownNodejsVersion(version.to_string()))?;
    let distribs = NODE_VERSIONS()
        .iter()
        .filter(|v| {
            let ord = (*v).cmp(&version);
            match comparator {
                Comparator::Greater => matches!(ord, Ordering::Greater),
                Comparator::Less => matches!(ord, Ordering::Less),
                Comparator::GreaterOrEqual => matches!(ord, Ordering::Greater | Ordering::Equal),
                Comparator::LessOrEqual => matches!(ord, Ordering::Less | Ordering::Equal),
            }
        })
        .map(|version| Distrib::new("node", version.to_string()))
        .collect();
    Ok(distribs)
}
