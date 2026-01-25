use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

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
    "node 8.a", Error::Parse(String::from(".a"));
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
