use super::{Distrib, QueryResult};

pub(super) fn op_mini() -> QueryResult {
    Ok(vec![Distrib::new("op_mini", "all")])
}
