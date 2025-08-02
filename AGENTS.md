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

### Build and Development

- Use `cargo check` for quick feedback during development
- Run `cargo clippy` for linting
- Use `cargo fmt` for code formatting
- Generate updated browser data with `cargo codegen` when needed

### Documentation

- Update API documentation for public interfaces
- Add inline comments for complex algorithms
- Update README.md if adding major features
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