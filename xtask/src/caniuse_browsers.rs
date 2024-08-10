use std::collections::HashMap;

use anyhow::Result;
use quote::quote;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::generate_rkyv;

use super::{generate_file, Caniuse};

#[derive(RkyvArchive, RkyvDeserialize, RkyvSerialize, Clone, Debug)]
pub struct BrowserStat {
    pub name: String,
    pub version_list: Vec<crate::VersionDetail>,
}

pub fn build_caniuse_browsers(data: &Caniuse) -> Result<()> {
    let browsers: HashMap<String, BrowserStat> = data
        .agents
        .iter()
        .map(|(name, agent)| {
            (
                name.to_owned(),
                BrowserStat { name: name.to_owned(), version_list: agent.version_list.clone() },
            )
        })
        .collect();

    let output = quote! {
        use crate::data::caniuse::CaniuseData;
        use rkyv::Deserialize;

        use std::sync::OnceLock;
        pub fn caniuse_browsers() -> &'static CaniuseData {
            static CANIUSE_BROWSERS: OnceLock<CaniuseData> = OnceLock::new();
            CANIUSE_BROWSERS.get_or_init(|| {
                let bytes = include_bytes!("caniuse_browsers.rkyv");
                let archived = unsafe { rkyv::archived_root::<CaniuseData>(bytes) };

                archived.deserialize(&mut rkyv::Infallible).unwrap()
            })
        }
    };

    generate_rkyv::<_, 256>("caniuse_browsers.rkyv", browsers);
    generate_file("caniuse_browsers.rs", output);

    Ok(())
}
