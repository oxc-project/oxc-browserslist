use std::{
    borrow::Cow,
    env,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use parser::parse;
use rustc_hash::FxHashMap;
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;

use crate::{error::Error, opts::Opts};

mod parser;

type Config = FxHashMap<String, Vec<String>>;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct PartialConfig {
    defaults: Vec<String>,
    env: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
#[serde(untagged)]
pub enum PkgConfig {
    Str(String),
    Arr(Vec<String>),
    Obj(Config),
}

impl Default for PkgConfig {
    fn default() -> Self {
        Self::Obj(FxHashMap::default())
    }
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Serialize))]
struct PackageJson {
    browserslist: Option<PkgConfig>,
}

const ERR_DUP_PLAIN: &str = "'browserslist' file";
const ERR_DUP_RC: &str = "'.browserslistrc' file";
const ERR_DUP_PKG: &str = "'package.json' file with `browserslist` field";

pub fn load(opts: &Opts) -> Result<Vec<String>, Error> {
    // Fast path: check BROWSERSLIST env var first
    if let Some(query) = check_browserslist_env() {
        return Ok(vec![query]);
    }

    // Check for explicit config path
    if let Some(config_path) = get_config_path(opts) {
        return load_from_config_path(Path::new(config_path.as_ref()), opts);
    }

    // Find config in filesystem
    let path = opts
        .path
        .as_ref()
        .map(PathBuf::from)
        .or_else(|| env::current_dir().ok())
        .ok_or(Error::FailedToAccessCurrentDir)?;

    load_from_found_config(find_config(&path)?, opts)
}

// Check BROWSERSLIST environment variable
#[inline(always)]
fn check_browserslist_env() -> Option<String> {
    env::var("BROWSERSLIST").ok()
}

// Get config path from options or environment
#[inline]
fn get_config_path(opts: &Opts) -> Option<Cow<'_, str>> {
    opts.config
        .as_ref()
        .map(Cow::from)
        .or_else(|| env::var("BROWSERSLIST_CONFIG").ok().map(Cow::Owned))
}

// Load config from a specific path
fn load_from_config_path(config_path: &Path, opts: &Opts) -> Result<Vec<String>, Error> {
    // Check if the file exists first
    if !config_path.exists() {
        return Err(create_read_error(config_path));
    }

    let env = get_env(opts);

    if config_path.file_name() == Some(std::ffi::OsStr::new("package.json")) {
        load_from_package_json(config_path, &env, opts.throw_on_missing)
    } else {
        load_from_browserslist_file(config_path, &env, opts.throw_on_missing)
    }
}

// Load from package.json file
fn load_from_package_json(
    path: &Path,
    env: &str,
    throw_on_missing: bool,
) -> Result<Vec<String>, Error> {
    let content = fs::read(path).map_err(|_| create_read_error(path))?;
    let pkg: PackageJson = serde_json::from_slice(&content).map_err(|_| create_read_error(path))?;
    let config =
        pkg.browserslist.ok_or_else(|| Error::MissingFieldInPkg(format!("{}", path.display())))?;
    pick_queries_by_env(config, env, throw_on_missing)
}

// Load from browserslist or .browserslistrc file
fn load_from_browserslist_file(
    path: &Path,
    env: &str,
    throw_on_missing: bool,
) -> Result<Vec<String>, Error> {
    let content = fs::read_to_string(path).map_err(|_| create_read_error(path))?;
    let config = parse(&content, env, throw_on_missing)?;
    Ok(config.env.unwrap_or(config.defaults))
}

// Load from found config
fn load_from_found_config(found: FindConfig, opts: &Opts) -> Result<Vec<String>, Error> {
    let env = get_env(opts);
    match found {
        FindConfig::String(s) => {
            let config = parse(&s, &env, opts.throw_on_missing)?;
            Ok(config.env.unwrap_or(config.defaults))
        }
        FindConfig::PkgConfig(config) => pick_queries_by_env(config, &env, opts.throw_on_missing),
    }
}

// Create a read error
#[cold]
fn create_read_error(path: &Path) -> Error {
    Error::FailedToReadConfig(format!("{}", path.display()))
}

pub fn load_with_config(config: PkgConfig, opts: &Opts) -> Result<Vec<String>, Error> {
    pick_queries_by_env(config, &get_env(opts), opts.throw_on_missing)
}

enum FindConfig {
    String(Cow<'static, str>),
    PkgConfig(PkgConfig),
}

fn find_config(path: &Path) -> Result<FindConfig, Error> {
    for dir in path.ancestors() {
        // Check file existence without opening them
        let path_plain = dir.join("browserslist");
        let path_rc = dir.join(".browserslistrc");
        let path_pkg = dir.join("package.json");

        let has_plain = path_plain.is_file();
        let has_rc = path_rc.is_file();
        let pkg_config = if path_pkg.is_file() { try_load_package_json(&path_pkg)? } else { None };

        // Handle all conflicts first (cold paths)
        if has_plain && has_rc {
            return create_duplicate_config_error(dir, ERR_DUP_PLAIN, ERR_DUP_RC);
        }
        if has_plain && pkg_config.is_some() {
            return create_duplicate_config_error(dir, ERR_DUP_PLAIN, ERR_DUP_PKG);
        }
        if has_rc && pkg_config.is_some() {
            return create_duplicate_config_error(dir, ERR_DUP_RC, ERR_DUP_PKG);
        }

        // Load the first config found (hot paths)
        if has_plain {
            let content =
                fs::read_to_string(&path_plain).map_err(|_| create_read_error(&path_plain))?;
            return Ok(FindConfig::String(Cow::Owned(content)));
        }
        if has_rc {
            let content = fs::read_to_string(&path_rc).map_err(|_| create_read_error(&path_rc))?;
            return Ok(FindConfig::String(Cow::Owned(content)));
        }
        if let Some(config) = pkg_config {
            return Ok(FindConfig::PkgConfig(config));
        }
    }

    Ok(FindConfig::String(Cow::Borrowed("defaults")))
}

// Try to load browserslist config from package.json
fn try_load_package_json(path: &Path) -> Result<Option<PkgConfig>, Error> {
    let file = File::open(path).ok();
    match file {
        Some(f) => {
            let reader = BufReader::new(f);
            match serde_json::from_reader::<_, PackageJson>(reader) {
                Ok(json) => Ok(json.browserslist),
                Err(_) => Ok(None),
            }
        }
        None => Ok(None),
    }
}

// Create duplicate config error
#[cold]
fn create_duplicate_config_error(
    dir: &Path,
    err1: &'static str,
    err2: &'static str,
) -> Result<FindConfig, Error> {
    Err(Error::DuplicatedConfig(format!("{}", dir.display()), err1, err2))
}

fn get_env(opts: &Opts) -> Cow<'_, str> {
    opts.env
        .as_ref()
        .map(Cow::from)
        .or_else(|| env::var("BROWSERSLIST_ENV").ok().map(Cow::Owned))
        .or_else(|| env::var("NODE_ENV").ok().map(Cow::Owned))
        .unwrap_or(Cow::Borrowed("production"))
}

fn pick_queries_by_env(
    config: PkgConfig,
    env: &str,
    throw_on_missing: bool,
) -> Result<Vec<String>, Error> {
    match config {
        PkgConfig::Str(query) => Ok(vec![query]),
        PkgConfig::Arr(queries) => Ok(queries),
        PkgConfig::Obj(mut config) => {
            if let Some(queries) = config.remove(env) {
                Ok(queries)
            } else if throw_on_missing && env != "defaults" {
                Err(Error::MissingEnv(env.to_string()))
            } else {
                Ok(config.remove("defaults").unwrap_or_default())
            }
        }
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use std::{
        env::{remove_var, set_var, temp_dir},
        fs,
    };

    use super::*;

    #[test]
    fn load_config() {
        assert_eq!(&*load(&Opts::default()).unwrap(), ["defaults"]);

        // read queries from env
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { set_var("BROWSERSLIST", "last 2 versions") };
        assert_eq!(&*load(&Opts::default()).unwrap(), ["last 2 versions"]);
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { remove_var("BROWSERSLIST") };

        // specify config file by env
        let tmp = temp_dir().join("browserslist");
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { set_var("BROWSERSLIST_CONFIG", &tmp) };

        assert_eq!(
            load(&Opts::default()).unwrap_err(),
            Error::FailedToReadConfig(format!("{}", tmp.display()))
        );

        fs::write(&tmp, "chrome > 90").unwrap();
        assert_eq!(load(&Opts::default()).as_deref().unwrap(), ["chrome > 90"]);
        // options `config` should have higher priority than environment variable
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { set_var("BROWSERSLIST_CONFIG", "./browserslist") };

        // specify config file by options
        fs::write(&tmp, "firefox > 90").unwrap();
        assert_eq!(
            load(&Opts { config: Some(tmp.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["firefox > 90"]
        );
        fs::remove_file(&tmp).unwrap();

        // package.json with single string format
        let tmp = temp_dir().join("package.json");
        fs::write(
            &tmp,
            serde_json::to_string(&PackageJson {
                browserslist: Some(PkgConfig::Str("node > 10".into())),
            })
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            load(&Opts { config: Some(tmp.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["node > 10"]
        );

        // package.json with array format
        fs::write(
            &tmp,
            serde_json::to_string(&PackageJson {
                browserslist: Some(PkgConfig::Arr(vec!["node > 7.4".to_string()])),
            })
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            load(&Opts { config: Some(tmp.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["node > 7.4"]
        );

        // package.json with object format
        let mut config_obj = FxHashMap::default();
        let _ = config_obj.insert("production".into(), vec!["> 1%".into(), "not dead".into()]);
        let _ = config_obj.insert("modern".into(), vec!["last 1 version".into()]);
        let _ = config_obj.insert("xp".into(), vec!["chrome >= 49".into()]);
        let _ = config_obj.insert("ssr".into(), vec!["node >= 12".into()]);
        fs::write(
            &tmp,
            serde_json::to_string(&PackageJson { browserslist: Some(PkgConfig::Obj(config_obj)) })
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            load(&Opts { config: Some(tmp.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["> 1%", "not dead"]
        );

        // pick queries by env
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { set_var("BROWSERSLIST_ENV", "modern") };
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { set_var("NODE_ENV", "ssr") };
        assert_eq!(
            load(&Opts {
                config: Some(tmp.to_str().unwrap().into()),
                env: Some("xp".into()),
                ..Default::default()
            })
            .as_deref()
            .unwrap(),
            ["chrome >= 49"]
        );
        assert_eq!(
            load(&Opts { config: Some(tmp.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["last 1 version"]
        );
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { remove_var("BROWSERSLIST_ENV") };
        assert_eq!(
            load(&Opts { config: Some(tmp.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["node >= 12"]
        );
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { remove_var("NODE_ENV") };

        let tmp = temp_dir().join("browserslist");
        fs::write(
            &tmp,
            r"
[development]
last 1 version

[production]
> 1%, not dead
        ",
        )
        .unwrap();
        assert_eq!(
            load(&Opts {
                config: Some(tmp.to_str().unwrap().into()),
                env: Some("development".into()),
                ..Default::default()
            })
            .as_deref()
            .unwrap(),
            ["last 1 version"]
        );

        fs::write(&tmp, "> 1%, not dead").unwrap();
        assert_eq!(
            load(&Opts {
                config: Some(tmp.to_str().unwrap().into()),
                env: Some("development".into()),
                ..Default::default()
            })
            .as_deref()
            .unwrap(),
            ["> 1%, not dead"]
        );

        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { remove_var("BROWSERSLIST_CONFIG") };

        // find configuration file
        let tmp_dir = temp_dir();
        let tmp = tmp_dir.to_str().unwrap();
        assert_eq!(
            load(&Opts { path: Some(tmp.into()), ..Default::default() }).unwrap_err(),
            Error::DuplicatedConfig(tmp.to_string(), ERR_DUP_PLAIN, ERR_DUP_PKG)
        );

        fs::write(tmp_dir.join(".browserslistrc"), "electron > 12.0").unwrap();
        assert_eq!(
            load(&Opts { path: Some(tmp.into()), ..Default::default() }).unwrap_err(),
            Error::DuplicatedConfig(tmp.to_string(), ERR_DUP_PLAIN, ERR_DUP_RC)
        );

        fs::remove_file(tmp_dir.join("browserslist")).unwrap();
        assert_eq!(
            load(&Opts { path: Some(tmp.into()), ..Default::default() }).unwrap_err(),
            Error::DuplicatedConfig(tmp.to_string(), ERR_DUP_RC, ERR_DUP_PKG)
        );

        let tmp_dir = tmp_dir.join("browserslist/1/2/3");
        fs::create_dir_all(&tmp_dir).unwrap();

        fs::write(temp_dir().join("browserslist/1/browserslist"), "node >= 16").unwrap();
        assert_eq!(
            load(&Opts { path: Some(tmp_dir.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["node >= 16"]
        );

        fs::write(temp_dir().join("browserslist/1/2/package.json"), "{}").unwrap();
        assert_eq!(
            load(&Opts { path: Some(tmp_dir.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["node >= 16"]
        );

        let tmp = temp_dir();
        fs::remove_file(tmp.join("package.json")).unwrap();
        fs::remove_file(tmp.join("browserslist/1/2/package.json")).unwrap();
        fs::remove_file(tmp.join("browserslist/1/browserslist")).unwrap();
        assert_eq!(
            load(&Opts { path: Some(tmp_dir.to_str().unwrap().into()), ..Default::default() })
                .as_deref()
                .unwrap(),
            ["electron > 12.0"]
        );

        fs::remove_dir_all(tmp.join("browserslist")).unwrap();

        // load config from current directory if no options set
        assert_eq!(&*load(&Opts::default()).unwrap(), ["defaults"]);
        let original_cwd = env::current_dir().unwrap();
        fs::write(tmp.join(".browserslistrc"), "not dead").unwrap();
        env::set_current_dir(&tmp).unwrap();
        assert_eq!(load(&Opts::default()).as_deref().unwrap(), ["not dead"]);
        env::set_current_dir(original_cwd).unwrap();

        // throw if env is missing
        assert_eq!(
            load(&Opts {
                env: Some("production".into()),
                path: Some(tmp.to_str().unwrap().into()),
                throw_on_missing: true,
                ..Default::default()
            })
            .unwrap_err(),
            Error::MissingEnv("production".into())
        );

        // don't throw if existed
        fs::write(tmp.join(".browserslistrc"), "[production]\nnot dead").unwrap();
        assert_eq!(
            load(&Opts {
                env: Some("production".into()),
                path: Some(tmp.to_str().unwrap().into()),
                throw_on_missing: true,
                ..Default::default()
            })
            .as_deref()
            .unwrap(),
            ["not dead"]
        );

        // don't throw if env is `defaults`
        assert!(
            load(&Opts {
                env: Some("defaults".into()),
                path: Some(tmp.to_str().unwrap().into()),
                throw_on_missing: true,
                ..Default::default()
            })
            .as_deref()
            .unwrap()
            .is_empty()
        );

        fs::remove_file(tmp.join(".browserslistrc")).unwrap();
    }
}
