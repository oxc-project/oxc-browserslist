use std::{collections::HashMap, fs};

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;

use crate::utils::{generate_file, root};

pub fn build_electron_to_chromium() -> Result<()> {
    let data_path = root().join("node_modules/electron-to-chromium/versions.json");

    // Pack the Chromium version strings into one deduplicated pool referenced by a u32 that
    // bitpacks `offset << 8 | len`, instead of an inline `&[(_, &str)]` whose every `&str`
    // would cost 16 bytes plus a load-time relocation entry in the binary.
    let mut pool = String::new();
    let mut seen: HashMap<String, u32> = HashMap::new();
    let data = serde_json::from_slice::<IndexMap<String, String>>(&fs::read(data_path)?)?
        .into_iter()
        .map(|(electron_version, chromium_version)| {
            let split = electron_version.split('.').collect::<Vec<_>>();
            assert!(split.len() == 2, "electron version must be in major.minor format");
            let major: u16 = split[0].parse().unwrap();
            let minor: u16 = split[1].parse().unwrap();
            let packed = *seen.entry(chromium_version.clone()).or_insert_with(|| {
                let offset = pool.len();
                assert!(chromium_version.len() < 256 && offset < (1 << 24), "pool overflow");
                pool.push_str(&chromium_version);
                ((offset as u32) << 8) | chromium_version.len() as u32
            });
            quote! {
                (ElectronVersion::new(#major, #minor), #packed)
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        use crate::data::electron::ElectronVersion;
        /// Concatenated Chromium-version pool indexed by the packed u32 in [`ELECTRON_VERSIONS`].
        pub static ELECTRON_CHROMIUM_VERSIONS: &str = #pool;
        pub static ELECTRON_VERSIONS: &[(ElectronVersion, u32)] = &[#(#data),*];
    };

    generate_file("electron_to_chromium.rs", output);

    Ok(())
}
