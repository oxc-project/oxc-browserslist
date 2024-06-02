use std::{fs, process::Command};

use anyhow::Result;

fn main() -> Result<()> {
    let dir = project_root::get_project_root().unwrap().join("src/generated");
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::create_dir(&dir);

    xtask::electron_to_chromium::build_electron_to_chromium()?;

    xtask::node_versions::build_node_versions()?;
    xtask::node_release_schedule::build_node_release_schedule()?;

    let caniuse = xtask::parse_caniuse_global()?;
    xtask::caniuse_feature_matching::build_caniuse_feature_matching(&caniuse)?;
    xtask::caniuse_global_usage::build_caniuse_global_usage(&caniuse)?;
    xtask::caniuse_browsers::build_caniuse_browsers(&caniuse)?;
    xtask::caniuse_region_matching::build_caniuse_region_matching(&caniuse)?;

    Command::new("cargo").arg("fmt").status().unwrap();

    Ok(())
}
