use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("unreleased edge versions"; "basic")]
#[test_case("Unreleased Chrome Versions"; "case insensitive")]
#[test_case("unreleased firefox version"; "support pluralization")]
#[test_case("unreleased    safari     versions"; "more spaces")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
