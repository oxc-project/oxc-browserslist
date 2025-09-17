use anyhow::Result;
use bincode::encode_to_vec;
use quote::quote;

use crate::data::Caniuse;
use crate::utils::{generate_file, save_bin_compressed};

pub fn build_caniuse_browsers(data: &Caniuse) -> Result<()> {
    // Prepare data for serialization - convert to Vec for bincode compatibility
    let browser_data: Vec<(String, String, Vec<(String, f32, Option<i64>)>)> = data
        .agents
        .iter()
        .map(|(name, agent)| {
            let version_list = agent
                .version_list
                .iter()
                .map(|version| {
                    (version.version.clone(), version.global_usage, version.release_date)
                })
                .collect();
            (name.clone(), name.clone(), version_list)
        })
        .collect();

    // Serialize and compress
    let serialized = encode_to_vec(&browser_data, bincode::config::standard()).unwrap();
    save_bin_compressed("caniuse_browsers.bin", &serialized);

    // Generate the runtime code
    let output = quote! {
        use std::num::NonZero;
        use std::sync::OnceLock;
        use rustc_hash::FxHashMap;
        use crate::data::caniuse::{BrowserStat, CaniuseData, VersionDetail, compression::decompress_deflate};

        pub fn caniuse_browsers() -> &'static CaniuseData {
            static CANIUSE_BROWSERS: OnceLock<CaniuseData> = OnceLock::new();
            CANIUSE_BROWSERS.get_or_init(|| {
                const COMPRESSED: &[u8] = include_bytes!("../generated/caniuse_browsers.bin.deflate");

                let decompressed = decompress_deflate(COMPRESSED);
                let data: Vec<(String, String, Vec<(String, f32, Option<i64>)>)> =
                    bincode::decode_from_slice(&decompressed, bincode::config::standard())
                        .unwrap()
                        .0;

                data.into_iter()
                    .map(|(_key, name, version_list)| {
                        let name_static = Box::leak(name.into_boxed_str());
                        let stat = BrowserStat {
                            name: name_static,
                            version_list: version_list
                                .into_iter()
                                .map(|(ver, usage, date)| {
                                    let ver_static = Box::leak(ver.into_boxed_str());
                                    VersionDetail(
                                        ver_static,
                                        usage,
                                        date.and_then(|d| NonZero::new(d))
                                    )
                                })
                                .collect(),
                        };
                        (name_static as &str, stat)
                    })
                    .collect()
            })
        }
    };

    generate_file("caniuse_browsers.rs", output);

    Ok(())
}
