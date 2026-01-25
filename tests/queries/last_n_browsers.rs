use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("last 2 versions"; "basic")]
#[test_case("last 31 versions"; "android")]
#[test_case("last 1 version"; "support pluralization")]
#[test_case("Last 02 Versions"; "case insensitive")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
