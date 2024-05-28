use anyhow::Result;
use indexmap::IndexMap;
use project_root::get_project_root;
use quote::quote;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs, io,
    path::PathBuf,
};

fn out_dir() -> PathBuf {
    get_project_root().unwrap().join("src/generated")
}

fn root() -> String {
    get_project_root().unwrap().to_string_lossy().to_string()
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
    agents: HashMap<String, Agent>,
    data: BTreeMap<String, Feature>,
}

#[derive(Deserialize)]
struct Agent {
    usage_global: HashMap<String, f32>,
    version_list: Vec<VersionDetail>,
}

#[derive(Clone, Deserialize, Serialize)]
struct VersionDetail {
    version: String,
    global_usage: f32,
    release_date: Option<i64>,
}

#[derive(Deserialize)]
pub struct Feature {
    stats: HashMap<String, IndexMap<String, String>>,
}

pub fn generate_browser_names_cache() -> Result<()> {
    string_cache_codegen::AtomType::new(
        "data::browser_name::BrowserNameAtom",
        "browser_name_atom!",
    )
    .atoms(&[
        "ie", "edge", "firefox", "chrome", "safari", "opera", "ios_saf", "op_mini", "android",
        "bb", "op_mob", "and_chr", "and_ff", "ie_mob", "and_uc", "samsung", "and_qq", "baidu",
        "kaios",
    ])
    .write_to_file(&out_dir().join("browser_name_atom.rs"))?;

    Ok(())
}

pub fn build_electron_to_chromium() -> Result<()> {
    let path = format!("{}/electron_to_chromium.rs", out_dir().to_string_lossy());

    let mut data = serde_json::from_slice::<BTreeMap<String, String>>(&fs::read(format!(
        "{}/vendor/electron-to-chromium/versions.json",
        root()
    ))?)?
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

    let path = format!("{}/node_versions.rs", out_dir().to_string_lossy());

    let releases: Vec<NodeRelease> = serde_json::from_slice(&fs::read(format!(
        "{}/vendor/node-releases/data/processed/envs.json",
        root()
    ))?)?;

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

    let path = format!("{}/node_release_schedule.rs", out_dir().to_string_lossy());

    let schedule: HashMap<String, NodeRelease> = serde_json::from_slice(&fs::read(format!(
        "{}/vendor/node-releases/data/release-schedule/release-schedule.json",
        root()
    ))?)?;
    let cap = schedule.len();
    let versions = schedule
        .into_iter()
        .map(|(version, NodeRelease { start, end })| {
            let version = version.trim_start_matches('v');
            quote! {
                map.insert(#version, (#start, #end));
            }
        });

    let output = quote! {
        use ahash::AHashMap;
        use chrono::{NaiveDate, NaiveDateTime};
        use once_cell::sync::Lazy;

        pub static RELEASE_SCHEDULE: Lazy<AHashMap<&'static str, (NaiveDateTime, NaiveDateTime)>> =
            Lazy::new(|| {
                let date_format = "%Y-%m-%d";

                let mut map = ahash::AHashMap::with_capacity(#cap);
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
    let out_path = out_dir();
    let out_dir = out_path.to_string_lossy();

    let data = parse_caniuse_global()?;

    let map_cap = data.agents.len();
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
            map.insert(BrowserNameAtom::from(#name), BrowserStat {
                name: BrowserNameAtom::from(#name),
                version_list: vec![#(#detail),*],
            });
        }
    });

    let map_cap = syn::Index::from(map_cap);

    let output = quote! {
        use ahash::AHashMap;
        use crate::data::browser_name::BrowserNameAtom;
        use crate::data::caniuse::VersionDetail;
        use crate::data::caniuse::BrowserStat;
        use crate::data::caniuse::CaniuseData;
        use once_cell::sync::Lazy;
        pub static CANIUSE_BROWSERS: Lazy<CaniuseData> =
            Lazy::new(|| {
                let mut map = AHashMap::with_capacity(#map_cap);
                #(#browser_stat)*;
                map
            });
    };

    fs::write(
        format!("{}/caniuse_browsers.rs", &out_dir),
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
                        (BrowserNameAtom::from(#name), #version, #usage)
                    },
                )
            })
        })
        .collect::<Vec<_>>();
    global_usage.sort_unstable_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
    let push_usage = global_usage.into_iter().map(|(_, tokens)| tokens);

    let output = quote! {
        use once_cell::sync::Lazy;
        use crate::data::browser_name::BrowserNameAtom;
        pub static CANIUSE_GLOBAL_USAGE: Lazy<Vec<(BrowserNameAtom, &'static str, f32)>> = Lazy::new(|| vec![#(#push_usage),*]);
    };

    fs::write(
        format!("{}/caniuse_global_usage.rs", &out_dir),
        format_token_stream(output),
    )?;

    let features_dir = format!("{}/features", &out_dir);
    if matches!(fs::File::open(&features_dir), Err(e) if e.kind() == io::ErrorKind::NotFound) {
        fs::create_dir(&features_dir)?;
    }
    for (name, feature) in &data.data {
        fs::write(
            format!("{}/{}.json", &features_dir, name),
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
                    .collect::<HashMap<_, _>>(),
            )?,
        )?;
    }
    let features = data.data.keys().collect::<Vec<_>>();
    let files = features
        .iter()
        .map(|f| fs::read_to_string(out_path.join("features").join(format!("{f}.json"))).unwrap())
        .collect::<Vec<_>>();

    let output = quote! {
        use ahash::AHashMap;
        use indexmap::IndexMap;
        use once_cell::sync::Lazy;
        use serde_json::from_str;
        use crate::data::browser_name::BrowserNameAtom;

        type Feature = AHashMap<BrowserNameAtom, IndexMap<&'static str, u8>>;

        pub(crate) fn _get_feature_stat(name: &str) -> Option<&'static Feature> {
            match name {
                #( #features => {
                    static STAT: Lazy<AHashMap<BrowserNameAtom, IndexMap<&'static str, u8>>> = Lazy::new(|| {
                        from_str::<AHashMap::<u8, IndexMap<&'static str, u8>>>(#files)
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
        format!("{}/caniuse_feature_matching.rs", &out_dir),
        format_token_stream(output),
    )?;

    Ok(())
}

fn parse_caniuse_global() -> Result<Caniuse> {
    Ok(serde_json::from_slice(&fs::read(format!(
        "{}/vendor/caniuse/fulldata-json/data-2.0.json",
        root()
    ))?)?)
}

pub fn build_caniuse_region() -> Result<()> {
    #[derive(Deserialize)]
    struct RegionData {
        data: HashMap<String, HashMap<String, Option<f32>>>,
    }

    let out_path = out_dir();
    let out_dir = out_path.to_string_lossy();

    let files = fs::read_dir(format!("{}/vendor/caniuse/region-usage-json", root()))?
        .map(|entry| entry.map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>>>()?;

    let Caniuse { agents, .. } = parse_caniuse_global()?;

    let region_dir = format!("{}/region", &out_dir);
    if matches!(fs::File::open(&region_dir), Err(e) if e.kind() == io::ErrorKind::NotFound) {
        fs::create_dir(&region_dir)?;
    }

    for file in &files {
        let RegionData { data } = serde_json::from_slice(&fs::read(file.path())?)?;
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
        fs::write(
            format!("{}/region/{}", &out_dir, file.file_name().to_str().unwrap()),
            serde_json::to_string(&usage)?,
        )?;
    }
    let regions = files
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
    let files = regions
        .iter()
        .map(|f| fs::read_to_string(out_path.join("region").join(format!("{f}.json"))).unwrap())
        .collect::<Vec<_>>();
    let output = quote! {
        use once_cell::sync::Lazy;
        use serde_json::from_str;
        use crate::data::browser_name::BrowserNameAtom;

        type RegionData = Vec<(BrowserNameAtom, &'static str, f32)>;

        pub fn get_usage_by_region(region: &str) -> Option<&'static RegionData> {
            match region {
                #( #regions => {
                    static USAGE: Lazy<Vec<(BrowserNameAtom, &'static str, f32)>> = Lazy::new(|| {
                        from_str::<Vec<(u8, &'static str, f32)>>(#files)
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
