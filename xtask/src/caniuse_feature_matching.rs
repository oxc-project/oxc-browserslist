use anyhow::Result;
use bincode::encode_to_vec;
use quote::quote;

use super::{Caniuse, create_ranges, encode_browser_name, generate_file, save_bin};

pub fn build_caniuse_feature_matching(data: &Caniuse) -> Result<()> {
    let features = data
        .data
        .iter()
        .map(|(_name, feature)| {
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

    let keys = data.data.keys().collect::<Vec<_>>();

    let data = features
        .iter()
        .map(|v| encode_to_vec(v, bincode::config::standard()).unwrap())
        .collect::<Vec<_>>();
    let data_bytes = data.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin("caniuse_feature_matching.bin", &data_bytes);
    let data_ranges = create_ranges(&data);
    let ranges = data_ranges.iter().map(|(a, b)| quote! {(#a, #b)});

    let output = quote! {
        use crate::data::caniuse::features::Feature;

        pub fn get_feature_stat(name: &str) -> Option<Feature> {
            let ranges = match name {
                #( #keys => #ranges, )*
                _ => return None,
            };
            Some(Feature::new(ranges.0, ranges.1))
        }
    };

    generate_file("caniuse_feature_matching.rs", output);

    Ok(())
}
