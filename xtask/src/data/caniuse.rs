use std::fs;

use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;

use crate::utils::root;

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

pub fn parse_caniuse_global() -> Result<Caniuse> {
    let path = root().join("node_modules/caniuse-db/fulldata-json/data-2.0.json");
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}
