use super::BrowserName;

pub use crate::generated::caniuse_feature_matching::get_feature_stat;

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
    pub data: Vec<(BrowserName, FeatureSet)>,
}

impl Feature {
    pub fn new(data: Vec<(BrowserName, FeatureSet)>) -> Self {
        Self { data }
    }
}
