use crate::data::electron::{ArchivedElectronVersion, ElectronVersion};
use rkyv::string::ArchivedString;
use rkyv::vec::ArchivedVec;
use std::sync::OnceLock;
type Data = Vec<(ElectronVersion, String)>;
type ArchivedData = ArchivedVec<(ArchivedElectronVersion, ArchivedString)>;
pub fn get_electron_versions() -> &'static ArchivedData {
    static ELECTRON_VERSIONS: OnceLock<&ArchivedData> = OnceLock::new();
    ELECTRON_VERSIONS.get_or_init(|| {
        let bytes = include_bytes!("electron_to_chromium.rkyv");
        #[allow(unsafe_code)]
        unsafe {
            rkyv::archived_root::<Data>(bytes)
        }
    })
}
