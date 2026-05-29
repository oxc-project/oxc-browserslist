use super::{Distrib, QueryResult};
use crate::data::{
    caniuse::{CANIUSE_GLOBAL_USAGE, GLOBAL_USAGE_VERSIONS},
    decode_browser_name, unpack_str,
};

pub(super) fn cover(coverage: f32) -> QueryResult {
    let mut distribs = vec![];
    let mut total = 0.0;
    for (name, version, usage) in CANIUSE_GLOBAL_USAGE {
        if total >= coverage || *usage == 0.0 {
            break;
        }
        let version = unpack_str(GLOBAL_USAGE_VERSIONS, *version);
        distribs.push(Distrib::new(decode_browser_name(*name), version));
        total += usage;
    }
    Ok(distribs)
}
