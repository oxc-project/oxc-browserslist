use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

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
    "electron 1-8.a", Error::Parse(String::from(".a"));
    "malformed version 1"
)]
#[test_case(
    "electron 1.1.1.1-2", Error::UnknownElectronVersion(String::from("1.1.1.1"));
    "malformed version 2"
)]
fn invalid(query: &str, error: Error) {
    assert_eq!(should_failed(query, &Opts::default()), error);
}
