use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("ie > 9"; "greater")]
#[test_case("ie >= 10"; "greater or equal")]
#[test_case("ie < 10"; "less")]
#[test_case("ie <= 9"; "less or equal")]
#[test_case("Explorer > 10"; "case insensitive")]
#[test_case("android >= 4.2"; "android 1")]
#[test_case("android >= 4.3"; "android 2")]
#[test_case("ie<=9"; "no spaces")]
#[test_case("and_qq > 0"; "browser with one version")]
fn default_options(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case("chromeandroid >= 52 and chromeandroid < 54"; "chrome")]
fn mobile_to_desktop(query: &str) {
    run_compare(query, &Opts { mobile_to_desktop: true, ..Default::default() }, None);
}

#[test_case(
    "unknown > 10", Error::BrowserNotFound(String::from("unknown"));
    "unknown browser"
)]
fn invalid(query: &str, error: Error) {
    assert_eq!(should_failed(query, &Opts::default()), error);
}
