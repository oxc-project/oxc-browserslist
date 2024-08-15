use std::{borrow::Cow, sync::OnceLock};

use rustc_hash::FxHashMap;

use crate::data::BrowserName;

pub mod features;
pub mod region;

pub const ANDROID_EVERGREEN_FIRST: f32 = 37.0;
pub const OP_MOB_BLINK_FIRST: u32 = 14;

use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(RkyvArchive, RkyvDeserialize, RkyvSerialize, Clone, Debug)]
pub struct BrowserStat {
    pub name: String,
    pub version_list: Vec<VersionDetail>,
}

#[derive(RkyvArchive, RkyvDeserialize, RkyvSerialize, Clone, Debug)]
pub struct VersionDetail {
    pub version: String,
    pub global_usage: f32,
    pub release_date: Option<i64>,
}

pub type ArchivedCaniuseData =
    rkyv::collections::ArchivedHashMap<rkyv::string::ArchivedString, ArchivedBrowserStat>;
pub type CaniuseData = std::collections::HashMap<String, BrowserStat>;

pub use crate::generated::{
    caniuse_browsers::caniuse_browsers, caniuse_global_usage::caniuse_global_usage,
};

pub fn browser_version_aliases(
) -> &'static FxHashMap<BrowserName, FxHashMap<&'static str, &'static str>> {
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
                            .version
                            .split_once('-')
                            .map(|(bottom, top)| (bottom, top, version.version.as_str()))
                    })
                    .fold(
                        FxHashMap::<&str, &str>::default(),
                        move |mut aliases, (bottom, top, version)| {
                            let _ = aliases.insert(bottom, version);
                            let _ = aliases.insert(top, version);
                            aliases
                        },
                    );
                if aliases.is_empty() {
                    None
                } else {
                    Some((name.as_str(), aliases))
                }
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

pub fn get_browser_stat(
    name: &str,
    mobile_to_desktop: bool,
) -> Option<(&'static str, &'static ArchivedBrowserStat)> {
    let name = if name.bytes().all(|b| b.is_ascii_lowercase()) {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(name.to_ascii_lowercase())
    };
    let name = get_browser_alias(&name);

    let browsers = caniuse_browsers();

    if mobile_to_desktop {
        if let Some(desktop_name) = to_desktop_name(name) {
            match name {
                "android" => Some(("android", browsers.get("android_to_desktop").unwrap())),
                "op_mob" => Some(("op_mob", browsers.get("opera").unwrap())),
                _ => browsers
                    .get(desktop_name)
                    .map(|stat| (get_mobile_by_desktop_name(desktop_name), stat)),
            }
        } else {
            browsers.get(name).map(|stat| (stat.name.as_str(), stat))
        }
    } else {
        browsers.get(name).map(|stat| (stat.name.as_str(), stat))
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

fn get_mobile_by_desktop_name(name: &str) -> &'static str {
    match name {
        "chrome" => "and_chr", // "android" has been handled as a special case
        "firefox" => "and_ff",
        "ie" => "ie_mob",
        "opera" => "op_mob",
        _ => unreachable!(),
    }
}

pub fn normalize_version<'a>(
    stat: &'static ArchivedBrowserStat,
    version: &'a str,
) -> Option<&'a str> {
    if stat.version_list.iter().any(|v| v.version == version) {
        Some(version)
    } else if let Some(version) =
        browser_version_aliases().get(stat.name.as_str()).and_then(|aliases| aliases.get(version))
    {
        Some(version)
    } else if stat.version_list.len() == 1 {
        stat.version_list.first().map(|s| s.version.as_str())
    } else {
        None
    }
}
