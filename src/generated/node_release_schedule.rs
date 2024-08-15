use crate::semver::{ArchivedVersion, Version};
use rkyv::vec::ArchivedVec;
use std::sync::OnceLock;
type Data = Vec<(Version, i32, i32)>;
type ArchivedData = ArchivedVec<(ArchivedVersion, i32, i32)>;
pub fn get_release_schedule() -> &'static ArchivedData {
    static RELEASE_SCHEDULE: OnceLock<&ArchivedData> = OnceLock::new();
    RELEASE_SCHEDULE.get_or_init(|| {
        let bytes = include_bytes!("node_release_schedule.rkyv");
        unsafe { rkyv::archived_root::<Data>(bytes) }
    })
}
