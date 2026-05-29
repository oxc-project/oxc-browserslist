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

    // A feature's per-browser `y`/`a` list is the set of versions that support it, and browser
    // support is almost always "from version N onward" — so in per-browser version order the list
    // is one contiguous run. Build a per-browser version order (each browser's feature-versions
    // sorted by version number) and store every list as ascending `(start, length)` runs of local
    // indices into that order, instead of one index per version. This collapses ~245k indices into
    // ~16k run endpoints (~9 KB smaller after deflate). Ordering by version number is what makes
    // each list contiguous, so it is load-bearing for the blob size (a plain lexicographic order
    // would fragment "9, 90, ..., 99, 9.1" and defeat the encoding). Correctness, though, only
    // needs the order to be deterministic and to match between this table and the run indices: the
    // runtime re-sorts the resolved version strings before binary-searching them.
    let max_browser = features.iter().flat_map(|f| f.iter().map(|(b, _, _)| *b)).max().unwrap_or(0);
    let mut version_sets: Vec<BTreeSet<u16>> = vec![BTreeSet::new(); max_browser as usize + 1];
    for feature in &features {
        for (b, y, a) in feature {
            let set = &mut version_sets[*b as usize];
            set.extend(y.iter().chain(a).map(|v| version_to_index[v.as_str()]));
        }
    }
    let browser_versions: Vec<Vec<u16>> = version_sets
        .into_iter()
        .map(|set| {
            let mut order: Vec<u16> = set.into_iter().collect();
            // `set` already yields indices in ascending (lexicographic) order, and the sort is
            // stable, so ties keep that order deterministically — no explicit tie-break needed.
            order.sort_by_cached_key(|&g| {
                version_table[g as usize]
                    .split(['.', '-'])
                    .map(|p| p.parse::<i64>().unwrap_or(-1))
                    .collect::<Vec<_>>()
            });
            order
        })
        .collect();
    save_bin_compressed(
        "caniuse_feature_browser_versions.bin",
        &to_allocvec(&browser_versions).unwrap(),
    );

    let local_index: Vec<HashMap<u16, u16>> = browser_versions
        .iter()
        .map(|order| order.iter().enumerate().map(|(i, &g)| (g, i as u16)).collect())
        .collect();
    let to_runs = |versions: &[String], b: u8| -> Vec<(u16, u16)> {
        let mut locals: Vec<u16> = versions
            .iter()
            .map(|v| local_index[b as usize][&version_to_index[v.as_str()]])
            .collect();
        locals.sort_unstable();
        let mut runs: Vec<(u16, u16)> = Vec::new();
        for &local in &locals {
            match runs.last_mut() {
                Some((start, len)) if *start + *len == local => *len += 1,
                _ => runs.push((local, 1)),
            }
        }
        runs
    };
    // Per feature: one `(browser, yes-runs, partial-runs)` entry per browser. This postcard layout
    // is hand-decoded (not `postcard::from_bytes`) by `caniuse::features::{read_varint,
    // read_versions}` at runtime to keep the decoder small, so it must stay a postcard-LEB128
    // stream: browser `u8`, then each run list as a varint length followed by `(start, length)`
    // varint pairs.
    let data = features
        .iter()
        .map(|feature| {
            let remapped: Vec<_> =
                feature.iter().map(|(b, y, a)| (*b, to_runs(y, *b), to_runs(a, *b))).collect();
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
