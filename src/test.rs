use crate::{resolve, Error, Opts};
use std::{path::Path, process::Command};

pub fn run_compare(query: &str, opts: &Opts, cwd: Option<&Path>) {
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
        .collect::<Vec<_>>();

    let actual = resolve([query], opts)
        .unwrap()
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>();

    assert_eq!(expected, actual);
}

pub fn should_failed(query: &str, opts: &Opts) -> Error {
    resolve([query], opts).unwrap_err()
}
