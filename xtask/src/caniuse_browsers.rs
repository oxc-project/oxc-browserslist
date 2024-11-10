use std::collections::HashMap;

use anyhow::Result;
use quote::quote;

use super::{generate_file, generate_rkyv, Caniuse, VersionDetail};

use rkyv::Archive;
use rkyv::Serialize;

const ANDROID_EVERGREEN_FIRST: f32 = 37.0;

#[derive(Archive, Serialize, Clone)]
pub struct BrowserStat {
    pub name: String,
    pub version_list: Vec<VersionDetail>,
}

fn android_to_desktop(browsers: &HashMap<String, BrowserStat>) -> BrowserStat {
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
    let browsers: HashMap<String, BrowserStat> = data
        .agents
        .iter()
        .map(|(name, agent)| {
            (
                name.clone(),
                BrowserStat { name: name.clone(), version_list: agent.version_list.clone() },
            )
        })
        .collect();

    let android_to_desktop = android_to_desktop(&browsers);

    generate_rkyv("caniuse_browsers_android_to_desktop.rkyv", &android_to_desktop);
    generate_rkyv("caniuse_browsers.rkyv", &browsers);

    let output = quote! {
        use std::sync::OnceLock;
        use rkyv::string::ArchivedString;
        use rkyv::collections::swiss_table::ArchivedHashMap;
        use crate::data::caniuse::ArchivedBrowserStat;

        pub type ArchivedCaniuseData = ArchivedHashMap<ArchivedString, ArchivedBrowserStat>;

        const RKYV_BYTES: &[u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }

            const ALIGNED: &Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("caniuse_browsers.rkyv") };

            &ALIGNED.bytes
        };

        pub fn caniuse_browsers() -> &'static ArchivedCaniuseData {
            static CANIUSE_BROWSERS: OnceLock<&ArchivedCaniuseData> = OnceLock::new();
            CANIUSE_BROWSERS.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::access_unchecked::<ArchivedCaniuseData>(RKYV_BYTES) }
            })
        }

        const RKYV_BYTES_2: &[u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }

            const ALIGNED: &Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("caniuse_browsers_android_to_desktop.rkyv") };

            &ALIGNED.bytes
        };

        pub fn caniuse_browsers_android_to_desktop() -> &'static ArchivedBrowserStat {
            static CANIUSE_BROWSERS: OnceLock<&ArchivedBrowserStat> = OnceLock::new();
            CANIUSE_BROWSERS.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::access_unchecked::<ArchivedBrowserStat>(RKYV_BYTES_2) }
            })
        }
    };

    generate_file("caniuse_browsers.rs", output);

    Ok(())
}
