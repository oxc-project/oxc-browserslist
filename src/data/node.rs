use std::sync::OnceLock;

use crate::semver::Version;

pub use crate::generated::node_release_schedule::RELEASE_SCHEDULE;

#[allow(non_snake_case)]
pub fn NODE_VERSIONS() -> &'static [Version] {
    static NODE_VERSIONS: OnceLock<Vec<Version>> = OnceLock::new();
    NODE_VERSIONS.get_or_init(|| {
        const COMPRESSED: &[u8] = include_bytes!("../generated/node_versions.bin.deflate");
        let decompressed = super::caniuse::compression::decompress_deflate(COMPRESSED);
        let versions: Vec<(u16, u16, u16)> = postcard::from_bytes(&decompressed).unwrap();
        versions.into_iter().map(|(major, minor, patch)| Version(major, minor, patch)).collect()
    })
}
