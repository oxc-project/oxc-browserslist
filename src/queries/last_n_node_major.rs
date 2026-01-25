use super::{Distrib, QueryResult};
use crate::data::node::NODE_VERSIONS;

pub(super) fn last_n_node_major(count: usize) -> QueryResult {
    let mut vec = NODE_VERSIONS().iter().rev().map(|version| version.major()).collect::<Vec<_>>();
    vec.dedup();
    let minimum = vec.into_iter().nth(count - 1).unwrap_or_default();

    let distribs = NODE_VERSIONS()
        .iter()
        .filter(|version| version.major() >= minimum)
        .rev()
        .map(|version| Distrib::new("node", version.to_string()))
        .collect();

    Ok(distribs)
}
