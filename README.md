<div align="center">

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]

[![MIT licensed][license-badge]][license-url]
[![Build Status][ci-badge]][ci-url]
[![Code Coverage][code-coverage-badge]][code-coverage-url]
[![CodSpeed Badge][codspeed-badge]][codspeed-url]
[![Sponsors][sponsors-badge]][sponsors-url]
[![Discord chat][discord-badge]][discord-url]

</div>

# oxc-browserslist

Rust port of [Browserslist](https://github.com/browserslist/browserslist), forked from [browserslist-rs](https://github.com/browserslist/browserslist-rs).

The original crate did not meet the criteria of `oxc`, the following changes are made:

* reduced compilation speed from one minute to a few seconds
* removed all unnecessary, heavy or slow dependencies: `ahash`, `chrono`, `either`, `indexmap`, `itertools`, `once_cell`, `string_cache`
* improved some runtime performance, e.g. [improve sort method](https://github.com/oxc-project/oxc-browserslist/pull/28), [precompute versions](https://github.com/oxc-project/oxc-browserslist/pull/10)

## Project Status

> Can I use this library?

Only custom usage is not supported: `> 0.5% in my stats` or `cover 99.5% in my stats`.

## Usage

See [docs.rs/oxc-browserslist](https://docs.rs/oxc-browserslist).

## Example Usage

You can try and inspect query result by running example with Cargo:

```sh
cargo run --example inspect -- <query>
```

You can also specify additional options, for example:

```sh
cargo run --example inspect -- --mobile-to-desktop 'last 2 versions, not dead'
```

## Future Work

* `nom` can be replaced by a hand written parser to improve runtime and compilation speed
* to improve runtime performance, all semver versions with their string representation can be precomputed and code generated - current code is doing a lot of `parse` and `to_string` on semver versions right now

[discord-badge]: https://img.shields.io/discord/1079625926024900739?logo=discord&label=Discord
[discord-url]: https://discord.gg/9uXCAwqQZW
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/oxc-project/oxc-browserslist/blob/main/LICENSE
[ci-badge]: https://github.com/oxc-project/oxc-browserslist/actions/workflows/CI.yml/badge.svg?event=push&branch=main
[ci-url]: https://github.com/oxc-project/oxc-browserslist/actions/workflows/CI.yml?query=event%3Apush+branch%3Amain
[code-coverage-badge]: https://codecov.io/github/oxc-project/oxc-browserslist/branch/main/graph/badge.svg
[code-coverage-url]: https://codecov.io/gh/oxc-project/oxc-browserslist
[sponsors-badge]: https://img.shields.io/github/sponsors/Boshen
[sponsors-url]: https://github.com/sponsors/Boshen
[codspeed-badge]: https://img.shields.io/endpoint?url=https://codspeed.io/badge.json
[codspeed-url]: https://codspeed.io/oxc-project/oxc-browserslist
[crates-badge]: https://img.shields.io/crates/d/oxc-browserslist?label=crates.io
[crates-url]: https://crates.io/crates/oxc-browserslist
[docs-badge]: https://img.shields.io/docsrs/oxc-browserslist
[docs-url]: https://docs.rs/oxc-browserslist
