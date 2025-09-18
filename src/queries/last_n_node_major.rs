use super::{Distrib, QueryResult};
use crate::data::node::NODE_VERSIONS;

pub(super) fn last_n_node_major(count: usize) -> QueryResult {
    let mut seen = std::collections::HashSet::new();
    let mut unique_majors = 0;
    let mut minimum = 0;

    for version in NODE_VERSIONS().iter().rev() {
        let major = version.major();
        if seen.insert(major) {
            unique_majors += 1;
            if unique_majors == count {
                minimum = major;
                break;
            }
        }
    }

    let distribs = NODE_VERSIONS()
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
