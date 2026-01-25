use std::fs;

use anyhow::Result;
use indexmap::IndexMap;
use postcard::to_allocvec;
use quote::quote;
use serde::Deserialize;

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
    let serialized = to_allocvec(&versions)?;
    save_bin_compressed("node_versions.bin", &serialized);

    Ok(())
}

/// Convert a calendar date to Julian Day Number.
#[allow(clippy::cast_possible_truncation)]
const fn date_to_julian_day(year: i32, month: u32, day: u32) -> i32 {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    (day as i32) + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

/// Parse ISO 8601 date string (YYYY-MM-DD) to Julian Day Number.
fn parse_date(s: &str) -> i32 {
    let parts: Vec<&str> = s.split('-').collect();
    assert_eq!(parts.len(), 3, "Invalid date format: {s}");
    let year: i32 = parts[0].parse().unwrap();
    let month: u32 = parts[1].parse().unwrap();
    let day: u32 = parts[2].parse().unwrap();
    date_to_julian_day(year, month, day)
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
