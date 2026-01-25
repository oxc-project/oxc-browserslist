use super::{Distrib, QueryResult};
use crate::data::node::{NODE_VERSIONS, RELEASE_SCHEDULE};
use crate::date::now_julian_day;

pub(super) fn maintained_node() -> QueryResult {
    let now = now_julian_day();

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
