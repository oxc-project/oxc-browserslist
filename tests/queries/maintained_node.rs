use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("maintained node versions"; "basic")]
#[test_case("Maintained Node Versions"; "case insensitive")]
#[test_case("maintained   node     versions"; "more spaces")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
