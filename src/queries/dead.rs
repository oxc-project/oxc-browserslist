use super::QueryResult;
use crate::{opts::Opts, resolve};

pub(super) fn dead(opts: &Opts) -> QueryResult {
    resolve(
        &["Baidu >= 0", "ie <= 11", "ie_mob <= 11", "bb <= 10", "op_mob <= 12.1", "samsung 4"],
        opts,
    )
}
