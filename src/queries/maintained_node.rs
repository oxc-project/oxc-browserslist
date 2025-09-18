use time::OffsetDateTime;

use super::{Distrib, QueryResult};
use crate::data::node::{NODE_VERSIONS, RELEASE_SCHEDULE};

pub(super) fn maintained_node() -> QueryResult {
    let now = OffsetDateTime::now_utc().to_julian_day();

    let versions = RELEASE_SCHEDULE
        .iter()
        .filter(|(_, start, end)| *start < now && now < *end)
        .filter_map(|(version, _, _)| {
            NODE_VERSIONS().iter().rfind(|v| v.major() == version.major())
        })
        .map(|version| Distrib::new("node", version.to_string()))
        .collect();
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{opts::Opts, test::run_compare};

    #[test_case("maintained node versions"; "basic")]
    #[test_case("Maintained Node Versions"; "case insensitive")]
    #[test_case("maintained   node     versions"; "more spaces")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
