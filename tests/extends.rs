//! Integration tests for the `extends` query.

#![cfg(not(miri))]
#![allow(unsafe_code)]

mod common;

use std::{env, fs, path::PathBuf, sync::Mutex};

use browserslist::{Error, Opts, resolve};
use serde_json::json;

use common::run_compare;

/// Mutex to serialize tests that use the shared temp directory.
static TEST_MUTEX: Mutex<()> = Mutex::new(());

fn base_test_dir() -> PathBuf {
    let dir = env::temp_dir().join("browserslist-test-pkgs");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn mock(name: &str, value: serde_json::Value) {
    let dir = base_test_dir().join("node_modules").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("index.js"),
        format!("module.exports = {}", serde_json::to_string(&value).unwrap()),
    )
    .unwrap();
}

fn clean(name: &str) {
    let _ = fs::remove_dir_all(base_test_dir().join("node_modules").join(name));
}

/// Run a test with the BROWSERSLIST_TEST_DIR environment variable set.
fn with_test_dir<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // SAFETY: Tests are serialized via TEST_MUTEX, so no concurrent access to env vars.
    unsafe {
        env::set_var("BROWSERSLIST_TEST_DIR", base_test_dir());
    }
    let result = f();
    unsafe {
        env::remove_var("BROWSERSLIST_TEST_DIR");
    }
    result
}

fn should_fail(query: &str, opts: &Opts) -> Error {
    resolve(&[query], opts).unwrap_err()
}

mod valid {
    use super::*;
    use test_case::test_case;

    #[test_case("browserslist-config-test", json!(["ie 11"]), "extends browserslist-config-test"; "package")]
    #[test_case("browserslist-config-test-file/ie", json!(["ie 11"]), "extends browserslist-config-test-file/ie"; "file in package")]
    #[test_case("@scope/browserslist-config-test", json!(["ie 11"]), "extends @scope/browserslist-config-test"; "scoped package")]
    #[test_case("@example.com/browserslist-config-test", json!(["ie 11"]), "extends @example.com/browserslist-config-test"; "scoped package with dot in name")]
    #[test_case("@scope/browserslist-config-test-file/ie", json!(["ie 11"]), "extends @scope/browserslist-config-test-file/ie"; "file in scoped package")]
    #[test_case("@scope/browserslist-config", json!(["ie 11"]), "extends @scope/browserslist-config"; "file-less scoped package")]
    #[test_case("browserslist-config-rel", json!(["ie 9-10"]), "extends browserslist-config-rel and not ie 9"; "with override")]
    #[test_case("browserslist-config-with-env-a", json!({ "someEnv": ["ie 10"] }), "extends browserslist-config-with-env-a"; "no default env")]
    #[test_case("browserslist-config-with-defaults", json!({ "defaults": ["ie 10"] }), "extends browserslist-config-with-defaults"; "default env")]
    fn valid(pkg: &str, value: serde_json::Value, query: &str) {
        let _lock = TEST_MUTEX.lock().unwrap();
        mock(pkg, value);
        with_test_dir(|| {
            run_compare(query, &Opts::default(), Some(&base_test_dir()));
        });
        clean(pkg);
    }
}

#[test]
fn dangerous_extend() {
    let _lock = TEST_MUTEX.lock().unwrap();
    mock("pkg", json!(["ie 11"]));
    with_test_dir(|| {
        run_compare(
            "extends pkg",
            &Opts { dangerous_extend: true, ..Default::default() },
            Some(&base_test_dir()),
        );
    });
    clean("pkg");
}

#[test]
fn recursively_import() {
    let _lock = TEST_MUTEX.lock().unwrap();
    mock("browserslist-config-a", json!(["extends browserslist-config-b", "ie 9"]));
    mock("browserslist-config-b", json!(["ie 10"]));
    with_test_dir(|| {
        run_compare("extends browserslist-config-a", &Opts::default(), Some(&base_test_dir()));
    });
    clean("browserslist-config-a");
    clean("browserslist-config-b");
}

#[test]
fn specific_env() {
    let _lock = TEST_MUTEX.lock().unwrap();
    mock("browserslist-config-with-env-b", json!(["ie 11"]));
    with_test_dir(|| {
        run_compare(
            "extends browserslist-config-with-env-b",
            &Opts { env: Some("someEnv".into()), ..Default::default() },
            Some(&base_test_dir()),
        );
    });
    clean("browserslist-config-with-env-b");
}

mod invalid {
    use super::*;
    use test_case::test_case;

    #[test_case("browserslist-config-wrong", json!(null), "extends browserslist-config-wrong"; "empty export")]
    fn invalid(pkg: &str, value: serde_json::Value, query: &str) {
        let _lock = TEST_MUTEX.lock().unwrap();
        mock(pkg, value);
        with_test_dir(|| {
            assert!(matches!(
                should_fail(query, &Opts::default()),
                Error::FailedToResolveExtend(..)
            ));
        });
        clean(pkg);
    }
}

mod invalid_name {
    use super::*;
    use test_case::test_case;

    #[test_case("extends thing-without-prefix"; "without prefix")]
    #[test_case("extends browserslist-config-package/../something"; "has dot")]
    #[test_case("extends browserslist-config-test/node_modules/a"; "has node_modules")]
    fn invalid_name(query: &str) {
        assert!(matches!(should_fail(query, &Opts::default()), Error::InvalidExtendName(..)));
    }
}
