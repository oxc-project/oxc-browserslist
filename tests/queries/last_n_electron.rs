use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("last 2 electron versions"; "basic")]
#[test_case("last 2 Electron versions"; "case insensitive")]
#[test_case("last 2 electron version"; "support pluralization")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
