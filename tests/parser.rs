#![cfg(not(miri))]

use std::{collections::HashSet, path::Path, process::Command};

use browserslist::{Opts, resolve};

#[expect(clippy::print_stdout)]
#[track_caller]
fn run_compare(query: &str, opts: &Opts, cwd: Option<&Path>) {
    #[cfg(target_os = "windows")]
    let path = "./node_modules/.bin/browserslist.exe";
    #[cfg(not(target_os = "windows"))]
    let path = "./node_modules/.bin/browserslist";
    let mut command = Command::new(Path::new(path).canonicalize().unwrap());
    if opts.mobile_to_desktop {
        command.arg("--mobile-to-desktop");
    }
    if opts.ignore_unknown_versions {
        command.arg("--ignore-unknown-versions");
    }
    if let Some(env) = &opts.env {
        command.env("BROWSERSLIST_ENV", env);
    }
    if opts.dangerous_extend {
        command.env("BROWSERSLIST_DANGEROUS_EXTEND", "1");
    }
    command.arg(query);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = String::from_utf8(command.output().unwrap().stdout).unwrap();
    let expected = output
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    let actual =
        resolve(&[query], opts).unwrap().iter().map(ToString::to_string).collect::<HashSet<_>>();

    if expected != actual {
        println!("actual - expected: {:?}", actual.difference(&expected).collect::<Vec<_>>());
        println!("expected - actual: {:?}", expected.difference(&actual).collect::<Vec<_>>());
        panic!();
    }
}

mod valid {
    use super::*;
    use test_case::test_case;

    #[test_case(""; "empty")]
    #[test_case("ie >= 6, ie <= 7"; "comma")]
    #[test_case("ie >= 6 and ie <= 7"; "and")]
    #[test_case("ie < 11 and not ie 7"; "and with not")]
    #[test_case("last 1 Baidu version and not <2%"; "with not and one-version browsers as and query")]
    #[test_case("ie >= 6 or ie <= 7"; "or")]
    #[test_case("ie < 11 or not ie 7"; "or with not")]
    #[test_case("last 2 versions and > 1%"; "swc issue 4871")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod percentage_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("> .5%"; "float with leading dot")]
    #[test_case(">= 0.1%"; "percentage with zero")]
    #[test_case("< 1%"; "less than percentage")]
    #[test_case("<= 5%"; "less or equal percentage")]
    #[test_case("> 1% in US"; "percentage in region")]
    #[test_case("> 1% in alt-AS"; "percentage in alt region")]
    #[test_case("cover 0.5% in US"; "cover in region")]
    #[test_case("cover 0.1% in alt-EU"; "cover in alt region")]
    fn percentage_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod since_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("since 2020"; "since year only")]
    #[test_case("since 2020-06"; "since year month")]
    #[test_case("since 2020-06-15"; "since year month day")]
    fn since_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod years_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("last 1 year"; "singular year")]
    #[test_case("last 2 years"; "plural years")]
    #[test_case("last 1.5 years"; "fractional years")]
    fn years_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod last_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("last 1 version"; "singular version")]
    #[test_case("last 2 versions"; "plural versions")]
    #[test_case("last 1 major version"; "singular major version")]
    #[test_case("last 2 major versions"; "plural major versions")]
    #[test_case("last 1 Chrome version"; "browser singular")]
    #[test_case("last 2 Chrome versions"; "browser plural")]
    #[test_case("last 1 Chrome major version"; "browser major singular")]
    #[test_case("last 2 Chrome major versions"; "browser major plural")]
    fn last_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod unreleased_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("unreleased versions"; "unreleased all")]
    #[test_case("unreleased Chrome versions"; "unreleased browser")]
    #[test_case("unreleased electron versions"; "unreleased electron")]
    fn unreleased_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod supports_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("supports es6-module"; "supports")]
    #[test_case("fully supports es6-module"; "fully supports")]
    #[test_case("partially supports es6-module"; "partially supports")]
    fn supports_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod firefox_esr_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("firefox esr"; "firefox esr")]
    #[test_case("ff esr"; "ff esr")]
    #[test_case("fx esr"; "fx esr")]
    #[test_case("Firefox ESR"; "firefox esr uppercase")]
    fn firefox_esr_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod operamini_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("operamini all"; "operamini all")]
    #[test_case("op_mini all"; "op mini all")]
    fn operamini_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod phantom_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("phantomjs 2.1"; "phantom 2.1")]
    #[test_case("phantomjs 1.9"; "phantom 1.9")]
    fn phantom_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod maintained_node_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("maintained node versions"; "maintained node")]
    fn maintained_node_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod node_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("node >= 10"; "node unbounded")]
    #[test_case("node 10 - 14"; "node bounded")]
    #[test_case("node 18"; "node accurate")]
    fn node_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod electron_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("electron >= 1.0"; "electron unbounded")]
    #[test_case("electron 0.36 - 1.2"; "electron bounded")]
    #[test_case("electron 1.1"; "electron accurate")]
    fn electron_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod defaults_dead_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("defaults"; "defaults")]
    #[test_case("dead"; "dead")]
    #[test_case("Defaults"; "defaults uppercase")]
    #[test_case("Dead"; "dead uppercase")]
    fn defaults_dead_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod browser_tp_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("safari tp"; "safari tp")]
    #[test_case("Safari TP"; "safari tp uppercase")]
    fn browser_tp_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

mod browser_range_edge_cases {
    use super::*;
    use test_case::test_case;

    #[test_case("ie 8 - 10"; "ie range")]
    #[test_case("chrome >= 50"; "chrome unbounded")]
    #[test_case("chrome 90"; "chrome accurate")]
    fn browser_range_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}

#[test]
fn invalid_queries_return_unknown() {
    let result = resolve(&["unknown_query_xyz"], &Opts::default());
    assert!(result.is_err());
}

#[test]
fn parse_composition_with_extra_spaces() {
    run_compare("ie >= 6   and   ie <= 7", &Opts::default(), None);
    run_compare("ie >= 6  ,  ie <= 7", &Opts::default(), None);
    run_compare("ie >= 6   or   ie <= 7", &Opts::default(), None);
}

#[test]
fn case_insensitive_keywords() {
    assert!(resolve(&["LAST 2 VERSIONS"], &Opts::default()).is_ok());
    assert!(resolve(&["DEFAULTS"], &Opts::default()).is_ok());
    assert!(resolve(&["DEAD"], &Opts::default()).is_ok());
    assert!(resolve(&["SUPPORTS es6-module"], &Opts::default()).is_ok());
    assert!(resolve(&["COVER 0.1%"], &Opts::default()).is_ok());
    assert!(resolve(&["SINCE 2020"], &Opts::default()).is_ok());
}
