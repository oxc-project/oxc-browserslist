use anyhow::Result;
use bincode::encode_to_vec;
use quote::quote;

use super::{Caniuse, create_range_vec, encode_browser_name, generate_file, save_bin_compressed};

pub fn build_caniuse_feature_matching(data: &Caniuse) -> Result<()> {
    let mut sorted_data = data.data.clone();
    sorted_data.sort_unstable_keys();
    let features = sorted_data
        .values()
        .map(|feature| {
            feature
                .stats
                .iter()
                .filter_map(|(name, versions)| {
                    let name = encode_browser_name(name);
                    let versions = versions
                        .into_iter()
                        .filter(|(_version, flag)| *flag != "n")
                        .collect::<Vec<_>>();
                    let mut y = versions
                        .iter()
                        .filter(|(_, flag)| flag.contains('y'))
                        .map(|x| x.0.clone())
                        .collect::<Vec<_>>();
                    y.sort_unstable();
                    let mut a = versions
                        .iter()
                        .filter(|(_, flag)| flag.contains('a'))
                        .map(|x| x.0.clone())
                        .collect::<Vec<_>>();
                    a.sort_unstable();
                    if y.is_empty() && a.is_empty() { None } else { Some((name, y, a)) }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let keys = sorted_data.keys().cloned().collect::<Vec<_>>();

    // Create a compact lookup table mapping feature name hashes to indices
    let mut lookup_map: rustc_hash::FxHashMap<String, u32> = rustc_hash::FxHashMap::default();
    for (idx, key) in keys.iter().enumerate() {
        lookup_map.insert(key.clone(), idx as u32);
    }

    // Serialize the lookup map
    let lookup_bytes = encode_to_vec(&lookup_map, bincode::config::standard()).unwrap();
    save_bin_compressed("caniuse_feature_lookup.bin", &lookup_bytes);

    let data = features
        .iter()
        .map(|v| encode_to_vec(v, bincode::config::standard()).unwrap())
        .collect::<Vec<_>>();
    let data_bytes = data.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin_compressed("caniuse_feature_matching.bin", &data_bytes);

    let data_range = create_range_vec(&data);

    // Store ranges in binary format too to reduce static data
    let ranges_bytes = encode_to_vec(&data_range, bincode::config::standard()).unwrap();
    save_bin_compressed("caniuse_feature_ranges.bin", &ranges_bytes);

    let output = quote! {
        use crate::data::caniuse::features::Feature;
        use crate::data::caniuse::{decode, decompress_deflate};
        use rustc_hash::FxHashMap;
        use std::sync::OnceLock;

        static LOOKUP_COMPRESSED: &[u8] = include_bytes!("caniuse_feature_lookup.bin.deflate");
        static RANGES_COMPRESSED: &[u8] = include_bytes!("caniuse_feature_ranges.bin.deflate");
        static LOOKUP_DATA: OnceLock<FxHashMap<String, u32>> = OnceLock::new();
        static RANGES_DATA: OnceLock<Vec<u32>> = OnceLock::new();

        fn get_lookup_map() -> &'static FxHashMap<String, u32> {
            LOOKUP_DATA.get_or_init(|| {
                let decompressed = decompress_deflate(LOOKUP_COMPRESSED);
                bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0
            })
        }

        fn get_ranges() -> &'static Vec<u32> {
            RANGES_DATA.get_or_init(|| {
                let decompressed = decompress_deflate(RANGES_COMPRESSED);
                bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0
            })
        }

        pub fn get_feature_stat(name: &str) -> Option<Feature> {
            let lookup_map = get_lookup_map();
            let ranges = get_ranges();

            if let Some(&idx) = lookup_map.get(name) {
                let start = ranges[idx as usize];
                let end = ranges[idx as usize + 1];
                Some(Feature::new(start, end))
            } else {
                None
            }
        }
    };

    generate_file("caniuse_feature_matching.rs", output);

    Ok(())
}
