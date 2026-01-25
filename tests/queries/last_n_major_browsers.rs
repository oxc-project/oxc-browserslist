use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("last 2 major versions"; "basic")]
#[test_case("last 1 major version"; "support pluralization")]
#[test_case("Last 01 MaJoR Version"; "case insensitive")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
