use std::sync::OnceLock;

use crate::error::Error;

use super::caniuse::compression::{decompress_blob, read_varint};

/// Electron (major, minor) -> Chromium version, ascending. Each u32 packs
/// `major << 24 | minor << 16 | canonical version-table index`; decoded once from a delta
/// varint stream (see `build_electron_to_chromium` in xtask for the layout).
pub fn electron_versions() -> &'static [u32] {
    static ELECTRON_VERSIONS: OnceLock<Vec<u32>> = OnceLock::new();
    ELECTRON_VERSIONS.get_or_init(|| {
        let data = decompress_blob(include_bytes!("../generated/electron.bin.deflate"));
        let data = data.as_slice();
        assert_eq!(data[0], 1, "unsupported electron data format");
        let mut pos = 1;
        let count = read_varint(data, &mut pos);
        let mut versions = Vec::with_capacity(count);
        let mut previous = 0u32;
        for _ in 0..count {
            previous += read_varint(data, &mut pos) as u32;
            versions.push(previous);
        }
        assert_eq!(pos, data.len(), "trailing electron data");
        versions
    })
}

/// Unpack the Electron version from an [`electron_versions`] entry.
pub fn unpack_version(packed: u32) -> ElectronVersion {
    ElectronVersion::new((packed >> 24) as u16, ((packed >> 16) & 0xff) as u16)
}

/// Unpack the Chromium version string from an [`electron_versions`] entry.
pub fn unpack_chromium(packed: u32) -> &'static str {
    super::caniuse::version_table()[(packed & 0xffff) as usize].as_str()
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ElectronVersion {
    pub major: u16,
    pub minor: u16,
}

impl ElectronVersion {
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub fn parse(major: &str, minor: &str) -> Result<Self, std::num::ParseIntError> {
        let major = major.parse()?;
        let minor = minor.parse()?;
        Ok(Self { major, minor })
    }
}

pub fn parse_version(version: &str) -> Result<ElectronVersion, Error> {
    let mut split = version.split('.');

    let Some(first) = split.next() else {
        return Err(err(version));
    };

    let Some(second) = split.next().filter(|n| check_number(n)) else {
        return Err(err(version));
    };

    if split.next().is_some() && split.next().is_some() {
        return Err(err(version));
    }

    ElectronVersion::parse(first, second).map_err(|_| err(version))
}

fn check_number(n: &str) -> bool {
    n == "0" || !n.starts_with('0')
}

fn err(version: &str) -> Error {
    Error::UnknownElectronVersion(version.to_string())
}
