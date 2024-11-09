use crate::data::electron::{ArchivedElectronVersion, ElectronVersion};
use rkyv::string::ArchivedString;
use rkyv::vec::ArchivedVec;
use std::sync::OnceLock;
type ArchivedData = ArchivedVec<(ArchivedElectronVersion, ArchivedString)>;
const RKYV_BYTES: &'static [u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &'static Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("electron_to_chromium.rkyv") };
    &ALIGNED.bytes
};
pub fn get_electron_versions() -> &'static ArchivedData {
    static ELECTRON_VERSIONS: OnceLock<&ArchivedData> = OnceLock::new();
    ELECTRON_VERSIONS.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::access_unchecked::<ArchivedData>(RKYV_BYTES)
        }
    })
}
