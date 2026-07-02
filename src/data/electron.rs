use crate::error::Error;
pub use crate::generated::electron_to_chromium::ELECTRON_VERSIONS;

use crate::generated::electron_to_chromium::ELECTRON_CHROMIUM_VERSIONS;

/// Unpack the Electron version from an [`ELECTRON_VERSIONS`] entry
/// (`major << 24 | minor << 16 | pool_offset << 4 | pool_len`).
pub fn unpack_version(packed: u32) -> ElectronVersion {
    ElectronVersion::new((packed >> 24) as u16, ((packed >> 16) & 0xff) as u16)
}

/// Unpack the Chromium version string from an [`ELECTRON_VERSIONS`] entry.
pub fn unpack_chromium(packed: u32) -> &'static str {
    crate::data::unpack_str(ELECTRON_CHROMIUM_VERSIONS, (packed >> 4) & 0xfff, packed & 0xf)
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

    let election_version = ElectronVersion::parse(first, second).map_err(|_| err(version))?;
    Ok(election_version)
}

fn check_number(n: &str) -> bool {
    if n == "0" { true } else { !n.starts_with('0') }
}

fn err(version: &str) -> Error {
    Error::UnknownElectronVersion(version.to_string())
}
