use crate::data::caniuse::{
    BrowserStat, CaniuseData, VersionDetail, compression::decompress_deflate,
};
use rustc_hash::FxHashMap;
use std::num::NonZero;
use std::sync::OnceLock;
pub fn caniuse_browsers() -> &'static CaniuseData {
    static CANIUSE_BROWSERS: OnceLock<CaniuseData> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        const COMPRESSED: &[u8] = include_bytes!("../generated/caniuse_browsers.bin.deflate");
        let decompressed = decompress_deflate(COMPRESSED);
        let data: Vec<(String, String, Vec<(String, f32, Option<i64>)>)> =
            bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0;
        data.into_iter()
            .map(|(_key, name, version_list)| {
                let name_static = Box::leak(name.into_boxed_str());
                let stat = BrowserStat {
                    name: name_static,
                    version_list: version_list
                        .into_iter()
                        .map(|(ver, usage, date)| {
                            let ver_static = Box::leak(ver.into_boxed_str());
                            VersionDetail(ver_static, usage, date.and_then(|d| NonZero::new(d)))
                        })
                        .collect(),
                };
                (name_static as &str, stat)
            })
            .collect()
    })
}
