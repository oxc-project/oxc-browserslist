use std::{collections::HashMap, fs};

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;
use serde::Deserialize;

use super::{generate_file, generate_rkyv, root, Caniuse};

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

    let usage = files.iter().map(|file| {
        let RegionData { data } = serde_json::from_slice(&fs::read(file.path()).unwrap()).unwrap();
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
                    usage.map(|usage| (name.clone(), version, usage))
                })
            })
            .collect::<Vec<_>>();
        usage.sort_unstable_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());
        usage
    });

    let regions = files
        .iter()
        .map(|entry| entry.path().file_stem().unwrap().to_str().map(|s| s.to_owned()).unwrap())
        .map(|k| k.replace("-", "_").to_ascii_uppercase());

    let region_to_usage: HashMap<String, Vec<(String, String, f32)>> = regions.zip(usage).collect();

    let output = quote! {
        use rkyv::collections::ArchivedHashMap;
        use rkyv::string::ArchivedString;
        use rkyv::vec::ArchivedVec;
        use std::collections::HashMap;
        use std::sync::OnceLock;

        type RegionData = Vec<(String, String, f32)>;
        type Data = HashMap<String, RegionData>;

        type ArchivedRegionData = ArchivedVec<(ArchivedString, ArchivedString, f32)>;
        type ArchivedData = ArchivedHashMap<ArchivedString, ArchivedRegionData>;

        const RKYV_BYTES: &'static [u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }
            const ALIGNED: &'static Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("caniuse_region_matching.rkyv") };
            &ALIGNED.bytes
        };

        pub fn get_usage_by_region(region: &str) -> Option<&'static ArchivedRegionData> {
            static CANIUSE_USAGE_BY_REGION: OnceLock<&ArchivedData> = OnceLock::new();
            let region_to_usage = CANIUSE_USAGE_BY_REGION.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::archived_root::<Data>(RKYV_BYTES) }
            });

            region_to_usage.get(region)
        }

    };

    generate_rkyv::<_, 256>("caniuse_region_matching.rkyv", region_to_usage);
    generate_file("caniuse_region_matching.rs", output);

    Ok(())
}
