use rkyv::collections::ArchivedHashMap;
use rkyv::string::ArchivedString;
use rkyv::vec::ArchivedVec;
use std::collections::HashMap;
use std::sync::OnceLock;
type RegionData = Vec<(String, String, f32)>;
type Data = HashMap<String, RegionData>;
type ArchivedRegionData = ArchivedVec<(ArchivedString, ArchivedString, f32)>;
type ArchivedData = ArchivedHashMap<ArchivedString, ArchivedRegionData>;
const RKYV_BYTES: &'static [u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &'static Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("caniuse_region_matching.rkyv") };
    &ALIGNED.bytes
};
pub fn get_usage_by_region(region: &str) -> Option<&'static ArchivedRegionData> {
    static CANIUSE_USAGE_BY_REGION: OnceLock<&ArchivedData> = OnceLock::new();
    let region_to_usage = CANIUSE_USAGE_BY_REGION.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::archived_root::<Data>(RKYV_BYTES)
        }
    });
    region_to_usage.get(region)
}
