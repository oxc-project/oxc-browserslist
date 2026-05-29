pub mod browser;
pub mod caniuse;
pub mod caniuse_lite;

pub use browser::encode_browser_name;
pub use caniuse::{Agent, Caniuse, Feature, RegionStats, VersionDetail, parse_caniuse_global};
