use rkyv::collections::ArchivedHashMap;
use rkyv::string::ArchivedString;
use rkyv::vec::ArchivedVec;
use std::collections::HashMap;
use std::sync::OnceLock;
type RegionData = Vec<(String, String, f32)>;
type Data = HashMap<String, RegionData>;
type ArchivedRegionData = ArchivedVec<(ArchivedString, ArchivedString, f32)>;
type ArchivedData = ArchivedHashMap<ArchivedString, ArchivedRegionData>;
pub fn get_usage_by_region(region: &str) -> Option<&'static ArchivedRegionData> {
    static CANIUSE_USAGE_BY_REGION: OnceLock<&ArchivedData> = OnceLock::new();
    let region_to_usage = CANIUSE_USAGE_BY_REGION.get_or_init(|| {
        let bytes = include_bytes!("caniuse_region_matching.rkyv");
        unsafe { rkyv::archived_root::<Data>(bytes) }
    });
    region_to_usage.get(region)
}
