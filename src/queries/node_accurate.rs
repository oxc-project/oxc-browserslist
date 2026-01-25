use super::{Distrib, QueryResult};
use crate::{data::node::NODE_VERSIONS, error::Error, opts::Opts};

pub(super) fn node_accurate(version_str: &str, opts: &Opts) -> QueryResult {
    for v in version_str.split('.') {
        let is_valid = if v == "0" { true } else { !v.starts_with('0') };
        if !is_valid {
            return Err(Error::UnknownNodejsVersion(version_str.to_string()));
        }
    }

    let mut s = version_str.split('.');
    let major = s.next().map(|n| n.parse::<u16>().unwrap_or_default());
    let minor = s.next().map(|n| n.parse::<u16>().unwrap_or_default());
    let patch = s.next().map(|n| n.parse::<u16>().unwrap_or_default());

    let distribs = NODE_VERSIONS()
        .iter()
        .rev()
        .find(|v| {
            if let Some(major) = major {
                let major_eq = major == v.0;
                if let Some(minor) = minor {
                    let minor_eq = minor == v.1;
                    if let Some(patch) = patch {
                        return major_eq && minor_eq && patch == v.2;
                    }
                    return major_eq && minor_eq;
                }
                return major_eq;
            }
            false
        })
        .map(|version| vec![Distrib::new("node", version.to_string())]);
    if opts.ignore_unknown_versions {
        Ok(distribs.unwrap_or_default())
    } else {
        distribs.ok_or_else(|| Error::UnknownNodejsVersion(version_str.to_string()))
    }
}
