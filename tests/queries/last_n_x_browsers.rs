use super::run_compare;
use browserslist::Opts;
use test_case::test_case;

#[test_case("last 2 ie versions"; "basic")]
#[test_case("last 2 safari versions"; "do not include unreleased versions")]
#[test_case("last 1 ie version"; "support pluralization")]
#[test_case("last 01 Explorer version"; "alias")]
#[test_case("Last 01 IE Version"; "case insensitive")]
#[test_case("last 4 android versions"; "android 1")]
#[test_case("last 5 android versions"; "android 2")]
#[test_case("last 31 android versions"; "android 3")]
#[test_case("last 4 op_mob versions"; "op_mob 1")]
#[test_case("last 5 op_mob versions"; "op_mob 2")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}
