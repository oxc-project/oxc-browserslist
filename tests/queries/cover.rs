use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("cover 0.1%"; "global")]
#[test_case("Cover 0.1%"; "global case insensitive")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
