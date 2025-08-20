use crate::data::caniuse::features::Feature;
use crate::data::caniuse::decompress_deflate;
use rustc_hash::FxHashMap;
use std::sync::OnceLock;
static LOOKUP_COMPRESSED: &[u8] = include_bytes!("caniuse_feature_lookup.bin.deflate");
static RANGES_COMPRESSED: &[u8] = include_bytes!("caniuse_feature_ranges.bin.deflate");
static LOOKUP_DATA: OnceLock<FxHashMap<String, u32>> = OnceLock::new();
static RANGES_DATA: OnceLock<Vec<u32>> = OnceLock::new();
fn get_lookup_map() -> &'static FxHashMap<String, u32> {
    LOOKUP_DATA.get_or_init(|| {
        let decompressed = decompress_deflate(LOOKUP_COMPRESSED);
        bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0
    })
}
fn get_ranges() -> &'static Vec<u32> {
    RANGES_DATA.get_or_init(|| {
        let decompressed = decompress_deflate(RANGES_COMPRESSED);
        bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0
    })
}
pub fn get_feature_stat(name: &str) -> Option<Feature> {
    let lookup_map = get_lookup_map();
    let ranges = get_ranges();
    if let Some(&idx) = lookup_map.get(name) {
        let start = ranges[idx as usize];
        let end = ranges[idx as usize + 1];
        Some(Feature::new(start, end))
    } else {
        None
    }
}
