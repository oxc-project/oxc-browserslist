use std::sync::OnceLock;

use crate::semver::Version;

use super::caniuse::compression::{decompress_blob, read_varint, read_zigzag};

struct NodeData {
    versions: Vec<(Version, Box<str>)>,
    schedule: Vec<(Version, i32, i32)>,
}

/// Hand-decode the node blob (see `build_node` in xtask for the layout): the release list as
/// per-component zigzag deltas, then the release schedule with its Julian-day windows.
fn node_data() -> &'static NodeData {
    static NODE_DATA: OnceLock<NodeData> = OnceLock::new();
    NODE_DATA.get_or_init(|| {
        let data = decompress_blob(include_bytes!("../generated/node.bin.deflate"));
        let data = data.as_slice();
        assert_eq!(data[0], 1, "unsupported node data format");
        let mut pos = 1;

        let version_count = read_varint(data, &mut pos);
        let mut versions = Vec::with_capacity(version_count);
        let (mut major, mut minor, mut patch) = (0i64, 0i64, 0i64);
        for _ in 0..version_count {
            major += read_zigzag(data, &mut pos);
            minor += read_zigzag(data, &mut pos);
            patch += read_zigzag(data, &mut pos);
            let version = Version(major as u16, minor as u16, patch as u16);
            // The string is built once here so node queries can hand out a borrowed
            // `&'static str` instead of allocating a fresh `String` per result per `resolve`.
            versions.push((version, version.to_string().into_boxed_str()));
        }

        let schedule_count = read_varint(data, &mut pos);
        let mut schedule = Vec::with_capacity(schedule_count);
        let mut previous_start = 0i64;
        for _ in 0..schedule_count {
            let major = read_varint(data, &mut pos) as u16;
            let minor = read_varint(data, &mut pos) as u16;
            let patch = read_varint(data, &mut pos) as u16;
            previous_start += read_zigzag(data, &mut pos);
            let end = previous_start + read_varint(data, &mut pos) as i64;
            schedule.push((Version(major, minor, patch), previous_start as i32, end as i32));
        }
        assert_eq!(pos, data.len(), "trailing node data");

        NodeData { versions, schedule }
    })
}

/// Node.js versions, each paired with its formatted `major.minor.patch` string.
#[allow(non_snake_case)]
pub fn NODE_VERSIONS() -> &'static [(Version, Box<str>)] {
    &node_data().versions
}

/// The Node.js release schedule: `(version, start, end)` with Julian-day windows.
pub fn release_schedule() -> &'static [(Version, i32, i32)] {
    &node_data().schedule
}
