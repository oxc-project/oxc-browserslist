use std::sync::OnceLock;

use super::{BrowserName, compression::decompress_deflate};

use crate::data::decode_browser_name;
pub use crate::generated::caniuse_feature_matching::get_feature_stat;

static FEATURES: OnceLock<Vec<u8>> = OnceLock::new();
static VERSION_TABLE: OnceLock<Vec<String>> = OnceLock::new();
/// Per-browser version order (browser id -> version-table indices, in version order). Feature
/// support lists are stored as runs of local indices into this order; see `create_data`.
static BROWSER_VERSIONS: OnceLock<Vec<Vec<u16>>> = OnceLock::new();

fn version_table() -> &'static [String] {
    VERSION_TABLE.get_or_init(|| {
        postcard::from_bytes(&decompress_deflate(include_bytes!(
            "../../generated/caniuse_feature_version_table.bin.deflate"
        )))
        .unwrap()
    })
}

fn browser_versions() -> &'static [Vec<u16>] {
    BROWSER_VERSIONS.get_or_init(|| {
        postcard::from_bytes(&decompress_deflate(include_bytes!(
            "../../generated/caniuse_feature_browser_versions.bin.deflate"
        )))
        .unwrap()
    })
}

pub struct FeatureSet {
    yes: Vec</* version */ &'static str>,
    partial: Vec</* version */ &'static str>,
}

impl FeatureSet {
    pub fn new(yes: Vec<&'static str>, partial: Vec<&'static str>) -> Self {
        Self { yes, partial }
    }

    pub fn supports(&self, version: &str, include_partial: bool) -> bool {
        self.yes.binary_search(&version).is_ok()
            || (include_partial && self.partial.binary_search(&version).is_ok())
    }
}

pub struct Feature {
    start: u32,
    end: u32,
}

impl Feature {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn create_data(&self) -> Vec<(BrowserName, FeatureSet)> {
        let features_data = FEATURES.get_or_init(|| {
            decompress_deflate(include_bytes!(
                "../../generated/caniuse_feature_matching.bin.deflate"
            ))
        });
        let table = version_table();
        let browser_versions = browser_versions();
        // The slice is hand-decoded rather than handed to `postcard::from_bytes`: a generic
        // deserializer for this nested type monomorphizes into several KB of code that would dwarf
        // the data it saves. The layout (postcard-compatible) is, per browser entry: one browser
        // byte, then the `yes` list, then the `partial` list. Each list is a varint run count
        // followed by `(start, length)` varint runs of local indices into the browser's version
        // order.
        let bytes = &features_data[self.start as usize..self.end as usize];
        let mut pos = 0;
        let entry_count = read_varint(bytes, &mut pos);
        let mut data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            let browser = bytes[pos];
            pos += 1;
            let order = &browser_versions[browser as usize];
            let yes = read_versions(bytes, &mut pos, order, table);
            let partial = read_versions(bytes, &mut pos, order, table);
            data.push((decode_browser_name(browser), FeatureSet::new(yes, partial)));
        }
        data
    }
}

fn read_varint(bytes: &[u8], pos: &mut usize) -> usize {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let byte = bytes[*pos];
        *pos += 1;
        result |= ((byte & 0x7f) as usize) << shift;
        if byte & 0x80 == 0 {
            return result;
        }
        shift += 7;
    }
}

/// Expand one stored list: `(start, length)` runs of local indices into `order` (the browser's
/// version order) become version-table indices, which are then sorted (the table is
/// lexicographically ordered, so sorting the indices yields the version strings in the order
/// `FeatureSet`'s binary searches expect) and resolved to strings.
fn read_versions(
    bytes: &[u8],
    pos: &mut usize,
    order: &[u16],
    table: &'static [String],
) -> Vec<&'static str> {
    let run_count = read_varint(bytes, pos);
    let mut indices = Vec::new();
    for _ in 0..run_count {
        let start = read_varint(bytes, pos);
        let len = read_varint(bytes, pos);
        indices.extend(order[start..start + len].iter().copied());
    }
    // Insertion sort: these lists hold only a few dozen entries, so this stays cheaper in code
    // size than instantiating the generic `sort_unstable` machinery (which measurably bloated
    // `.text`/`.eh_frame`) while easily fast enough at this length.
    for i in 1..indices.len() {
        let mut j = i;
        while j > 0 && indices[j - 1] > indices[j] {
            indices.swap(j - 1, j);
            j -= 1;
        }
    }
    indices.into_iter().map(|i| table[i as usize].as_str()).collect()
}
