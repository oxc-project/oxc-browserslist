use std::path::PathBuf;

use project_root::get_project_root;

pub fn root() -> PathBuf {
    get_project_root().unwrap()
}
