use super::{Distrib, QueryResult};
use crate::data::{
    caniuse::{CANIUSE_GLOBAL_USAGE, GLOBAL_USAGE_VERSIONS},
    decode_browser_name, unpack_str,
};

pub(super) fn cover(coverage: f32) -> QueryResult {
    let mut distribs = vec![];
    let mut total = 0.0;
    for (packed, usage) in CANIUSE_GLOBAL_USAGE {
        if total >= coverage || *usage == 0.0 {
            break;
        }
        // The u32 bitpacks `browser_id << 24 | offset << 8 | len`; the low 24 bits are the
        // `unpack_str` reference into the version pool.
        let version = unpack_str(GLOBAL_USAGE_VERSIONS, packed & 0x00ff_ffff);
        distribs.push(Distrib::new(decode_browser_name((packed >> 24) as u8), version));
        total += usage;
    }
    Ok(distribs)
}
