# Stdlib Mapping Plugin System (DEPYLER-0506)

## Overview

Depyler now supports a plugin system for extending Python→Rust standard library API mappings. This allows users to add custom mappings for third-party libraries or override built-in mappings.

## Quick Start

### Creating a Simple Plugin

```rust
use depyler_core::stdlib_mappings::{StdlibPlugin, StdlibMappings, StdlibApiMapping, RustPattern};

struct RequestsPlugin;

impl StdlibPlugin for RequestsPlugin {
    fn register_mappings(&self, registry: &mut StdlibMappings) {
        // Map requests.Session.get() → session.get()?
        registry.register(StdlibApiMapping {
            module: "requests",
            class: "Session",
            python_attr: "get",
            rust_pattern: RustPattern::MethodCall {
                method: "get",
                extra_args: vec![],
                propagate_error: true,
            },
        });

        // Map requests.Session.post() → session.post()?
        registry.register(StdlibApiMapping {
            module: "requests",
            class: "Session",
            python_attr: "post",
            rust_pattern: RustPattern::MethodCall {
                method: "post",
                extra_args: vec![],
                propagate_error: true,
            },
        });
    }

    fn name(&self) -> &str {
        "requests"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }
}
```

### Loading Plugins

```rust
use depyler_core::stdlib_mappings::StdlibMappings;

let mut mappings = StdlibMappings::new();
let requests_plugin = RequestsPlugin;

// Load single plugin
mappings.load_plugin(&requests_plugin);

// Or load multiple plugins
let numpy_plugin = NumpyPlugin;
mappings.load_plugins(&[&requests_plugin, &numpy_plugin]);
```

## RustPattern Types

### MethodCall
Converts Python method call to Rust method call with optional error propagation.

```rust
RustPattern::MethodCall {
    method: "headers",
    extra_args: vec![],
    propagate_error: true,  // Adds ? operator
}
```

**Example**: `reader.fieldnames` → `reader.headers()?`

### PropertyToMethod
Converts Python property access to Rust method call.

```rust
RustPattern::PropertyToMethod {
    method: "len",
    propagate_error: false,
}
```

**Example**: `data.length` → `data.len()`

### IterationPattern
Custom iteration patterns with optional type hints.

```rust
RustPattern::IterationPattern {
    iter_method: "deserialize",
    element_type: Some("HashMap<String, String>"),
    yields_results: true,  // Marks iterator yields Result types
}
```

**Example**: `for row in reader` → `for row in reader.deserialize::<HashMap<String, String>>()?`

### CustomTemplate
Fully custom code generation with `{var}` placeholder.

```rust
RustPattern::CustomTemplate {
    template: "BufReader::new({var}).lines()",
}
```

**Example**: `for line in file` → `for line in BufReader::new(file).lines()`

## Advanced Examples

### NumPy Plugin

```rust
struct NumpyPlugin;

impl StdlibPlugin for NumpyPlugin {
    fn register_mappings(&self, registry: &mut StdlibMappings) {
        // ndarray.shape property
        registry.register(StdlibApiMapping {
            module: "numpy",
            class: "ndarray",
            python_attr: "shape",
            rust_pattern: RustPattern::PropertyToMethod {
                method: "shape",
                propagate_error: false,
            },
        });

        // ndarray.reshape() method
        registry.register(StdlibApiMapping {
            module: "numpy",
            class: "ndarray",
            python_attr: "reshape",
            rust_pattern: RustPattern::MethodCall {
                method: "reshape",
                extra_args: vec![],
                propagate_error: false,
            },
        });
    }

    fn name(&self) -> &str {
        "numpy"
    }
}
```

### Batch Registration

For many mappings at once:

```rust
impl StdlibPlugin for LargePlugin {
    fn register_mappings(&self, registry: &mut StdlibMappings) {
        let mappings = vec![
            StdlibApiMapping { /* ... */ },
            StdlibApiMapping { /* ... */ },
            // ... many more
        ];

        registry.register_batch(mappings);
    }

    fn name(&self) -> &str {
        "large_library"
    }
}
```

### Overriding Built-in Mappings

Plugins can override built-in mappings:

```rust
struct CustomCsvPlugin;

impl StdlibPlugin for CustomCsvPlugin {
    fn register_mappings(&self, registry: &mut StdlibMappings) {
        // Override csv.DictReader.fieldnames
        registry.register(StdlibApiMapping {
            module: "csv",
            class: "DictReader",
            python_attr: "fieldnames",
            rust_pattern: RustPattern::PropertyToMethod {
                method: "get_headers",  // Use different method
                propagate_error: true,
            },
        });
    }

    fn name(&self) -> &str {
        "custom_csv"
    }
}
```

**Transpiles**: `reader.fieldnames` → `reader.get_headers()?` (instead of default `reader.headers()?`)

## Plugin Architecture

### Design Principles

1. **Simple API**: Just implement 2 methods (`register_mappings`, `name`)
2. **Type Safe**: Rust's type system ensures correctness
3. **Composable**: Load multiple plugins without conflicts
4. **Override Support**: Later plugins override earlier mappings

### Integration Points

Plugins integrate at transpilation time:

```
Python AST → HIR → Stdlib Lookup → Rust Codegen
                        ↑
                   Plugin Mappings
```

When the transpiler encounters `obj.attr`, it:
1. Checks plugin mappings first
2. Falls back to built-in mappings
3. Generates Rust code using `RustPattern`

## Testing Plugins

```rust
#[test]
fn test_my_plugin() {
    let mut mappings = StdlibMappings::new();
    let plugin = MyPlugin;

    mappings.load_plugin(&plugin);

    // Verify mapping registered
    let pattern = mappings.lookup("mylib", "MyClass", "my_method");
    assert!(pattern.is_some());

    // Test code generation
    let rust_code = pattern.unwrap().generate_rust_code("obj", &[]);
    assert_eq!(rust_code, "obj.expected_rust_code()");
}
```

## Built-in Mappings

Depyler includes these built-in mappings:

### CSV Module
- `csv.DictReader.fieldnames` → `reader.headers()?`
- `csv.DictReader.__iter__` → `reader.deserialize::<HashMap<String, String>>()?`

### File I/O
- `file.__iter__` → `BufReader::new(file).lines()`
- `io.TextIOWrapper.__iter__` → `BufReader::new(wrapper).lines()`

## Roadmap

Future enhancements:

- **Dynamic Loading**: Load plugins from `.so`/`.dylib` files
- **Config Files**: TOML/JSON plugin definitions
- **Plugin Registry**: Central repository for community plugins
- **Auto-Discovery**: Automatically find plugins in project

## API Reference

### `StdlibPlugin` Trait

```rust
pub trait StdlibPlugin {
    fn register_mappings(&self, registry: &mut StdlibMappings);
    fn name(&self) -> &str;
    fn version(&self) -> &str { "0.1.0" }
}
```

### `StdlibMappings` Methods

```rust
impl StdlibMappings {
    pub fn new() -> Self;
    pub fn register(&mut self, mapping: StdlibApiMapping);
    pub fn register_batch(&mut self, mappings: Vec<StdlibApiMapping>);
    pub fn load_plugin(&mut self, plugin: &dyn StdlibPlugin);
    pub fn load_plugins(&mut self, plugins: &[&dyn StdlibPlugin]);
    pub fn lookup(&self, module: &str, class: &str, attribute: &str) -> Option<&RustPattern>;
}
```

## Contributing

To contribute a plugin:

1. Implement `StdlibPlugin` trait
2. Add comprehensive tests
3. Document Python→Rust mapping behavior
4. Submit PR to depyler-plugins repo (future)

## License

Plugin system is part of Depyler core, licensed under MIT OR Apache-2.0.
