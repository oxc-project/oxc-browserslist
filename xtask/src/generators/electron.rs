use std::{collections::HashMap, fs};

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;

use crate::utils::{generate_file, root};

pub fn build_electron_to_chromium() -> Result<()> {
    let data_path = root().join("node_modules/electron-to-chromium/versions.json");

    // Pack each mapping into a single u32 — `major << 24 | minor << 16 | offset << 4 | len`,
    // where offset/len reference the (deduplicated) Chromium version strings concatenated into
    // one pool — instead of an inline `&[(ElectronVersion, u32)]` whose 8-byte entries spend
    // half their space on field padding. Entries stay in ascending (major, minor) order.
    let mut pool = String::new();
    let mut seen: HashMap<String, u32> = HashMap::new();
    let data = serde_json::from_slice::<IndexMap<String, String>>(&fs::read(data_path)?)?
        .into_iter()
        .map(|(electron_version, chromium_version)| {
            let split = electron_version.split('.').collect::<Vec<_>>();
            assert!(split.len() == 2, "electron version must be in major.minor format");
            let major: u32 = split[0].parse().unwrap();
            let minor: u32 = split[1].parse().unwrap();
            assert!(major < 256 && minor < 256, "electron version overflow");
            let chromium = *seen.entry(chromium_version.clone()).or_insert_with(|| {
                let offset = pool.len();
                assert!(chromium_version.len() < 16 && offset < (1 << 12), "pool overflow");
                pool.push_str(&chromium_version);
                ((offset as u32) << 4) | chromium_version.len() as u32
            });
            let packed = major << 24 | minor << 16 | chromium;
            quote! { #packed }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        /// Concatenated Chromium-version pool referenced by [`ELECTRON_VERSIONS`].
        pub static ELECTRON_CHROMIUM_VERSIONS: &str = #pool;
        /// Electron (major, minor) -> Chromium version, ascending. Each u32 bitpacks
        /// `major << 24 | minor << 16 | pool_offset << 4 | pool_len`; unpack with
        /// `electron::packed_version` and `electron::packed_chromium`.
        pub static ELECTRON_VERSIONS: &[u32] = &[#(#data),*];
    };

    generate_file("electron_to_chromium.rs", output);

    Ok(())
}
