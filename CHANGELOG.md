# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.3](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v1.0.2...oxc-browserslist-v1.0.3) - 2024-09-03

### Fixed
- downgrade caniuse-db to `1.0.30001639`

### Other
- update dependency rust to v1.81.0
- print raw strings in `caniuse_region_matching` and `caniuse_feature_matching` ([#69](https://github.com/oxc-project/oxc-browserslist/pull/69))
- add autofix.ci ([#68](https://github.com/oxc-project/oxc-browserslist/pull/68))
- add ci.yml
- rm CI.yml
- *(deps)* update npm packages ([#65](https://github.com/oxc-project/oxc-browserslist/pull/65))
- *(deps)* update rust crate prettyplease to v0.2.22
- *(deps)* update rust crates
- *(deps)* update npm packages ([#64](https://github.com/oxc-project/oxc-browserslist/pull/64))
- Revert "bump deps"
- bump deps
- bump deps
- sort caniuse_region_matching data
- install rustfmt in test for `cargo codegen`
- ??
- try peerDependencies
- `git diff --exit-code --quiet`
- test change of generated files; run `cargo codegen` after lock file change
- *(deps)* update rust crates
- *(deps)* update rust crates
- *(deps)* update rust crate criterion2 to v1
- *(deps)* update dependency rust to v1.80.0 ([#59](https://github.com/oxc-project/oxc-browserslist/pull/59))

## [1.0.2](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v1.0.1...oxc-browserslist-v1.0.2) - 2024-07-01

### Other
- *(deps)* update npm packages ([#52](https://github.com/oxc-project/oxc-browserslist/pull/52))

## [1.0.1](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v1.0.0...oxc-browserslist-v1.0.1) - 2024-06-24

### Other
- *(deps)* update rust crates
- *(deps)* update rust crate rustc-hash to v2

## [0.17.1](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v0.17.0...oxc-browserslist-v0.17.1) - 2024-06-17

### Other
- *(deps)* update npm packages ([#45](https://github.com/oxc-project/oxc-browserslist/pull/45))

## [0.17.0](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v0.16.2...oxc-browserslist-v0.17.0) - 2024-06-01

### Added
- [**breaking**] change `Error::Nom` to `Error::parse` for future compatibility ([#39](https://github.com/oxc-project/oxc-browserslist/pull/39))
- [**breaking**] change API to accept `&[S]` instead of `IntoIterator<Item = S>` ([#29](https://github.com/oxc-project/oxc-browserslist/pull/29))

### Other
- bump `electron-to-chromium`
- remove `once_cell` ([#33](https://github.com/oxc-project/oxc-browserslist/pull/33))
- shrink generated code size ([#32](https://github.com/oxc-project/oxc-browserslist/pull/32))
- remove `once_cell` from CANIUSE_BROWSERS ([#30](https://github.com/oxc-project/oxc-browserslist/pull/30))
- remove `crate-type` from Cargo.toml
- improve sort method ([#28](https://github.com/oxc-project/oxc-browserslist/pull/28))
- remove `itertools` ([#27](https://github.com/oxc-project/oxc-browserslist/pull/27))
- remove `either` ([#26](https://github.com/oxc-project/oxc-browserslist/pull/26))
- remove `chrono` ([#24](https://github.com/oxc-project/oxc-browserslist/pull/24))

## [0.16.2](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v0.16.1...oxc-browserslist-v0.16.2) - 2024-05-30

### Other
- clean up node version and node releases ([#21](https://github.com/oxc-project/oxc-browserslist/pull/21))
- reduce the size of json data in "caniuse_feature_matching" ([#20](https://github.com/oxc-project/oxc-browserslist/pull/20))

## [0.16.1](https://github.com/oxc-project/oxc-browserslist/compare/oxc-browserslist-v0.16.0...oxc-browserslist-v0.16.1) - 2024-05-29

### Other

- Made everything slightly better
