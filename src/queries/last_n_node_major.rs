use super::{Distrib, QueryResult};
use crate::data::node::node_versions;

pub(super) fn last_n_node_major(count: usize) -> QueryResult {
    let mut vec =
        node_versions().iter().rev().map(|(version, _)| version.major()).collect::<Vec<_>>();
    vec.dedup();
    // `count` can be 0 ("last 0 node major versions"); like browserslist-js, a minimum below
    // every version selects them all.
    let minimum = count.checked_sub(1).and_then(|n| vec.into_iter().nth(n)).unwrap_or_default();

    let distribs = node_versions()
        .iter()
        .filter(|(version, _)| version.major() >= minimum)
        .rev()
        .map(|(_, text)| Distrib::new("node", text.as_ref()))
        .collect();

    Ok(distribs)
}
