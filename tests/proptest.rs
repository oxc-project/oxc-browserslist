//! Property-based testing with proptest to compare results with npm browserslist.
//!
//! This module generates random browserslist queries and compares the results
//! with the npm browserslist CLI to ensure compatibility.

#![cfg(not(miri))]

use std::{collections::HashSet, process::Command};

use browserslist::{Opts, resolve};
use proptest::prelude::*;

// =============================================================================
// Test utilities
// =============================================================================

#[expect(clippy::print_stdout)]
#[track_caller]
fn run_compare(query: &str, opts: &Opts) {
    #[cfg(target_os = "windows")]
    let bin = "browserslist.exe";
    #[cfg(not(target_os = "windows"))]
    let bin = "browserslist";
    // Use absolute path without canonicalize() to avoid flaky failures on macOS
    // where symlinks created by pnpm may not be immediately visible.
    let path = std::env::current_dir().unwrap().join("node_modules/.bin").join(bin);
    let mut command = Command::new(&path);
    if opts.mobile_to_desktop {
        command.arg("--mobile-to-desktop");
    }
    if opts.ignore_unknown_versions {
        command.arg("--ignore-unknown-versions");
    }
    command.arg(query);
    let output = command.output().unwrap();

    let npm_success = output.status.success();
    let npm_output = String::from_utf8_lossy(&output.stdout);
    let npm_stderr = String::from_utf8_lossy(&output.stderr);

    let rust_result = resolve(&[query], opts);

    match (npm_success, rust_result) {
        (true, Ok(browsers)) => {
            let expected = npm_output
                .trim()
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|s| s.to_string())
                .collect::<HashSet<_>>();
            let actual = browsers.iter().map(ToString::to_string).collect::<HashSet<_>>();
            if expected != actual {
                println!("Query: {:?}", query);
                println!(
                    "actual - expected: {:?}",
                    actual.difference(&expected).collect::<Vec<_>>()
                );
                println!(
                    "expected - actual: {:?}",
                    expected.difference(&actual).collect::<Vec<_>>()
                );
                panic!("Results differ");
            }
        }
        (false, Err(_)) => {
            // Both failed - this is acceptable
        }
        (true, Err(e)) => {
            panic!(
                "npm succeeded but Rust failed for query {:?}\nnpm output: {}\nRust error: {:?}",
                query, npm_output, e
            );
        }
        (false, Ok(browsers)) => {
            panic!(
                "Rust succeeded but npm failed for query {:?}\nnpm stderr: {}\nRust result: {:?}",
                query, npm_stderr, browsers
            );
        }
    }
}

// =============================================================================
// Property-based testing strategies
// =============================================================================

// Browser names for random testing (excluding browsers with special version handling)
// NOTE: ios_saf uses version ranges like "4.0-4.1" which npm handles differently in comparisons.
const BROWSERS: &[&str] = &[
    "ie", "edge", "firefox", "chrome", "safari", "opera", "android", "and_chr", "and_ff", "ie_mob",
    "and_uc", "samsung", "and_qq", "baidu", "kaios",
];

// Browser name aliases
// NOTE: "ios" is excluded because ios_saf uses version ranges like "4.0-4.1" which npm
// handles differently in comparisons (e.g., "ios > 4" includes "ios_saf 4.0-4.1").
const BROWSER_ALIASES: &[&str] = &[
    "fx",
    "ff",
    "explorer",
    "explorermobile",
    "chromeandroid",
    "firefoxandroid",
    "ucandroid",
    "qqandroid",
];

// Caniuse features for "supports" queries
const FEATURES: &[&str] = &[
    "es6-module",
    "flexbox",
    "css-grid",
    "webp",
    "fetch",
    "arrow-functions",
    "promises",
    "async-functions",
    "rtcpeerconnection",
];

// Regions for percentage queries
const REGIONS: &[&str] = &["US", "CN", "DE", "GB", "JP", "FR", "IN", "BR", "RU", "AU"];

fn browser_with_alias_strategy() -> impl Strategy<Value = String> {
    let all_browsers: Vec<&str> = BROWSERS.iter().chain(BROWSER_ALIASES.iter()).copied().collect();
    prop::sample::select(all_browsers).prop_map(|s| s.to_string())
}

// Version strategy - using patterns that avoid npm's quirky version comparison
fn version_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (1u32..100).prop_map(|v| v.to_string()),
        (1u32..100, 0u32..10).prop_map(|(major, minor)| format!("{}.{}", major, minor)),
    ]
}

fn comparator_strategy() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![">", ">=", "<", "<="])
}

fn last_versions_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (1u32..10).prop_map(|n| format!("last {} versions", n)),
        (1u32..10, browser_with_alias_strategy())
            .prop_map(|(n, browser)| format!("last {} {} versions", n, browser)),
        (1u32..5).prop_map(|n| format!("last {} major versions", n)),
        (1u32..5, browser_with_alias_strategy())
            .prop_map(|(n, browser)| format!("last {} {} major versions", n, browser)),
    ]
}

fn browser_accurate_strategy() -> impl Strategy<Value = String> {
    (browser_with_alias_strategy(), version_strategy())
        .prop_map(|(browser, version)| format!("{} {}", browser, version))
}

fn browser_bounded_range_strategy() -> impl Strategy<Value = String> {
    (browser_with_alias_strategy(), version_strategy(), version_strategy())
        .prop_map(|(browser, from, to)| format!("{} {} - {}", browser, from, to))
}

fn browser_unbounded_range_strategy() -> impl Strategy<Value = String> {
    (browser_with_alias_strategy(), comparator_strategy(), version_strategy())
        .prop_map(|(browser, cmp, version)| format!("{} {} {}", browser, cmp, version))
}

fn percentage_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (comparator_strategy(), 0.1f64..10.0).prop_map(|(cmp, pct)| format!("{} {:.1}%", cmp, pct)),
        (comparator_strategy(), 0.1f64..10.0, prop::sample::select(REGIONS.to_vec()))
            .prop_map(|(cmp, pct, region)| format!("{} {:.1}% in {}", cmp, pct, region)),
    ]
}

fn cover_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (0.1f64..50.0).prop_map(|pct| format!("cover {:.1}%", pct)),
        (0.1f64..50.0, prop::sample::select(REGIONS.to_vec()))
            .prop_map(|(pct, region)| format!("cover {:.1}% in {}", pct, region)),
    ]
}

fn since_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (2010i32..2025).prop_map(|year| format!("since {}", year)),
        (2010i32..2025, 1u32..13).prop_map(|(year, month)| format!("since {}-{:02}", year, month)),
        (2010i32..2025, 1u32..13, 1u32..29)
            .prop_map(|(year, month, day)| format!("since {}-{:02}-{:02}", year, month, day)),
    ]
}

fn last_years_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (1u32..10).prop_map(|n| format!("last {} years", n)),
        (1u32..5, 0u32..10).prop_map(|(n, frac)| format!("last {}.{} years", n, frac)),
    ]
}

fn supports_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        prop::sample::select(FEATURES.to_vec()).prop_map(|f| format!("supports {}", f)),
        prop::sample::select(FEATURES.to_vec()).prop_map(|f| format!("fully supports {}", f)),
        prop::sample::select(FEATURES.to_vec()).prop_map(|f| format!("partially supports {}", f)),
    ]
}

fn node_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        version_strategy().prop_map(|v| format!("node {}", v)),
        (comparator_strategy(), version_strategy())
            .prop_map(|(cmp, v)| format!("node {} {}", cmp, v)),
        (version_strategy(), version_strategy())
            .prop_map(|(from, to)| format!("node {} - {}", from, to)),
        Just("maintained node versions".to_string()),
        Just("current node".to_string()),
    ]
}

// Electron strategy using known-good versions
// NOTE: npm has quirky version comparison behavior for electron versions that
// makes it hard to generate arbitrary versions and match npm exactly.
fn electron_strategy() -> impl Strategy<Value = String> {
    let known_versions = vec![
        "0.20", "0.21", "0.22", "0.23", "0.24", "0.25", "0.26", "0.27", "0.28", "0.29", "0.30",
        "0.31", "0.32", "0.33", "0.34", "0.35", "0.36", "0.37", "1.0", "1.1", "1.2", "1.3", "1.4",
        "1.5", "1.6", "1.7", "1.8", "2.0", "2.1", "3.0", "3.1", "4.0", "4.1", "4.2", "5.0", "6.0",
        "6.1", "7.0", "7.1", "7.2", "7.3", "8.0", "8.1", "8.2", "8.3", "8.4", "8.5", "9.0", "10.0",
        "10.1", "11.0", "12.0", "13.0", "14.0", "15.0",
    ];
    prop_oneof![
        prop::sample::select(known_versions.clone()).prop_map(|v| format!("electron {}", v)),
        (comparator_strategy(), prop::sample::select(known_versions.clone()))
            .prop_map(|(cmp, v)| format!("electron {} {}", cmp, v)),
        (prop::sample::select(known_versions.clone()), prop::sample::select(known_versions))
            .prop_map(|(from, to)| format!("electron {} - {}", from, to)),
    ]
}

fn unreleased_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("unreleased versions".to_string()),
        browser_with_alias_strategy().prop_map(|b| format!("unreleased {} versions", b)),
    ]
}

fn special_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "defaults",
        "dead",
        "firefox esr",
        "ff esr",
        "fx esr",
        "operamini all",
        "op_mini all",
        "phantomjs 2.1",
        "phantomjs 1.9",
        "safari tp",
    ])
    .prop_map(|s| s.to_string())
}

fn single_query_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        4 => last_versions_strategy(),
        3 => browser_accurate_strategy(),
        2 => browser_bounded_range_strategy(),
        2 => browser_unbounded_range_strategy(),
        2 => percentage_strategy(),
        1 => cover_strategy(),
        2 => since_strategy(),
        1 => last_years_strategy(),
        2 => supports_strategy(),
        2 => node_strategy(),
        1 => electron_strategy(),
        1 => unreleased_strategy(),
        2 => special_strategy(),
    ]
}

fn maybe_negated_query_strategy() -> impl Strategy<Value = String> {
    (any::<bool>(), single_query_strategy())
        .prop_map(|(negated, query)| if negated { format!("not {}", query) } else { query })
}

fn composed_query_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        single_query_strategy(),
        (single_query_strategy(), single_query_strategy())
            .prop_map(|(a, b)| format!("{}, {}", a, b)),
        (single_query_strategy(), maybe_negated_query_strategy())
            .prop_map(|(a, b)| format!("{} and {}", a, b)),
        (single_query_strategy(), single_query_strategy())
            .prop_map(|(a, b)| format!("{} or {}", a, b)),
    ]
}

// =============================================================================
// Property-based tests
// =============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn proptest_last_versions(query in last_versions_strategy()) {
        run_compare(&query, &Opts::default());
    }

    #[test]
    fn proptest_browser_accurate(query in browser_accurate_strategy()) {
        run_compare(&query, &Opts { ignore_unknown_versions: true, ..Default::default() });
    }

    #[test]
    fn proptest_browser_bounded_range(query in browser_bounded_range_strategy()) {
        run_compare(&query, &Opts { ignore_unknown_versions: true, ..Default::default() });
    }

    #[test]
    fn proptest_browser_unbounded_range(query in browser_unbounded_range_strategy()) {
        run_compare(&query, &Opts { ignore_unknown_versions: true, ..Default::default() });
    }

    #[test]
    fn proptest_percentage(query in percentage_strategy()) {
        run_compare(&query, &Opts::default());
    }

    #[test]
    fn proptest_cover(query in cover_strategy()) {
        run_compare(&query, &Opts::default());
    }

    #[test]
    fn proptest_since(query in since_strategy()) {
        run_compare(&query, &Opts::default());
    }

    #[test]
    fn proptest_last_years(query in last_years_strategy()) {
        run_compare(&query, &Opts::default());
    }

    #[test]
    fn proptest_supports(query in supports_strategy()) {
        run_compare(&query, &Opts::default());
    }

    #[test]
    fn proptest_node(query in node_strategy()) {
        run_compare(&query, &Opts { ignore_unknown_versions: true, ..Default::default() });
    }

    #[test]
    fn proptest_electron(query in electron_strategy()) {
        run_compare(&query, &Opts { ignore_unknown_versions: true, ..Default::default() });
    }

    #[test]
    fn proptest_unreleased(query in unreleased_strategy()) {
        run_compare(&query, &Opts::default());
    }

    #[test]
    fn proptest_special(query in special_strategy()) {
        run_compare(&query, &Opts::default());
    }

    #[test]
    fn proptest_composed(query in composed_query_strategy()) {
        run_compare(&query, &Opts { ignore_unknown_versions: true, ..Default::default() });
    }
}
