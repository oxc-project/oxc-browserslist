use std::{fs, str::FromStr};

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;
use serde::Deserialize;

use super::{Caniuse, encode_browser_name, generate_file, root};

#[derive(Deserialize)]
struct RegionData {
    data: IndexMap<String, IndexMap<String, Option<f32>>>,
}

pub fn build_caniuse_region_matching(data: &Caniuse) -> Result<()> {
    let agents = &data.agents;
    let files_path = root().join("node_modules/caniuse-db/region-usage-json");
    let files = fs::read_dir(files_path)?
        .map(|entry| entry.map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>>>()?;

    let mut data = files
        .iter()
        .map(|file| {
            let RegionData { data } =
                serde_json::from_slice(&fs::read(file.path()).unwrap()).unwrap();
            let mut usage = data
                .into_iter()
                .flat_map(|(name, stat)| {
                    let agent = agents.get(&name).unwrap();
                    stat.into_iter().filter_map(move |(version, usage)| {
                        let version = if version.as_str() == "0" {
                            agent.version_list.last().unwrap().version.clone()
                        } else {
                            version
                        };
                        usage.map(|usage| (encode_browser_name(&name), version, usage))
                    })
                })
                .collect::<Vec<_>>();
            usage.sort_unstable_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());
            let key = file.path().file_stem().unwrap().to_str().map(|s| s.to_owned()).unwrap();
            let value = {
                let s = serde_json::to_string(&usage).unwrap();
                let wrapped = format!("r#\"{}\"#", s);
                proc_macro2::Literal::from_str(&wrapped).unwrap()
            };
            (key, value)
        })
        .collect::<Vec<_>>();

    data.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let keys = data.iter().map(|(key, _)| key).collect::<Vec<_>>();

    let idents = keys
        .iter()
        .map(|k| quote::format_ident!("_{}", k.replace('-', "_").to_ascii_uppercase()))
        .collect::<Vec<_>>();

    let data = data.iter().map(|(_, value)| value).collect::<Vec<_>>();

    let output = quote! {
        use serde_json::from_str;
        use crate::data::BrowserName;
        use crate::data::browser_name::decode_browser_name;

        type RegionData = Vec<(BrowserName, &'static str, f32)>;

        fn convert(s: &'static str) -> RegionData {
            from_str::<Vec<(u8, &'static str, f32)>>(s)
                .unwrap()
                .into_iter()
                .map(|(browser, version, usage)| (decode_browser_name(browser), version, usage))
                .collect::<Vec<_>>()
        }

        pub fn get_usage_by_region(region: &str) -> Option<RegionData> {
            match region {
                #( #keys => Some(convert(#idents)), )*
                _ => None,
            }
        }

        #(const #idents: &str = #data;)*
    };
    generate_file("caniuse_region_matching.rs", output);

    Ok(())
}
