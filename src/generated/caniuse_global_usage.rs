use std::sync::OnceLock;
type Data = Vec<(String, String, f32)>;
type ArchivedData =
    rkyv::vec::ArchivedVec<(rkyv::string::ArchivedString, rkyv::string::ArchivedString, f32)>;
pub fn caniuse_global_usage() -> &'static ArchivedData {
    static CANIUSE_GLOBAL_USAGE: OnceLock<&ArchivedData> = OnceLock::new();
    CANIUSE_GLOBAL_USAGE.get_or_init(|| {
        let bytes = include_bytes!("caniuse_global_usage.rkyv");
        unsafe { rkyv::archived_root::<Data>(bytes) }
    })
}
