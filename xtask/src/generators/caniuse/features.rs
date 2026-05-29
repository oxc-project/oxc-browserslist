use std::collections::{BTreeSet, HashMap};

use anyhow::Result;
use postcard::to_allocvec;
use quote::quote;

use crate::data::{Caniuse, encode_browser_name};
use crate::utils::{create_range_vec, generate_file, save_bin_compressed};

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

    // Store the feature-name lookup keys as a compressed blob rather than an inline
    // `&[&str]`: the slice would cost 16 bytes of (relocated) fat pointer per entry plus the
    // raw string bytes, all uncompressed in the binary. The blob is decoded once on first use.
    let keys = sorted_data.keys().cloned().collect::<Vec<String>>();
    save_bin_compressed("caniuse_feature_keys.bin", &to_allocvec(&keys).unwrap());

    // The `y`/`a` lists hold version strings drawn from a small shared vocabulary that repeats
    // across every feature. Intern them into one lexicographically-sorted table and store u16
    // indices instead. Because the table is sorted, each (already lexicographically-sorted) list
    // becomes a strictly ascending index sequence — compact for deflate and still correctly
    // ordered for the runtime's binary searches.
    let mut all_versions = BTreeSet::new();
    for feature in &features {
        for (_, y, a) in feature {
            all_versions.extend(y.iter().cloned());
            all_versions.extend(a.iter().cloned());
        }
    }
    let version_table: Vec<String> = all_versions.into_iter().collect();
    let version_to_index: HashMap<&str, u16> =
        version_table.iter().enumerate().map(|(i, v)| (v.as_str(), i as u16)).collect();
    let table_bytes = to_allocvec(&version_table).unwrap();
    save_bin_compressed("caniuse_feature_version_table.bin", &table_bytes);

    // Store absolute indices (not gaps): the same index runs recur across features, and deflate
    // exploits that cross-feature repetition far better than it does delta-encoded gaps.
    let intern = |versions: &[String]| -> Vec<u16> {
        versions.iter().map(|v| version_to_index[v.as_str()]).collect()
    };
    let data = features
        .iter()
        .map(|feature| {
            let remapped: Vec<(u8, Vec<u16>, Vec<u16>)> =
                feature.iter().map(|(b, y, a)| (*b, intern(y), intern(a))).collect();
            to_allocvec(&remapped).unwrap()
        })
        .collect::<Vec<_>>();
    let data_bytes = data.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin_compressed("caniuse_feature_matching.bin", &data_bytes);

    let data_range = create_range_vec(&data);

    let output = quote! {
        use std::sync::OnceLock;

        use crate::data::caniuse::{compression::decompress_deflate, features::Feature};

        static KEYS: OnceLock<Vec<String>> = OnceLock::new();
        static RANGES: &[u32] = &[#(#data_range),*];

        pub fn get_feature_stat(name: &str) -> Option<Feature> {
            let keys = KEYS.get_or_init(|| {
                postcard::from_bytes(&decompress_deflate(include_bytes!("caniuse_feature_keys.bin.deflate")))
                    .unwrap()
            });
            match keys.binary_search_by(|key| key.as_str().cmp(name)) {
                Ok(idx) => {
                    let start = RANGES[idx];
                    let end = RANGES[idx + 1];
                    Some(Feature::new(start, end))
                },
                Err(_) => None,
            }
        }
    };

    generate_file("caniuse_feature_matching.rs", output);

    Ok(())
}
