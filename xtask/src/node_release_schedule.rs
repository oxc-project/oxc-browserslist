use std::fs;

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;
use serde::Deserialize;
use time::OffsetDateTime;

use super::{generate_file, root};

#[derive(Deserialize)]
struct NodeRelease {
    start: String,
    end: String,
}

fn parse_date(s: &str) -> i32 {
    let format = time::format_description::well_known::Iso8601::DATE;
    let s = format!("{s}T00:00:00.000000000-00:00");
    OffsetDateTime::parse(&s, &format).unwrap().to_julian_day()
}

pub fn build_node_release_schedule() -> Result<()> {
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
            let minor: u32 = version.get(1).map(|v| v.parse().unwrap()).unwrap_or_default();
            let patch: u32 = version.get(2).map(|v| v.parse().unwrap()).unwrap_or_default();
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
