use std::sync::OnceLock;

use crate::semver::Version;

pub use crate::generated::node_release_schedule::RELEASE_SCHEDULE;

#[allow(non_snake_case)]
pub fn NODE_VERSIONS() -> &'static [Version] {
    static NODE_VERSIONS: OnceLock<Vec<Version>> = OnceLock::new();
    NODE_VERSIONS.get_or_init(|| {
        let versions: Vec<(u16, u16, u16)> = super::caniuse::compression::load(include_bytes!(
            "../generated/node_versions.bin.deflate"
        ));
        versions.into_iter().map(|(major, minor, patch)| Version(major, minor, patch)).collect()
    })
}
