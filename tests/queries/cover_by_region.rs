use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("cover 0.1% in US"; "country")]
#[test_case("Cover 0.1% in us"; "country case insensitive")]
#[test_case("cover 0.1% in alt-eu"; "country alt")]
#[test_case("Cover 0.1% in Alt-EU"; "country alt case insensitive")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
