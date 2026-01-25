use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("defaults", &Opts::default(); "no options")]
#[test_case("Defaults", &Opts::default(); "case insensitive")]
#[test_case("defaults", &Opts { mobile_to_desktop: true, ..Default::default() }; "respect options")]
#[test_case("defaults, ie 6", &Opts::default(); "with other queries")]
fn valid(query: &str, opts: &Opts) {
    run_compare(query, opts, None);
}
