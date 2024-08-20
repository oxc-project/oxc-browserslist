use super::{Distrib, QueryResult};
use crate::data::node::get_node_versions;

pub(super) fn last_n_node(count: usize) -> QueryResult {
    let distribs = get_node_versions()
        .iter()
        .rev()
        .take(count)
        .map(|version| Distrib::new("node", version.to_string()))
        .collect();
    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{opts::Opts, test::run_compare};

    #[test_case("last 2 node versions"; "basic")]
    #[test_case("last 2 Node versions"; "case insensitive")]
    #[test_case("last 2 node version"; "support pluralization")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
