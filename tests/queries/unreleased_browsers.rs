use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("unreleased versions"; "basic")]
#[test_case("Unreleased Versions"; "case insensitive")]
#[test_case("unreleased        versions"; "more spaces")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
