use crate::data::caniuse::features::{ArchivedFeature, ArchivedFeatures};
use std::sync::OnceLock;
const RKYV_BYTES: &'static [u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &'static Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("caniuse_feature_matching.rkyv") };
    &ALIGNED.bytes
};
pub(crate) fn get_feature_stat(name: &str) -> Option<&'static ArchivedFeature> {
    static CANIUSE_FEATURE_MATCHING: OnceLock<&ArchivedFeatures> = OnceLock::new();
    let stats = CANIUSE_FEATURE_MATCHING.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::access_unchecked::<ArchivedFeatures>(RKYV_BYTES)
        }
    });
    return stats.get(name);
}
