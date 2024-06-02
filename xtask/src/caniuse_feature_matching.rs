use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;

use super::{encode_browser_name, generate_file, Caniuse};

pub fn build_caniuse_feature_matching(data: &Caniuse) -> Result<()> {
    let features = data
        .data
        .iter()
        .map(|(_name, feature)| {
            serde_json::to_string(
                &feature
                    .stats
                    .iter()
                    .filter_map(|(name, versions)| {
                        let name = encode_browser_name(name);
                        let versions = versions
                            .into_iter()
                            .filter(|(_version, flag)| *flag != "n")
                            .collect::<Vec<_>>();
                        let y = versions
                            .iter()
                            .filter(|(_, flag)| flag.contains('y'))
                            .map(|x| x.0.clone())
                            .collect::<Vec<_>>();
                        let a = versions
                            .iter()
                            .filter(|(_, flag)| flag.contains('a'))
                            .map(|x| x.0.clone())
                            .collect::<Vec<_>>();
                        if y.is_empty() && a.is_empty() {
                            None
                        } else {
                            Some((name, (y, a)))
                        }
                    })
                    .collect::<IndexMap<_, _>>(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    let keys = data.data.keys().collect::<Vec<_>>();
    let idents = keys
        .iter()
        .map(|k| quote::format_ident!("_{}", k.replace('-', "_").to_ascii_uppercase()))
        .collect::<Vec<_>>();

    let output = quote! {
        use rustc_hash::FxHashMap;
        use std::sync::OnceLock;
        use serde_json::from_str;
        use crate::data::caniuse::features::{Feature, FeatureSet};
        use crate::data::browser_name::decode_browser_name;

        fn convert(s: &'static str) -> Feature {
            from_str::<FxHashMap::<u8, FeatureSet>>(s)
                .unwrap()
                .into_iter()
                .map(|(browser, versions)| (decode_browser_name(browser), versions))
                .collect()
        }

        pub(crate) fn get_feature_stat(name: &str) -> Option<&'static Feature> {
            match name {
                #( #keys => {
                    static STAT: OnceLock<Feature> = OnceLock::new();
                    Some(STAT.get_or_init(|| convert(#idents)))
                }, )*
                _ => None,
            }
        }

        #(const #idents: &str = #features;)*
    };

    generate_file("caniuse_feature_matching.rs", output);

    Ok(())
}
