use std::{borrow::Cow, num::NonZero, sync::OnceLock};

use rustc_hash::FxHashMap;

use crate::data::{BrowserName, decode_browser_name};

use compression::{LazyData, read_varint, read_zigzag};

pub mod compression;
pub mod features;
pub mod region;

pub const ANDROID_EVERGREEN_FIRST: f32 = 37.0;
pub const OP_MOB_BLINK_FIRST: u16 = 14;

#[derive(Clone, Debug)]
pub struct BrowserStat {
    pub name: Cow<'static, str>,
    pub version_list: Vec<VersionDetail>,
}

#[derive(Debug, Clone)]
pub struct VersionDetail(
    /* version */ pub Cow<'static, str>,
    /* global_usage */ pub f32,
    /* release_date */ pub Option<NonZero<i64>>,
);

impl VersionDetail {
    pub fn version(&self) -> &str {
        &self.0
    }

    pub fn global_usage(&self) -> f32 {
        self.1
    }

    pub fn release_date(&self) -> Option<NonZero<i64>> {
        self.2
    }
}

pub type CaniuseData = FxHashMap<BrowserName, BrowserStat>;

/// The canonical version-string table: every version string any dataset references, stored once.
/// It must stay lexicographically sorted — `features` binary-searches resolved indices, and the
/// region pair order depends on monotone index remaps across data updates.
static VERSION_TABLE: LazyData<Vec<String>> =
    LazyData::new(include_bytes!("../generated/caniuse_version_table.bin.deflate"));

pub(crate) fn version_table() -> &'static [String] {
    VERSION_TABLE.get()
}

/// Everything decoded from the browsers blob in one pass: the browser stats map, the
/// usage-descending global-usage order (for `> N%`/`cover` queries), and each browser's
/// version list as canonical-table indices (the feature blob's run encoding is relative
/// to this order).
struct CaniuseCore {
    browsers: CaniuseData,
    global_usage: Vec<(u8, u16, f32)>,
    version_orders: Vec<Vec<u16>>,
}

fn core() -> &'static CaniuseCore {
    static CORE: OnceLock<CaniuseCore> = OnceLock::new();
    CORE.get_or_init(|| {
        let data = compression::decompress_blob(include_bytes!(
            "../generated/caniuse_browsers.bin.deflate"
        ));
        decode_browsers(&data, version_table())
    })
}

/// Hand-decode the browsers blob, reading each section in the order `build_caniuse_browsers`
/// in xtask writes them (its doc comment is the layout reference). A generic deserializer for
/// the old nested tuple format monomorphized into far more code than these loops.
fn decode_browsers(data: &[u8], table: &'static [String]) -> CaniuseCore {
    let mut pos = 0;

    // Header: format version, then the date unit (seconds per stored day value).
    assert_eq!(data[pos], 2, "unsupported caniuse browsers data format");
    pos += 1;
    let date_unit = i64::from(u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()));
    pos += 4;

    // Usage intern table: the distinct nonzero usage values, stored as f32 bits.
    let usage_count = read_varint(data, &mut pos);
    let mut usage_table = Vec::with_capacity(usage_count);
    for _ in 0..usage_count {
        usage_table
            .push(f32::from_bits(u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap())));
        pos += 4;
    }

    // Browser ids, then per-browser version counts and released (dated) counts.
    let browser_count = usize::from(data[pos]);
    pos += 1;
    let ids = &data[pos..pos + browser_count];
    pos += browser_count;
    let version_counts: Vec<usize> =
        (0..browser_count).map(|_| read_varint(data, &mut pos)).collect();
    let released_counts: Vec<usize> =
        (0..browser_count).map(|_| read_varint(data, &mut pos)).collect();
    let total_versions: usize = version_counts.iter().sum();

    // Each browser's version list as canonical-table indices, indexed by browser id.
    let max_id = usize::from(ids.iter().copied().max().unwrap_or(0));
    let mut version_orders: Vec<Vec<u16>> = vec![Vec::new(); max_id + 1];
    for (&id, &count) in ids.iter().zip(&version_counts) {
        version_orders[usize::from(id)] =
            (0..count).map(|_| u16::try_from(read_varint(data, &mut pos)).unwrap()).collect();
    }

    // Per-version usage values, one flat list in browser order (index 0 means "no usage").
    let mut usages = Vec::with_capacity(total_versions);
    for _ in 0..total_versions {
        let index = read_varint(data, &mut pos);
        usages.push(if index == 0 { 0.0 } else { usage_table[index - 1] });
    }

    // Assemble the stats map, reading the release-date section as we go (it is the next
    // section in the stream: per browser, `released_counts[i]` zigzag day deltas). The walk
    // also records the flat (browser id, canonical index) stream that the global-usage
    // positions below point into. Insertion order is blob order (caniuse agents order).
    let mut browsers = CaniuseData::default();
    let mut flat_pairs = Vec::with_capacity(total_versions);
    for (i, &id) in ids.iter().enumerate() {
        let order = &version_orders[usize::from(id)];
        let mut version_list = Vec::with_capacity(order.len());
        let mut day = 0i64;
        for (j, &index) in order.iter().enumerate() {
            // Only the released prefix has dates; the versions after it are unreleased (None).
            let date = if j < released_counts[i] {
                day += read_zigzag(data, &mut pos);
                NonZero::new(day * date_unit)
            } else {
                None
            };
            let version = Cow::Borrowed(table[usize::from(index)].as_str());
            version_list.push(VersionDetail(version, usages[flat_pairs.len()], date));
            flat_pairs.push((id, index));
        }
        let name = decode_browser_name(id);
        browsers.insert(name.clone(), BrowserStat { name, version_list });
    }

    // The global-usage order: flat positions, resolved as they are read.
    let global_usage_count = read_varint(data, &mut pos);
    let mut global_usage = Vec::with_capacity(global_usage_count);
    for _ in 0..global_usage_count {
        let position = read_varint(data, &mut pos);
        let (id, index) = flat_pairs[position];
        global_usage.push((id, index, usages[position]));
    }
    assert_eq!(pos, data.len(), "trailing caniuse browsers data");

    CaniuseCore { browsers, global_usage, version_orders }
}

pub fn caniuse_browsers() -> &'static CaniuseData {
    &core().browsers
}

/// `(browser id, canonical version index, usage)` in usage-descending order — the iteration
/// order `cover` selects by, preserved verbatim from upstream (equal-usage ties included).
pub(crate) fn global_usage() -> &'static [(u8, u16, f32)] {
    &core().global_usage
}

/// Each browser's version list as canonical-table indices, indexed by browser id.
pub(crate) fn version_orders() -> &'static [Vec<u16>] {
    &core().version_orders
}

pub fn browser_version_aliases()
-> &'static FxHashMap<Cow<'static, str>, FxHashMap<&'static str, &'static str>> {
    static BROWSER_VERSION_ALIASES: OnceLock<
        FxHashMap<Cow<'static, str>, FxHashMap<&'static str, &'static str>>,
    > = OnceLock::new();
    BROWSER_VERSION_ALIASES.get_or_init(|| {
        let mut aliases = caniuse_browsers()
            .iter()
            .filter_map(|(name, stat)| {
                let aliases = stat
                    .version_list
                    .iter()
                    .filter_map(|version| {
                        version
                            .version()
                            .split_once('-')
                            .map(|(bottom, top)| (bottom, top, version.version()))
                    })
                    .fold(
                        FxHashMap::<&str, &str>::default(),
                        move |mut aliases, (bottom, top, version)| {
                            let _ = aliases.insert(bottom, version);
                            let _ = aliases.insert(top, version);
                            aliases
                        },
                    );
                if aliases.is_empty() { None } else { Some((name.clone(), aliases)) }
            })
            .collect::<FxHashMap<Cow<'static, str>, _>>();
        let _ = aliases.insert(Cow::Borrowed("op_mob"), {
            let mut aliases = FxHashMap::default();
            let _ = aliases.insert("59", "58");
            aliases
        });
        aliases
    })
}

fn android_to_desktop() -> &'static BrowserStat {
    static ANDROID_TO_DESKTOP: OnceLock<BrowserStat> = OnceLock::new();
    ANDROID_TO_DESKTOP.get_or_init(|| {
        let chrome = &caniuse_browsers()["chrome"];
        let android = &caniuse_browsers()["android"];

        // Pre-calculate chrome skip index to avoid repeated work
        let chrome_skip_index = find_chrome_evergreen_start(chrome);

        // Build version list more efficiently
        let mut version_list = Vec::new();

        // Add legacy android versions (2.x, 3.x, 4.x)
        version_list.extend(
            android
                .version_list
                .iter()
                .filter(|version| is_legacy_android_version(version.version()))
                .cloned(),
        );

        // Add chrome versions from evergreen point onwards
        version_list.extend(chrome.version_list.iter().skip(chrome_skip_index).cloned());

        BrowserStat { name: android.name.clone(), version_list }
    })
}

// Extract filtering logic to separate functions for better optimization
#[inline]
fn is_legacy_android_version(version: &str) -> bool {
    version.starts_with("2.")
        || version.starts_with("3.")
        || version.starts_with("4.")
        || version == "3"
        || version == "4"
}

// Extract chrome start index calculation
fn find_chrome_evergreen_start(chrome: &BrowserStat) -> usize {
    chrome
        .version_list
        .iter()
        .position(|version| {
            version
                .version()
                .parse::<usize>()
                .map(|v| v == ANDROID_EVERGREEN_FIRST as usize)
                .unwrap_or(false)
        })
        .unwrap_or(0)
}

pub fn get_browser_stat(
    name: &str,
    mobile_to_desktop: bool,
) -> Option<(&'static str, &'static BrowserStat)> {
    // Optimize string processing: fast path for already lowercase names
    let normalized_name = if name.bytes().all(|b| b.is_ascii_lowercase()) {
        get_browser_alias(name)
    } else {
        get_browser_alias_lowercase(name)
    };

    if mobile_to_desktop {
        get_browser_stat_mobile_to_desktop(normalized_name.as_ref())
    } else {
        caniuse_browsers().get(&normalized_name).map(|stat| (stat.name.as_ref(), stat))
    }
}

// Extract mobile-to-desktop logic - preserves original semantics
fn get_browser_stat_mobile_to_desktop(name: &str) -> Option<(&'static str, &'static BrowserStat)> {
    // Reproduce original logic: first check if we have a desktop mapping
    match name {
        // Browsers that have desktop equivalents
        "and_chr" => caniuse_browsers().get(&Cow::Borrowed("chrome")).map(|stat| ("and_chr", stat)),
        "android" => Some(("android", android_to_desktop())), // Special case for android
        "and_ff" => caniuse_browsers().get(&Cow::Borrowed("firefox")).map(|stat| ("and_ff", stat)),
        "ie_mob" => caniuse_browsers().get(&Cow::Borrowed("ie")).map(|stat| ("ie_mob", stat)),
        // All other browsers (including op_mob) return their own data
        _ => caniuse_browsers().get(name).map(|stat| (stat.name.as_ref(), stat)),
    }
}

fn resolve_alias(name: &str) -> Option<&'static str> {
    match name {
        "fx" | "ff" => Some("firefox"),
        "ios" => Some("ios_saf"),
        "explorer" => Some("ie"),
        "blackberry" => Some("bb"),
        "explorermobile" => Some("ie_mob"),
        "operamini" => Some("op_mini"),
        "operamobile" => Some("op_mob"),
        "chromeandroid" => Some("and_chr"),
        "firefoxandroid" => Some("and_ff"),
        "ucandroid" => Some("and_uc"),
        "qqandroid" => Some("and_qq"),
        _ => None,
    }
}

// Cold path for case conversion - only called when input contains uppercase
#[cold]
fn get_browser_alias_lowercase(name: &str) -> Cow<'static, str> {
    let lowercase = name.to_ascii_lowercase();
    if let Some(alias) = resolve_alias(&lowercase) {
        return Cow::Borrowed(alias);
    }
    if caniuse_browsers().contains_key(&Cow::Owned(lowercase.clone())) {
        Cow::Owned(lowercase)
    } else {
        Cow::Owned(name.to_string())
    }
}

fn get_browser_alias(name: &str) -> Cow<'static, str> {
    if let Some(alias) = resolve_alias(name) {
        return Cow::Borrowed(alias);
    }
    Cow::Owned(name.to_string())
}

pub fn to_desktop_name(name: &str) -> Option<&'static str> {
    match name {
        "and_chr" | "android" => Some("chrome"),
        "and_ff" => Some("firefox"),
        "ie_mob" => Some("ie"),
        _ => None,
    }
}

pub fn normalize_version<'a>(stat: &'static BrowserStat, version: &'a str) -> Option<Cow<'a, str>> {
    if stat.version_list.iter().any(|v| v.version() == version) {
        Some(Cow::Borrowed(version))
    } else if let Some(version) =
        browser_version_aliases().get(&stat.name).and_then(|aliases| aliases.get(version))
    {
        Some(Cow::Borrowed(version))
    } else if stat.version_list.len() == 1 {
        stat.version_list.first().map(|s| Cow::Owned(s.version().to_string()))
    } else {
        None
    }
}
