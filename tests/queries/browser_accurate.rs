use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("ie 10"; "by name")]
#[test_case("IE 10"; "case insensitive")]
#[test_case("Explorer 10"; "alias")]
#[test_case("ios 7.0"; "work with joined versions 1")]
#[test_case("ios 7.1"; "work with joined versions 2")]
#[test_case("ios 7"; "allow missing zero 1")]
#[test_case("ios 8.0"; "allow missing zero 2")]
#[test_case("safari tp"; "safari tp")]
#[test_case("Safari TP"; "safari tp case insensitive")]
#[test_case("and_uc 10"; "cut version")]
#[test_case("chromeandroid 53"; "missing mobile versions 1")]
#[test_case("and_ff 60"; "missing mobile versions 2")]
fn default_options(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case("chromeandroid 53"; "chrome 1")]
#[test_case("and_ff 60"; "firefox")]
#[test_case("ie_mob 9"; "ie mobile")]
fn mobile_to_desktop(query: &str) {
    run_compare(query, &Opts { mobile_to_desktop: true, ..Default::default() }, None);
}

#[test]
fn ignore_unknown_versions() {
    run_compare("IE 1, IE 9", &Opts { ignore_unknown_versions: true, ..Default::default() }, None);
}

#[test_case(
    "unknown 10", Error::BrowserNotFound(String::from("unknown"));
    "unknown browser"
)]
#[test_case(
    "IE 1", Error::UnknownBrowserVersion(String::from("IE"), String::from("1"));
    "unknown version"
)]
#[test_case(
    "chrome 70, ie 11, safari 12.2, safari 12",
    Error::UnknownBrowserVersion(String::from("safari"), String::from("12.2"));
    "use correct browser name in error"
)]
#[test_case(
    "ie_mob 9", Error::UnknownBrowserVersion(String::from("ie_mob"), String::from("9"));
    "missing mobile versions 1"
)]
#[test_case(
    "op_mob 30", Error::UnknownBrowserVersion(String::from("op_mob"), String::from("30"));
    "missing mobile versions 2"
)]
fn invalid(query: &str, error: Error) {
    assert_eq!(should_failed(query, &Opts::default()), error);
}
