use anyhow::Result;
use bincode::encode_to_vec;
use quote::quote;

use super::{Caniuse, generate_file, save_bin_compressed};

pub fn build_caniuse_browsers(data: &Caniuse) -> Result<()> {
    // Convert to simple tuples for easier serialization
    let compact_browsers: Vec<(String, Vec<(String, f32, Option<i64>)>)> = data
        .agents
        .iter()
        .map(|(name, agent)| {
            let compact_versions = agent
                .version_list
                .iter()
                .map(|version| {
                    (version.version.clone(), version.global_usage, version.release_date)
                })
                .collect();

            (name.clone(), compact_versions)
        })
        .collect();

    // Serialize and compress the browser data
    let browsers_bytes = encode_to_vec(&compact_browsers, bincode::config::standard()).unwrap();
    save_bin_compressed("caniuse_browsers.bin", &browsers_bytes);

    let output = quote! {
        use std::num::NonZero;
        use std::sync::OnceLock;
        use rustc_hash::FxHashMap;
        use crate::data::caniuse::{BrowserStat, CaniuseData, VersionDetail};
        use crate::data::caniuse::compression::decompress_deflate;

        static BROWSERS_COMPRESSED: &[u8] = include_bytes!("caniuse_browsers.bin.deflate");
        static BROWSERS_DATA: OnceLock<CaniuseData> = OnceLock::new();

        pub fn caniuse_browsers() -> &'static CaniuseData {
            BROWSERS_DATA.get_or_init(|| {
                use std::collections::HashMap;

                let decompressed = decompress_deflate(BROWSERS_COMPRESSED);
                let compact_browsers: Vec<(String, Vec<(String, f32, Option<i64>)>)> =
                    bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0;

                let mut browsers = FxHashMap::default();
                for (name, versions) in compact_browsers {
                    let version_list = versions.into_iter().map(|(version, global_usage, release_date)| {
                        let release_date = release_date.map(|ts| {
                            NonZero::new(ts).unwrap()
                        });
                        VersionDetail(version.leak(), global_usage, release_date)
                    }).collect();

                    browsers.insert(
                        name.leak(),
                        BrowserStat {
                            name: name.leak(),
                            version_list,
                        }
                    );
                }
                browsers
            })
        }
    };

    generate_file("caniuse_browsers.rs", output);

    Ok(())
}
