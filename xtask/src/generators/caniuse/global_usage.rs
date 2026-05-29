use std::collections::HashMap;

use anyhow::Result;
use quote::quote;

use crate::data::{Caniuse, encode_browser_name};
use crate::utils::generate_file;

pub fn build_caniuse_global_usage(data: &Caniuse) -> Result<()> {
    let mut global_usage = data
        .agents
        .iter()
        .flat_map(|(name, agent)| {
            let browser_id = encode_browser_name(name);
            agent
                .usage_global
                .iter()
                .filter(|(_, usage)| **usage > 0.0f32)
                .map(move |(version, usage)| (browser_id, version.clone(), *usage))
        })
        .collect::<Vec<(u8, String, f32)>>();
    global_usage.sort_unstable_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());

    // Concatenate the (deduplicated) version strings into one pool and reference each by a u32
    // bitpacking `offset << 8 | len`. This keeps the table free of `&str` fat pointers, which
    // each cost 16 bytes plus a load-time relocation entry in the binary.
    let mut pool = String::new();
    let mut seen: HashMap<String, u32> = HashMap::new();
    let entries = global_usage
        .iter()
        .map(|&(browser, ref version, usage)| {
            let packed = *seen.entry(version.clone()).or_insert_with(|| {
                let offset = pool.len();
                assert!(version.len() < 256 && offset < (1 << 24), "version pool overflow");
                pool.push_str(version);
                ((offset as u32) << 8) | version.len() as u32
            });
            quote! { (#browser, #packed, #usage) }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        /// Concatenated version-string pool indexed by the packed u32 in [`CANIUSE_GLOBAL_USAGE`].
        pub static GLOBAL_USAGE_VERSIONS: &str = #pool;
        /// only includes browsers with global usage > 0.0%; the u32 bitpacks `offset << 8 | len`
        /// into [`GLOBAL_USAGE_VERSIONS`].
        pub static CANIUSE_GLOBAL_USAGE: &[(u8, u32, f32)] = &[
            #(#entries),*
        ];
    };

    generate_file("caniuse_global_usage.rs", output);

    Ok(())
}
