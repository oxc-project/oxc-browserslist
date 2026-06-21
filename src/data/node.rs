use std::sync::OnceLock;

use crate::semver::Version;

pub use crate::generated::node_release_schedule::RELEASE_SCHEDULE;

/// Node.js versions, each paired with its formatted `major.minor.patch` string.
///
/// The string is built once here so node queries can hand out a borrowed `&'static str`
/// instead of allocating a fresh `String` per result on every `resolve`.
#[allow(non_snake_case)]
pub fn NODE_VERSIONS() -> &'static [(Version, Box<str>)] {
    static NODE_VERSIONS: OnceLock<Vec<(Version, Box<str>)>> = OnceLock::new();
    NODE_VERSIONS.get_or_init(|| {
        let versions: Vec<(u16, u16, u16)> = super::caniuse::compression::load(include_bytes!(
            "../generated/node_versions.bin.deflate"
        ));
        versions
            .into_iter()
            .map(|(major, minor, patch)| {
                let version = Version(major, minor, patch);
                let text = version.to_string().into_boxed_str();
                (version, text)
            })
            .collect()
    })
}
