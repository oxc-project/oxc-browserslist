use browserslist::{Error, Opts, resolve};
use test_case::test_case;

use super::{run_compare, should_failed};

#[test_case("baseline 2020"; "year")]
#[test_case("baseline 2016"; "early year")]
#[test_case("baseline 2015"; "first baseline year")]
#[test_case("baseline 2013"; "pre baseline year selects everything")]
#[test_case("baseline 2030"; "future year selects latest minimums")]
#[test_case("baseline 0090"; "four digit iso year in the past")]
#[test_case("baseline newly available"; "newly")]
#[test_case("baseline widely available"; "widely")]
#[test_case("baseline widely available on 2022-07-01"; "widely on date")]
#[test_case("baseline widely available on 2018-01-29"; "date hitting the first timeline section")]
#[test_case("baseline widely available on 2016-01-01"; "date shifting before baseline")]
#[test_case("baseline widely available on 2024-02-31"; "date day overflow rolls over like js")]
#[test_case("baseline widely available on 2023-02-29"; "non leap year date rolls over like js")]
#[test_case("baseline widely available on 2024-13-01"; "invalid month selects nothing")]
#[test_case("baseline widely available on 2024-00-10"; "zero month selects nothing")]
#[test_case("baseline widely available on 2024-06-00"; "zero day selects nothing")]
#[test_case("baseline 2020 with downstream"; "year with downstream")]
#[test_case("baseline 2020 including kaios"; "year including kaios")]
#[test_case("baseline 2020 with downstream including kaios"; "year with all suffixes")]
#[test_case("baseline widely available with downstream"; "widely with downstream")]
#[test_case("baseline newly available including kaios"; "newly including kaios")]
#[test_case("baseline widely available on 2022-07-01 with downstream including kaios"; "date with all suffixes")]
#[test_case("BASELINE Widely Available"; "case insensitive")]
#[test_case("Baseline 2020 WITH Downstream Including KaiOS"; "case insensitive suffixes")]
#[test_case("baseline    2020"; "extra whitespace")]
#[test_case("baseline widely  available  on  2022-07-01"; "extra whitespace around date")]
#[test_case("baseline 2020 and chrome > 100"; "and composition")]
#[test_case("defaults and not baseline 2020"; "negated composition")]
#[test_case("baseline 2020, baseline newly available"; "or composition")]
fn valid(query: &str) {
    run_compare(query, &Opts::default(), None);
}

#[test_case("baseline 2020"; "year")]
#[test_case("baseline widely available"; "widely")]
#[test_case("baseline 2020 with downstream including kaios"; "with all suffixes")]
fn valid_mobile_to_desktop(query: &str) {
    run_compare(query, &Opts { mobile_to_desktop: true, ..Default::default() }, None);
}

#[test_case(
    "baseline", Error::UnknownQuery(String::from("baseline"));
    "bare baseline"
)]
#[test_case(
    "baseline widely", Error::UnknownQuery(String::from("baseline widely"));
    "missing available"
)]
#[test_case(
    "baseline with downstream", Error::UnknownQuery(String::from("baseline with downstream"));
    "suffix without target"
)]
#[test_case(
    "baseline widely available on 2022-7-1",
    Error::UnknownQuery(String::from("baseline widely available on 2022-7-1"));
    "date must be zero padded"
)]
#[test_case(
    "baseline widely available on 2022-07",
    Error::UnknownQuery(String::from("baseline widely available on 2022-07"));
    "incomplete date"
)]
#[test_case(
    "baseline widely available including kaios with downstream",
    Error::UnknownQuery(String::from("baseline widely available including kaios with downstream"));
    "suffixes in wrong order"
)]
#[test_case(
    "baseline newly available on 2028-01-01", Error::BaselineNewlyWithDate;
    "newly with date"
)]
#[test_case(
    "defaults and not baseline newly available on 2028-01-01", Error::BaselineNewlyWithDate;
    "negated newly with date"
)]
fn invalid(query: &str, error: Error) {
    assert_eq!(should_failed(query, &Opts::default()), error);
}

// ---- Ports of test/baseline.test.js from browserslist (the two cases needing crate
// internals live in src/queries/baseline.rs) ----

fn versions<S: AsRef<str>>(queries: &[S]) -> Vec<String> {
    resolve(queries, &Opts::default()).unwrap().iter().map(ToString::to_string).collect()
}

/// `Selects proper core versions for baseline 2020`
#[test]
fn selects_proper_core_versions_for_baseline_2020() {
    let baseline_2020 = versions(&["baseline 2020 with downstream including kaios"]);
    for version in ["chrome 87", "edge 87", "firefox 83", "safari 14", "ios_saf 14.0-14.4"] {
        assert!(baseline_2020.contains(&version.to_string()), "{version} missing");
    }
}

/// `Selects proper downstream versions for baseline 2020`
#[test]
fn selects_proper_downstream_versions_for_baseline_2020() {
    let baseline_2020 = versions(&["baseline 2020 with downstream including kaios"]);
    for version in [
        "chrome 87",
        "edge 87",
        "firefox 83",
        "safari 14",
        "ios_saf 14.0-14.4",
        "samsung 14.0",
        "opera 73",
        "kaios 3.0-3.1",
    ] {
        assert!(baseline_2020.contains(&version.to_string()), "{version} missing");
    }
}

/// `Adds KaiOS and nothing else when downstream is not requested`
#[test]
fn adds_kaios_and_nothing_else_when_downstream_is_not_requested() {
    let core = versions(&["baseline 2020"]);
    let with_kaios = versions(&["baseline 2020 including kaios"]);
    let extra = with_kaios.iter().filter(|version| !core.contains(version)).collect::<Vec<_>>();
    assert_eq!(extra, vec!["kaios 3.0-3.1"]);
    let missing = core.iter().filter(|version| !with_kaios.contains(version)).collect::<Vec<_>>();
    assert!(missing.is_empty(), "including kaios dropped {missing:?}");
}

/// `Accepts "including kaios" without downstream in every baseline shape`
#[test]
fn accepts_including_kaios_without_downstream_in_every_baseline_shape() {
    for query in [
        "baseline widely available",
        "baseline newly available",
        "baseline 2020",
        "baseline widely available on 2022-07-01",
    ] {
        let core = versions(&[query]);
        let extra = versions(&[format!("{query} including kaios")])
            .into_iter()
            .filter(|version| !core.contains(version))
            .collect::<Vec<_>>();
        assert!(
            extra.iter().all(|version| version.starts_with("kaios ")),
            "{query} added {extra:?}"
        );
    }
}

/// `Throws an error when "newly available on YYYY-MM-DD" is used`
#[test]
fn throws_an_error_when_newly_available_on_date_is_used() {
    assert_eq!(
        should_failed("baseline newly available on 2022-07-01", &Opts::default()),
        Error::BaselineNewlyWithDate
    );
}

/// `Treats "newly available" as case insensitive`
#[test]
fn treats_newly_available_as_case_insensitive() {
    let newly = versions(&["baseline newly available"]);
    assert_eq!(versions(&["BASELINE NEWLY AVAILABLE"]), newly);
    assert_eq!(versions(&["baseline NEWLY available"]), newly);
}
