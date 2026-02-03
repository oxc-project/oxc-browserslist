use super::QueryResult;
use crate::{error::Error, opts::Opts};

#[cfg(target_arch = "wasm32")]
pub(super) fn extends(pkg: &str, opts: &Opts) -> QueryResult {
    if opts.dangerous_extend {
        Err(Error::UnsupportedExtends)
    } else {
        check_extend_name(pkg).map(|_| Default::default())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn extends(pkg: &str, opts: &Opts) -> QueryResult {
    use std::{env, process};

    use crate::{config, resolve};

    let dangerous_extend =
        opts.dangerous_extend || env::var("BROWSERSLIST_DANGEROUS_EXTEND").is_ok();
    if !dangerous_extend {
        check_extend_name(pkg)?;
    }

    let mut command = process::Command::new("node");
    command.args(["-p", &format!("JSON.stringify(require('{pkg}'))")]);
    // Allow tests to override the working directory via environment variable
    if let Ok(test_dir) = env::var("BROWSERSLIST_TEST_DIR") {
        command.current_dir(test_dir);
    }
    let output = command.output().map_err(|_| Error::UnsupportedExtends)?.stdout;
    let config = serde_json::from_str(&String::from_utf8_lossy(&output))
        .map_err(|_| Error::FailedToResolveExtend(pkg.to_string()))?;

    resolve(&config::load_with_config(config, opts)?, opts)
}

fn check_extend_name(pkg: &str) -> Result<(), Error> {
    let unscoped =
        pkg.strip_prefix('@').and_then(|s| s.find('/').and_then(|i| s.get(i + 1..))).unwrap_or(pkg);
    if !(unscoped.starts_with("browserslist-config-")
        || pkg.starts_with('@') && unscoped == "browserslist-config")
    {
        return Err(Error::InvalidExtendName(
            "Browserslist config needs `browserslist-config-` prefix.",
        ));
    }
    if unscoped.contains('.') {
        return Err(Error::InvalidExtendName("`.` not allowed in Browserslist config name."));
    }
    if pkg.contains("node_modules") {
        return Err(Error::InvalidExtendName("`node_modules` not allowed in Browserslist config."));
    }

    Ok(())
}
