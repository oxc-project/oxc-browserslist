pub mod caniuse_browsers;
pub mod caniuse_feature_matching;
pub mod caniuse_global_usage;
pub mod caniuse_region_matching;
pub mod electron_to_chromium;
pub mod node_release_schedule;
pub mod node_versions;

use std::{fs, path::PathBuf};

use anyhow::Result;
use indexmap::IndexMap;
use project_root::get_project_root;
use serde::Deserialize;

use rkyv::Archive as RkyvArchive;
use rkyv::Serialize as RkyvSerialize;

fn root() -> PathBuf {
    get_project_root().unwrap()
}

fn generate_rkyv(
    file: &str,
    value: &impl for<'a> rkyv::Serialize<
        rkyv::api::high::HighSerializer<
            rkyv::util::AlignedVec,
            rkyv::ser::allocator::ArenaHandle<'a>,
            rkyv::rancor::Error,
        >,
    >,
) {
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(value).unwrap();
    let path = root().join("src/generated").join(file);
    std::fs::write(path, bytes.as_slice()).unwrap();
}

fn generate_file(file: &str, token_stream: proc_macro2::TokenStream) {
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    fs::write(root().join("src/generated").join(file), code).unwrap();
}

pub fn parse_caniuse_global() -> Result<Caniuse> {
    let path = root().join("node_modules/caniuse-db/fulldata-json/data-2.0.json");
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

#[derive(Deserialize)]
pub struct Caniuse {
    pub agents: IndexMap<String, Agent>,
    pub data: IndexMap<String, Feature>,
}

#[derive(Deserialize)]
pub struct Agent {
    pub usage_global: IndexMap<String, f32>,
    pub version_list: Vec<VersionDetail>,
}

#[derive(RkyvArchive, RkyvSerialize, Deserialize, Clone)]
pub struct VersionDetail {
    pub version: String,
    pub global_usage: f32,
    pub release_date: Option<i64>, // unix timestamp (seconds)
}

#[derive(Deserialize)]
pub struct Feature {
    pub stats: IndexMap<String, IndexMap<String, String>>,
}
