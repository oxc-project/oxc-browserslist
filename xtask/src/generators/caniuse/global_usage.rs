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
    // bitpacking `browser << 24 | offset << 8 | len` (unpacked by `caniuse::unpack_usage`) —
    // folding the browser id into the spare high byte makes each entry `(u32, f32)` instead of
    // a padded 12-byte `(u8, u32, f32)`. This keeps the table free of `&str` fat pointers,
    // which each cost 16 bytes plus a load-time relocation entry.
    let mut pool = String::new();
    let mut seen: HashMap<String, u32> = HashMap::new();
    let entries = global_usage
        .iter()
        .map(|&(browser, ref version, usage)| {
            let packed = u32::from(browser) << 24
                | *seen.entry(version.clone()).or_insert_with(|| {
                    let offset = pool.len();
                    assert!(version.len() < 256 && offset < (1 << 16), "version pool overflow");
                    pool.push_str(version);
                    ((offset as u32) << 8) | version.len() as u32
                });
            quote! { (#packed, #usage) }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        /// Concatenated version-string pool indexed by the packed u32 in [`CANIUSE_GLOBAL_USAGE`].
        pub static GLOBAL_USAGE_VERSIONS: &str = #pool;
        /// only includes browsers with global usage > 0.0%; the u32 bitpacks
        /// `browser_id << 24 | offset << 8 | len` into [`GLOBAL_USAGE_VERSIONS`], unpacked by
        /// `caniuse::unpack_usage`.
        pub static CANIUSE_GLOBAL_USAGE: &[(u32, f32)] = &[
            #(#entries),*
        ];
    };

    generate_file("caniuse_global_usage.rs", output);

    Ok(())
}
