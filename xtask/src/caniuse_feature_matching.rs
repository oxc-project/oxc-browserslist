use std::str::FromStr;

use anyhow::Result;
use quote::quote;

use super::{Caniuse, encode_browser_name, generate_file};

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
        .map(|list| {
            let s = serde_json::to_string(&list).unwrap();
            let wrapped = format!("r#\"{}\"#", s);
            proc_macro2::Literal::from_str(&wrapped).unwrap()
        })
        .collect::<Vec<_>>();

    let keys = data.data.keys().collect::<Vec<_>>();
    let idents = keys
        .iter()
        .map(|k| quote::format_ident!("_{}", k.replace('-', "_").to_ascii_uppercase()))
        .collect::<Vec<_>>();

    let output = quote! {
        use std::sync::OnceLock;
        use serde_json::from_str;
        use crate::data::caniuse::features::{Feature, FeatureSet};
        use crate::data::browser_name::decode_browser_name;

        fn convert(s: &'static str) -> Feature {
            Feature::new(from_str::<Vec<(u8, Vec<&'static str>, Vec<&'static str>)>>(s)
                .unwrap()
                .into_iter()
                .map(|(browser, yes, partial)| (decode_browser_name(browser), FeatureSet::new(yes, partial)))
                .collect())
        }

        pub fn get_feature_stat(name: &str) -> Option<&'static Feature> {
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
