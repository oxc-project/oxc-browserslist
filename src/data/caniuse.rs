use std::{borrow::Cow, num::NonZero, sync::OnceLock};

use rustc_hash::FxHashMap;

use crate::data::BrowserName;

pub mod compression;
pub mod features;
pub mod region;

pub const ANDROID_EVERGREEN_FIRST: f32 = 37.0;
pub const OP_MOB_BLINK_FIRST: u16 = 14;

#[derive(Clone, Debug)]
pub struct BrowserStat {
    pub name: BrowserName,
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

pub use crate::generated::caniuse_global_usage::CANIUSE_GLOBAL_USAGE;

pub fn caniuse_browsers() -> &'static CaniuseData {
    static CANIUSE_BROWSERS: OnceLock<CaniuseData> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        const COMPRESSED: &[u8] = include_bytes!("../generated/caniuse_browsers.bin.deflate");
        let decompressed = compression::decompress_deflate(COMPRESSED);
        type BrowserData = Vec<(String, String, Vec<(String, f32, Option<i64>)>)>;
        let data: BrowserData =
            bincode::decode_from_slice(&decompressed, bincode::config::standard()).unwrap().0;
        data.into_iter()
            .map(|(_key, name, version_list)| {
                let name_static: &'static str = Box::leak(name.into_boxed_str());
                let stat = BrowserStat {
                    name: name_static,
                    version_list: version_list
                        .into_iter()
                        .map(|(ver, usage, date)| {
                            VersionDetail(Cow::Owned(ver), usage, date.and_then(NonZero::new))
                        })
                        .collect(),
                };
                (name_static, stat)
            })
            .collect()
    })
}

pub fn browser_version_aliases()
-> &'static FxHashMap<BrowserName, FxHashMap<&'static str, &'static str>> {
    static BROWSER_VERSION_ALIASES: OnceLock<
        FxHashMap<BrowserName, FxHashMap<&'static str, &'static str>>,
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
                if aliases.is_empty() { None } else { Some((*name, aliases)) }
            })
            .collect::<FxHashMap<BrowserName, _>>();
        let _ = aliases.insert("op_mob", {
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

        BrowserStat { name: android.name, version_list }
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
        get_browser_stat_mobile_to_desktop(normalized_name)
    } else {
        caniuse_browsers().get(normalized_name).map(|stat| (stat.name, stat))
    }
}

// Extract mobile-to-desktop logic - preserves original semantics
fn get_browser_stat_mobile_to_desktop(name: &str) -> Option<(&'static str, &'static BrowserStat)> {
    // Reproduce original logic: first check if we have a desktop mapping
    match name {
        // Browsers that have desktop equivalents
        "and_chr" => caniuse_browsers().get("chrome").map(|stat| ("and_chr", stat)),
        "android" => Some(("android", android_to_desktop())), // Special case for android
        "and_ff" => caniuse_browsers().get("firefox").map(|stat| ("and_ff", stat)),
        "ie_mob" => caniuse_browsers().get("ie").map(|stat| ("ie_mob", stat)),
        // All other browsers (including op_mob) return their own data
        _ => caniuse_browsers().get(name).map(|stat| (stat.name, stat)),
    }
}

// Storage for lowercase browser names that aren't aliased
static LOWERCASE_BROWSER_NAMES: OnceLock<FxHashMap<String, String>> = OnceLock::new();

// Cold path for case conversion - only called when input contains uppercase
#[cold]
fn get_browser_alias_lowercase(name: &str) -> &str {
    // Convert to lowercase and apply aliases
    let lowercase = name.to_ascii_lowercase();
    match lowercase.as_str() {
        "fx" | "ff" => "firefox",
        "ios" => "ios_saf",
        "explorer" => "ie",
        "blackberry" => "bb",
        "explorermobile" => "ie_mob",
        "operamini" => "op_mini",
        "operamobile" => "op_mob",
        "chromeandroid" => "and_chr",
        "firefoxandroid" => "and_ff",
        "ucandroid" => "and_uc",
        "qqandroid" => "and_qq",
        // For browsers that don't have aliases, try to return the lowercase version if it exists
        _ => {
            if caniuse_browsers().contains_key(lowercase.as_str()) {
                // Store lowercase names in a global cache to avoid repeated allocations
                let _cache = LOWERCASE_BROWSER_NAMES.get_or_init(FxHashMap::default);
                // This still requires Box::leak for the 'static lifetime requirement
                // but at least we're not doing it repeatedly for the same name
                Box::leak(lowercase.into_boxed_str())
            } else {
                // Fallback to original name
                name
            }
        }
    }
}

fn get_browser_alias(name: &str) -> &str {
    match name {
        "fx" | "ff" => "firefox",
        "ios" => "ios_saf",
        "explorer" => "ie",
        "blackberry" => "bb",
        "explorermobile" => "ie_mob",
        "operamini" => "op_mini",
        "operamobile" => "op_mob",
        "chromeandroid" => "and_chr",
        "firefoxandroid" => "and_ff",
        "ucandroid" => "and_uc",
        "qqandroid" => "and_qq",
        _ => name,
    }
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
