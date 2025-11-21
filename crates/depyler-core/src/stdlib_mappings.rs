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
    CustomTemplate {
        template: &'static str,
    },
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
    fn register_csv_mappings(
        mappings: &mut HashMap<(String, String, String), RustPattern>,
    ) {
        // csv.DictReader.fieldnames → reader.headers()?
        mappings.insert(
            ("csv".to_string(), "DictReader".to_string(), "fieldnames".to_string()),
            RustPattern::PropertyToMethod {
                method: "headers",
                propagate_error: true,
            },
        );

        // csv.DictReader iteration → deserialize::<HashMap<String, String>>()
        mappings.insert(
            ("csv".to_string(), "DictReader".to_string(), "__iter__".to_string()),
            RustPattern::IterationPattern {
                iter_method: "deserialize",
                element_type: Some("HashMap<String, String>"),
                yields_results: true,
            },
        );

        // csv.Reader.fieldnames (also support basic Reader)
        mappings.insert(
            ("csv".to_string(), "Reader".to_string(), "fieldnames".to_string()),
            RustPattern::PropertyToMethod {
                method: "headers",
                propagate_error: true,
            },
        );
    }

    /// Register file I/O API mappings
    fn register_file_mappings(
        mappings: &mut HashMap<(String, String, String), RustPattern>,
    ) {
        // File iteration: for line in file
        mappings.insert(
            ("builtins".to_string(), "file".to_string(), "__iter__".to_string()),
            RustPattern::CustomTemplate {
                template: "BufReader::new({var}).lines()",
            },
        );

        // TextIOWrapper iteration (from open())
        mappings.insert(
            ("io".to_string(), "TextIOWrapper".to_string(), "__iter__".to_string()),
            RustPattern::CustomTemplate {
                template: "BufReader::new({var}).lines()",
            },
        );
    }

    /// Look up mapping for a Python API call
    pub fn lookup(
        &self,
        module: &str,
        class: &str,
        attribute: &str,
    ) -> Option<&RustPattern> {
        self.mappings.get(&(
            module.to_string(),
            class.to_string(),
            attribute.to_string(),
        ))
    }

    /// Check if a class has iteration mapping
    pub fn has_iteration_mapping(&self, module: &str, class: &str) -> bool {
        self.lookup(module, class, "__iter__").is_some()
    }

    /// Get iteration pattern for a class
    pub fn get_iteration_pattern(&self, module: &str, class: &str) -> Option<&RustPattern> {
        self.lookup(module, class, "__iter__")
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

            RustPattern::CustomTemplate { template } => {
                template.replace("{var}", base_expr)
            }
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
}
