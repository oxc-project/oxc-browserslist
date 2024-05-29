use anyhow::Result;
use indexmap::IndexMap;
use project_root::get_project_root;
use quote::quote;
use serde::Deserialize;
use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    get_project_root().unwrap()
}

fn out_dir() -> PathBuf {
    root().join("src/generated")
}

fn format_token_stream(token_stream: proc_macro2::TokenStream) -> String {
    let syntax_tree = syn::parse2(token_stream).unwrap();
    prettyplease::unparse(&syntax_tree)
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

#[derive(Deserialize)]
struct Caniuse {
    agents: IndexMap<String, Agent>,
    data: IndexMap<String, Feature>,
}

#[derive(Deserialize)]
struct Agent {
    usage_global: IndexMap<String, f32>,
    version_list: Vec<VersionDetail>,
}

#[derive(Clone, Deserialize)]
struct VersionDetail {
    version: String,
    global_usage: f32,
    release_date: Option<i64>,
}

#[derive(Deserialize)]
pub struct Feature {
    stats: IndexMap<String, IndexMap<String, String>>,
}

pub fn build_electron_to_chromium() -> Result<()> {
    let path = out_dir().join("electron_to_chromium.rs");

    let data_path = root().join("node_modules/electron-to-chromium/versions.json");
    let mut data = serde_json::from_slice::<IndexMap<String, String>>(&fs::read(data_path)?)?
        .into_iter()
        .map(|(electron_version, chromium_version)| {
            (electron_version.parse::<f32>().unwrap(), chromium_version)
        })
        .collect::<Vec<_>>();
    data.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    let data = data
        .into_iter()
        .map(|(electron_version, chromium_version)| {
            quote! {
                (#electron_version, #chromium_version)
            }
        });

    let output = quote! {
        use once_cell::sync::Lazy;
        pub static ELECTRON_VERSIONS: Lazy<Vec<(f32, &'static str)>> = Lazy::new(|| vec![#(#data),*]);
    };

    fs::write(path, format_token_stream(output))?;

    Ok(())
}

pub fn build_node_versions() -> Result<()> {
    #[derive(Deserialize)]
    struct NodeRelease {
        version: String,
    }

    let path = out_dir().join("node_versions.rs");

    let releases_path = root().join("node_modules/node-releases/data/processed/envs.json");
    let releases: Vec<NodeRelease> = serde_json::from_slice(&fs::read(releases_path)?)?;

    let versions = releases.into_iter().map(|release| release.version);
    let output = quote! {
        use once_cell::sync::Lazy;
        pub static NODE_VERSIONS: Lazy<Vec<&'static str>> = Lazy::new(|| vec![#(#versions),*]);
    };

    fs::write(path, format_token_stream(output))?;

    Ok(())
}

pub fn build_node_release_schedule() -> Result<()> {
    #[derive(Deserialize)]
    struct NodeRelease {
        start: String,
        end: String,
    }

    let path = out_dir().join("node_release_schedule.rs");

    let schedule_path =
        root().join("node_modules/node-releases/data/release-schedule/release-schedule.json");
    let schedule: IndexMap<String, NodeRelease> =
        serde_json::from_slice(&fs::read(schedule_path)?)?;
    let versions = schedule
        .into_iter()
        .map(|(version, NodeRelease { start, end })| {
            let version = version.trim_start_matches('v');
            quote! {
                map.insert(#version, (#start, #end));
            }
        });

    let output = quote! {
        use rustc_hash::FxHashMap;
        use chrono::{NaiveDate, NaiveDateTime};
        use once_cell::sync::Lazy;

        pub static RELEASE_SCHEDULE: Lazy<FxHashMap<&'static str, (NaiveDateTime, NaiveDateTime)>> =
            Lazy::new(|| {
                let date_format = "%Y-%m-%d";

                let mut map = FxHashMap::default();
                #(#versions)*;
                map
                    .into_iter()
                    .map(|(version, (start, end))| {
                        (
                            version,
                            (
                                NaiveDate::parse_from_str(start, date_format)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap(),
                                NaiveDate::parse_from_str(end, date_format)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap(),
                            ),
                        )
                    })
                .collect()
            });
    };

    fs::write(path, format_token_stream(output))?;

    Ok(())
}

pub fn build_caniuse_global() -> Result<()> {
    let data = parse_caniuse_global()?;

    let browser_stat = data.agents.iter().map(|(name, agent)| {
        let detail = agent.version_list.iter().map(|version| {
            let ver = &version.version;
            let global_usage = version.global_usage;
            let release_date = if let Some(release_date) = version.release_date {
                quote! { Some(#release_date) }
            } else {
                quote! { None }
            };
            quote! {
                VersionDetail {
                    version: #ver,
                    global_usage: #global_usage,
                    release_date: #release_date,
                }
            }
        });
        quote! {
            map.insert(#name, BrowserStat {
                name: #name,
                version_list: vec![#(#detail),*],
            });
        }
    });

    let output = quote! {
        use rustc_hash::FxHashMap;
        use crate::data::caniuse::VersionDetail;
        use crate::data::caniuse::BrowserStat;
        use crate::data::caniuse::CaniuseData;
        use once_cell::sync::Lazy;
        pub static CANIUSE_BROWSERS: Lazy<CaniuseData> =
            Lazy::new(|| {
                let mut map = FxHashMap::default();
                #(#browser_stat)*;
                map
            });
    };

    fs::write(
        out_dir().join("caniuse_browsers.rs"),
        format_token_stream(output),
    )?;

    let mut global_usage = data
        .agents
        .iter()
        .flat_map(|(name, agent)| {
            agent.usage_global.iter().map(move |(version, usage)| {
                (
                    usage,
                    quote! {
                        (#name, #version, #usage)
                    },
                )
            })
        })
        .collect::<Vec<_>>();
    global_usage.sort_unstable_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
    let push_usage = global_usage.into_iter().map(|(_, tokens)| tokens);

    let output = quote! {
        use once_cell::sync::Lazy;
        use crate::data::BrowserName;
        pub static CANIUSE_GLOBAL_USAGE: Lazy<Vec<(BrowserName, &'static str, f32)>> = Lazy::new(|| vec![#(#push_usage),*]);
    };

    fs::write(
        out_dir().join("caniuse_global_usage.rs"),
        format_token_stream(output),
    )?;

    let features = data
        .data
        .iter()
        .map(|(_name, feature)| {
            serde_json::to_string(
                &feature
                    .stats
                    .iter()
                    .map(|(name, versions)| {
                        (
                            encode_browser_name(name),
                            versions
                                .into_iter()
                                .map(|(version, flags)| {
                                    let mut bit = 0;
                                    if flags.contains('y') {
                                        bit |= 1;
                                    }
                                    if flags.contains('a') {
                                        bit |= 2;
                                    }
                                    (version, bit)
                                })
                                .collect::<IndexMap<_, u8>>(),
                        )
                    })
                    .collect::<IndexMap<_, _>>(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    let keys = data.data.keys().collect::<Vec<_>>();

    let output = quote! {
        use rustc_hash::FxHashMap;
        use indexmap::IndexMap;
        use once_cell::sync::Lazy;
        use serde_json::from_str;
        use crate::data::BrowserName;

        type Feature = FxHashMap<BrowserName, IndexMap<&'static str, u8>>;

        pub(crate) fn _get_feature_stat(name: &str) -> Option<&'static Feature> {
            match name {
                #( #keys => {
                    static STAT: Lazy<FxHashMap<BrowserName, IndexMap<&'static str, u8>>> = Lazy::new(|| {
                        from_str::<FxHashMap::<u8, IndexMap<&'static str, u8>>>(#features)
                            .unwrap()
                            .into_iter()
                            .map(|(browser, versions)| (crate::data::browser_name::decode_browser_name(browser), versions))
                            .collect()
                    });
                    Some(&*STAT)
                }, )*
                _ => None,
            }
        }
    };

    fs::write(
        out_dir().join("caniuse_feature_matching.rs"),
        format_token_stream(output),
    )?;

    Ok(())
}

fn parse_caniuse_global() -> Result<Caniuse> {
    let path = root().join("node_modules/caniuse-db/fulldata-json/data-2.0.json");
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

pub fn build_caniuse_region() -> Result<()> {
    #[derive(Deserialize)]
    struct RegionData {
        data: IndexMap<String, IndexMap<String, Option<f32>>>,
    }

    let out_path = out_dir();
    let out_dir = out_path.to_string_lossy();

    let files_path = root().join("node_modules/caniuse-db/region-usage-json");
    let files = fs::read_dir(files_path)?
        .map(|entry| entry.map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>>>()?;

    let Caniuse { agents, .. } = parse_caniuse_global()?;

    let data = files
        .iter()
        .map(|file| {
            let RegionData { data } =
                serde_json::from_slice(&fs::read(file.path()).unwrap()).unwrap();
            let mut usage = data
                .into_iter()
                .flat_map(|(name, stat)| {
                    let agent = agents.get(&name).unwrap();
                    stat.into_iter().filter_map(move |(version, usage)| {
                        let version = if version.as_str() == "0" {
                            agent.version_list.last().unwrap().version.clone()
                        } else {
                            version
                        };
                        usage.map(|usage| (encode_browser_name(&name), version, usage))
                    })
                })
                .collect::<Vec<_>>();
            usage.sort_unstable_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());
            serde_json::to_string(&usage).unwrap()
        })
        .collect::<Vec<_>>();

    let keys = files
        .iter()
        .map(|entry| {
            entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .map(|s| s.to_owned())
                .unwrap()
        })
        .collect::<Vec<_>>();

    let output = quote! {
        use once_cell::sync::Lazy;
        use serde_json::from_str;
        use crate::data::BrowserName;

        type RegionData = Vec<(BrowserName, &'static str, f32)>;

        pub fn get_usage_by_region(region: &str) -> Option<&'static RegionData> {
            match region {
                #( #keys => {
                    static USAGE: Lazy<Vec<(BrowserName, &'static str, f32)>> = Lazy::new(|| {
                        from_str::<Vec<(u8, &'static str, f32)>>(#data)
                            .unwrap()
                            .into_iter()
                            .map(|(browser, version, usage)| (crate::data::browser_name::decode_browser_name(browser), version, usage))
                            .collect()
                    });
                    Some(&*USAGE)
                }, )*
                _ => None,
            }
        }
    };
    fs::write(
        format!("{}/caniuse_region_matching.rs", &out_dir),
        format_token_stream(output),
    )?;

    Ok(())
}
