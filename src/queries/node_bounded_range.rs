use std::cmp::Ordering;

use super::{Distrib, QueryResult};
use crate::data::node::NODE_VERSIONS;

pub(super) fn node_bounded_range(from: &str, to: &str) -> QueryResult {
    let distribs = NODE_VERSIONS()
        .iter()
        .filter(|(version, _)| {
            matches!(version.loose_compare(from), Ordering::Greater | Ordering::Equal)
                && matches!(version.loose_compare(to), Ordering::Less | Ordering::Equal)
        })
        .map(|(_, text)| Distrib::new("node", text.as_ref()))
        .collect();
    Ok(distribs)
}
