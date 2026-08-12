use super::{Distrib, QueryResult};
use crate::data::{
    caniuse::{global_usage, version_table},
    decode_browser_name,
};

pub(super) fn cover(coverage: f32) -> QueryResult {
    let table = version_table();
    let mut distribs = vec![];
    let mut total = 0.0;
    // Entries are usage-descending; take browsers until the requested coverage is reached.
    for &(browser_id, version_index, usage) in global_usage() {
        if total >= coverage || usage == 0.0 {
            break;
        }
        let version = table[usize::from(version_index)].as_str();
        distribs.push(Distrib::new(decode_browser_name(browser_id), version));
        total += usage;
    }
    Ok(distribs)
}
