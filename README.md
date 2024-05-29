# oxc-browserslist

Rust port of [Browserslist](https://github.com/browserslist/browserslist), forked from [browserslist-rs](https://github.com/browserslist/browserslist-rs).

## Project Status

> Can I use this library?

Only custom usage is not supported: `> 0.5% in my stats` or `cover 99.5% in my stats`.

## Usage

See [docs.rs/oxc-browserslist-rs](https://docs.rs/oxc-browserslist-rs).

## Example Usage

You can try and inspect query result by running example with Cargo:

```sh
cargo run --example inspect -- <query>
```

You can also specify additional options, for example:

```sh
cargo run --example inspect -- --mobile-to-desktop 'last 2 versions, not dead'
```

To get more help, you can run:

```sh
cargo run --example inspect -- -h
```

## Limitations

The features below aren't supported currently:

