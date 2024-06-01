# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.17.0](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v0.16.2...oxc-browserslist-v0.17.0) - 2024-06-01

### Added
- [**breaking**] change `Error::Nom` to `Error::parse` for future compatibility ([#39](https://github.com/oxc-project/oxc-browserslist/pull/39))
- [**breaking**] change API to accept `&[S]` instead of `IntoIterator<Item = S>` ([#29](https://github.com/oxc-project/oxc-browserslist/pull/29))

### Other
- update README
- remove `#![allow(missing_docs, dead_code, clippy::pedantic)]` from generated/
- update README
- update README
- bump `electron-to-chromium`
- clean up some formatting
- update docs
- check doc
- add cargo fmt
- add typos
- update README
- benchmark build the main crate only ([#35](https://github.com/oxc-project/oxc-browserslist/pull/35))
- remove some generic code
- clean up some cold
- format code `use_small_heuristics = "Max"`
- *(xtask)* clean up some code
- remove `once_cell` ([#33](https://github.com/oxc-project/oxc-browserslist/pull/33))
- shrink generated code size ([#32](https://github.com/oxc-project/oxc-browserslist/pull/32))
- remove `once_cell` from CANIUSE_BROWSERS ([#30](https://github.com/oxc-project/oxc-browserslist/pull/30))
- remove `crate-type` from Cargo.toml
- improve sort method ([#28](https://github.com/oxc-project/oxc-browserslist/pull/28))
- remove itertools ([#27](https://github.com/oxc-project/oxc-browserslist/pull/27))
- remove `either` ([#26](https://github.com/oxc-project/oxc-browserslist/pull/26))
- remove `chrono` ([#24](https://github.com/oxc-project/oxc-browserslist/pull/24))

## [0.16.2](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v0.16.1...oxc-browserslist-v0.16.2) - 2024-05-30

### Other
- clean up node version and node releases ([#21](https://github.com/oxc-project/oxc-browserslist/pull/21))
- reduce the size of json data in "caniuse_feature_matching" ([#20](https://github.com/oxc-project/oxc-browserslist/pull/20))

## [0.16.1](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v0.16.0...oxc-browserslist-v0.16.1) - 2024-05-29

### Other

- Made everything slightly better
