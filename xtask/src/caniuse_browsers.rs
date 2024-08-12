use std::collections::HashMap;

use anyhow::Result;
use quote::quote;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::generate_rkyv;

use super::{generate_file, Caniuse};

const ANDROID_EVERGREEN_FIRST: f32 = 37.0;

#[derive(RkyvArchive, RkyvDeserialize, RkyvSerialize, Clone, Debug)]
pub struct BrowserStat {
    pub name: String,
    pub version_list: Vec<crate::VersionDetail>,
}

pub fn android_to_desktop(browsers: &HashMap<String, BrowserStat>) -> BrowserStat {
    let chrome = browsers.get("chrome").unwrap();
    let mut android = browsers.get("android").unwrap().clone();

    let android_evergreen_first_idx = chrome
        .version_list
        .iter()
        .position(|version| {
            version.version.parse::<usize>().unwrap() == ANDROID_EVERGREEN_FIRST as usize
        })
        .unwrap();

    android.version_list = android
        .version_list
        .into_iter()
        .filter(|version| {
            let version = version.version.as_str();
            version.starts_with("2.")
                || version.starts_with("3.")
                || version.starts_with("4.")
                || version == "3"
                || version == "4"
        })
        .chain(chrome.version_list.iter().skip(android_evergreen_first_idx).cloned())
        .collect();

    android
}

pub fn build_caniuse_browsers(data: &Caniuse) -> Result<()> {
    let mut browsers: HashMap<String, BrowserStat> = data
        .agents
        .iter()
        .map(|(name, agent)| {
            (
                name.to_owned(),
                BrowserStat { name: name.to_owned(), version_list: agent.version_list.clone() },
            )
        })
        .collect();

    let android_to_desktop = android_to_desktop(&browsers);
    browsers.insert("android_to_desktop".to_string(), android_to_desktop);

    let output = quote! {
        use crate::data::caniuse::{ArchivedCaniuseData, CaniuseData};
        use std::sync::OnceLock;

        pub fn caniuse_browsers() -> &'static ArchivedCaniuseData {
            static CANIUSE_BROWSERS: OnceLock<&ArchivedCaniuseData> = OnceLock::new();
            CANIUSE_BROWSERS.get_or_init(|| {
                let bytes = include_bytes!("caniuse_browsers.rkyv");
                unsafe { rkyv::archived_root::<CaniuseData>(bytes) }
            })
        }
    };

    generate_rkyv::<_, 256>("caniuse_browsers.rkyv", browsers);
    generate_file("caniuse_browsers.rs", output);

    Ok(())
}
