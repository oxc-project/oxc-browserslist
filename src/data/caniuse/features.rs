use super::BrowserName;
use rustc_hash::{FxHashMap, FxHashSet};

pub type FeatureSet =
    (/* yes */ FxHashSet<&'static str>, /* partial */ FxHashSet<&'static str>);
pub type Feature = FxHashMap<BrowserName, FeatureSet>;

pub fn get_feature_stat(name: &str) -> Option<&'static Feature> {
    crate::generated::caniuse_feature_matching::get_feature_stat(name)
}
