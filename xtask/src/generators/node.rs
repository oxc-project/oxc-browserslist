use std::fs;

use anyhow::Result;
use bincode::encode_to_vec;
use indexmap::IndexMap;
use quote::quote;
use serde::Deserialize;
use time::OffsetDateTime;

use crate::utils::{generate_file, root, save_bin_compressed};

// Node versions structures
#[derive(Deserialize)]
struct NodeRelease {
    version: String,
}

// Node release schedule structures
#[derive(Deserialize)]
struct NodeScheduleRelease {
    start: String,
    end: String,
}

pub fn build_node_versions() -> Result<()> {
    let releases_path = root().join("node_modules/node-releases/data/processed/envs.json");
    let releases: Vec<NodeRelease> = serde_json::from_slice(&fs::read(releases_path)?)?;

    // Convert releases to a Vec of (u16, u16, u16) tuples for compression
    let versions: Vec<(u16, u16, u16)> = releases
        .into_iter()
        .map(|release| {
            let version = release.version.split('.').collect::<Vec<_>>();
            assert_eq!(version.len(), 3);
            let major: u16 = version[0].parse().unwrap();
            let minor: u16 = version[1].parse().unwrap();
            let patch: u16 = version[2].parse().unwrap();
            (major, minor, patch)
        })
        .collect();

    // Serialize and compress the data
    let serialized = encode_to_vec(&versions, bincode::config::standard())?;
    save_bin_compressed("node_versions.bin", &serialized);

    Ok(())
}

fn parse_date(s: &str) -> i32 {
    let format = time::format_description::well_known::Iso8601::DATE;
    let s = format!("{s}T00:00:00.000000000-00:00");
    OffsetDateTime::parse(&s, &format).unwrap().to_julian_day()
}

pub fn build_node_release_schedule() -> Result<()> {
    let schedule_path =
        root().join("node_modules/node-releases/data/release-schedule/release-schedule.json");
    let schedule: IndexMap<String, NodeScheduleRelease> =
        serde_json::from_slice(&fs::read(schedule_path)?)?;
    let versions = schedule
        .into_iter()
        .map(|(version, NodeScheduleRelease { start, end })| {
            let version = version.trim_start_matches('v');
            let version = version.split('.').collect::<Vec<_>>();
            assert!(version.len() > 0);
            let major: u16 = version[0].parse().unwrap();
            let minor: u16 = version.get(1).map(|v| v.parse().unwrap()).unwrap_or_default();
            let patch: u16 = version.get(2).map(|v| v.parse().unwrap()).unwrap_or_default();
            let start_julian_day = parse_date(&start);
            let end_julian_day = parse_date(&end);
            quote! {
                (Version(#major, #minor, #patch), #start_julian_day, #end_julian_day)
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        use crate::semver::Version;
        pub static RELEASE_SCHEDULE: &[(Version, /* julian day */ i32, /* julian day */ i32)] = &[#(#versions),*];
    };

    generate_file("node_release_schedule.rs", output);

    Ok(())
}
