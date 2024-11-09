use std::str::FromStr;

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;

use std::collections::HashMap;
use std::collections::HashSet;

type SupportMap = HashMap<
    /* browser */ String,
    (/* fully */ HashSet<String>, /* partial */ HashSet<String>),
>;

use super::{encode_browser_name, generate_file, generate_rkyv, Caniuse};

pub fn build_caniuse_feature_matching(data: &Caniuse) -> Result<()> {
    let mut features: HashMap<String, SupportMap> = HashMap::new();

    for (name, feature) in &data.data {
        let mut support_map: SupportMap = SupportMap::new();

        for (browser, versions) in &feature.stats {
            let fully = versions
                .iter()
                .filter(|(_, flag)| flag.contains('y'))
                .map(|x| x.0.clone())
                .collect::<HashSet<_>>();

            let partial = versions
                .iter()
                .filter(|(_, flag)| flag.contains('a'))
                .map(|x| x.0.clone())
                .collect::<HashSet<_>>();

            support_map.insert(browser.clone(), (fully, partial));
        }

        features.insert(name.clone(), support_map);
    }

    let output = quote! {
        use crate::data::caniuse::features::{ArchivedFeature, ArchivedFeatures, Features};
        use std::sync::OnceLock;

        const RKYV_BYTES: &'static [u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }
            const ALIGNED: &'static Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("caniuse_feature_matching.rkyv") };
            &ALIGNED.bytes
        };

        pub(crate) fn get_feature_stat(name: &str) -> Option<&'static ArchivedFeature> {
            static CANIUSE_FEATURE_MATCHING: OnceLock<&ArchivedFeatures> = OnceLock::new();
            let stats = CANIUSE_FEATURE_MATCHING.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::access_unchecked::<ArchivedFeatures>(RKYV_BYTES) }
            });

            return stats.get(name);
        }
    };

    generate_rkyv("caniuse_feature_matching.rkyv", &features);
    generate_file("caniuse_feature_matching.rs", output);

    Ok(())
}
