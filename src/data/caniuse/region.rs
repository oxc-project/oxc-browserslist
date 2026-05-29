use std::sync::OnceLock;

use super::{
    BrowserName,
    compression::{LazyBlob, load},
};

use crate::data::decode_browser_name;
pub use crate::generated::caniuse_region_matching::get_usage_by_region;

static PAIR_INDICES: LazyBlob =
    LazyBlob::new(include_bytes!("../../generated/caniuse_region_pair_indices.bin.deflate"));
static PERCENTAGES: LazyBlob =
    LazyBlob::new(include_bytes!("../../generated/caniuse_region_percentages.bin.deflate"));
/// Global (browser, version) intern table shared by every region: the browser id and resolved
/// version string at each pair index.
static PAIR_TABLE: OnceLock<(Vec<u8>, Vec<String>)> = OnceLock::new();

pub struct RegionData {
    /// Element offsets shared by the pair-index and percentage byte planes (both have one slot
    /// per datum in the same per-region order).
    start: u32,
    end: u32,
}

fn pair_table() -> &'static (Vec<u8>, Vec<String>) {
    PAIR_TABLE.get_or_init(|| {
        let (browsers, version_indices): (Vec<u8>, Vec<u16>) =
            load(include_bytes!("../../generated/caniuse_region_pairs.bin.deflate"));
        let table: Vec<String> =
            load(include_bytes!("../../generated/caniuse_region_version_table.bin.deflate"));
        let versions = version_indices.into_iter().map(|i| table[i as usize].clone()).collect();
        (browsers, versions)
    })
}

impl RegionData {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn iter(&self) -> impl Iterator<Item = (BrowserName, &'static str, f32)> {
        let (pair_browsers, pair_versions) = pair_table();
        let (start, end) = (self.start as usize, self.end as usize);

        // The blob is two byte planes — every low byte, then every high byte — so its length is
        // twice the datum count; split there and recombine `lo | hi << 8`. The range bounds
        // address the planes directly (one slot per datum).
        let pair_data = PAIR_INDICES.get();
        let pair_count = pair_data.len() / 2;
        let (pair_lo, pair_hi) = pair_data.split_at(pair_count);
        let pair_indices =
            (start..end).map(move |i| (pair_lo[i] as usize) | ((pair_hi[i] as usize) << 8));

        // The percentages are stored the same way as the pair indices — fixed-width byte planes
        // addressed by the same element offsets — but split three ways (low, middle, high) to hold
        // the per-datum deltas. Recombine each delta, then undo the delta in place (the values are
        // a non-increasing sequence stored as `prev - curr`).
        let percentages_data = PERCENTAGES.get();
        let pct_count = percentages_data.len() / 3;
        let (pct_lo, rest) = percentages_data.split_at(pct_count);
        let (pct_mid, pct_hi) = rest.split_at(pct_count);
        let mut percentages = (start..end)
            .map(|i| {
                u32::from(pct_lo[i]) | (u32::from(pct_mid[i]) << 8) | (u32::from(pct_hi[i]) << 16)
            })
            .collect::<Vec<_>>();
        for i in 1..percentages.len() {
            percentages[i] = percentages[i - 1] - percentages[i];
        }

        pair_indices.zip(percentages).map(move |(pair_index, p)| {
            (
                decode_browser_name(pair_browsers[pair_index]),
                pair_versions[pair_index].as_str(),
                p as f32 / 100_000.0,
            )
        })
    }
}
