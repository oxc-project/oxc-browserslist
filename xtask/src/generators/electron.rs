use std::fs;

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;

use crate::utils::{generate_file, root};

pub fn build_electron_to_chromium() -> Result<()> {
    let data_path = root().join("node_modules/electron-to-chromium/versions.json");
    let data = serde_json::from_slice::<IndexMap<String, String>>(&fs::read(data_path)?)?
        .into_iter()
        .map(|(electron_version, chromium_version)| {
            let split = electron_version.split('.').collect::<Vec<_>>();
            assert!(split.len() == 2, "electron version must be in major.minor format");
            let major: u16 = split[0].parse().unwrap();
            let minor: u16 = split[1].parse().unwrap();
            quote! {
                (ElectronVersion::new(#major, #minor), #chromium_version)
            }
        });

    let output = quote! {
        use crate::data::electron::ElectronVersion;
        pub static ELECTRON_VERSIONS: &[(ElectronVersion, &str)] = &[#(#data),*];
    };

    generate_file("electron_to_chromium.rs", output);

    Ok(())
}
