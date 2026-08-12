use super::{Distrib, QueryResult};
use crate::{data::node::node_versions, error::Error, opts::Opts};

pub(super) fn node_accurate(version_str: &str, opts: &Opts) -> QueryResult {
    // Reject leading zeros (e.g. "08"), like the JavaScript implementation.
    if version_str.split('.').any(|n| n != "0" && n.starts_with('0')) {
        return Err(Error::UnknownNodejsVersion(version_str.to_string()));
    }

    let mut s = version_str.split('.');
    let major = s.next().map(|n| n.parse::<u16>().unwrap_or_default());
    let minor = s.next().map(|n| n.parse::<u16>().unwrap_or_default());
    let patch = s.next().map(|n| n.parse::<u16>().unwrap_or_default());

    let distribs = node_versions()
        .iter()
        .rev()
        .find(|(v, _)| {
            major.is_some_and(|major| major == v.0)
                && minor.is_none_or(|minor| minor == v.1)
                && patch.is_none_or(|patch| patch == v.2)
        })
        .map(|(_, text)| vec![Distrib::new("node", text.as_ref())]);
    if opts.ignore_unknown_versions {
        Ok(distribs.unwrap_or_default())
    } else {
        distribs.ok_or_else(|| Error::UnknownNodejsVersion(version_str.to_string()))
    }
}
