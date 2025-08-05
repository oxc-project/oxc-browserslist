use std::fs;

use anyhow::Result;
use quote::quote;
use serde::Deserialize;

use super::{generate_file, root};

#[derive(Deserialize)]
struct NodeRelease {
    version: String,
}

pub fn build_node_versions() -> Result<()> {
    let releases_path = root().join("node_modules/node-releases/data/processed/envs.json");
    let releases: Vec<NodeRelease> = serde_json::from_slice(&fs::read(releases_path)?)?;

    let versions = releases.into_iter().map(|release| {
        let version = release.version.split('.').collect::<Vec<_>>();
        assert_eq!(version.len(), 3);
        let major: u16 = version[0].parse().unwrap();
        let minor: u16 = version[1].parse().unwrap();
        let patch: u16 = version[2].parse().unwrap();
        quote! {
            Version(#major, #minor, #patch)
        }
    });
    let output = quote! {
        use crate::semver::Version;

        pub static NODE_VERSIONS: &[Version] = &[#(#versions),*];
    };

    generate_file("node_versions.rs", output);

    Ok(())
}
