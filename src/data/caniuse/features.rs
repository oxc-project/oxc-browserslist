use super::BrowserName;
use indexmap::IndexMap;
use rustc_hash::FxHashMap;

type Feature = FxHashMap<BrowserName, IndexMap<&'static str, u8>>;

pub fn get_feature_stat(name: &str) -> Option<&'static Feature> {
    crate::generated::caniuse_feature_matching::_get_feature_stat(name)
}
