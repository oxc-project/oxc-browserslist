use std::fs;

use anyhow::Result;
use bincode::{config::Configuration, encode_to_vec};
use indexmap::IndexMap;
use quote::quote;
use serde::Deserialize;

use super::{
    Caniuse, create_ranges, encode_browser_name, generate_file, root, save_bin_compressed,
};

#[derive(Deserialize)]
struct RegionData {
    data: IndexMap<String, IndexMap<String, Option<f32>>>,
}

struct RegionDatum {
    browser: u8,
    version: String,
    usage: f32,
}

const STANDARD: Configuration = bincode::config::standard();

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
                        usage.map(|usage| RegionDatum {
                            browser: encode_browser_name(&name),
                            version,
                            usage,
                        })
                    })
                })
                .collect::<Vec<_>>();
            usage.sort_unstable_by(|a, b| b.usage.partial_cmp(&a.usage).unwrap());
            let key = file.path().file_stem().unwrap().to_str().map(|s| s.to_owned()).unwrap();
            (key, usage)
        })
        .collect::<Vec<_>>();

    data.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let keys = data.iter().map(|(key, _)| key).collect::<Vec<_>>();

    let browsers = data
        .iter()
        .map(|(_region, datums)| datums.iter().map(|x| x.browser).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let browsers_ranges = create_ranges(&browsers);
    let browsers_bytes = browsers.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin_compressed("caniuse_region_browsers.bin", &browsers_bytes);

    let versions = data
        .iter()
        .map(|(_, datums)| {
            let versions = datums.iter().map(|x| x.version.clone()).collect::<Vec<_>>();
            encode_to_vec(versions, STANDARD).unwrap()
        })
        .collect::<Vec<_>>();
    let version_ranges = create_ranges(&versions);
    let version_bytes = versions.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin_compressed("caniuse_region_versions.bin", &version_bytes);

    let percentages = data
        .iter()
        .map(|(_region, datums)| {
            let percentages = datums.iter().map(|x| x.usage).collect::<Vec<_>>();
            encode_to_vec(percentages, STANDARD).unwrap()
        })
        .collect::<Vec<_>>();
    let percent_ranges = create_ranges(&percentages);
    let percent_bytes = percentages.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin_compressed("caniuse_region_percentages.bin", &percent_bytes);

    let ranges = browsers_ranges
        .iter()
        .zip(version_ranges.iter())
        .zip(percent_ranges.iter())
        .map(|(((a, b), (c, d)), (e, f))| quote! { (#a, #b, #c, #d, #e, #f) });

    let output = quote! {
        use crate::data::caniuse::region::RegionData;

        pub fn get_usage_by_region(region: &str) -> Option<RegionData> {
            let ranges = match region {
                #( #keys => #ranges, )*
                _ => return None,
            };
            Some(RegionData::new(ranges.0, ranges.1, ranges.2, ranges.3, ranges.4, ranges.5))
        }
    };
    generate_file("caniuse_region_matching.rs", output);

    Ok(())
}
