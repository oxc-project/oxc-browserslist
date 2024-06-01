use anyhow::Result;
use std::fs;
use std::process::Command;

fn main() -> Result<()> {
    let dir = project_root::get_project_root().unwrap().join("src/generated");
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::create_dir(&dir);

    xtask::build_electron_to_chromium()?;
    xtask::build_node_versions()?;
    xtask::build_node_release_schedule()?;
    xtask::build_caniuse_global()?;
    xtask::build_caniuse_region()?;

    Command::new("cargo").arg("fmt").status().unwrap();

    Ok(())
}
