use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("browserslist config"; "basic")]
#[test_case("Browserslist Config"; "case insensitive")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
