use crate::semver::{ArchivedVersion, Version};
use rkyv::vec::ArchivedVec;
use std::sync::OnceLock;
type Data = Vec<(Version, i32, i32)>;
type ArchivedData = ArchivedVec<(ArchivedVersion, i32, i32)>;
const RKYV_BYTES: &'static [u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &'static Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("node_release_schedule.rkyv") };
    &ALIGNED.bytes
};
pub fn get_release_schedule() -> &'static ArchivedData {
    static RELEASE_SCHEDULE: OnceLock<&ArchivedData> = OnceLock::new();
    RELEASE_SCHEDULE.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::archived_root::<Data>(RKYV_BYTES)
        }
    })
}
