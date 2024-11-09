use std::fs;

use anyhow::Result;
use indexmap::IndexMap;
use quote::quote;
use rkyv::Archive as RkyvArchive;
use rkyv::Serialize as RkyvSerialize;
use serde::Deserialize;
use time::OffsetDateTime;

use super::{generate_file, generate_rkyv, root};

#[derive(Deserialize)]
struct NodeRelease {
    start: String,
    end: String,
}

#[derive(RkyvSerialize, RkyvArchive)]
pub struct Version(u32, u32, u32);

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
            (Version(major, minor, patch), start_julian_day, end_julian_day)
        })
        .collect::<Vec<_>>();

    let output = quote! {
        use crate::semver::ArchivedVersion;

        use rkyv::vec::ArchivedVec;
        use std::sync::OnceLock;

        type ArchivedData = ArchivedVec<(ArchivedVersion, i32, i32)>;

        const RKYV_BYTES: &'static [u8] = {
            #[repr(C)]
            struct Aligned<T: ?Sized> {
                _align: [usize; 0],
                bytes: T,
            }
            const ALIGNED: &'static Aligned<[u8]> =
                &Aligned { _align: [], bytes: *include_bytes!("node_release_schedule.rkyv") };
            &ALIGNED.bytes
        };

        pub fn get_release_schedule() -> &'static ArchivedData {
            static RELEASE_SCHEDULE: OnceLock<&ArchivedData> = OnceLock::new();
            RELEASE_SCHEDULE.get_or_init(|| {
                #[allow(unsafe_code)]
                unsafe { rkyv::access_unchecked::<ArchivedData>(RKYV_BYTES) }
            })
        }
    };

    generate_rkyv("node_release_schedule.rkyv", &versions);
    generate_file("node_release_schedule.rs", output);

    Ok(())
}
