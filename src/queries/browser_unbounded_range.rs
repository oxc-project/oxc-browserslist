use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{browser_version_aliases, get_browser_stat},
    error::Error,
    opts::Opts,
    parser::Comparator,
    semver::Version,
};

pub(super) fn browser_unbounded_range(
    name: &str,
    comparator: Comparator,
    version: &str,
    opts: &Opts,
) -> QueryResult {
    let (name, stat) = get_browser_stat(name, opts.mobile_to_desktop)
        .ok_or_else(|| Error::BrowserNotFound(name.to_string()))?;
    let version: Version = browser_version_aliases()
        .get(name)
        .and_then(|alias| alias.get(version).copied())
        .unwrap_or(version)
        .parse()
        .unwrap_or_default();

    let distribs = stat
        .version_list
        .iter()
        .filter(|version| version.release_date().is_some())
        .map(|version| version.version())
        .filter(|v| {
            let v: Version = v.parse().unwrap_or_default();
            match comparator {
                Comparator::Greater => v > version,
                Comparator::Less => v < version,
                Comparator::GreaterOrEqual => v >= version,
                Comparator::LessOrEqual => v <= version,
            }
        })
        .map(|version| Distrib::new(name, version))
        .collect();
    Ok(distribs)
}
