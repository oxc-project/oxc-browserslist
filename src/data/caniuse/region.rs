use std::sync::OnceLock;

use super::{
    BrowserName,
    compression::{decode, decompress_deflate},
};

use crate::data::decode_browser_name;
pub use crate::generated::caniuse_region_matching::get_usage_by_region;

static BROWSER_NAMES_COMPRESSED: &[u8] =
    include_bytes!("../../generated/caniuse_region_browsers.bin.deflate");
static VERSIONS_COMPRESSED: &[u8] =
    include_bytes!("../../generated/caniuse_region_versions.bin.deflate");
static VERSION_TABLE_COMPRESSED: &[u8] =
    include_bytes!("../../generated/caniuse_region_version_table.bin.deflate");
static PERCENTAGES_COMPRESSED: &[u8] =
    include_bytes!("../../generated/caniuse_region_percentages.bin.deflate");

static BROWSER_NAMES: OnceLock<Vec<u8>> = OnceLock::new();
static VERSIONS: OnceLock<Vec<u8>> = OnceLock::new();
static VERSION_TABLE_DATA: OnceLock<Vec<u8>> = OnceLock::new();
static VERSION_TABLE: OnceLock<Vec<&'static str>> = OnceLock::new();
static PERCENTAGES: OnceLock<Vec<u8>> = OnceLock::new();

pub struct RegionData {
    browser_names_start: u32,
    browser_names_end: u32,
    versions_start: u32,
    versions_end: u32,
    percentages_start: u32,
    percentages_end: u32,
}

fn version_table() -> &'static [&'static str] {
    VERSION_TABLE.get_or_init(|| {
        let data = VERSION_TABLE_DATA.get_or_init(|| decompress_deflate(VERSION_TABLE_COMPRESSED));
        postcard::from_bytes(data).unwrap()
    })
}

impl RegionData {
    pub fn new(
        browser_names_start: u32,
        browser_names_end: u32,
        versions_start: u32,
        versions_end: u32,
        percentages_start: u32,
        percentages_end: u32,
    ) -> Self {
        Self {
            browser_names_start,
            browser_names_end,
            versions_start,
            versions_end,
            percentages_start,
            percentages_end,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (BrowserName, &'static str, f32)> {
        let browser_names =
            BROWSER_NAMES.get_or_init(|| decompress_deflate(BROWSER_NAMES_COMPRESSED));
        let browser_names =
            &browser_names[self.browser_names_start as usize..self.browser_names_end as usize];

        let versions_data = VERSIONS.get_or_init(|| decompress_deflate(VERSIONS_COMPRESSED));
        let version_indices = decode::<u16>(versions_data, self.versions_start, self.versions_end);
        let table = version_table();

        let percentages_data =
            PERCENTAGES.get_or_init(|| decompress_deflate(PERCENTAGES_COMPRESSED));
        let percentages =
            decode::<u32>(percentages_data, self.percentages_start, self.percentages_end);

        browser_names.iter().zip(version_indices).zip(percentages).map(move |((b, idx), p)| {
            (decode_browser_name(*b), table[idx as usize], p as f32 / 100_000.0)
        })
    }
}
