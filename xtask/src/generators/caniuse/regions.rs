use std::collections::{BTreeSet, HashMap};

use anyhow::Result;
use postcard::to_allocvec;
use quote::quote;

use crate::data::{Caniuse, encode_browser_name};
use crate::utils::{create_range_vec, generate_file, save_bin_compressed};

struct RegionDatum {
    browser: u8,
    version: String,
    usage: f32,
}

pub fn build_caniuse_region_matching(data: &Caniuse) -> Result<()> {
    let agents = &data.agents;

    let mut data = data
        .regions
        .iter()
        .map(|(key, region)| {
            let mut usage = region
                .data
                .iter()
                .flat_map(|(name, stat)| {
                    let agent = agents.get(name).unwrap();
                    stat.iter().filter_map(move |(version, usage)| {
                        let version = if version == "0" {
                            agent.version_list.last().unwrap().version.clone()
                        } else {
                            version.clone()
                        };
                        usage.map(|usage| RegionDatum {
                            browser: encode_browser_name(name),
                            version,
                            usage,
                        })
                    })
                })
                .collect::<Vec<_>>();
            usage.sort_unstable_by(|a, b| b.usage.partial_cmp(&a.usage).unwrap());
            (key.clone(), usage)
        })
        .collect::<Vec<_>>();

    data.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // Store region-code lookup keys as a compressed blob instead of an inline `&[&str]`
    // (16-byte fat pointer + raw bytes per entry, uncompressed). Decoded once on first use.
    let keys = data.iter().map(|(key, _)| key.clone()).collect::<Vec<String>>();
    save_bin_compressed("caniuse_region_keys.bin", &to_allocvec(&keys).unwrap());

    // Build version string intern table (deduplicated, sorted)
    let mut all_versions = BTreeSet::new();
    for (_, datums) in &data {
        for datum in datums {
            all_versions.insert(datum.version.clone());
        }
    }
    let version_table: Vec<String> = all_versions.into_iter().collect();
    let version_to_index: HashMap<&str, u16> =
        version_table.iter().enumerate().map(|(i, v)| (v.as_str(), i as u16)).collect();

    // Serialize and compress the string table
    let table_bytes = to_allocvec(&version_table).unwrap();
    save_bin_compressed("caniuse_region_version_table.bin", &table_bytes);

    // Only a few hundred distinct (browser, version) pairs exist, yet they recur ~47k times
    // across all regions. Intern them into one global table and store a single u16 pair-index
    // per datum, instead of a per-region browser byte *and* version index. The table is ordered
    // by descending global usage so the high-usage entries that lead each region's
    // (usage-sorted) list map to small, repetitive indices that deflate packs tightly.
    let mut pair_usage: HashMap<(u8, u16), f64> = HashMap::new();
    for (_, datums) in &data {
        for datum in datums {
            let vi = version_to_index[datum.version.as_str()];
            *pair_usage.entry((datum.browser, vi)).or_default() += f64::from(datum.usage);
        }
    }
    let mut pairs: Vec<(u8, u16)> = pair_usage.keys().copied().collect();
    pairs.sort_unstable_by(|a, b| {
        pair_usage[b].partial_cmp(&pair_usage[a]).unwrap().then_with(|| a.cmp(b))
    });
    let pair_to_index: HashMap<(u8, u16), u16> =
        pairs.iter().enumerate().map(|(i, p)| (*p, i as u16)).collect();

    // Pair table: parallel browser-id and version-index arrays, serialized as one blob.
    let pair_browsers: Vec<u8> = pairs.iter().map(|(b, _)| *b).collect();
    let pair_versions: Vec<u16> = pairs.iter().map(|(_, v)| *v).collect();
    let pair_table_bytes = to_allocvec(&(pair_browsers, pair_versions)).unwrap();
    save_bin_compressed("caniuse_region_pairs.bin", &pair_table_bytes);

    // For each region, store u16 pair indices (replaces the per-region browser + version arrays).
    let pair_indices = data
        .iter()
        .map(|(_, datums)| {
            let indices: Vec<u16> = datums
                .iter()
                .map(|x| {
                    let vi = version_to_index[x.version.as_str()];
                    pair_to_index[&(x.browser, vi)]
                })
                .collect();
            to_allocvec(&indices).unwrap()
        })
        .collect::<Vec<_>>();
    let pair_ranges = create_range_vec(&pair_indices);
    let pair_bytes = pair_indices.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin_compressed("caniuse_region_pair_indices.bin", &pair_bytes);

    let percentages = data
        .iter()
        .map(|(_region, datums)| {
            // `datums` is sorted by usage descending, so the rounded percentages form a
            // non-increasing sequence. Store them delta-encoded (`prev - curr`, always >= 0):
            // the deltas are small and contain long runs of zeros (many equal usages),
            // which deflate compresses far better than the raw values.
            let mut prev = 0u32;
            let deltas: Vec<u32> = datums
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let curr = (x.usage * 100_000.0).round() as u32;
                    let delta = if i == 0 { curr } else { prev - curr };
                    prev = curr;
                    delta
                })
                .collect();
            to_allocvec(&deltas).unwrap()
        })
        .collect::<Vec<_>>();
    let percent_ranges = create_range_vec(&percentages);
    let percent_bytes = percentages.iter().flat_map(|x| x.iter()).copied().collect::<Vec<_>>();
    save_bin_compressed("caniuse_region_percentages.bin", &percent_bytes);

    let output = quote! {
        use std::sync::OnceLock;

        use crate::data::caniuse::{compression::decompress_deflate, region::RegionData};

        static KEYS: OnceLock<Vec<String>> = OnceLock::new();
        const PAIR_RANGES: &[u32] = &[#(#pair_ranges,)*];
        const PERCENT_RANGES: &[u32] = &[#(#percent_ranges,)*];

        pub fn get_usage_by_region(region: &str) -> Option<RegionData> {
            let keys = KEYS.get_or_init(|| {
                postcard::from_bytes(&decompress_deflate(include_bytes!("caniuse_region_keys.bin.deflate")))
                    .unwrap()
            });
            let index = keys.binary_search_by(|key| key.as_str().cmp(region)).ok()?;
            Some(RegionData::new(
                PAIR_RANGES[index],
                PAIR_RANGES[index + 1],
                PERCENT_RANGES[index],
                PERCENT_RANGES[index + 1],
            ))
        }
    };
    generate_file("caniuse_region_matching.rs", output);

    Ok(())
}
