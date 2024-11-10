use crate::data::caniuse::ArchivedBrowserStat;
use rkyv::collections::swiss_table::ArchivedHashMap;
use rkyv::string::ArchivedString;
use std::sync::OnceLock;
pub type ArchivedCaniuseData = ArchivedHashMap<ArchivedString, ArchivedBrowserStat>;
const RKYV_BYTES: &[u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("caniuse_browsers.rkyv") };
    &ALIGNED.bytes
};
pub fn caniuse_browsers() -> &'static ArchivedCaniuseData {
    static CANIUSE_BROWSERS: OnceLock<&ArchivedCaniuseData> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::access_unchecked::<ArchivedCaniuseData>(RKYV_BYTES)
        }
    })
}
const RKYV_BYTES_2: &[u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("caniuse_browsers_android_to_desktop.rkyv") };
    &ALIGNED.bytes
};
pub fn caniuse_browsers_android_to_desktop() -> &'static ArchivedBrowserStat {
    static CANIUSE_BROWSERS: OnceLock<&ArchivedBrowserStat> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::access_unchecked::<ArchivedBrowserStat>(RKYV_BYTES_2)
        }
    })
}
