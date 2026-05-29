//! In-Rust port of caniuse-lite's JS unpackers (`dist/unpacker/*.js`).
//!
//! Reads packed `module.exports = {...}` data files via JSON5, then expands the
//! single-letter browser/version codes into full strings — matching what the JS
//! `browserslist` CLI sees at runtime, so the codegen and the reference
//! implementation share one source of truth.
use std::{fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use indexmap::IndexMap;
use serde::Deserialize;

use crate::data::caniuse::{Agent, Caniuse, Feature, RegionStats, VersionDetail};
use crate::utils::root;

fn caniuse_lite_dir() -> PathBuf {
    root().join("node_modules/caniuse-lite")
}

fn read_data_file(rel: &str) -> Result<String> {
    let path = caniuse_lite_dir().join("data").join(rel);
    fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))
}

fn parse_module<T: serde::de::DeserializeOwned>(content: &str) -> Result<T> {
    let body = content
        .trim_start()
        .trim_start_matches("module.exports")
        .trim_start()
        .trim_start_matches('=')
        .trim_start()
        .trim_end()
        .trim_end_matches(';')
        .trim_end()
        // JSON5 lacks `undefined`; a few feature files use `D:undefined`.
        .replace("undefined", "null");
    json5::from_str(&body).context("parse module.exports body")
}

fn load_browsers() -> Result<IndexMap<String, String>> {
    parse_module(&read_data_file("browsers.js")?)
}

fn load_browser_versions() -> Result<IndexMap<String, String>> {
    parse_module(&read_data_file("browserVersions.js")?)
}

#[derive(Deserialize)]
struct PackedAgent {
    #[serde(rename = "A", default)]
    usage_global: IndexMap<String, f32>,
    #[serde(rename = "C", default)]
    versions: Vec<String>,
    #[serde(rename = "F", default)]
    release_date: IndexMap<String, Option<i64>>,
}

fn lookup<'a>(table: &'a IndexMap<String, String>, code: &str, kind: &str) -> Result<&'a str> {
    table.get(code).map(String::as_str).with_context(|| format!("unknown {kind} code: {code}"))
}

fn load_agents(
    browsers: &IndexMap<String, String>,
    versions: &IndexMap<String, String>,
) -> Result<IndexMap<String, Agent>> {
    let packed: IndexMap<String, PackedAgent> = parse_module(&read_data_file("agents.js")?)?;
    let mut out = IndexMap::with_capacity(packed.len());
    for (code, agent) in &packed {
        let name = lookup(browsers, code, "browser")?;
        let usage_global = remap_keys(&agent.usage_global, versions)?;
        let release_date = remap_keys(&agent.release_date, versions)?;
        let version_list = agent
            .versions
            .iter()
            .filter(|c| !c.is_empty())
            .map(|c| -> Result<VersionDetail> {
                let version = lookup(versions, c, "version")?.to_string();
                let global_usage = usage_global.get(&version).copied().unwrap_or(0.0);
                let release = release_date.get(&version).copied().flatten();
                Ok(VersionDetail { version, global_usage, release_date: release })
            })
            .collect::<Result<Vec<_>>>()?;
        out.insert(name.to_string(), Agent { usage_global, version_list });
    }
    Ok(out)
}

fn remap_keys<V: Copy>(
    packed: &IndexMap<String, V>,
    versions: &IndexMap<String, String>,
) -> Result<IndexMap<String, V>> {
    packed
        .iter()
        .map(|(code, v)| {
            let ver = lookup(versions, code, "version")?.to_string();
            Ok((ver, *v))
        })
        .collect()
}

#[derive(Deserialize)]
struct PackedFeature {
    #[serde(rename = "A")]
    stats: IndexMap<String, IndexMap<String, String>>,
}

const SUPPORT_FLAGS: &[(u32, &str)] =
    &[(1, "y"), (2, "n"), (4, "a"), (8, "p"), (16, "u"), (32, "x"), (64, "d")];

fn unpack_support(cipher: u32) -> String {
    let mut parts: Vec<String> = SUPPORT_FLAGS
        .iter()
        .filter(|(mask, _)| cipher & mask != 0)
        .map(|(_, name)| (*name).to_string())
        .collect();
    let notes_start = parts.len();
    let mut notes = cipher >> 7;
    while notes != 0 {
        let bit = 32 - notes.leading_zeros();
        parts.insert(notes_start, format!("#{bit}"));
        notes -= 1u32 << (bit - 1);
    }
    parts.join(" ")
}

fn load_features(
    browsers: &IndexMap<String, String>,
    versions: &IndexMap<String, String>,
) -> Result<IndexMap<String, Feature>> {
    let dir = caniuse_lite_dir().join("data/features");
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)?
        .map(|e| Ok::<_, anyhow::Error>(e?.path()))
        .collect::<Result<Vec<_>>>()?;
    entries.retain(|p| p.extension().is_some_and(|ext| ext == "js"));
    entries.sort();

    let mut out = IndexMap::with_capacity(entries.len());
    for path in entries {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(String::from)
            .with_context(|| format!("invalid feature path: {}", path.display()))?;
        let content =
            fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let packed: PackedFeature =
            parse_module(&content).with_context(|| format!("parse {}", path.display()))?;
        let mut stats = IndexMap::with_capacity(packed.stats.len());
        for (browser_code, browser_data) in &packed.stats {
            let browser_name = lookup(browsers, browser_code, "browser")?;
            let mut by_version: IndexMap<String, String> = IndexMap::new();
            for (cipher_str, packed_versions) in browser_data {
                let cipher: u32 = cipher_str
                    .parse()
                    .with_context(|| format!("non-numeric support cipher: {cipher_str}"))?;
                let support = unpack_support(cipher);
                for code in packed_versions.split(' ').filter(|s| !s.is_empty()) {
                    let version = lookup(versions, code, "version")?.to_string();
                    by_version.insert(version, support.clone());
                }
            }
            stats.insert(browser_name.to_string(), by_version);
        }
        out.insert(name, Feature { stats });
    }
    Ok(out)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RegionEntryValue {
    Usage(f32),
    NullList(String),
}

type PackedRegion = IndexMap<String, IndexMap<String, RegionEntryValue>>;

fn load_regions(browsers: &IndexMap<String, String>) -> Result<IndexMap<String, RegionStats>> {
    let dir = caniuse_lite_dir().join("data/regions");
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)?
        .map(|e| Ok::<_, anyhow::Error>(e?.path()))
        .collect::<Result<Vec<_>>>()?;
    entries.retain(|p| p.extension().is_some_and(|ext| ext == "js"));
    entries.sort();

    let mut out = IndexMap::with_capacity(entries.len());
    for path in entries {
        let code = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(String::from)
            .with_context(|| format!("invalid region path: {}", path.display()))?;
        let content =
            fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let packed: PackedRegion =
            parse_module(&content).with_context(|| format!("parse {}", path.display()))?;
        let mut data = IndexMap::with_capacity(packed.len());
        for (browser_code, inner) in packed {
            let browser_name = lookup(browsers, &browser_code, "browser")?;
            let mut versions_usage: IndexMap<String, Option<f32>> = IndexMap::new();
            for (key, value) in inner {
                if key == "_" {
                    match value {
                        RegionEntryValue::NullList(s) => {
                            for v in s.split_whitespace() {
                                versions_usage.insert(v.to_string(), None);
                            }
                        }
                        RegionEntryValue::Usage(_) => bail!("region {code} `_` was not a string"),
                    }
                } else {
                    let usage = match value {
                        RegionEntryValue::Usage(u) => Some(u),
                        RegionEntryValue::NullList(_) => None,
                    };
                    versions_usage.insert(key, usage);
                }
            }
            data.insert(browser_name.to_string(), versions_usage);
        }
        out.insert(code, RegionStats { data });
    }
    Ok(out)
}

pub fn load() -> Result<Caniuse> {
    let browsers = load_browsers()?;
    let versions = load_browser_versions()?;
    let agents = load_agents(&browsers, &versions)?;
    let data = load_features(&browsers, &versions)?;
    let regions = load_regions(&browsers)?;
    Ok(Caniuse { agents, data, regions })
}
