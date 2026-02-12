// DEPYLER-0452: Stdlib API Mapping System
//
// Maps Python stdlib API patterns to correct Rust equivalents.
// Handles cases where Python and Rust APIs don't have 1:1 correspondence.
//
// Example: csv.DictReader.fieldnames → reader.headers()?
//
// Created: 2025-11-21

use std::collections::HashMap;

/// Represents a mapping from Python stdlib API to Rust code pattern
#[derive(Debug, Clone)]
pub struct StdlibApiMapping {
    /// Python module (e.g., "csv", "os", "json")
    pub module: &'static str,
    /// Python class/type (e.g., "DictReader", "Path")
    pub class: &'static str,
    /// Python attribute/method (e.g., "fieldnames", "__iter__")
    pub python_attr: &'static str,
    /// Rust code generation pattern
    pub rust_pattern: RustPattern,
}

/// Defines how to generate Rust code for a Python pattern
#[derive(Debug, Clone)]
pub enum RustPattern {
    /// Simple method call: obj.method(args)
    MethodCall {
        method: &'static str,
        /// Additional arguments to insert (beyond original)
        extra_args: Vec<&'static str>,
        /// Whether to add ? for Result handling
        propagate_error: bool,
    },

    /// Property access becomes method call: obj.property → obj.method()
    PropertyToMethod {
        method: &'static str,
        propagate_error: bool,
    },

    /// Custom iteration pattern
    IterationPattern {
        /// Iterator method to call
        iter_method: &'static str,
        /// Element type for deserialize patterns
        element_type: Option<&'static str>,
        /// Whether iteration yields Results
        yields_results: bool,
    },

    /// Custom code template with {var} placeholders
    CustomTemplate { template: &'static str },
}

/// Plugin trait for extending stdlib mappings
///
/// Implement this trait to add custom Python→Rust API mappings.
///
/// # Example
/// ```rust
/// use depyler_core::stdlib_mappings::{StdlibPlugin, StdlibMappings, StdlibApiMapping, RustPattern};
///
/// struct RequestsPlugin;
///
/// impl StdlibPlugin for RequestsPlugin {
///     fn register_mappings(&self, registry: &mut StdlibMappings) {
///         registry.register(StdlibApiMapping {
///             module: "requests",
///             class: "Session",
///             python_attr: "get",
///             rust_pattern: RustPattern::MethodCall {
///                 method: "get",
///                 extra_args: vec![],
///                 propagate_error: true,
///             },
///         });
///     }
///
///     fn name(&self) -> &str {
///         "requests"
///     }
/// }
/// ```
pub trait StdlibPlugin {
    /// Register this plugin's mappings into the registry
    fn register_mappings(&self, registry: &mut StdlibMappings);

    /// Plugin name for identification
    fn name(&self) -> &str;

    /// Optional: Plugin version
    fn version(&self) -> &str {
        "0.1.0"
    }
}

/// Stdlib API mapping registry
pub struct StdlibMappings {
    /// Lookup by (module, class, attribute)
    mappings: HashMap<(String, String, String), RustPattern>,
}

impl StdlibMappings {
    /// Create a new mapping registry with built-in stdlib mappings
    pub fn new() -> Self {
        let mut mappings = HashMap::new();

        // CSV module mappings
        Self::register_csv_mappings(&mut mappings);

        // File I/O mappings
        Self::register_file_mappings(&mut mappings);

        Self { mappings }
    }

    /// Register CSV module API mappings
    fn register_csv_mappings(mappings: &mut HashMap<(String, String, String), RustPattern>) {
        // csv.DictReader.fieldnames → reader.headers()?
        mappings.insert(
            (
                "csv".to_string(),
                "DictReader".to_string(),
                "fieldnames".to_string(),
            ),
            RustPattern::PropertyToMethod {
                method: "headers",
                propagate_error: true,
            },
        );

        // csv.DictReader iteration → deserialize::<HashMap<String, String>>()
        mappings.insert(
            (
                "csv".to_string(),
                "DictReader".to_string(),
                "__iter__".to_string(),
            ),
            RustPattern::IterationPattern {
                iter_method: "deserialize",
                element_type: Some("HashMap<String, String>"),
                yields_results: true,
            },
        );

        // csv.Reader.fieldnames (also support basic Reader)
        mappings.insert(
            (
                "csv".to_string(),
                "Reader".to_string(),
                "fieldnames".to_string(),
            ),
            RustPattern::PropertyToMethod {
                method: "headers",
                propagate_error: true,
            },
        );
    }

    /// Register file I/O API mappings
    fn register_file_mappings(mappings: &mut HashMap<(String, String, String), RustPattern>) {
        // File iteration: for line in file
        mappings.insert(
            (
                "builtins".to_string(),
                "file".to_string(),
                "__iter__".to_string(),
            ),
            RustPattern::CustomTemplate {
                template: "BufReader::new({var}).lines()",
            },
        );

        // TextIOWrapper iteration (from open())
        mappings.insert(
            (
                "io".to_string(),
                "TextIOWrapper".to_string(),
                "__iter__".to_string(),
            ),
            RustPattern::CustomTemplate {
                template: "BufReader::new({var}).lines()",
            },
        );
    }

    /// Look up mapping for a Python API call
    pub fn lookup(&self, module: &str, class: &str, attribute: &str) -> Option<&RustPattern> {
        self.mappings
            .get(&(module.to_string(), class.to_string(), attribute.to_string()))
    }

    /// Check if a class has iteration mapping
    pub fn has_iteration_mapping(&self, module: &str, class: &str) -> bool {
        self.lookup(module, class, "__iter__").is_some()
    }

    /// Get iteration pattern for a class
    pub fn get_iteration_pattern(&self, module: &str, class: &str) -> Option<&RustPattern> {
        self.lookup(module, class, "__iter__")
    }

    /// Register a custom mapping (public API for plugins)
    ///
    /// # Example
    /// ```rust
    /// use depyler_core::stdlib_mappings::{StdlibMappings, StdlibApiMapping, RustPattern};
    ///
    /// let mut mappings = StdlibMappings::new();
    /// mappings.register(StdlibApiMapping {
    ///     module: "requests",
    ///     class: "Session",
    ///     python_attr: "get",
    ///     rust_pattern: RustPattern::MethodCall {
    ///         method: "get",
    ///         extra_args: vec![],
    ///         propagate_error: true,
    ///     },
    /// });
    /// ```
    pub fn register(&mut self, mapping: StdlibApiMapping) {
        let key = (
            mapping.module.to_string(),
            mapping.class.to_string(),
            mapping.python_attr.to_string(),
        );
        self.mappings.insert(key, mapping.rust_pattern);
    }

    /// Register multiple mappings at once
    pub fn register_batch(&mut self, mappings: Vec<StdlibApiMapping>) {
        for mapping in mappings {
            self.register(mapping);
        }
    }

    /// Load a plugin into the registry
    ///
    /// # Example
    /// ```rust
    /// use depyler_core::stdlib_mappings::{StdlibMappings, StdlibPlugin};
    ///
    /// struct MyPlugin;
    /// impl StdlibPlugin for MyPlugin {
    ///     fn register_mappings(&self, registry: &mut StdlibMappings) {
    ///         // Register custom mappings
    ///     }
    ///     fn name(&self) -> &str { "my_plugin" }
    /// }
    ///
    /// let mut mappings = StdlibMappings::new();
    /// mappings.load_plugin(&MyPlugin);
    /// ```
    pub fn load_plugin(&mut self, plugin: &dyn StdlibPlugin) {
        plugin.register_mappings(self);
    }

    /// Load multiple plugins at once
    pub fn load_plugins(&mut self, plugins: &[&dyn StdlibPlugin]) {
        for plugin in plugins {
            self.load_plugin(*plugin);
        }
    }
}

impl Default for StdlibMappings {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for generating Rust code from patterns
impl RustPattern {
    /// Generate Rust code for this pattern
    ///
    /// # Arguments
    /// * `base_expr` - The object/variable being operated on
    /// * `original_args` - Original arguments from Python call
    pub fn generate_rust_code(&self, base_expr: &str, original_args: &[String]) -> String {
        match self {
            RustPattern::MethodCall {
                method,
                extra_args,
                propagate_error,
            } => {
                let mut all_args = original_args.to_vec();
                all_args.extend(extra_args.iter().map(|s| s.to_string()));
                let args_str = all_args.join(", ");
                let call = if args_str.is_empty() {
                    format!("{}.{}()", base_expr, method)
                } else {
                    format!("{}.{}({})", base_expr, method, args_str)
                };

                if *propagate_error {
                    format!("{}?", call)
                } else {
                    call
                }
            }

            RustPattern::PropertyToMethod {
                method,
                propagate_error,
            } => {
                let call = format!("{}.{}()", base_expr, method);
                if *propagate_error {
                    format!("{}?", call)
                } else {
                    call
                }
            }

            RustPattern::IterationPattern {
                iter_method,
                element_type,
                yields_results: _,
            } => {
                if let Some(elem_type) = element_type {
                    format!("{}.{}::<{}>()", base_expr, iter_method, elem_type)
                } else {
                    format!("{}.{}()", base_expr, iter_method)
                }
            }

            RustPattern::CustomTemplate { template } => template.replace("{var}", base_expr),
        }
    }

    /// Check if this pattern yields Result types in iteration
    pub fn yields_results(&self) -> bool {
        matches!(
            self,
            RustPattern::IterationPattern {
                yields_results: true,
                ..
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_fieldnames_mapping() {
        let mappings = StdlibMappings::new();
        let pattern = mappings.lookup("csv", "DictReader", "fieldnames");
        assert!(pattern.is_some());

        let rust_code = pattern.unwrap().generate_rust_code("reader", &[]);
        assert_eq!(rust_code, "reader.headers()?");
    }

    #[test]
    fn test_csv_iteration_mapping() {
        let mappings = StdlibMappings::new();
        let pattern = mappings.get_iteration_pattern("csv", "DictReader");
        assert!(pattern.is_some());

        let rust_code = pattern.unwrap().generate_rust_code("reader", &[]);
        assert_eq!(rust_code, "reader.deserialize::<HashMap<String, String>>()");
    }

    #[test]
    fn test_file_iteration_mapping() {
        let mappings = StdlibMappings::new();
        let pattern = mappings.lookup("builtins", "file", "__iter__");
        assert!(pattern.is_some());

        let rust_code = pattern.unwrap().generate_rust_code("f", &[]);
        assert_eq!(rust_code, "BufReader::new(f).lines()");
    }

    // DEPYLER-0506: Plugin system tests

    #[test]
    fn test_register_custom_mapping() {
        let mut mappings = StdlibMappings::new();

        // Register custom mapping
        mappings.register(StdlibApiMapping {
            module: "requests",
            class: "Session",
            python_attr: "get",
            rust_pattern: RustPattern::MethodCall {
                method: "get",
                extra_args: vec![],
                propagate_error: true,
            },
        });

        // Verify it's registered
        let pattern = mappings.lookup("requests", "Session", "get");
        assert!(pattern.is_some());

        let rust_code = pattern.unwrap().generate_rust_code("session", &[]);
        assert_eq!(rust_code, "session.get()?");
    }

    #[test]
    fn test_register_batch() {
        let mut mappings = StdlibMappings::new();

        let batch = vec![
            StdlibApiMapping {
                module: "numpy",
                class: "ndarray",
                python_attr: "shape",
                rust_pattern: RustPattern::PropertyToMethod {
                    method: "shape",
                    propagate_error: false,
                },
            },
            StdlibApiMapping {
                module: "numpy",
                class: "ndarray",
                python_attr: "dtype",
                rust_pattern: RustPattern::PropertyToMethod {
                    method: "dtype",
                    propagate_error: false,
                },
            },
        ];

        mappings.register_batch(batch);

        assert!(mappings.lookup("numpy", "ndarray", "shape").is_some());
        assert!(mappings.lookup("numpy", "ndarray", "dtype").is_some());
    }

    // Example plugin for testing
    struct TestRequestsPlugin;

    impl StdlibPlugin for TestRequestsPlugin {
        fn register_mappings(&self, registry: &mut StdlibMappings) {
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

    #[test]
    fn test_load_plugin() {
        let mut mappings = StdlibMappings::new();
        let plugin = TestRequestsPlugin;

        mappings.load_plugin(&plugin);

        // Verify plugin mappings registered
        assert!(mappings.lookup("requests", "Session", "get").is_some());
        assert!(mappings.lookup("requests", "Session", "post").is_some());

        // Test code generation
        let get_pattern = mappings.lookup("requests", "Session", "get").unwrap();
        assert_eq!(
            get_pattern.generate_rust_code("session", &[]),
            "session.get()?"
        );
    }

    struct TestNumpyPlugin;

    impl StdlibPlugin for TestNumpyPlugin {
        fn register_mappings(&self, registry: &mut StdlibMappings) {
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

    #[test]
    fn test_load_multiple_plugins() {
        let mut mappings = StdlibMappings::new();
        let requests_plugin = TestRequestsPlugin;
        let numpy_plugin = TestNumpyPlugin;

        mappings.load_plugins(&[&requests_plugin, &numpy_plugin]);

        // Verify both plugins loaded
        assert!(mappings.lookup("requests", "Session", "get").is_some());
        assert!(mappings.lookup("numpy", "ndarray", "reshape").is_some());
    }

    #[test]
    fn test_plugin_override_builtin() {
        let mut mappings = StdlibMappings::new();

        // Built-in csv mapping exists
        assert!(mappings.lookup("csv", "DictReader", "fieldnames").is_some());

        // Plugin can override it
        struct OverridePlugin;
        impl StdlibPlugin for OverridePlugin {
            fn register_mappings(&self, registry: &mut StdlibMappings) {
                registry.register(StdlibApiMapping {
                    module: "csv",
                    class: "DictReader",
                    python_attr: "fieldnames",
                    rust_pattern: RustPattern::PropertyToMethod {
                        method: "get_headers", // Different method
                        propagate_error: true,
                    },
                });
            }
            fn name(&self) -> &str {
                "csv_override"
            }
        }

        mappings.load_plugin(&OverridePlugin);

        // Verify override worked
        let pattern = mappings.lookup("csv", "DictReader", "fieldnames").unwrap();
        let code = pattern.generate_rust_code("reader", &[]);
        assert_eq!(code, "reader.get_headers()?");
    }

    // ============================================================
    // DEPYLER-COVERAGE-95: Additional comprehensive tests
    // ============================================================

    #[test]
    fn test_stdlib_mappings_default() {
        let mappings = StdlibMappings::default();
        // Should have built-in CSV mappings
        assert!(mappings.lookup("csv", "DictReader", "fieldnames").is_some());
        assert!(mappings.lookup("csv", "DictReader", "__iter__").is_some());
    }

    #[test]
    fn test_stdlib_api_mapping_clone() {
        let mapping = StdlibApiMapping {
            module: "csv",
            class: "Reader",
            python_attr: "test",
            rust_pattern: RustPattern::PropertyToMethod {
                method: "test",
                propagate_error: false,
            },
        };
        let cloned = mapping.clone();
        assert_eq!(cloned.module, "csv");
        assert_eq!(cloned.class, "Reader");
    }

    #[test]
    fn test_stdlib_api_mapping_debug() {
        let mapping = StdlibApiMapping {
            module: "csv",
            class: "Reader",
            python_attr: "test",
            rust_pattern: RustPattern::PropertyToMethod {
                method: "test",
                propagate_error: false,
            },
        };
        let debug = format!("{:?}", mapping);
        assert!(debug.contains("csv"));
        assert!(debug.contains("Reader"));
    }

    #[test]
    fn test_rust_pattern_debug() {
        let pattern = RustPattern::MethodCall {
            method: "test",
            extra_args: vec![],
            propagate_error: false,
        };
        let debug = format!("{:?}", pattern);
        assert!(debug.contains("MethodCall"));
    }

    #[test]
    fn test_rust_pattern_clone() {
        let pattern = RustPattern::CustomTemplate {
            template: "test({var})",
        };
        let cloned = pattern.clone();
        if let RustPattern::CustomTemplate { template } = cloned {
            assert_eq!(template, "test({var})");
        } else {
            panic!("Clone should preserve variant");
        }
    }

    #[test]
    fn test_method_call_with_args() {
        let pattern = RustPattern::MethodCall {
            method: "fetch",
            extra_args: vec![],
            propagate_error: false,
        };
        let code = pattern.generate_rust_code("client", &["url".to_string()]);
        assert_eq!(code, "client.fetch(url)");
    }

    #[test]
    fn test_method_call_with_extra_args() {
        let pattern = RustPattern::MethodCall {
            method: "fetch",
            extra_args: vec!["timeout"],
            propagate_error: false,
        };
        let code = pattern.generate_rust_code("client", &["url".to_string()]);
        assert_eq!(code, "client.fetch(url, timeout)");
    }

    #[test]
    fn test_method_call_no_propagate_error() {
        let pattern = RustPattern::MethodCall {
            method: "get",
            extra_args: vec![],
            propagate_error: false,
        };
        let code = pattern.generate_rust_code("obj", &[]);
        assert_eq!(code, "obj.get()");
    }

    #[test]
    fn test_method_call_propagate_error() {
        let pattern = RustPattern::MethodCall {
            method: "get",
            extra_args: vec![],
            propagate_error: true,
        };
        let code = pattern.generate_rust_code("obj", &[]);
        assert_eq!(code, "obj.get()?");
    }

    #[test]
    fn test_property_to_method_no_error() {
        let pattern = RustPattern::PropertyToMethod {
            method: "len",
            propagate_error: false,
        };
        let code = pattern.generate_rust_code("list", &[]);
        assert_eq!(code, "list.len()");
    }

    #[test]
    fn test_property_to_method_with_error() {
        let pattern = RustPattern::PropertyToMethod {
            method: "headers",
            propagate_error: true,
        };
        let code = pattern.generate_rust_code("reader", &[]);
        assert_eq!(code, "reader.headers()?");
    }

    #[test]
    fn test_iteration_pattern_no_element_type() {
        let pattern = RustPattern::IterationPattern {
            iter_method: "iter",
            element_type: None,
            yields_results: false,
        };
        let code = pattern.generate_rust_code("collection", &[]);
        assert_eq!(code, "collection.iter()");
    }

    #[test]
    fn test_iteration_pattern_with_element_type() {
        let pattern = RustPattern::IterationPattern {
            iter_method: "deserialize",
            element_type: Some("Record"),
            yields_results: true,
        };
        let code = pattern.generate_rust_code("reader", &[]);
        assert_eq!(code, "reader.deserialize::<Record>()");
    }

    #[test]
    fn test_custom_template_with_var() {
        let pattern = RustPattern::CustomTemplate {
            template: "Box::new({var})",
        };
        let code = pattern.generate_rust_code("value", &[]);
        assert_eq!(code, "Box::new(value)");
    }

    #[test]
    fn test_custom_template_multiple_vars() {
        let pattern = RustPattern::CustomTemplate {
            template: "process({var}).map(|x| x + {var})",
        };
        let code = pattern.generate_rust_code("n", &[]);
        assert_eq!(code, "process(n).map(|x| x + n)");
    }

    #[test]
    fn test_yields_results_true() {
        let pattern = RustPattern::IterationPattern {
            iter_method: "deserialize",
            element_type: Some("Row"),
            yields_results: true,
        };
        assert!(pattern.yields_results());
    }

    #[test]
    fn test_yields_results_false() {
        let pattern = RustPattern::IterationPattern {
            iter_method: "iter",
            element_type: None,
            yields_results: false,
        };
        assert!(!pattern.yields_results());
    }

    #[test]
    fn test_yields_results_method_call() {
        let pattern = RustPattern::MethodCall {
            method: "test",
            extra_args: vec![],
            propagate_error: true,
        };
        assert!(!pattern.yields_results());
    }

    #[test]
    fn test_yields_results_property_to_method() {
        let pattern = RustPattern::PropertyToMethod {
            method: "test",
            propagate_error: true,
        };
        assert!(!pattern.yields_results());
    }

    #[test]
    fn test_yields_results_custom_template() {
        let pattern = RustPattern::CustomTemplate {
            template: "{var}.iter()",
        };
        assert!(!pattern.yields_results());
    }

    #[test]
    fn test_has_iteration_mapping_true() {
        let mappings = StdlibMappings::new();
        assert!(mappings.has_iteration_mapping("csv", "DictReader"));
    }

    #[test]
    fn test_has_iteration_mapping_false() {
        let mappings = StdlibMappings::new();
        assert!(!mappings.has_iteration_mapping("unknown", "Unknown"));
    }

    #[test]
    fn test_lookup_nonexistent() {
        let mappings = StdlibMappings::new();
        assert!(mappings.lookup("nonexistent", "Foo", "bar").is_none());
    }

    #[test]
    fn test_csv_reader_fieldnames() {
        let mappings = StdlibMappings::new();
        let pattern = mappings.lookup("csv", "Reader", "fieldnames");
        assert!(pattern.is_some());
        let code = pattern.unwrap().generate_rust_code("reader", &[]);
        assert_eq!(code, "reader.headers()?");
    }

    #[test]
    fn test_io_text_wrapper_iteration() {
        let mappings = StdlibMappings::new();
        let pattern = mappings.lookup("io", "TextIOWrapper", "__iter__");
        assert!(pattern.is_some());
        let code = pattern.unwrap().generate_rust_code("file", &[]);
        assert_eq!(code, "BufReader::new(file).lines()");
    }

    #[test]
    fn test_plugin_default_version() {
        struct MinimalPlugin;
        impl StdlibPlugin for MinimalPlugin {
            fn register_mappings(&self, _registry: &mut StdlibMappings) {}
            fn name(&self) -> &str {
                "minimal"
            }
        }
        let plugin = MinimalPlugin;
        assert_eq!(plugin.version(), "0.1.0");
        assert_eq!(plugin.name(), "minimal");
    }

    #[test]
    fn test_plugin_custom_version() {
        assert_eq!(TestRequestsPlugin.version(), "1.0.0");
    }

    #[test]
    fn test_get_iteration_pattern_nonexistent() {
        let mappings = StdlibMappings::new();
        assert!(mappings
            .get_iteration_pattern("unknown", "Unknown")
            .is_none());
    }

    #[test]
    fn test_method_call_with_multiple_extra_args() {
        let pattern = RustPattern::MethodCall {
            method: "request",
            extra_args: vec!["headers", "timeout"],
            propagate_error: true,
        };
        let code = pattern.generate_rust_code("client", &["url".to_string()]);
        assert_eq!(code, "client.request(url, headers, timeout)?");
    }

    #[test]
    fn test_empty_module_lookup() {
        let mappings = StdlibMappings::new();
        assert!(mappings.lookup("", "DictReader", "fieldnames").is_none());
    }

    #[test]
    fn test_empty_class_lookup() {
        let mappings = StdlibMappings::new();
        assert!(mappings.lookup("csv", "", "fieldnames").is_none());
    }

    #[test]
    fn test_empty_attribute_lookup() {
        let mappings = StdlibMappings::new();
        assert!(mappings.lookup("csv", "DictReader", "").is_none());
    }
}
