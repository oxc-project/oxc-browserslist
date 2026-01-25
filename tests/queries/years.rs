use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("last 2 years"; "basic")]
#[test_case("last 1 year"; "one year")]
#[test_case("last 1.4 years"; "year fraction")]
#[test_case("Last 5 Years"; "case insensitive")]
#[test_case("last    2     years"; "more spaces")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
