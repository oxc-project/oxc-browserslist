use std::sync::OnceLock;

use super::{
    BrowserName,
    compression::{decode, decompress_deflate},
};

use crate::data::decode_browser_name;
pub use crate::generated::caniuse_region_matching::get_usage_by_region;

static PAIRS_COMPRESSED: &[u8] = include_bytes!("../../generated/caniuse_region_pairs.bin.deflate");
static PAIR_INDICES_COMPRESSED: &[u8] =
    include_bytes!("../../generated/caniuse_region_pair_indices.bin.deflate");
static VERSION_TABLE_COMPRESSED: &[u8] =
    include_bytes!("../../generated/caniuse_region_version_table.bin.deflate");
static PERCENTAGES_COMPRESSED: &[u8] =
    include_bytes!("../../generated/caniuse_region_percentages.bin.deflate");

static PAIR_INDICES: OnceLock<Vec<u8>> = OnceLock::new();
static VERSION_TABLE_DATA: OnceLock<Vec<u8>> = OnceLock::new();
static VERSION_TABLE: OnceLock<Vec<&'static str>> = OnceLock::new();
static PERCENTAGES: OnceLock<Vec<u8>> = OnceLock::new();
/// Global (browser, version) intern table shared by every region: a browser id and the
/// resolved version string at each pair index.
static PAIR_TABLE: OnceLock<(Vec<u8>, Vec<&'static str>)> = OnceLock::new();

pub struct RegionData {
    pair_indices_start: u32,
    pair_indices_end: u32,
    percentages_start: u32,
    percentages_end: u32,
}

fn version_table() -> &'static [&'static str] {
    VERSION_TABLE.get_or_init(|| {
        let data = VERSION_TABLE_DATA.get_or_init(|| decompress_deflate(VERSION_TABLE_COMPRESSED));
        postcard::from_bytes(data).unwrap()
    })
}

fn pair_table() -> &'static (Vec<u8>, Vec<&'static str>) {
    PAIR_TABLE.get_or_init(|| {
        let decompressed = decompress_deflate(PAIRS_COMPRESSED);
        let (browsers, version_indices): (Vec<u8>, Vec<u16>) =
            postcard::from_bytes(&decompressed).unwrap();
        let table = version_table();
        let versions = version_indices.into_iter().map(|i| table[i as usize]).collect();
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

        let pair_data = PAIR_INDICES.get_or_init(|| decompress_deflate(PAIR_INDICES_COMPRESSED));
        let pair_indices = decode::<u16>(pair_data, self.pair_indices_start, self.pair_indices_end);

        let percentages_data =
            PERCENTAGES.get_or_init(|| decompress_deflate(PERCENTAGES_COMPRESSED));
        // Stored delta-encoded (non-increasing sequence as `prev - curr`); undo it in place.
        let mut percentages =
            decode::<u32>(percentages_data, self.percentages_start, self.percentages_end);
        for i in 1..percentages.len() {
            percentages[i] = percentages[i - 1] - percentages[i];
        }

        pair_indices.into_iter().zip(percentages).map(move |(pair_index, p)| {
            let pair_index = pair_index as usize;
            (
                decode_browser_name(pair_browsers[pair_index]),
                pair_versions[pair_index],
                p as f32 / 100_000.0,
            )
        })
    }
}
