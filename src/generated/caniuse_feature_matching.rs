use crate::data::caniuse::features::{ArchivedFeature, ArchivedFeatures, Features};
use std::sync::OnceLock;
pub(crate) fn get_feature_stat(name: &str) -> Option<&'static ArchivedFeature> {
    static CANIUSE_FEATURE_MATCHING: OnceLock<&ArchivedFeatures> = OnceLock::new();
    let stats = CANIUSE_FEATURE_MATCHING.get_or_init(|| {
        let bytes = include_bytes!("caniuse_feature_matching.rkyv");
        #[allow(unsafe_code)]
        unsafe {
            rkyv::archived_root::<Features>(bytes)
        }
    });
    return stats.get(name);
}
