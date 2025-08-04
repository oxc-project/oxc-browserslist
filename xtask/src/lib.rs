pub mod caniuse_browsers;
pub mod caniuse_feature_matching;
pub mod caniuse_global_usage;
pub mod caniuse_region_matching;
pub mod electron_to_chromium;
pub mod node_release_schedule;
pub mod node_versions;

use std::{fs, io::Write, path::PathBuf};

use anyhow::Result;
use flate2::{Compression, write::GzEncoder};
use indexmap::IndexMap;
use project_root::get_project_root;
use serde::Deserialize;

fn root() -> PathBuf {
    get_project_root().unwrap()
}

fn save_bin_compressed(file: &str, bytes: &[u8]) {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(bytes).unwrap();
    let compressed = encoder.finish().unwrap();
    let file = format!("{}.gz", file);
    fs::write(root().join("src/generated").join(file), compressed).unwrap();
}

fn generate_file(file: &str, token_stream: proc_macro2::TokenStream) {
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    fs::write(root().join("src/generated").join(file), code).unwrap();
}

fn create_range_vec(v: &Vec<Vec<u8>>) -> Vec<u32> {
    let mut offset = 0;
    // [start0, start1, ..., startN, endN]
    let mut ranges = vec![];
    for values in v {
        ranges.push(offset as u32);
        offset += values.len();
    }
    ranges.push(offset as u32);
    ranges
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

#[derive(Deserialize, Clone)]
pub struct VersionDetail {
    pub version: String,
    pub global_usage: f32,
    pub release_date: Option<i64>, // unix timestamp (seconds)
}

#[derive(Deserialize, Clone)]
pub struct Feature {
    pub stats: IndexMap<String, IndexMap<String, String>>,
}

fn encode_browser_name(name: &str) -> u8 {
    match name {
        "ie" => 1,
        "edge" => 2,
        "firefox" => 3,
        "chrome" => 4,
        "safari" => 5,
        "opera" => 6,
        "ios_saf" => 7,
        "op_mini" => 8,
        "android" => 9,
        "bb" => 10,
        "op_mob" => 11,
        "and_chr" => 12,
        "and_ff" => 13,
        "ie_mob" => 14,
        "and_uc" => 15,
        "samsung" => 16,
        "and_qq" => 17,
        "baidu" => 18,
        "kaios" => 19,
        _ => unreachable!("unknown browser name"),
    }
}
