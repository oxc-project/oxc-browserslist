//! Integration tests for query modules.
//! Each submodule corresponds to a source file in src/queries/.

#![cfg(not(miri))]

use std::{collections::HashSet, path::Path, process::Command};

use browserslist::{Error, Opts, resolve};

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

#[track_caller]
fn should_failed(query: &str, opts: &Opts) -> Error {
    resolve(&[query], opts).unwrap_err()
}

#[path = "queries/browser_accurate.rs"]
mod browser_accurate;
#[path = "queries/browser_bounded_range.rs"]
mod browser_bounded_range;
#[path = "queries/browser_unbounded_range.rs"]
mod browser_unbounded_range;
#[path = "queries/browserslist_config.rs"]
mod browserslist_config;
#[path = "queries/cover.rs"]
mod cover;
#[path = "queries/cover_by_region.rs"]
mod cover_by_region;
#[path = "queries/current_node.rs"]
mod current_node;
#[path = "queries/dead.rs"]
mod dead;
#[path = "queries/defaults.rs"]
mod defaults;
#[path = "queries/electron_accurate.rs"]
mod electron_accurate;
#[path = "queries/electron_bounded_range.rs"]
mod electron_bounded_range;
#[path = "queries/electron_unbounded_range.rs"]
mod electron_unbounded_range;
#[path = "queries/firefox_esr.rs"]
mod firefox_esr;
#[path = "queries/last_n_browsers.rs"]
mod last_n_browsers;
#[path = "queries/last_n_electron.rs"]
mod last_n_electron;
#[path = "queries/last_n_electron_major.rs"]
mod last_n_electron_major;
#[path = "queries/last_n_major_browsers.rs"]
mod last_n_major_browsers;
#[path = "queries/last_n_node.rs"]
mod last_n_node;
#[path = "queries/last_n_node_major.rs"]
mod last_n_node_major;
#[path = "queries/last_n_x_browsers.rs"]
mod last_n_x_browsers;
#[path = "queries/last_n_x_major_browsers.rs"]
mod last_n_x_major_browsers;
#[path = "queries/maintained_node.rs"]
mod maintained_node;
#[path = "queries/node_accurate.rs"]
mod node_accurate;
#[path = "queries/node_bounded_range.rs"]
mod node_bounded_range;
#[path = "queries/node_unbounded_range.rs"]
mod node_unbounded_range;
#[path = "queries/op_mini.rs"]
mod op_mini;
#[path = "queries/percentage.rs"]
mod percentage;
#[path = "queries/percentage_by_region.rs"]
mod percentage_by_region;
#[path = "queries/phantom.rs"]
mod phantom;
#[path = "queries/since.rs"]
mod since;
#[path = "queries/supports.rs"]
mod supports;
#[path = "queries/unreleased_browsers.rs"]
mod unreleased_browsers;
#[path = "queries/unreleased_electron.rs"]
mod unreleased_electron;
#[path = "queries/unreleased_x_browsers.rs"]
mod unreleased_x_browsers;
#[path = "queries/years.rs"]
mod years;
