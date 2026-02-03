//! Integration tests for query modules.
//! Each submodule corresponds to a source file in src/queries/.

#![cfg(not(miri))]

mod common;

use browserslist::{Error, Opts, resolve};

pub use common::run_compare;

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
