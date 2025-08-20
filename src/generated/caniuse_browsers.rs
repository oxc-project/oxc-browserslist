use crate::data::caniuse::compression::decompress_deflate;
use crate::data::caniuse::{BrowserStat, CaniuseData, VersionDetail};
use rustc_hash::FxHashMap;
use std::num::NonZero;
use std::sync::OnceLock;
static BROWSERS_COMPRESSED: &[u8] = include_bytes!("caniuse_browsers.bin.deflate");
static BROWSERS_DATA: OnceLock<CaniuseData> = OnceLock::new();
pub fn caniuse_browsers() -> &'static CaniuseData {
    BROWSERS_DATA.get_or_init(|| {
        use std::collections::HashMap;
        let decompressed = decompress_deflate(BROWSERS_COMPRESSED);
        let compact_browsers: Vec<(String, Vec<(String, f32, Option<i64>)>)> =
            bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0;
        let mut browsers = FxHashMap::default();
        for (name, versions) in compact_browsers {
            let version_list = versions
                .into_iter()
                .map(|(version, global_usage, release_date)| {
                    let release_date = release_date.map(|ts| NonZero::new(ts).unwrap());
                    VersionDetail(version.leak(), global_usage, release_date)
                })
                .collect();
            browsers.insert(name.leak(), BrowserStat { name: name.leak(), version_list });
        }
        browsers
    })
}
