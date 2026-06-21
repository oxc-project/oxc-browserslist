use super::{Distrib, QueryResult};
use crate::data::node::NODE_VERSIONS;

pub(super) fn last_n_node(count: usize) -> QueryResult {
    let distribs = NODE_VERSIONS()
        .iter()
        .rev()
        .take(count)
        .map(|(_, text)| Distrib::new("node", text.as_ref()))
        .collect();
    Ok(distribs)
}
