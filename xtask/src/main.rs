use std::{fs, process::Command};

use anyhow::Result;

fn main() -> Result<()> {
    run()?;
    Ok(())
}

fn run() -> Result<()> {
    // Clean and create the generated directory
    let dir = project_root::get_project_root()?.join("src/generated");
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::create_dir(&dir);

    // Generate electron to chromium mappings
    xtask::generators::build_electron_to_chromium()?;

    // Generate node version data
    xtask::generators::build_node_versions()?;
    xtask::generators::build_node_release_schedule()?;

    // Parse caniuse data once and use for all generators
    let caniuse = xtask::data::parse_caniuse_global()?;

    // Generate caniuse data
    xtask::generators::caniuse::build_caniuse_browsers(&caniuse)?;
    xtask::generators::caniuse::build_caniuse_feature_matching(&caniuse)?;
    xtask::generators::caniuse::build_caniuse_global_usage(&caniuse)?;
    xtask::generators::caniuse::build_caniuse_region_matching(&caniuse)?;

    // Format the generated code
    Command::new("cargo").arg("fmt").status()?;

    Ok(())
}
