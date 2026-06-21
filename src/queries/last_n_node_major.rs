use super::{Distrib, QueryResult};
use crate::data::node::NODE_VERSIONS;

pub(super) fn last_n_node_major(count: usize) -> QueryResult {
    let mut vec =
        NODE_VERSIONS().iter().rev().map(|(version, _)| version.major()).collect::<Vec<_>>();
    vec.dedup();
    let minimum = vec.into_iter().nth(count - 1).unwrap_or_default();

    let distribs = NODE_VERSIONS()
        .iter()
        .filter(|(version, _)| version.major() >= minimum)
        .rev()
        .map(|(_, text)| Distrib::new("node", text.as_ref()))
        .collect();

    Ok(distribs)
}
