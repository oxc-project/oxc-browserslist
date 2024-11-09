use rkyv::string::ArchivedString;
use rkyv::vec::ArchivedVec;
use std::sync::OnceLock;
type ArchivedData = ArchivedVec<(ArchivedString, ArchivedString, f32)>;
const RKYV_BYTES: &'static [u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &'static Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("global_usage.rkyv") };
    &ALIGNED.bytes
};
pub fn caniuse_global_usage() -> &'static ArchivedData {
    static CANIUSE_GLOBAL_USAGE: OnceLock<&ArchivedData> = OnceLock::new();
    CANIUSE_GLOBAL_USAGE.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::access_unchecked::<ArchivedData>(RKYV_BYTES)
        }
    })
}
