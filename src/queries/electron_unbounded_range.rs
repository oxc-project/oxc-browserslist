use super::{Distrib, QueryResult};
use crate::{
    data::electron::{ELECTRON_VERSIONS, parse_version},
    parser::Comparator,
};

pub(super) fn electron_unbounded_range(comparator: Comparator, version: &str) -> QueryResult {
    let version = parse_version(version)?;

    let distribs = ELECTRON_VERSIONS
        .iter()
        .filter(|(electron_version, _)| match comparator {
            Comparator::Greater => *electron_version > version,
            Comparator::Less => *electron_version < version,
            Comparator::GreaterOrEqual => *electron_version >= version,
            Comparator::LessOrEqual => *electron_version <= version,
        })
        .map(|(_, chromium_version)| Distrib::new("chrome", *chromium_version))
        .collect();
    Ok(distribs)
}

#[cfg(all(test, not(miri)))]
mod tests {
    use test_case::test_case;

    use crate::{
        error::Error,
        opts::Opts,
        test::{run_compare, should_failed},
    };

    #[test_case("electron <= 0.21"; "basic")]
    #[test_case("Electron < 0.21"; "case insensitive")]
    #[test_case("Electron < 0.21.5"; "with semver patch version")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case(
        "electron < 8.a", Error::Parse(String::from("a"));
        "malformed version 1"
    )]
    #[test_case(
        "electron >= 1.1.1.1", Error::UnknownElectronVersion(String::from("1.1.1.1"));
        "malformed version 2"
    )]
    fn invalid(query: &str, error: Error) {
        assert_eq!(should_failed(query, &Opts::default()), error);
    }
}
