//! **oxc-browserslist** is a Rust-based implementation of [Browserslist](https://github.com/browserslist/browserslist).
//!
//! ## Introduction
//!
//! This library bundles Can I Use data, Electron versions list and Node.js releases list,
//! so it won't and doesn't need to access any data files.
//!
//! Except several non-widely/non-frequently used features,
//! this library works as same as the JavaScript-based
//! implementation [Browserslist](https://github.com/browserslist/browserslist).
//!
//! ## Usage
//!
//! It provides a simple API for querying which accepts a sequence of strings and options [`Opts`],
//! then returns the result.
//!
//! ```
//! use browserslist::{Distrib, Opts, resolve, Error};
//!
//! let distribs = resolve(&["ie <= 6"], &Opts::default()).unwrap();
//! assert_eq!(distribs[0].name(), "ie");
//! assert_eq!(distribs[0].version(), "6");
//! assert_eq!(distribs[1].name(), "ie");
//! assert_eq!(distribs[1].version(), "5.5");
//!
//! assert_eq!(
//!     resolve(&["yuru 1.0"], &Opts::default()),
//!     Err(Error::BrowserNotFound(String::from("yuru")))
//! );
//! ```
//!
//! The result isn't a list of strings, instead, it's a tuple struct called [`Distrib`].
//! If you need to retrieve something like JavaScript-based implementation of
//! [Browserslist](https://github.com/browserslist/browserslist),
//! you can convert them to strings:
//!
//! ```
//! use browserslist::{Distrib, Opts, resolve, Error};
//!
//! let distribs = resolve(&["ie <= 6"], &Opts::default()).unwrap();
//! assert_eq!(
//!     distribs.into_iter().map(|d| d.to_string()).collect::<Vec<_>>(),
//!     vec![String::from("ie 6"), String::from("ie 5.5")]
//! );
//! ```
//!
//! ## WebAssembly
//!
//! This crate can be compiled as WebAssembly, without configuring any features manually.
//!
//! Please note that browser and Deno can run WebAssembly,
//! but those environments aren't Node.js,
//! so you will receive an error when querying `current node` in those environments.

use parser::parse_browserslist_query;
#[cfg(all(feature = "wasm_bindgen", target_arch = "wasm32"))]
pub use wasm::browserslist;
pub use {error::Error, opts::Opts, queries::Distrib};

#[cfg(not(target_arch = "wasm32"))]
mod config;
mod data;
mod error;
mod generated;
mod opts;
mod parser;
mod queries;
mod semver;
#[cfg(test)]
mod test;
#[cfg(all(feature = "wasm_bindgen", target_arch = "wasm32"))]
mod wasm;

/// Resolve browserslist queries.
///
/// This is a low-level API.
/// If you want to load queries from configuration file and
/// resolve them automatically,
/// use the higher-level API [`execute`] instead.
///
/// ```
/// use browserslist::{Distrib, Opts, resolve};
///
/// let distribs = resolve(&["ie <= 6"], &Opts::default()).unwrap();
/// assert_eq!(distribs[0].name(), "ie");
/// assert_eq!(distribs[0].version(), "6");
/// assert_eq!(distribs[1].name(), "ie");
/// assert_eq!(distribs[1].version(), "5.5");
/// ```
pub fn resolve<S>(queries: &[S], opts: &Opts) -> Result<Vec<Distrib>, Error>
where
    S: AsRef<str>,
{
    if queries.len() == 1 {
        _resolve(queries[0].as_ref(), opts)
    } else {
        let s = &queries.iter().map(|q| q.as_ref()).collect::<Vec<_>>().join(", ");
        _resolve(s, opts)
    }
}

// reduce generic monomorphization
fn _resolve(query: &str, opts: &Opts) -> Result<Vec<Distrib>, Error> {
    let queries = parse_browserslist_query(query)?;
    let mut distribs = vec![];
    for (i, current) in queries.1.into_iter().enumerate() {
        if i == 0 && current.negated {
            return Err(Error::NotAtFirst(current.raw.to_string()));
        }

        let mut dist = queries::query(current.atom, opts)?;
        if current.negated {
            distribs.retain(|distrib| !dist.contains(distrib));
        } else if current.is_and {
            distribs.retain(|distrib| dist.contains(distrib));
        } else {
            distribs.append(&mut dist);
        }
    }

    distribs.sort_by_cached_key(|d| {
        let version = d.version().parse::<semver::Version>().unwrap_or_default();
        (d.name(), std::cmp::Reverse(version))
    });
    distribs.dedup();

    Ok(distribs)
}

#[cfg(not(target_arch = "wasm32"))]
/// Load queries from configuration with environment information,
/// then resolve those queries.
///
/// If you want to resolve custom queries (not from configuration file),
/// use the lower-level API [`resolve`] instead.
///
/// ```
/// use browserslist::{Opts, execute};
///
/// // when no config found, it use `defaults` query
/// assert!(!execute(&Opts::default()).unwrap().is_empty());
/// ```
pub fn execute(opts: &Opts) -> Result<Vec<Distrib>, Error> {
    resolve(&config::load(opts)?, opts)
}
