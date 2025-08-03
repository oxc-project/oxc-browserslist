use std::sync::OnceLock;

use super::{
    BrowserName,
    compression::{decode, decompress_deflate},
};

use crate::data::decode_browser_name;
pub use crate::generated::caniuse_feature_matching::get_feature_stat;

static FEATURES_COMPRESSED: &[u8] =
    include_bytes!("../../generated/caniuse_feature_matching.bin.deflate");
static FEATURES_DECOMPRESSED: OnceLock<Vec<u8>> = OnceLock::new();

pub struct FeatureSet {
    yes: Vec</* version */ &'static str>,
    partial: Vec</* version */ &'static str>,
}

impl FeatureSet {
    pub fn new(yes: Vec<&'static str>, partial: Vec<&'static str>) -> Self {
        Self { yes, partial }
    }

    pub fn supports(&self, version: &str, include_partial: bool) -> bool {
        self.yes.binary_search(&version).is_ok()
            || (include_partial && self.partial.binary_search(&version).is_ok())
    }
}

pub struct Feature {
    start: u32,
    end: u32,
}

impl Feature {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn create_data(&self) -> Vec<(BrowserName, FeatureSet)> {
        let data = FEATURES_DECOMPRESSED.get_or_init(|| decompress_deflate(FEATURES_COMPRESSED));
        let features =
            decode::<(u8, Vec<&'static str>, Vec<&'static str>)>(data, self.start, self.end);
        features
            .into_iter()
            .map(|(b, yes, partial)| (decode_browser_name(b), FeatureSet::new(yes, partial)))
            .collect::<Vec<_>>()
    }
}
