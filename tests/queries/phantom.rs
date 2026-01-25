use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("phantomjs 2.1"; "2.1")]
#[test_case("PhantomJS 2.1"; "2.1 case insensitive")]
#[test_case("phantomjs 1.9"; "1.9")]
#[test_case("PhantomJS 1.9"; "1.9 case insensitive")]
#[test_case("phantomjs    2.1"; "more spaces")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
