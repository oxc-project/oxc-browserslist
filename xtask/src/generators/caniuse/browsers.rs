use anyhow::Result;
use quote::quote;

use crate::data::Caniuse;
use crate::utils::generate_file;

pub fn build_caniuse_browsers(data: &Caniuse) -> Result<()> {
    let browser_stat = data.agents.iter().map(|(name, agent)| {
        let detail = agent.version_list.iter().map(|version| {
            let ver = &version.version;
            let global_usage = version.global_usage;
            let release_date = if let Some(release_date) = version.release_date {
                assert_ne!(release_date, 0); // So that we can use NonZero
                quote! { Some(NonZero::new(#release_date).unwrap()) }
            } else {
                quote! { None }
            };
            quote! {
                VersionDetail(#ver, #global_usage, #release_date)
            }
        });
        quote! {
            (#name, BrowserStat {
                name: #name,
                version_list: vec![#(#detail),*]
            })
        }
    });

    let output = quote! {
        use std::num::NonZero;
        use std::sync::OnceLock;
        use rustc_hash::FxHashMap;
        use crate::data::caniuse::{BrowserStat, CaniuseData, VersionDetail};

        pub fn caniuse_browsers() -> &'static CaniuseData {
            static CANIUSE_BROWSERS: OnceLock<CaniuseData> = OnceLock::new();
            CANIUSE_BROWSERS.get_or_init(|| {
                FxHashMap::from_iter([
                    #(#browser_stat),*
                ])
            })
        }
    };

    generate_file("caniuse_browsers.rs", output);

    Ok(())
}
