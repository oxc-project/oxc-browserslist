use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("last 2 electron major versions"; "basic")]
#[test_case("last 2 Electron major versions"; "case insensitive")]
#[test_case("last 2 electron major version"; "support pluralization")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
