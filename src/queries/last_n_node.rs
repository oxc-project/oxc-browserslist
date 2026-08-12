use super::{Distrib, QueryResult};
use crate::data::node::node_versions;

pub(super) fn last_n_node(count: usize) -> QueryResult {
    let distribs = node_versions()
        .iter()
        .rev()
        .take(count)
        .map(|(_, text)| Distrib::new("node", text.as_ref()))
        .collect();
    Ok(distribs)
}
