use crate::data::caniuse::{ArchivedBrowserStat, ArchivedCaniuseData, BrowserStat, CaniuseData};
use std::sync::OnceLock;
const RKYV_BYTES: &'static [u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &'static Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("caniuse_browsers.rkyv") };
    &ALIGNED.bytes
};
pub fn caniuse_browsers() -> &'static ArchivedCaniuseData {
    static CANIUSE_BROWSERS: OnceLock<&ArchivedCaniuseData> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::archived_root::<CaniuseData>(RKYV_BYTES)
        }
    })
}
const RKYV_BYTES_2: &'static [u8] = {
    #[repr(C)]
    struct Aligned<T: ?Sized> {
        _align: [usize; 0],
        bytes: T,
    }
    const ALIGNED: &'static Aligned<[u8]> =
        &Aligned { _align: [], bytes: *include_bytes!("caniuse_browsers_android_to_desktop.rkyv") };
    &ALIGNED.bytes
};
pub fn caniuse_browsers_android_to_desktop() -> &'static ArchivedBrowserStat {
    static CANIUSE_BROWSERS: OnceLock<&ArchivedBrowserStat> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        #[allow(unsafe_code)]
        unsafe {
            rkyv::archived_root::<BrowserStat>(RKYV_BYTES_2)
        }
    })
}
