use std::borrow::Cow;

use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{get_browser_stat, normalize_version},
    error::Error,
    opts::Opts,
};

pub(super) fn browser_accurate(name: &str, version: &str, opts: &Opts) -> QueryResult {
    let original_name = name;
    let original_version = version;
    let version = if original_version.eq_ignore_ascii_case("tp") { "TP" } else { version };

    let (name, stat) = get_browser_stat(name, opts.mobile_to_desktop)
        .ok_or_else(|| Error::BrowserNotFound(name.to_string()))?;

    if let Some(version) = normalize_version(
        stat,
        if original_version.eq_ignore_ascii_case("tp") { "TP" } else { version },
    ) {
        Ok(vec![Distrib::new(name, version.into_owned())])
    } else {
        let version = if version.contains('.') {
            Cow::Borrowed(version.trim_end_matches(".0"))
        } else {
            let mut v = version.to_owned();
            v.push_str(".0");
            Cow::Owned(v)
        };
        if let Some(version) = normalize_version(stat, &version) {
            Ok(vec![Distrib::new(name, version.into_owned())])
        } else if opts.ignore_unknown_versions {
            Ok(vec![])
        } else {
            Err(Error::UnknownBrowserVersion(
                original_name.to_string(),
                original_version.to_string(),
            ))
        }
    }
}
