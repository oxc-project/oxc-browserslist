use super::{Distrib, QueryResult};
use crate::data::caniuse::{CANIUSE_GLOBAL_USAGE, unpack_usage};

pub(super) fn cover(coverage: f32) -> QueryResult {
    let mut distribs = vec![];
    let mut total = 0.0;
    for (packed, usage) in CANIUSE_GLOBAL_USAGE {
        if total >= coverage || *usage == 0.0 {
            break;
        }
        let (name, version) = unpack_usage(*packed);
        distribs.push(Distrib::new(name, version));
        total += usage;
    }
    Ok(distribs)
}
