use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("supports objectrtc"; "case 1")]
#[test_case("supports    rtcpeerconnection"; "case 2")]
#[test_case("supports        arrow-functions"; "case 3")]
#[test_case("partially supports rtcpeerconnection"; "partially")]
#[test_case("fully     supports rtcpeerconnection"; "fully")]
fn default_options(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case("supports filesystem"; "case 1")]
#[test_case("supports  font-smooth"; "case 2")]
fn mobile_to_desktop(query: &str) {
    run_compare(query, &Opts { mobile_to_desktop: true, ..Default::default() }, None);
}

#[test]
fn invalid() {
    assert_eq!(
        should_failed("supports xxxyyyzzz", &Opts::default()),
        Error::UnknownBrowserFeature(String::from("xxxyyyzzz"))
    );
}
