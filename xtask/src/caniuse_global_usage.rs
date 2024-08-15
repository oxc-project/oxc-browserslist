use anyhow::Result;
use quote::quote;

use super::{generate_file, generate_rkyv, Caniuse};

pub fn build_caniuse_global_usage(data: &Caniuse) -> Result<()> {
    let global_usage = {
        let mut global_usage: Vec<_> = data
            .agents
            .iter()
            .flat_map(|(name, agent)| {
                agent
                    .usage_global
                    .iter()
                    .map(|(version, usage)| (name.clone(), version.clone(), usage.clone()))
            })
            .collect();

        global_usage.sort_unstable_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());
        global_usage
    };

    generate_rkyv::<_, 256>("caniuse_global_usage.rkyv", global_usage);

    let output = quote! {
        use std::sync::OnceLock;

        type Data = Vec<(String, String, f32)>;
        type ArchivedData =
            rkyv::vec::ArchivedVec<(rkyv::string::ArchivedString, rkyv::string::ArchivedString, f32)>;

        pub fn caniuse_global_usage() -> &'static ArchivedData {
            static CANIUSE_GLOBAL_USAGE: OnceLock<&ArchivedData> = OnceLock::new();
            CANIUSE_GLOBAL_USAGE.get_or_init(|| {
                let bytes = include_bytes!("caniuse_global_usage.rkyv");
                unsafe { rkyv::archived_root::<Data>(bytes) }
            })
        }
    };

    generate_file("caniuse_global_usage.rs", output);

    Ok(())
}
