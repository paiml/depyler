# Migration Guide: Depyler v2.x to v3.0

## Overview

Depyler v3.0 introduces multi-target transpilation support, allowing you to transpile Python code to either Rust (default) or Ruchy script format. This is a major version bump but maintains backward compatibility for existing Rust transpilation workflows.

## What's New in v3.0

### Multi-Target Support

The biggest change in v3.0 is the ability to transpile to multiple target languages:

- **Rust** (default): Same high-quality, safe Rust code generation as before
- **Ruchy**: New functional programming target with pipeline operators

### New Backend Architecture

- Introduced `TranspilationBackend` trait for extensible target support
- Simplified HIR (High-level Intermediate Representation) for backend consumption
- Target-specific optimizations for each backend

## Breaking Changes

### None for Rust Target Users

If you're using Depyler to transpile to Rust, there are **no breaking changes**. Your existing code and workflows will continue to work exactly as before.

### API Changes (Library Users)

If you're using Depyler as a library, some internal APIs have changed:

#### Old API (v2.x)
```rust
use depyler_core::transpile;

let rust_code = transpile(python_code)?;
```

#### New API (v3.0)
```rust
use depyler_core::{transpile, TranspilationTarget};

// For Rust (default)
let rust_code = transpile(python_code)?;  // Still works!

// For explicit target selection
use depyler_core::transpile_with_target;
let rust_code = transpile_with_target(python_code, TranspilationTarget::Rust)?;
let ruchy_code = transpile_with_target(python_code, TranspilationTarget::Ruchy)?;
```

## CLI Changes

### New `--target` Flag

```bash
# Old (still works, defaults to Rust)
depyler transpile example.py

# New (explicit target selection)
depyler transpile example.py --target=rust    # Same as default
depyler transpile example.py --target=ruchy   # New Ruchy target
```

## Migration Steps

### For CLI Users

1. **No action required** if you're happy with Rust output
2. To try Ruchy output, add `--target=ruchy` to your commands

### For Library Users

1. Update your `Cargo.toml`:
   ```toml
   [dependencies]
   depyler = "3.0"
   ```

2. If you need Ruchy support, enable the feature:
   ```toml
   [dependencies]
   depyler = { version = "3.0", features = ["ruchy"] }
   ```

3. Update any direct usage of internal HIR types (if applicable)

### For Plugin/Extension Authors

If you've built extensions or plugins for Depyler:

1. Implement the `TranspilationBackend` trait for custom targets
2. Update to use `HirModule` instead of the old HIR structure
3. See the `depyler-ruchy` crate for a reference implementation

## New Features to Explore

### Ruchy Script Format

Try the new Ruchy backend for functional programming style:

```python
# Python input
numbers = [x * 2 for x in range(10) if x > 5]
formatted = f"Result: {numbers}"
```

```ruchy
# Ruchy output
numbers = range(10) |> filter(x => x > 5) |> map(x => x * 2)
formatted = "Result: ${numbers}"
```

### Benefits of Ruchy Target

- **Functional pipelines**: More readable data transformations
- **Native string interpolation**: Cleaner string formatting
- **Actor-based concurrency**: Alternative to async/await
- **DataFrame operations**: Built-in support for data science workflows

## Compatibility Matrix

| Feature | v2.x | v3.0 Rust | v3.0 Ruchy |
|---------|------|-----------|------------|
| Basic transpilation | ✅ | ✅ | ✅ |
| Type inference | ✅ | ✅ | ✅ |
| Verification | ✅ | ✅ | ⚠️ Basic |
| Optimization | ✅ | ✅ | ✅ |
| LSP support | ✅ | ✅ | 🚧 |
| MCP integration | ✅ | ✅ | ✅ |

Legend: ✅ Full support | ⚠️ Partial support | 🚧 In development | ❌ Not supported

## Getting Help

- **Documentation**: [docs.rs/depyler](https://docs.rs/depyler)
- **GitHub Issues**: [github.com/paiml/depyler/issues](https://github.com/paiml/depyler/issues)
- **Changelog**: See [CHANGELOG.md](CHANGELOG.md) for detailed changes

## Rollback Instructions

If you encounter issues with v3.0:

```bash
# Rollback to v2.x
cargo install depyler --version 2.3.0 --force

# Or in Cargo.toml
[dependencies]
depyler = "=2.3.0"
```

## Future Roadmap

- Additional transpilation targets (C++, Julia, Zig)
- Enhanced Ruchy verification
- Cross-target optimization strategies
- Unified debugging experience across targets