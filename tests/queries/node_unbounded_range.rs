use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("node <= 5"; "less or equal")]
#[test_case("node < 5"; "less")]
#[test_case("node >= 9"; "greater or equal")]
#[test_case("node > 9"; "greater")]
#[test_case("Node <= 5"; "case insensitive")]
#[test_case("node > 10.12"; "with semver minor")]
#[test_case("node > 10.12.1"; "with semver patch")]
#[test_case("node >= 8.8.8.8"; "malformed version")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case(
    "node < 8.a", Error::Parse(String::from("a"));
    "malformed version"
)]
fn invalid(query: &str, error: Error) {
    assert_eq!(should_failed(query, &Opts::default()), error);
}
