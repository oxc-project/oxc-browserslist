use std::collections::HashMap;

use anyhow::Result;
use postcard::to_allocvec;

use crate::data::{Caniuse, encode_browser_name};
use crate::utils::{create_range_vec, generate_keyed_lookup, save_bin_compressed};

/// Every version string the feature blob will reference, for the canonical version table.
/// Must select exactly what `build_caniuse_feature_matching` below puts in its `y`/`a` lists.
pub fn feature_versions(data: &Caniuse) -> impl Iterator<Item = String> + '_ {
    data.data.values().flat_map(|feature| {
        feature.stats.values().flat_map(|versions| {
            versions.iter().filter_map(|(version, flag)| {
                (flag != "n" && (flag.contains('y') || flag.contains('a'))).then(|| version.clone())
            })
        })
    })
}

pub fn build_caniuse_feature_matching(
    data: &Caniuse,
    canonical: &HashMap<String, u16>,
) -> Result<()> {
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

    // The feature-name lookup keys and the per-feature data are written together at the end by
    // `generate_keyed_lookup`; collect the (already sorted) keys here.
    let keys = sorted_data.keys().cloned().collect::<Vec<String>>();

    // A feature's per-browser `y`/`a` list is the set of versions that support it, and browser
    // support is almost always "from version N onward" — so in per-browser release order the list
    // is one contiguous run. Store every list as ascending `(start, length)` runs of local indices
    // into the browser's version_list order, instead of one index per version. This collapses
    // ~245k indices into ~16k run endpoints (~9 KB smaller after deflate). The order used to be a
    // bundled numeric-sort permutation table; it is now simply each browser's version_list mapped
    // to canonical version-table indices — which the runtime derives from the browsers blob it
    // already decodes, so no order table is bundled at all. (Empirically the two orders are
    // identical for every browser except safari, where "TP" moves from front to back — which
    // makes safari's runs MORE contiguous.) Correctness only needs the order to be deterministic
    // and to match between this generator and the runtime's derivation: the runtime re-sorts the
    // resolved version strings before binary-searching them.
    let browser_versions: Vec<Vec<u16>> = {
        let max_id = data.agents.keys().map(|name| encode_browser_name(name)).max().unwrap_or(0);
        let mut orders: Vec<Vec<u16>> = vec![Vec::new(); usize::from(max_id) + 1];
        for (name, agent) in &data.agents {
            orders[encode_browser_name(name) as usize] =
                agent.version_list.iter().map(|v| canonical[v.version.as_str()]).collect();
        }
        orders
    };

    let local_index: Vec<HashMap<u16, u16>> = browser_versions
        .iter()
        .map(|order| order.iter().enumerate().map(|(i, &g)| (g, i as u16)).collect())
        .collect();
    let to_runs = |versions: &[String], b: u8| -> Vec<(u16, u16)> {
        // A feature version absent from the browser's version_list (or from the canonical
        // table) is a data invariant violation; fail codegen with an actionable message.
        let mut locals: Vec<u16> = versions
            .iter()
            .map(|v| {
                let index = canonical
                    .get(v.as_str())
                    .unwrap_or_else(|| panic!("feature version {v} missing from canonical table"));
                *local_index[b as usize].get(index).unwrap_or_else(|| {
                    panic!("feature version {v} not in browser id {b}'s version_list")
                })
            })
            .collect();
        locals.sort_unstable();
        let mut runs: Vec<(u16, u16)> = Vec::new();
        for &local in &locals {
            match runs.last_mut() {
                Some((start, len)) if *start + *len == local => *len += 1,
                _ => runs.push((local, 1)),
            }
        }
        // A run reaching the end of the browser's version order is stored with length 0 (a real
        // run is never empty): support is usually "version N onward", so ~46% of all lists become
        // a `(start, 0)` pair that repeats byte-for-byte across features, which deflate rewards
        // (~1.2 KB off the blob). The runtime (`read_versions`) expands 0 back to `order.len() - start`. Only the
        // last run can reach the end (runs are ascending and disjoint).
        if let Some(run) = runs.last_mut() {
            if run.0 + run.1 == browser_versions[b as usize].len() as u16 {
                run.1 = 0;
            }
        }
        runs
    };
    // Per feature: one `(browser, yes-runs, partial-runs)` entry per browser. This postcard layout
    // is hand-decoded (not `postcard::from_bytes`) by `caniuse::features::read_versions` (using
    // the shared `compression::read_varint`) at runtime to keep the decoder small, so it must
    // stay a postcard-LEB128 stream: browser `u8`, then each run list as a varint length followed
    // by `(start, length)` varint pairs.
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
    generate_keyed_lookup(
        "caniuse_feature_matching.rs",
        "caniuse_feature_keys.bin",
        &keys,
        &data_range,
        "features",
        "Feature",
        "get_feature_stat",
    );

    Ok(())
}
