## Binary Size Comparison

### Original (before optimizations):
- Library (.rlib): 4.6MB
- Example binary: 2.3MB

### With compression (default features):
- Library (.rlib): 2.3MB (50% reduction)
- Example binary: 1.1MB (52% reduction)

### With compression, without regions:
- Library (.rlib): 1.9MB (59% reduction)
- Example binary: 929KB (60% reduction)

### Data files:
- Original total: 1.4MB
- Compressed total: 188KB (87% reduction)

## Features

The library now supports optional features:

- `regions` (enabled by default): Includes regional browserslist data for queries like `> 1% in US`

To disable regions and save additional space:
```toml
[dependencies]
oxc-browserslist = { version = "2.0", default-features = false }
```
