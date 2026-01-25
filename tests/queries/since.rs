use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("since 2017"; "year only")]
#[test_case("Since 2017"; "case insensitive")]
#[test_case("since 2017-02"; "with month")]
#[test_case("since 2017-02-15"; "with day")]
#[test_case("since 1970"; "unix timestamp zero")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case(
    "since 21082138", Error::InvalidDate(String::from("21082138-1-1"));
    "year overflow"
)]
fn invalid(query: &str, error: Error) {
    assert_eq!(should_failed(query, &Opts::default()), error);
}
