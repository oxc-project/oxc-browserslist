use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("current node"; "basic")]
#[test_case("Current Node"; "case insensitive")]
#[test_case("current      node"; "more spaces")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
