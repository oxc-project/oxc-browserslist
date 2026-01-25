use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("> 10%"; "greater")]
#[test_case(">= 5%"; "greater or equal")]
#[test_case("< 5%"; "less")]
#[test_case("<= 5%"; "less or equal")]
#[test_case(">10%"; "no space")]
#[test_case("> 10.2%"; "with float")]
#[test_case("> .2%"; "with float that has a leading dot")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
