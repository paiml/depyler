# O(1) Module Mapping

This document provides an overview of depyler's module mapping system, which translates Python standard library and third-party modules to their Rust equivalents.

## Architecture

Depyler uses an O(1) amortized lookup HashMap to map Python modules to Rust crates:

```rust
// Core lookup in module_mapper.rs
pub fn get_mapping(&self, module_name: &str) -> Option<&ModuleMapping> {
    self.module_map.get(module_name)  // O(1) amortized
}
```

## Supported Modules

| Python Module | Rust Crate | Priority |
|---------------|------------|----------|
| `argparse` | `clap` | P0 |
| `json` | `serde_json` | P0 |
| `numpy` | `trueno` | P0 |
| `sklearn.*` | `aprender::*` | P0 |
| `re` | `regex` | P0 |
| `datetime` | `chrono` | P1 |
| `random` | `rand` | P1 |

See the full mapping table in the [specification](../docs/specifications/o-1-lookup-lib-mapping-rust-python-spec.md).

## Typeshed Ingestion

To eliminate manual mapping effort, depyler can auto-generate mappings from Python's typeshed `.pyi` stub files:

```rust
use depyler_core::typeshed_ingest::parse_pyi;

let json_pyi = std::fs::read_to_string("typeshed/stdlib/json.pyi")?;
let mapping = parse_pyi(&json_pyi, "json");
// mapping.item_map contains: loads→from_str, dumps→to_string, etc.
```

## Specification

Full specification: [DEPYLER-O1MAP-001](../docs/specifications/o-1-lookup-lib-mapping-rust-python-spec.md)

### Implementation Status (v1.2.0)

| Component | Status |
|-----------|--------|
| HashMap O(1) Lookup | ✅ Production |
| 35+ Module Mappings | ✅ Production |
| Typeshed Parser | ✅ Complete |
| PHF Optimization | ✅ Complete |
| Semantic Tests | ✅ 17 tests |
| CI Integration | ✅ Weekly automation |
| Benchmarks | ✅ Criterion suite |

## Usage

```rust
use depyler_core::module_mapper::ModuleMapper;

let mapper = ModuleMapper::new();

// Get mapping for a Python module
if let Some(mapping) = mapper.get_mapping("json") {
    println!("Rust crate: {}", mapping.rust_path);  // "serde_json"
    println!("External: {}", mapping.is_external);   // true
}
```

## PHF Compile-Time Optimization

For O(1) worst-case lookup (no hash collision risk), enable the `phf-lookup` feature:

```bash
cargo build --features phf-lookup
```

```rust
#[cfg(feature = "phf-lookup")]
use depyler_core::module_mapper_phf::{get_module_mapping, get_item_mapping};

// O(1) worst-case lookup via perfect hash
if let Some(mapping) = get_module_mapping("numpy") {
    println!("Rust crate: {}", mapping.rust_path);  // "trueno"
}

// Item-level lookup
if let Some(rust_fn) = get_item_mapping("json", "loads") {
    println!("Rust equivalent: {}", rust_fn);  // "from_str"
}
```

## Semantic Equivalence

All mappings are verified via semantic equivalence tests ensuring Python behavior is preserved:

| Python | Rust | Semantic Guarantee |
|--------|------|-------------------|
| `json.loads(s)` | `serde_json::from_str(s)` | Same JSON parsing |
| `math.sqrt(x)` | `x.sqrt()` | Same result (NaN for negative) |
| `re.match(p, s)` | `Regex::is_match(s)` | Same pattern matching |
| `os.getcwd()` | `env::current_dir()` | Same directory |

See [semantic tests](../crates/depyler-core/tests/module_mapper_semantic_tests.rs) for full coverage.

## Enterprise Library Mapping (DEPYLER-0903)

For enterprise environments requiring custom library mappings, depyler provides an extensible mapping system with priority chains.

### Architecture

```text
┌─────────────────────────────────────────────────────┐
│                  MappingRegistry                     │
├─────────────────────────────────────────────────────┤
│  Priority 1: User Overrides (highest)               │
│  Priority 2: Enterprise Extensions                  │
│  Priority 3: Core Mappings (shipped with depyler)   │
└─────────────────────────────────────────────────────┘
```

### Usage

```rust
use depyler_core::library_mapping::{MappingRegistry, LibraryMapping, ItemMapping, TransformPattern};

// Start with default core mappings
let mut registry = MappingRegistry::with_defaults();

// Lookup follows priority chain: overrides > extensions > core
if let Some(item) = registry.lookup("json", "loads") {
    println!("Rust name: {}", item.rust_name);  // "from_str"
}
```

### Transform Patterns

| Pattern | Description | Example |
|---------|-------------|---------|
| `Direct` | 1:1 rename | `loads` → `from_str` |
| `MethodCall` | Extra args | `head(n)` → `head(n, None)` |
| `Constructor` | Type::new() | `DataFrame()` → `DataFrame::new()` |
| `ReorderArgs` | Swap args | `(a, b, c)` → `(a, c, b)` |
| `TypedTemplate` | Poka-yoke templates | Type-safe code generation |

### TOML Plugin Configuration

Enterprise extensions can be defined in TOML:

```toml
[plugin]
id = "my-enterprise-plugin"
version = "1.0.0"

[[mappings]]
python_module = "company.ml"
rust_crate = "company_ml_rs"
python_version_req = ">=3.9"
rust_crate_version = "2.0"
confidence = "Verified"
provenance = "internal://docs/ml-rs"

[mappings.items]
train = { rust_name = "train_model", pattern = "MethodCall" }
```

### Loading Plugins

```rust
use depyler_core::library_mapping::toml_plugin::TomlPlugin;
use std::path::Path;

// Load from file
let plugin = TomlPlugin::from_file(Path::new("enterprise-mappings.toml"))?;

// Register with registry
plugin.register(&mut registry);

// Plugin validation (ReorderArgs permutation check, TypedTemplate param check)
plugin.validate()?;
```

### Specification

Full specification: [DEPYLER-0903](../docs/specification/tractable-external-lib-mapping-with-customization.md)
