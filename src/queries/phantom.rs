use super::{Distrib, QueryResult};

pub(super) fn phantom(is_later_version: bool) -> QueryResult {
    let version = if is_later_version { "6" } else { "5" };
    Ok(vec![Distrib::new("safari", version)])
}
