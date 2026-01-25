use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("dead"; "basic")]
#[test_case("Dead"; "case insensitive")]
fn default_options(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case("> 0%, dead"; "all browsers")]
fn mobile_to_desktop(query: &str) {
    run_compare(query, &Opts { mobile_to_desktop: true, ..Default::default() }, None);
}

#[test]
fn invalid() {
    assert_eq!(
        should_failed("not dead", &Opts::default()),
        Error::NotAtFirst(String::from("not dead"))
    );
}
