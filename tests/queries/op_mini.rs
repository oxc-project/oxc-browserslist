use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("op_mini all"; "short")]
#[test_case("Op_Mini All"; "short case insensitive")]
#[test_case("operamini all"; "long")]
#[test_case("OperaMini All"; "long case insensitive")]
#[test_case("op_mini    all"; "more spaces")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
