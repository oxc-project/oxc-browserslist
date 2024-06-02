use anyhow::Result;
use quote::quote;

use super::{generate_file, Caniuse};

pub fn build_caniuse_global_usage(data: &Caniuse) -> Result<()> {
    let mut global_usage = data
        .agents
        .iter()
        .flat_map(|(name, agent)| {
            agent.usage_global.iter().map(move |(version, usage)| {
                (
                    usage,
                    quote! {
                        (#name, #version, #usage)
                    },
                )
            })
        })
        .collect::<Vec<_>>();
    global_usage.sort_unstable_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
    let push_usage = global_usage.into_iter().map(|(_, tokens)| tokens);

    let output = quote! {
        use crate::data::BrowserName;
        pub static CANIUSE_GLOBAL_USAGE: &[(BrowserName, &str, f32)] = &[#(#push_usage),*];
    };

    generate_file("caniuse_global_usage.rs", output);

    Ok(())
}
