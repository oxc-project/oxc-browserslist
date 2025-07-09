use crate::data::{BrowserName, browser_name::decode_browser_name};
pub use crate::generated::caniuse_region_matching::get_usage_by_region;

const BROWSER_NAMES: &[u8; 51537] = include_bytes!("../../generated/caniuse_region_browsers.bin");
const VERSIONS: &[u8; 233911] = include_bytes!("../../generated/caniuse_region_versions.bin");
const PERCENTAGES: &[u8; 206510] = include_bytes!("../../generated/caniuse_region_percentages.bin");

pub struct RegionData {
    browser_names_start: u32,
    browser_names_end: u32,
    versions_start: u32,
    versions_end: u32,
    percentages_start: u32,
    percentages_end: u32,
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
            &BROWSER_NAMES[self.browser_names_start as usize..self.browser_names_end as usize];
        let (versions, _): (Vec<&'static str>, _) = bincode::borrow_decode_from_slice(
            &VERSIONS[self.versions_start as usize..self.versions_end as usize],
            bincode::config::standard(),
        )
        .unwrap();
        let (percentages, _): (Vec<f32>, _) = bincode::borrow_decode_from_slice(
            &PERCENTAGES[self.percentages_start as usize..self.percentages_end as usize],
            bincode::config::standard(),
        )
        .unwrap();
        browser_names
            .iter()
            .zip(versions)
            .zip(percentages)
            .map(|((&b, v), p)| (decode_browser_name(b), v, p))
    }
}
