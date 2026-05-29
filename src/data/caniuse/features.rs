use std::sync::OnceLock;

use super::{
    BrowserName,
    compression::{decode, decompress_deflate},
};

use crate::data::decode_browser_name;
pub use crate::generated::caniuse_feature_matching::get_feature_stat;

static FEATURES: OnceLock<Vec<u8>> = OnceLock::new();
static VERSION_TABLE: OnceLock<Vec<String>> = OnceLock::new();

fn version_table() -> &'static [String] {
    VERSION_TABLE.get_or_init(|| {
        postcard::from_bytes(&decompress_deflate(include_bytes!(
            "../../generated/caniuse_feature_version_table.bin.deflate"
        )))
        .unwrap()
    })
}

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
        let features_data = FEATURES.get_or_init(|| {
            decompress_deflate(include_bytes!(
                "../../generated/caniuse_feature_matching.bin.deflate"
            ))
        });
        let features = decode::<(u8, Vec<u16>, Vec<u16>)>(features_data, self.start, self.end);
        let table = version_table();
        let resolve = |indices: Vec<u16>| -> Vec<&'static str> {
            indices.into_iter().map(|i| table[i as usize].as_str()).collect()
        };
        features
            .into_iter()
            .map(|(b, yes, partial)| {
                (decode_browser_name(b), FeatureSet::new(resolve(yes), resolve(partial)))
            })
            .collect::<Vec<_>>()
    }
}
