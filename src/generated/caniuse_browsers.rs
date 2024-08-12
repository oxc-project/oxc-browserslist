use crate::data::caniuse::{ArchivedCaniuseData, CaniuseData};
use std::sync::OnceLock;
pub fn caniuse_browsers() -> &'static ArchivedCaniuseData {
    static CANIUSE_BROWSERS: OnceLock<&ArchivedCaniuseData> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        let bytes = include_bytes!("caniuse_browsers.rkyv");
        unsafe { rkyv::archived_root::<CaniuseData>(bytes) }
    })
}
