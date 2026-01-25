use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("node 4-6"; "semver major only")]
#[test_case("node 4-6.0.0"; "different semver formats")]
#[test_case("node 6.5-7.5"; "with semver minor")]
#[test_case("node 6.6.4-7.7.5"; "with semver patch")]
#[test_case("Node 4   -    6"; "more spaces 1")]
#[test_case("node 6.5    -  7.5"; "more spaces 2")]
#[test_case("node 6.6.4    -    7.7.5"; "more spaces 3")]
#[test_case("node 8.8.8.8-9.9.9.9"; "malformed version")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case(
    "node 6-8.a", Error::Parse(String::from(".a"));
    "malformed version"
)]
fn invalid(query: &str, error: Error) {
    assert_eq!(should_failed(query, &Opts::default()), error);
}
