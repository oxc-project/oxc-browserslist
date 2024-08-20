use super::{Distrib, QueryResult};
use crate::data::node::get_node_versions;

pub(super) fn last_n_node_major(count: usize) -> QueryResult {
    let mut vec =
        get_node_versions().iter().rev().map(|version| version.major()).collect::<Vec<_>>();
    vec.dedup();
    let minimum = vec.into_iter().nth(count - 1).unwrap_or_default();

    let distribs = get_node_versions()
        .iter()
        .filter(|version| version.major() >= minimum)
        .rev()
        .map(|version| Distrib::new("node", version.to_string()))
        .collect();

    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{opts::Opts, test::run_compare};

    #[test_case("last 2 node major versions"; "basic")]
    #[test_case("last 2 Node major versions"; "case insensitive")]
    #[test_case("last 2 node major version"; "support pluralization")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
