//! Shared test utilities for integration tests.

use std::{collections::HashSet, path::Path, process::Command};

use browserslist::{Opts, resolve};

/// Runs a query through both the Rust implementation and the npm browserslist CLI,
/// then compares the results.
#[expect(clippy::print_stdout)]
#[track_caller]
pub fn run_compare(query: &str, opts: &Opts, cwd: Option<&Path>) {
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
