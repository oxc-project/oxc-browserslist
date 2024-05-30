use super::{Distrib, QueryResult};
use crate::{data::node::NODE_VERSIONS, parser::Comparator, semver::Version};
use std::{cmp::Ordering, str::FromStr};

pub(super) fn node_unbounded_range(comparator: Comparator, version: &str) -> QueryResult {
    let version = Version::from_str(version).unwrap();
    let distribs = NODE_VERSIONS
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

#[cfg(test)]
mod tests {
    use crate::{
        error::Error,
        opts::Opts,
        test::{run_compare, should_failed},
    };
    use test_case::test_case;

    #[test_case("node <= 5"; "less or equal")]
    #[test_case("node < 5"; "less")]
    #[test_case("node >= 9"; "greater or equal")]
    #[test_case("node > 9"; "greater")]
    #[test_case("Node <= 5"; "case insensitive")]
    #[test_case("node > 10.12"; "with semver minor")]
    #[test_case("node > 10.12.1"; "with semver patch")]
    #[test_case("node >= 8.8.8.8"; "malformed version")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case(
        "node < 8.a", Error::Nom(String::from("a"));
        "malformed version"
    )]
    fn invalid(query: &str, error: Error) {
        assert_eq!(should_failed(query, &Opts::default()), error);
    }
}
