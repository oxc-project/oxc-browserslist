use super::{Distrib, QueryResult};
use crate::{
    data::electron::{ELECTRON_VERSIONS, parse_version},
    error::Error,
};

pub(super) fn electron_bounded_range(from: &str, to: &str) -> QueryResult {
    let from_str = from;
    let to_str = to;
    let from = parse_version(from)?;
    let to = parse_version(to)?;

    if ELECTRON_VERSIONS.iter().all(|(version, _)| *version != from) {
        return Err(Error::UnknownElectronVersion(from_str.to_string()));
    }
    if ELECTRON_VERSIONS.iter().all(|(version, _)| *version != to) {
        return Err(Error::UnknownElectronVersion(to_str.to_string()));
    }

    let distribs = ELECTRON_VERSIONS
        .iter()
        .filter(|(version, _)| from <= *version && *version <= to)
        .map(|(_, version)| Distrib::new("chrome", *version))
        .collect();
    Ok(distribs)
}

#[cfg(all(test, not(miri)))]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::{
        opts::Opts,
        test::{run_compare, should_failed},
    };

    #[test_case("electron 0.36-1.2"; "basic")]
    #[test_case("Electron 0.37-1.0"; "case insensitive")]
    #[test_case("electron 0.37.5-1.0.3"; "with semver patch version")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case(
        "electron 0.1-1.2", Error::UnknownElectronVersion(String::from("0.1"));
        "unknown version 1"
    )]
    #[test_case(
        "electron 0.37-999.0", Error::UnknownElectronVersion(String::from("999.0"));
        "unknown version 2"
    )]
    #[test_case(
        "electron 1-8.a", Error::Parse(String::from("a"));
        "malformed version 1"
    )]
    #[test_case(
        "electron 1.1.1.1-2", Error::UnknownElectronVersion(String::from("1.1.1.1"));
        "malformed version 2"
    )]
    fn invalid(query: &str, error: Error) {
        assert_eq!(should_failed(query, &Opts::default()), error);
    }
}
