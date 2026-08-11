use browserslist::{Error, Opts, resolve};
use test_case::test_case;

use crate::run_compare;

fn resolve_strings(query: &str) -> Vec<String> {
    resolve(&[query], &Opts::default())
        .unwrap()
        .into_iter()
        .map(|version| version.to_string())
        .collect()
}

#[test]
fn selects_expected_2020_versions() {
    let versions = resolve_strings("baseline 2020 with downstream including kaios");
    for expected in [
        "chrome 87",
        "edge 87",
        "firefox 83",
        "safari 14",
        "ios_saf 14.0-14.4",
        "samsung 14.0",
        "opera 73",
        "kaios 3.0-3.1",
    ] {
        assert!(versions.iter().any(|version| version == expected), "missing {expected}");
    }
}

#[test]
fn newly_available_on_date_is_not_supported() {
    assert_eq!(
        resolve(&["baseline newly available on 2022-07-01"], &Opts::default()),
        Err(Error::BaselineNewlyAvailableOnDate)
    );
}

#[test_case("baseline newly available"; "newly available")]
#[test_case("baseline widely available"; "widely available")]
#[test_case("BASELINE NEWLY AVAILABLE"; "case insensitive")]
#[test_case("baseline 2014"; "pre baseline year")]
#[test_case("baseline 2020"; "baseline year")]
#[test_case("baseline 00001"; "legacy month-first year")]
#[test_case("baseline 00013"; "invalid legacy month")]
#[test_case("baseline 00032"; "legacy two-digit year")]
#[test_case("baseline 20200"; "five digit year")]
#[test_case("baseline 275760"; "year past the JavaScript date range")]
#[test_case("baseline 4294967296"; "overflowing year")]
#[test_case("baseline widely available on 2022-07-01"; "widely available on date")]
#[test_case("baseline widely available on 2018-01-29"; "first timeline boundary")]
#[test_case("baseline widely available on 2040-01-01"; "date past 32-bit time range")]
#[test_case("baseline 2020 with downstream"; "with downstream")]
#[test_case("baseline 2020 including kaios"; "including kaios")]
#[test_case("baseline 2020 with downstream including kaios"; "with downstream and kaios")]
fn matches_browserslist(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case("baseline widely available"; "widely available")]
#[test_case("baseline 2020 with downstream"; "year with downstream")]
fn matches_browserslist_with_mobile_to_desktop(query: &str) {
    run_compare(query, &Opts { mobile_to_desktop: true, ..Opts::default() }, None);
}
