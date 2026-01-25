use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("ie 8-10"; "basic")]
#[test_case("ie 8   -  10"; "more spaces")]
#[test_case("ie 1-12"; "out of range")]
#[test_case("android 4.3-37"; "android")]
fn default_options(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case("and_chr 52-53"; "chrome")]
#[test_case("android 4.4-38"; "android")]
fn mobile_to_desktop(query: &str) {
    run_compare(query, &Opts { mobile_to_desktop: true, ..Default::default() }, None);
}

#[test_case(
    "unknown 4-7", Error::BrowserNotFound(String::from("unknown"));
    "unknown browser"
)]
fn invalid(query: &str, error: Error) {
    assert_eq!(should_failed(query, &Opts::default()), error);
}
