use crate::data::caniuse::CaniuseData;
use rkyv::Deserialize;
use std::sync::OnceLock;
pub fn caniuse_browsers() -> &'static CaniuseData {
    static CANIUSE_BROWSERS: OnceLock<CaniuseData> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        let bytes = include_bytes!("caniuse_browsers.rkyv");
        let archived = unsafe { rkyv::archived_root::<CaniuseData>(bytes) };
        archived.deserialize(&mut rkyv::Infallible).unwrap()
    })
}
