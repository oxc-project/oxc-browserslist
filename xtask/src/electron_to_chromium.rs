use std::fs;

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;

use super::{generate_file, generate_rkyv, root};

use rkyv::Archive as RkyvArchive;
use rkyv::Serialize as RkyvSerialize;

#[derive(RkyvSerialize, RkyvArchive)]
pub struct ElectronVersion {
    pub major: u16,
    pub minor: u16,
}

pub fn build_electron_to_chromium() -> Result<()> {
    let data_path = root().join("node_modules/electron-to-chromium/versions.json");
    let data = fs::read(data_path)?;
    let prased_data: IndexMap<String, String> = serde_json::from_slice(&data)?;

    let electron_version: Vec<_> = prased_data
        .into_iter()
        .map(|(electron_version, chromium_version)| {
            let split = electron_version.split('.').collect::<Vec<_>>();
            assert!(split.len() == 2, "electron version must be in major.minor format");
            let major: u16 = split[0].parse().unwrap();
            let minor: u16 = split[1].parse().unwrap();
            (ElectronVersion { major, minor }, chromium_version)
        })
        .collect();

    let output = quote! {
        use rkyv::string::ArchivedString;
        use rkyv::vec::ArchivedVec;
        use std::sync::OnceLock;

        use crate::data::electron::{ElectronVersion, ArchivedElectronVersion};

        type Data = Vec<(ElectronVersion, String)>;
        type ArchivedData = ArchivedVec<(ArchivedElectronVersion, ArchivedString)>;

        const RKYV_BYTES: &'static [u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }
            const ALIGNED: &'static Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("electron_to_chromium.rkyv") };
            &ALIGNED.bytes
        };

        pub fn get_electron_versions() -> &'static ArchivedData {
            static ELECTRON_VERSIONS: OnceLock<&ArchivedData> = OnceLock::new();
            ELECTRON_VERSIONS.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::archived_root::<Data>(RKYV_BYTES) }
            })
        }
    };

    generate_rkyv::<_, 256>("electron_to_chromium.rkyv", electron_version);
    generate_file("electron_to_chromium.rs", output);

    Ok(())
}
