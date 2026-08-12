use std::collections::BTreeSet;

use crate::data::Caniuse;

pub mod browsers;
pub mod features;
pub mod regions;

pub use browsers::build_caniuse_browsers;
pub use features::build_caniuse_feature_matching;
pub use regions::build_caniuse_region_matching;

/// Every version string the caniuse generators will reference, for the shared canonical version
/// table. Must stay in sync with the filters in `build_caniuse_feature_matching` (y/a flags) and
/// `build_caniuse_region_matching` (non-null usage, `"0"` → newest); a drifted filter shows up as
/// a loud missing-key panic in the corresponding generator.
pub fn collect_versions(data: &Caniuse) -> BTreeSet<String> {
    let mut versions: BTreeSet<String> = data
        .agents
        .values()
        .flat_map(|agent| agent.version_list.iter().map(|v| v.version.clone()))
        .collect();
    for feature in data.data.values() {
        for browser_versions in feature.stats.values() {
            for (version, flag) in browser_versions {
                if flag != "n" && (flag.contains('y') || flag.contains('a')) {
                    versions.insert(version.clone());
                }
            }
        }
    }
    for region in data.regions.values() {
        for (name, stat) in &region.data {
            let agent = &data.agents[name];
            for (version, usage) in stat {
                if usage.is_some() {
                    versions.insert(if version == "0" {
                        agent.version_list.last().unwrap().version.clone()
                    } else {
                        version.clone()
                    });
                }
            }
        }
    }
    versions
}
