use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
/// Options for controlling the behavior of browserslist.
pub struct Opts {
    /// Use desktop browsers if Can I Use doesn’t have data about this mobile version.
    pub mobile_to_desktop: bool,

    /// If `true`, ignore unknown versions then return empty result;
    /// otherwise, reject with an error.
    pub ignore_unknown_versions: bool,
}
