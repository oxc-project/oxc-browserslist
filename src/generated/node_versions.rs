use crate::semver::{ArchivedVersion, Version};
use rkyv::vec::ArchivedVec;
use std::sync::OnceLock;
type Data = Vec<Version>;
type ArchivedData = ArchivedVec<ArchivedVersion>;
const RKYV_BYTES: &'static [u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &'static Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("node_versions.rkyv") };
    &ALIGNED.bytes
};
pub fn get_node_versions() -> &'static ArchivedData {
    static NODE_VERSIONS: OnceLock<&ArchivedData> = OnceLock::new();
    NODE_VERSIONS.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::archived_root::<Data>(RKYV_BYTES)
        }
    })
}
