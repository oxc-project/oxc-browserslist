use std::collections::BTreeSet;

use crate::data::Caniuse;

pub mod browsers;
pub mod features;
pub mod regions;

pub use browsers::build_caniuse_browsers;
pub use features::build_caniuse_feature_matching;
pub use regions::build_caniuse_region_matching;

/// Every version string the caniuse generators will reference, for the shared canonical version
/// table. Each generator exposes its own collector next to the build code it must agree with; a
/// drifted filter shows up as a loud missing-key panic in the corresponding generator.
pub fn collect_versions(data: &Caniuse) -> BTreeSet<String> {
    let mut versions: BTreeSet<String> = data
        .agents
        .values()
        .flat_map(|agent| agent.version_list.iter().map(|v| v.version.clone()))
        .collect();
    versions.extend(features::feature_versions(data));
    versions.extend(regions::region_versions(data));
    versions
}
