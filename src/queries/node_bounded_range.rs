use std::cmp::Ordering;

use super::{Distrib, QueryResult};
use crate::data::node::node_versions;

pub(super) fn node_bounded_range(from: &str, to: &str) -> QueryResult {
    let distribs = node_versions()
        .iter()
        .filter(|(version, _)| {
            matches!(version.loose_compare(from), Ordering::Greater | Ordering::Equal)
                && matches!(version.loose_compare(to), Ordering::Less | Ordering::Equal)
        })
        .map(|(_, text)| Distrib::new("node", text.as_ref()))
        .collect();
    Ok(distribs)
}
