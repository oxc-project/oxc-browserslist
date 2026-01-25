use super::{Distrib, QueryResult};
use crate::data::{caniuse::CANIUSE_GLOBAL_USAGE, decode_browser_name};

pub(super) fn cover(coverage: f32) -> QueryResult {
    let mut distribs = vec![];
    let mut total = 0.0;
    for (name, version, usage) in CANIUSE_GLOBAL_USAGE {
        if total >= coverage || *usage == 0.0 {
            break;
        }
        distribs.push(Distrib::new(decode_browser_name(*name), *version));
        total += usage;
    }
    Ok(distribs)
}
