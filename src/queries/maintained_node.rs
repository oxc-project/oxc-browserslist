use super::{Distrib, QueryResult};
use crate::data::node::{NODE_VERSIONS, release_schedule};
use crate::date::now_julian_day;

pub(super) fn maintained_node() -> QueryResult {
    let now = now_julian_day();

    let versions = release_schedule()
        .iter()
        .filter(|(_, start, end)| *start < now && now < *end)
        .filter_map(|(version, _, _)| {
            NODE_VERSIONS().iter().rfind(|(v, _)| v.major() == version.major())
        })
        .map(|(_, text)| Distrib::new("node", text.as_ref()))
        .collect();
    Ok(versions)
}
