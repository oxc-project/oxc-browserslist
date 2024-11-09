use anyhow::Result;
use quote::quote;

use super::{generate_file, generate_rkyv, Caniuse};

type RkyvData = Vec<(String, String, f32)>;

pub fn build_caniuse_global_usage(data: &Caniuse) -> Result<()> {
    let mut global_usage: RkyvData = data
        .agents
        .iter()
        .flat_map(|(name, agent)| {
            agent
                .usage_global
                .iter()
                .map(|(version, usage)| (name.clone(), version.clone(), *usage))
        })
        .collect::<Vec<_>>();

    global_usage.sort_unstable_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());

    generate_rkyv("global_usage.rkyv", &global_usage);

    let output = quote! {
        use std::sync::OnceLock;
        use rkyv::vec::ArchivedVec;
        use rkyv::string::ArchivedString;

        type ArchivedData = ArchivedVec<(ArchivedString, ArchivedString, f32)>;

        const RKYV_BYTES: &'static [u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }
            const ALIGNED: &'static Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("global_usage.rkyv") };
            &ALIGNED.bytes
        };

        pub fn caniuse_global_usage() -> &'static ArchivedData {
            static CANIUSE_GLOBAL_USAGE: OnceLock<&ArchivedData> = OnceLock::new();
            CANIUSE_GLOBAL_USAGE.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::access_unchecked::<ArchivedData>(RKYV_BYTES) }
            })
        }
    };

    generate_file("caniuse_global_usage.rs", output);

    Ok(())
}
