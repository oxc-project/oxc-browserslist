pub mod browser;
pub mod caniuse;

pub use browser::encode_browser_name;
pub use caniuse::{Agent, Caniuse, Feature, VersionDetail, parse_caniuse_global};
