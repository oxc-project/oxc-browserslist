use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let dir = project_root::get_project_root()
        .unwrap()
        .join("src/generated");
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::create_dir(&dir);

    xtask::generate_browser_names_cache()?;
    xtask::build_electron_to_chromium()?;
    xtask::build_node_versions()?;
    xtask::build_node_release_schedule()?;
    xtask::build_caniuse_global()?;
    xtask::build_caniuse_region()?;
    Ok(())
}
