use crate::semver::{ArchivedVersion, Version};
use rkyv::vec::ArchivedVec;
use std::sync::OnceLock;
type Data = Vec<Version>;
type ArchivedData = ArchivedVec<ArchivedVersion>;
pub fn get_node_versions() -> &'static ArchivedData {
    static NODE_VERSIONS: OnceLock<&ArchivedData> = OnceLock::new();
    NODE_VERSIONS.get_or_init(|| {
        let bytes = include_bytes!("node_versions.rkyv");
        #[allow(unsafe_code)]
        unsafe {
            rkyv::archived_root::<Data>(bytes)
        }
    })
}
