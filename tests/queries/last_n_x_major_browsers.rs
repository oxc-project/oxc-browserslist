use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("last 2 edge major versions"; "basic")]
#[test_case("last 1 bb major version"; "support pluralization")]
#[test_case("last 3 Chrome major versions"; "case insensitive")]
#[test_case("last 2 android major versions"; "android")]
#[test_case("last 2 bb major versions"; "non-sequential version numbers")]
#[test_case("last 3 bb major versions"; "more versions than have been released")]
fn default_options(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test]
fn mobile_to_desktop() {
    run_compare(
        "last 2 android major versions",
        &Opts { mobile_to_desktop: true, ..Default::default() },
        None,
    );
}
