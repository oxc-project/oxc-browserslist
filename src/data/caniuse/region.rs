#[cfg(feature = "regions")]
use std::sync::OnceLock;

use crate::data::BrowserName;
#[cfg(feature = "regions")] 
use crate::data::{compression::decompress_gzip, decode_browser_name};
pub use crate::generated::caniuse_region_matching::get_usage_by_region;

#[cfg(feature = "regions")]
static BROWSER_NAMES_COMPRESSED: &[u8] = include_bytes!("../../generated/caniuse_region_browsers.bin.gz");
#[cfg(feature = "regions")]
static VERSIONS_COMPRESSED: &[u8] = include_bytes!("../../generated/caniuse_region_versions.bin.gz");
#[cfg(feature = "regions")]
static PERCENTAGES_COMPRESSED: &[u8] = include_bytes!("../../generated/caniuse_region_percentages.bin.gz");

#[cfg(feature = "regions")]
static BROWSER_NAMES: OnceLock<Vec<u8>> = OnceLock::new();
#[cfg(feature = "regions")]
static VERSIONS: OnceLock<Vec<u8>> = OnceLock::new();
#[cfg(feature = "regions")]
static PERCENTAGES: OnceLock<Vec<u8>> = OnceLock::new();

pub struct RegionData {
    #[cfg(feature = "regions")]
    browser_names_start: u32,
    #[cfg(feature = "regions")]
    browser_names_end: u32,
    #[cfg(feature = "regions")]
    versions_start: u32,
    #[cfg(feature = "regions")]
    versions_end: u32,
    #[cfg(feature = "regions")]
    percentages_start: u32,
    #[cfg(feature = "regions")]
    percentages_end: u32,
}

impl RegionData {
    #[cfg(feature = "regions")]
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

    #[cfg(not(feature = "regions"))]
    pub fn new(
        _browser_names_start: u32,
        _browser_names_end: u32,
        _versions_start: u32,
        _versions_end: u32,
        _percentages_start: u32,
        _percentages_end: u32,
    ) -> Self {
        Self {}
    }

    #[cfg(feature = "regions")]
    pub fn iter(&self) -> impl Iterator<Item = (BrowserName, &'static str, f32)> {
        let browser_names_data = BROWSER_NAMES.get_or_init(|| decompress_gzip(BROWSER_NAMES_COMPRESSED));
        let browser_names = &browser_names_data[self.browser_names_start as usize..self.browser_names_end as usize];
        
        let versions_data = VERSIONS.get_or_init(|| decompress_gzip(VERSIONS_COMPRESSED));
        let (versions, _): (Vec<&'static str>, _) = bincode::borrow_decode_from_slice(
            &versions_data[self.versions_start as usize..self.versions_end as usize],
            bincode::config::standard(),
        )
        .unwrap();
        
        let percentages_data = PERCENTAGES.get_or_init(|| decompress_gzip(PERCENTAGES_COMPRESSED));
        let (percentages, _): (Vec<f32>, _) = bincode::borrow_decode_from_slice(
            &percentages_data[self.percentages_start as usize..self.percentages_end as usize],
            bincode::config::standard(),
        )
        .unwrap();
        
        browser_names
            .iter()
            .zip(versions)
            .zip(percentages)
            .map(|((&b, v), p)| (decode_browser_name(b), v, p))
    }

    #[cfg(not(feature = "regions"))]
    pub fn iter(&self) -> impl Iterator<Item = (BrowserName, &'static str, f32)> {
        std::iter::empty()
    }
}
