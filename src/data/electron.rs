use crate::error::Error;
pub use crate::generated::electron_to_chromium::get_electron_versions;

use rkyv::Archive as RkyvArchive;
use rkyv::Deserialize as RkyvDeserialize;

#[derive(
    RkyvArchive, RkyvDeserialize, Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd,
)]
#[rkyv(compare(PartialEq, PartialOrd), derive(Default, PartialEq, PartialOrd, Clone))]
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
    if n == "0" {
        true
    } else {
        !n.starts_with('0')
    }
}

fn err(version: &str) -> Error {
    Error::UnknownElectronVersion(version.to_string())
}
