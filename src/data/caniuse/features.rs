use rkyv::collections::swiss_table::{ArchivedHashMap, ArchivedHashSet};
use rkyv::string::ArchivedString;

pub type ArchivedFeatureSet = (
    /* yes */ ArchivedHashSet<ArchivedString>,
    /* partial */ ArchivedHashSet<ArchivedString>,
);
pub type ArchivedFeature = ArchivedHashMap</* browser */ ArchivedString, ArchivedFeatureSet>;
pub type ArchivedFeatures =
    ArchivedHashMap</* feature name */ ArchivedString, ArchivedFeature>;

pub fn get_feature_stat(name: &str) -> Option<&'static ArchivedFeature> {
    crate::generated::caniuse_feature_matching::get_feature_stat(name)
}
