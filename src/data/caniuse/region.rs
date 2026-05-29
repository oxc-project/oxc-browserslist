use std::sync::OnceLock;

use super::{
    BrowserName,
    compression::{decode, decompress_deflate},
};

use crate::data::decode_browser_name;
pub use crate::generated::caniuse_region_matching::get_usage_by_region;

static PAIR_INDICES: OnceLock<Vec<u8>> = OnceLock::new();
static PERCENTAGES: OnceLock<Vec<u8>> = OnceLock::new();
/// Global (browser, version) intern table shared by every region: the browser id and resolved
/// version string at each pair index.
static PAIR_TABLE: OnceLock<(Vec<u8>, Vec<String>)> = OnceLock::new();

pub struct RegionData {
    pair_indices_start: u32,
    pair_indices_end: u32,
    percentages_start: u32,
    percentages_end: u32,
}

fn pair_table() -> &'static (Vec<u8>, Vec<String>) {
    PAIR_TABLE.get_or_init(|| {
        let pairs =
            decompress_deflate(include_bytes!("../../generated/caniuse_region_pairs.bin.deflate"));
        let (browsers, version_indices): (Vec<u8>, Vec<u16>) =
            postcard::from_bytes(&pairs).unwrap();
        let table: Vec<String> = postcard::from_bytes(&decompress_deflate(include_bytes!(
            "../../generated/caniuse_region_version_table.bin.deflate"
        )))
        .unwrap();
        let versions = version_indices.into_iter().map(|i| table[i as usize].clone()).collect();
        (browsers, versions)
    })
}

impl RegionData {
    pub fn new(
        pair_indices_start: u32,
        pair_indices_end: u32,
        percentages_start: u32,
        percentages_end: u32,
    ) -> Self {
        Self { pair_indices_start, pair_indices_end, percentages_start, percentages_end }
    }

    pub fn iter(&self) -> impl Iterator<Item = (BrowserName, &'static str, f32)> {
        let (pair_browsers, pair_versions) = pair_table();

        // The blob is two byte planes — every low byte, then every high byte — so its length is
        // twice the datum count; split there and recombine `lo | hi << 8`. The range bounds
        // address the planes directly (one slot per datum).
        let pair_data = PAIR_INDICES.get_or_init(|| {
            decompress_deflate(include_bytes!(
                "../../generated/caniuse_region_pair_indices.bin.deflate"
            ))
        });
        let count = pair_data.len() / 2;
        let (pair_lo, pair_hi) = pair_data.split_at(count);
        let pair_indices = (self.pair_indices_start as usize..self.pair_indices_end as usize)
            .map(move |i| (pair_lo[i] as usize) | ((pair_hi[i] as usize) << 8));

        let percentages_data = PERCENTAGES.get_or_init(|| {
            decompress_deflate(include_bytes!(
                "../../generated/caniuse_region_percentages.bin.deflate"
            ))
        });
        // Stored delta-encoded (non-increasing sequence as `prev - curr`); undo it in place.
        let mut percentages =
            decode::<u32>(percentages_data, self.percentages_start, self.percentages_end);
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
