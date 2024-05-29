use crate::error::Error;
use nom::{
    character::complete::{char, u16},
    combinator::{all_consuming, opt},
    number::complete::float,
    sequence::{pair, terminated},
};

pub use crate::generated::electron_to_chromium::ELECTRON_VERSIONS;

pub fn parse_version(version: &str) -> Result<f32, Error> {
    all_consuming(terminated(float, opt(pair(char('.'), u16))))(version)
        .map(|(_, v)| v)
        .map_err(|_: nom::Err<nom::error::Error<_>>| {
            Error::UnknownElectronVersion(version.to_string())
        })
}
