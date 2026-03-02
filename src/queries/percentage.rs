use super::{Distrib, QueryResult};
use crate::{data::caniuse::caniuse_browsers, parser::Comparator};

pub(super) fn percentage(comparator: Comparator, popularity: f32) -> QueryResult {
    let distribs = caniuse_browsers()
        .iter()
        .flat_map(|(name, stat)| {
            stat.version_list
                .iter()
                .filter(|version| {
                    let usage = version.global_usage();
                    comparator.compare_f32(usage, popularity)
                })
                .map(move |version| Distrib::new(name.as_ref(), version.version()))
        })
        .collect();
    Ok(distribs)
}
