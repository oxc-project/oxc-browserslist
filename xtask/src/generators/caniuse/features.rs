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

    let keys = sorted_data.keys().cloned().collect::<Vec<_>>();

    let data = features.iter().map(|v| to_allocvec(v).unwrap()).collect::<Vec<_>>();
    let data_bytes = data.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin_compressed("caniuse_feature_matching.bin", &data_bytes);

    let data_range = create_range_vec(&data);

    let output = quote! {
        use crate::data::caniuse::features::Feature;

        static KEYS: &[&str] = &[#(#keys),*];
        static RANGES: &[u32] = &[#(#data_range),*];

        pub fn get_feature_stat(name: &str) -> Option<Feature> {
            match KEYS.binary_search(&name) {
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
