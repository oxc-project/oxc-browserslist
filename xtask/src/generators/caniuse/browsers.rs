use anyhow::Result;
use bincode::encode_to_vec;

use crate::{data::caniuse::Caniuse, utils::save_bin_compressed};

pub fn build_caniuse_browsers(data: &Caniuse) -> Result<()> {
    // Prepare data for serialization - convert IndexMap to Vec for bincode compatibility
    let browser_data: Vec<(String, String, Vec<(String, f32, Option<i64>)>)> = data
        .agents
        .iter()
        .map(|(name, agent)| {
            let version_list = agent
                .version_list
                .iter()
                .map(|version| {
                    (version.version.clone(), version.global_usage, version.release_date)
                })
                .collect();
            (name.clone(), name.clone(), version_list)
        })
        .collect();

    // Serialize and compress the data
    let serialized = encode_to_vec(&browser_data, bincode::config::standard())?;
    save_bin_compressed("caniuse_browsers.bin", &serialized);

    Ok(())
}
