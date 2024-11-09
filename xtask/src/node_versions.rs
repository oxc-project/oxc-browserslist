use std::fs;

use anyhow::Result;
use quote::quote;
use serde::Deserialize;

use rkyv::Archive as RkyvArchive;
use rkyv::Serialize as RkyvSerialize;

use super::{generate_file, generate_rkyv, root};

#[derive(Deserialize)]
struct NodeRelease {
    version: String,
}

#[derive(RkyvSerialize, RkyvArchive)]
pub struct Version(u32, u32, u32);

pub fn build_node_versions() -> Result<()> {
    let releases_path = root().join("node_modules/node-releases/data/processed/envs.json");
    let releases: Vec<NodeRelease> = serde_json::from_slice(&fs::read(releases_path)?)?;

    let versions: Vec<_> = releases
        .into_iter()
        .map(|release| {
            let version = release.version.split('.').collect::<Vec<_>>();
            assert_eq!(version.len(), 3);
            let major: u32 = version[0].parse().unwrap();
            let minor: u32 = version[1].parse().unwrap();
            let patch: u32 = version[2].parse().unwrap();
            Version(major, minor, patch)
        })
        .collect();

    let output = quote! {
        use crate::semver::{Version, ArchivedVersion};

        use rkyv::vec::ArchivedVec;
        use std::sync::OnceLock;

        type Data = Vec<Version>;
        type ArchivedData = ArchivedVec<ArchivedVersion>;

        const RKYV_BYTES: &'static [u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }
            const ALIGNED: &'static Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("node_versions.rkyv") };
            &ALIGNED.bytes
        };

        pub fn get_node_versions() -> &'static ArchivedData {
            static NODE_VERSIONS: OnceLock<&ArchivedData> = OnceLock::new();
            NODE_VERSIONS.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::access_unchecked::<ArchivedData>(RKYV_BYTES) }
            })
        }
    };

    generate_rkyv("node_versions.rkyv", &versions);
    generate_file("node_versions.rs", output);

    Ok(())
}
