use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{browser_version_aliases, get_browser_stat},
    error::Error,
    opts::Opts,
    parser::Comparator,
};

pub(super) fn browser_unbounded_range(
    name: &str,
    comparator: Comparator,
    version: &str,
    opts: &Opts,
) -> QueryResult {
    let (name, stat) = get_browser_stat(name, opts.mobile_to_desktop)
        .ok_or_else(|| Error::BrowserNotFound(name.to_string()))?;
    let version = browser_version_aliases()
        .get(name)
        .and_then(|alias| alias.get(version).copied())
        .unwrap_or(version);
    let Some(version) = parse_js_float(version) else {
        return Ok(Vec::new());
    };

    let distribs = stat
        .version_list
        .iter()
        .filter(|version| version.release_date().is_some())
        .map(|version| version.version())
        .filter(|v| match comparator {
            Comparator::Greater => parse_latest_js_float(v).is_some_and(|v| v > version),
            Comparator::Less => parse_js_float(v).is_some_and(|v| v < version),
            Comparator::GreaterOrEqual => parse_latest_js_float(v).is_some_and(|v| v >= version),
            Comparator::LessOrEqual => parse_js_float(v).is_some_and(|v| v <= version),
        })
        .map(|version| Distrib::new(name, version))
        .collect();
    Ok(distribs)
}

#[inline]
fn parse_latest_js_float(value: &str) -> Option<f64> {
    // Browserslist uses the upper bound of ranges for `>`/`>=` comparisons.
    parse_js_float(value.split_once('-').map_or(value, |(_, latest)| latest))
}

#[inline]
fn parse_js_float(value: &str) -> Option<f64> {
    let bytes = value.as_bytes();
    let mut end = 0;

    if matches!(bytes.first(), Some(b'+') | Some(b'-')) {
        end = 1;
    }

    let mut has_digit = false;
    let mut has_dot = false;
    while let Some(byte) = bytes.get(end) {
        match byte {
            b'0'..=b'9' => {
                has_digit = true;
                end += 1;
            }
            b'.' if !has_dot => {
                has_dot = true;
                end += 1;
            }
            _ => break,
        }
    }

    if !has_digit {
        return None;
    }

    value.get(..end)?.parse().ok()
}
