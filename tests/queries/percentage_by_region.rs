use super::{run_compare, should_failed};
use browserslist::{Error, Opts};
use test_case::test_case;

#[test_case("> 10% in US"; "greater")]
#[test_case(">= 5% in US"; "greater or equal")]
#[test_case("< 5% in US"; "less")]
#[test_case("<= 5% in US"; "less or equal")]
#[test_case("> 10.2% in US"; "with float")]
#[test_case("> .2% in US"; "with float that has a leading dot")]
#[test_case("> 10.2% in us"; "fixes country case")]
#[test_case("> 1% in RU"; "load country")]
#[test_case("> 1% in alt-AS"; "load continents")]
#[test_case(">10% in US"; "no space")]
#[test_case("> 1% in CN"; "normalize incorrect caniuse versions for and-prefixed")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test]
fn invalid() {
    assert_eq!(
        should_failed("> 1% in XX", &Opts::default()),
        Error::UnknownRegion(String::from("XX"))
    );
}
