# AI Agent Guidelines for oxc-browserslist

This document provides guidelines for AI agents and assistants working with the oxc-browserslist project.

## Project Overview

oxc-browserslist is a Rust port of [Browserslist](https://github.com/browserslist/browserslist), optimized for the [oxc](https://github.com/oxc-project/oxc) project. It helps determine which browsers to support based on usage statistics and version queries.

## Code Guidelines for AI Agents

### Understanding the Codebase

- **Language**: Primary language is Rust with some JavaScript/TypeScript for Node.js compatibility
- **Core functionality**: Browser query parsing and resolution
- **Performance focus**: This port prioritizes fast compilation and runtime performance
- **Dependencies**: Minimal dependencies (removed heavy deps like `chrono`, `itertools`, etc.)

### Making Changes

1. **Minimal modifications**: Always prefer the smallest possible changes
2. **Performance considerations**: Be mindful of compilation time and runtime performance
3. **Testing**: Run `cargo test` and `cargo check` before suggesting changes
4. **Compatibility**: Maintain compatibility with existing browserslist queries

### Key Areas

- **Query parsing** (`src/parser/`): Handle browserslist query syntax
- **Browser data** (`src/data/`): Browser version and usage data
- **Query resolution** (`src/queries/`): Logic for resolving queries to browser lists
- **Config handling** (`src/config/`): Configuration file parsing

### Common Tasks

- **Adding new query types**: Follow existing patterns in `src/queries/`
- **Updating browser data**: Use the codegen system (`cargo codegen`)
- **Performance optimization**: Focus on reducing allocations and improving algorithms
- **Bug fixes**: Write minimal test cases to reproduce issues

### Testing Guidelines

- Add tests for new functionality in the appropriate test modules
- Use `#[test_case]` macro for parameterized tests when applicable
- Focus on edge cases and browser compatibility scenarios
- Run benchmarks (`cargo bench`) for performance-critical changes

### Prerequisites

The project uses several tools for development:

- **Rust**: Latest stable (MSRV: 1.86.0)
- **Node.js**: Version specified in `.node-version`
- **pnpm**: Package manager for Node.js dependencies
- **just**: Command runner (alternative to make)

### Build and Development

`just init` has already been run, all tools (`watchexec-cli`, `typos-cli`, `cargo-shear`, `dprint`) are already installed, do not run `just init`.

Rust and `cargo` components `clippy`, `rust-docs` and `rustfmt` has already been installed, do not install them.

- Use `cargo check` for quick feedback during development
- Run `just lint` for linting
- Use `just fmt` for code formatting
- Generate updated browser data with `cargo codegen` when needed
- Rust `just ready` after all code changes are complete.

### Documentation

- Update API documentation for public interfaces
- Add inline comments for complex algorithms
- Follow Rust documentation conventions

## Browser Query Examples

When working with browser queries, common patterns include:

```
last 2 versions
> 1%
not dead
Chrome > 90
Safari >= 14
```

## Resources

- [Original Browserslist documentation](https://github.com/browserslist/browserslist)
- [oxc project documentation](https://github.com/oxc-project/oxc)
- [Rust documentation](https://doc.rust-lang.org/)

## Support

For questions or issues specific to AI agent contributions, please refer to the main project documentation and issue tracker.
