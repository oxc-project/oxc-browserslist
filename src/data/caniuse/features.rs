use super::BrowserNameAtom;
use ahash::AHashMap;
use indexmap::IndexMap;

type Feature = AHashMap<BrowserNameAtom, IndexMap<&'static str, u8>>;

pub(crate) fn get_feature_stat(name: &str) -> Option<&'static Feature> {
    crate::generated::caniuse_feature_matching::_get_feature_stat(name)
}
