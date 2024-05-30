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
    let data = serde_json::from_slice::<IndexMap<String, String>>(&fs::read(data_path)?)?
        .into_iter()
        .map(|(electron_version, chromium_version)| {
            let split = electron_version.split('.').collect::<Vec<_>>();
            assert!(
                split.len() == 2,
                "electron version must be in major.minor format"
            );
            let major: u16 = split[0].parse().unwrap();
            let minor: u16 = split[1].parse().unwrap();
            quote! {
                (ElectronVersion::new(#major, #minor), #chromium_version)
            }
        });

    let output = quote! {
        use crate::data::electron::ElectronVersion;
        pub static ELECTRON_VERSIONS: &[(ElectronVersion, &str)] = &[#(#data),*];
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

    let versions = releases.into_iter().map(|release| {
        let version = release.version.split('.').collect::<Vec<_>>();
        assert_eq!(version.len(), 3);
        let major: u32 = version[0].parse().unwrap();
        let minor: u32 = version[1].parse().unwrap();
        let patch: u32 = version[2].parse().unwrap();
        quote! {
            Version(#major, #minor, #patch)
        }
    });
    let output = quote! {
        use crate::semver::Version;

        pub static NODE_VERSIONS: &[Version] = &[#(#versions),*];
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
            let version = version.split('.').collect::<Vec<_>>();
            assert!(version.len() > 0);
            let major: u32 = version[0].parse().unwrap();
            let minor: u32 = version
                .get(1)
                .map(|v| v.parse().unwrap())
                .unwrap_or_default();
            let patch: u32 = version
                .get(2)
                .map(|v| v.parse().unwrap())
                .unwrap_or_default();
            quote! {
                (Version(#major, #minor, #patch), #start, #end)
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        use chrono::{NaiveDate, NaiveDateTime};
        use once_cell::sync::Lazy;
        use crate::semver::Version;

        pub static RELEASE_SCHEDULE: Lazy<Vec<(Version, NaiveDateTime, NaiveDateTime)>> =
            Lazy::new(|| {
                let date_format = "%Y-%m-%d";
                [#(#versions),*]
                    .into_iter()
                    .map(|(version, start, end)| {
                        (
                            version,
                                NaiveDate::parse_from_str(start, date_format)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap(),
                                NaiveDate::parse_from_str(end, date_format)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap(),
                        )
                    })
                .collect::<Vec<_>>()
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
        use crate::data::BrowserName;
        pub static CANIUSE_GLOBAL_USAGE: &[(BrowserName, &str, f32)] = &[#(#push_usage),*];
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
                    .filter_map(|(name, versions)| {
                        let name = encode_browser_name(name);
                        let versions = versions
                            .into_iter()
                            .filter(|(_version, flag)| *flag != "n")
                            .collect::<Vec<_>>();
                        let y = versions
                            .iter()
                            .filter(|(_, flag)| flag.contains('y'))
                            .map(|x| x.0.clone())
                            .collect::<Vec<_>>();
                        let a = versions
                            .iter()
                            .filter(|(_, flag)| flag.contains('a'))
                            .map(|x| x.0.clone())
                            .collect::<Vec<_>>();
                        if y.is_empty() && a.is_empty() {
                            None
                        } else {
                            Some((name, (y, a)))
                        }
                    })
                    .collect::<IndexMap<_, _>>(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    let keys = data.data.keys().collect::<Vec<_>>();

    let output = quote! {
        use rustc_hash::FxHashMap;
        use once_cell::sync::Lazy;
        use serde_json::from_str;
        use crate::data::caniuse::features::{Feature, FeatureSet};
        use crate::data::browser_name::decode_browser_name;

        pub(crate) fn get_feature_stat(name: &str) -> Option<&'static Feature> {
            match name {
                #( #keys => {
                    static STAT: Lazy<Feature> = Lazy::new(|| {
                        from_str::<FxHashMap::<u8, FeatureSet>>(#features)
                            .unwrap()
                            .into_iter()
                            .map(|(browser, versions)| (decode_browser_name(browser), versions))
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
        use crate::data::browser_name::decode_browser_name;

        type RegionData = Vec<(BrowserName, &'static str, f32)>;

        pub fn get_usage_by_region(region: &str) -> Option<&'static RegionData> {
            match region {
                #( #keys => {
                    static USAGE: Lazy<Vec<(BrowserName, &'static str, f32)>> = Lazy::new(|| {
                        from_str::<Vec<(u8, &'static str, f32)>>(#data)
                            .unwrap()
                            .into_iter()
                            .map(|(browser, version, usage)| (decode_browser_name(browser), version, usage))
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
