use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;

pub struct Caniuse {
    pub agents: IndexMap<String, Agent>,
    pub data: IndexMap<String, Feature>,
    pub regions: IndexMap<String, RegionStats>,
}

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

#[derive(Clone)]
pub struct Feature {
    pub stats: IndexMap<String, IndexMap<String, String>>,
}

pub struct RegionStats {
    pub data: IndexMap<String, IndexMap<String, Option<f32>>>,
}

pub fn parse_caniuse_global() -> Result<Caniuse> {
    super::caniuse_lite::load()
}
