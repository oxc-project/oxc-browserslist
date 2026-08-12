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

    // Load every upstream dataset first: the canonical version table is the union of every
    // version string any of them references, stored once and indexed by u16 everywhere.
    let caniuse = xtask::data::parse_caniuse_global()?;
    let timeline = xtask::data::baseline::load()?;
    let electron = xtask::generators::load_electron_versions()?;

    let mut versions = xtask::generators::caniuse::collect_versions(&caniuse);
    versions.extend(xtask::generators::baseline_versions(&timeline)?);
    versions.extend(electron.iter().map(|version| version.chromium.clone()));
    let (_, canonical) = xtask::utils::intern_table("caniuse_version_table.bin", versions);

    xtask::generators::build_electron_to_chromium(&electron, &canonical)?;
    xtask::generators::build_node()?;
    xtask::generators::build_baseline(&timeline, &canonical)?;
    xtask::generators::caniuse::build_caniuse_browsers(&caniuse, &canonical)?;
    xtask::generators::caniuse::build_caniuse_feature_matching(&caniuse, &canonical)?;
    xtask::generators::caniuse::build_caniuse_region_matching(&caniuse, &canonical)?;

    // Format the generated code
    Command::new("cargo").arg("fmt").status()?;

    Ok(())
}
