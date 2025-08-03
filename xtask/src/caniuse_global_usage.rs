use anyhow::Result;
use quote::quote;

use super::{Caniuse, encode_browser_name, generate_file};

pub fn build_caniuse_global_usage(data: &Caniuse) -> Result<()> {
    let mut global_usage = data
        .agents
        .iter()
        .flat_map(|(name, agent)| {
            let browser_id = encode_browser_name(name);
            agent.usage_global.iter().filter(|(_, usage)| **usage > 0.0f32).map(
                move |(version, usage)| {
                    (
                        usage,
                        quote! {
                            (#browser_id, #version, #usage)
                        },
                    )
                },
            )
        })
        .collect::<Vec<_>>();
    global_usage.sort_unstable_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
    let push_usage = global_usage.into_iter().map(|(_, tokens)| tokens);

    let output = quote! {
        /// only includes browsers with global usage > 0.0%
        pub static CANIUSE_GLOBAL_USAGE: &[(u8, &str, f32)] = &[
            #(#push_usage),*
        ];
    };

    generate_file("caniuse_global_usage.rs", output);

    Ok(())
}
