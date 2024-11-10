use crate::semver::ArchivedVersion;
use rkyv::vec::ArchivedVec;
use std::sync::OnceLock;
type ArchivedData = ArchivedVec<ArchivedVersion>;
const RKYV_BYTES: &[u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("node_versions.rkyv") };
    &ALIGNED.bytes
};
pub fn get_node_versions() -> &'static ArchivedData {
    static NODE_VERSIONS: OnceLock<&ArchivedData> = OnceLock::new();
    NODE_VERSIONS.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::access_unchecked::<ArchivedData>(RKYV_BYTES)
        }
    })
}
