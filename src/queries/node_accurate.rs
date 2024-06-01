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
    let major = s.next().map(|n| n.parse::<u32>().unwrap_or_default());
    let minor = s.next().map(|n| n.parse::<u32>().unwrap_or_default());
    let patch = s.next().map(|n| n.parse::<u32>().unwrap_or_default());

    let distribs = NODE_VERSIONS
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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::test::{run_compare, should_failed};

    #[test_case("node 7.5.0"; "basic")]
    #[test_case("Node 7.5.0"; "case insensitive")]
    #[test_case("node 5.1"; "without semver patch")]
    #[test_case("node 5"; "semver major only")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case(
        "node 3", Error::UnknownNodejsVersion(String::from("3"));
        "unknown version"
    )]
    #[test_case(
        "node 8.a", Error::Parse(String::from("a"));
        "malformed version 1"
    )]
    #[test_case(
        "node 8.8.8.8", Error::UnknownNodejsVersion(String::from("8.8.8.8"));
        "malformed version 2"
    )]
    #[test_case(
        "node 8.01", Error::UnknownNodejsVersion(String::from("8.01"));
        "malformed version 3"
    )]
    fn invalid(query: &str, error: Error) {
        assert_eq!(should_failed(query, &Opts::default()), error);
    }

    #[test]
    fn ignore_unknown_versions() {
        run_compare("node 3", &Opts { ignore_unknown_versions: true, ..Default::default() }, None);
    }
}
