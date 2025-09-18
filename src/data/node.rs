use std::sync::OnceLock;

pub use crate::generated::node_release_schedule::RELEASE_SCHEDULE;
use crate::semver::Version;

#[allow(non_snake_case)]
pub fn NODE_VERSIONS() -> &'static [Version] {
    static NODE_VERSIONS_DATA: OnceLock<Vec<Version>> = OnceLock::new();
    NODE_VERSIONS_DATA.get_or_init(|| {
        const COMPRESSED: &[u8] = include_bytes!("../generated/node_versions.bin.deflate");
        let decompressed = crate::data::caniuse::compression::decompress_deflate(COMPRESSED);
        let data: Vec<(u16, u16, u16)> =
            bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0;
        data.into_iter().map(|(major, minor, patch)| Version(major, minor, patch)).collect()
    })
}
