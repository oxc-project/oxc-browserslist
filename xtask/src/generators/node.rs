use std::fs;

use anyhow::{Result, ensure};
use indexmap::IndexMap;
use serde::Deserialize;

use crate::utils::{push_varint, root, save_bin_compressed, zigzag};

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

const FORMAT_VERSION: u8 = 1;

/// Convert a calendar date to Julian Day Number.
#[allow(clippy::cast_possible_truncation)]
const fn date_to_julian_day(year: i32, month: u32, day: u32) -> i32 {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    (day as i32) + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

/// Parse ISO 8601 date string (YYYY-MM-DD) to Julian Day Number.
fn parse_date(s: &str) -> Result<i32> {
    let parts: Vec<&str> = s.split('-').collect();
    ensure!(parts.len() == 3, "invalid date format: {s}");
    let year: i32 = parts[0].parse()?;
    let month: u32 = parts[1].parse()?;
    let day: u32 = parts[2].parse()?;
    Ok(date_to_julian_day(year, month, day))
}

/// Both node-releases datasets in one compressed varint stream, replacing a postcard triple
/// blob and an inline release-schedule table. Consecutive node versions differ by one patch or
/// minor step, so per-component zigzag deltas are almost entirely `(0, 0, 1)`-shaped and deflate
/// crushes them (the absolute postcard triples were ~6x larger compressed).
///
/// Layout: `u8 FORMAT_VERSION`;
/// varint version count, then per version three varints — zigzag deltas of (major, minor, patch)
/// against the previous entry (first entry absolute), preserving envs.json order (the `last N
/// node versions` queries walk it back-to-front);
/// varint schedule count, then per entry five varints — major, minor, patch, zigzag delta of the
/// start Julian day against the previous entry's start (first absolute), and `end - start`
/// (non-negative — a schedule window never ends before it starts).
pub fn build_node() -> Result<()> {
    let mut bytes = vec![FORMAT_VERSION];

    let releases_path = root().join("node_modules/node-releases/data/processed/envs.json");
    let releases: Vec<NodeRelease> = serde_json::from_slice(&fs::read(releases_path)?)?;
    push_varint(&mut bytes, releases.len() as u64);
    let mut previous = [0i64; 3];
    for release in &releases {
        let version = release.version.split('.').collect::<Vec<_>>();
        ensure!(version.len() == 3, "invalid node version: {}", release.version);
        for (component, previous) in version.iter().zip(&mut previous) {
            let component: i64 = i64::from(component.parse::<u16>()?);
            push_varint(&mut bytes, zigzag(component - *previous));
            *previous = component;
        }
    }

    let schedule_path =
        root().join("node_modules/node-releases/data/release-schedule/release-schedule.json");
    let schedule: IndexMap<String, NodeScheduleRelease> =
        serde_json::from_slice(&fs::read(schedule_path)?)?;
    push_varint(&mut bytes, schedule.len() as u64);
    let mut previous_start = 0i64;
    for (version, NodeScheduleRelease { start, end }) in &schedule {
        let version = version.trim_start_matches('v');
        let version = version.split('.').collect::<Vec<_>>();
        ensure!(!version.is_empty(), "empty node schedule version");
        let major: u16 = version[0].parse()?;
        let minor: u16 = version.get(1).map(|v| v.parse()).transpose()?.unwrap_or_default();
        let patch: u16 = version.get(2).map(|v| v.parse()).transpose()?.unwrap_or_default();
        let start = i64::from(parse_date(start)?);
        let end = i64::from(parse_date(end)?);
        ensure!(end >= start, "node schedule window ends before it starts");
        push_varint(&mut bytes, u64::from(major));
        push_varint(&mut bytes, u64::from(minor));
        push_varint(&mut bytes, u64::from(patch));
        push_varint(&mut bytes, zigzag(start - previous_start));
        push_varint(&mut bytes, (end - start) as u64);
        previous_start = start;
    }

    save_bin_compressed("node.bin", &bytes);
    Ok(())
}
