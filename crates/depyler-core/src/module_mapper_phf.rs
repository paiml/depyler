//! PHF-based Module Mapping with O(1) Worst-Case Lookup
//!
//! DEPYLER-O1MAP-001: Compile-time perfect hash function implementation
//!
//! This module provides a static, compile-time generated perfect hash map
//! for Python-to-Rust module mappings, guaranteeing O(1) worst-case lookup
//! with zero runtime allocation.
//!
//! ## Usage
//!
//! Enable with feature flag:
//! ```toml
//! [dependencies]
//! depyler-core = { version = "3.21", features = ["phf-lookup"] }
//! ```
//!
//! ## Performance
//!
//! | Metric | HashMap | PHF |
//! |--------|---------|-----|
//! | Lookup | O(1) amortized | O(1) worst-case |
//! | Memory | ~25 KB heap | ~8 KB .rodata |
//! | Init | Runtime | Compile-time |

#[cfg(feature = "phf-lookup")]
use phf::phf_map;

/// Static module mapping entry for PHF lookup
#[derive(Debug, Clone, Copy)]
pub struct StaticModuleMapping {
    /// Rust crate or module path
    pub rust_path: &'static str,
    /// Whether this is an external crate
    pub is_external: bool,
    /// Cargo.toml version requirement
    pub version: Option<&'static str>,
}

/// Static item mapping for function/type lookups
#[derive(Debug, Clone, Copy)]
pub struct StaticItemMapping {
    /// Python item name
    pub python_name: &'static str,
    /// Rust equivalent name
    pub rust_name: &'static str,
}

// ============================================================================
// PHF Compile-Time Module Map
// ============================================================================

#[cfg(feature = "phf-lookup")]
static MODULE_MAP: phf::Map<&'static str, StaticModuleMapping> = phf_map! {
    // Standard Library - Core
    "os" => StaticModuleMapping { rust_path: "std", is_external: false, version: None },
    "os.path" => StaticModuleMapping { rust_path: "std::path", is_external: false, version: None },
    "sys" => StaticModuleMapping { rust_path: "std", is_external: false, version: None },
    "io" => StaticModuleMapping { rust_path: "std::io", is_external: false, version: None },
    "math" => StaticModuleMapping { rust_path: "std::f64", is_external: false, version: None },
    "collections" => StaticModuleMapping { rust_path: "std::collections", is_external: false, version: None },
    "typing" => StaticModuleMapping { rust_path: "", is_external: false, version: None },
    "pathlib" => StaticModuleMapping { rust_path: "std::path", is_external: false, version: None },
    "functools" => StaticModuleMapping { rust_path: "std", is_external: false, version: None },
    "subprocess" => StaticModuleMapping { rust_path: "std::process", is_external: false, version: None },
    "threading" => StaticModuleMapping { rust_path: "std::thread", is_external: false, version: None },

    // External Crates - Data Serialization
    "json" => StaticModuleMapping { rust_path: "serde_json", is_external: true, version: Some("1.0") },
    "csv" => StaticModuleMapping { rust_path: "csv", is_external: true, version: Some("1.3") },

    // External Crates - Text Processing
    "re" => StaticModuleMapping { rust_path: "regex", is_external: true, version: Some("1.10") },

    // External Crates - Date/Time
    "datetime" => StaticModuleMapping { rust_path: "chrono", is_external: true, version: Some("0.4") },

    // External Crates - Random/Crypto
    "random" => StaticModuleMapping { rust_path: "rand", is_external: true, version: Some("0.8") },
    "hashlib" => StaticModuleMapping { rust_path: "sha2", is_external: true, version: Some("0.10") },
    "base64" => StaticModuleMapping { rust_path: "base64", is_external: true, version: Some("0.21") },

    // External Crates - CLI
    "argparse" => StaticModuleMapping { rust_path: "clap", is_external: true, version: Some("4.5") },

    // External Crates - Iteration
    "itertools" => StaticModuleMapping { rust_path: "itertools", is_external: true, version: Some("0.12") },

    // External Crates - Async
    "asyncio" => StaticModuleMapping { rust_path: "tokio", is_external: true, version: Some("1.35") },

    // External Crates - Binary
    "struct" => StaticModuleMapping { rust_path: "byteorder", is_external: true, version: Some("1.5") },

    // External Crates - Statistics
    "statistics" => StaticModuleMapping { rust_path: "statrs", is_external: true, version: Some("0.16") },

    // External Crates - Temporary File Operations
    "tempfile" => StaticModuleMapping { rust_path: "tempfile", is_external: true, version: Some("3.0") },

    // External Crates - URL
    "urllib.parse" => StaticModuleMapping { rust_path: "url", is_external: true, version: Some("2.5") },

    // Batuta Stack - NumPy → Trueno
    "numpy" => StaticModuleMapping { rust_path: "trueno", is_external: true, version: Some("0.7") },
    "numpy.linalg" => StaticModuleMapping { rust_path: "trueno::linalg", is_external: true, version: Some("0.7") },

    // Batuta Stack - Sklearn → Aprender
    "sklearn.linear_model" => StaticModuleMapping { rust_path: "aprender::linear", is_external: true, version: Some("0.14") },
    "sklearn.cluster" => StaticModuleMapping { rust_path: "aprender::cluster", is_external: true, version: Some("0.14") },
    "sklearn.tree" => StaticModuleMapping { rust_path: "aprender::tree", is_external: true, version: Some("0.14") },
    "sklearn.ensemble" => StaticModuleMapping { rust_path: "aprender::ensemble", is_external: true, version: Some("0.14") },
    "sklearn.preprocessing" => StaticModuleMapping { rust_path: "aprender::preprocessing", is_external: true, version: Some("0.14") },
    "sklearn.decomposition" => StaticModuleMapping { rust_path: "aprender::decomposition", is_external: true, version: Some("0.14") },
    "sklearn.model_selection" => StaticModuleMapping { rust_path: "aprender::model_selection", is_external: true, version: Some("0.14") },
    "sklearn.metrics" => StaticModuleMapping { rust_path: "aprender::metrics", is_external: true, version: Some("0.14") },
};

// ============================================================================
// PHF Item Maps (Function/Type Mappings)
// ============================================================================

#[cfg(feature = "phf-lookup")]
static JSON_ITEMS: phf::Map<&'static str, &'static str> = phf_map! {
    "loads" => "from_str",
    "dumps" => "to_string",
    "load" => "from_reader",
    "dump" => "to_writer",
};

#[cfg(feature = "phf-lookup")]
static MATH_ITEMS: phf::Map<&'static str, &'static str> = phf_map! {
    "sqrt" => "sqrt",
    "sin" => "sin",
    "cos" => "cos",
    "tan" => "tan",
    "floor" => "floor",
    "ceil" => "ceil",
    "abs" => "abs",
    "pow" => "powf",
    "pi" => "consts::PI",
    "e" => "consts::E",
    // DEPYLER-0771: isqrt is handled specially in expr_gen.rs (not a direct method call)
    "isqrt" => "isqrt",
};

#[cfg(feature = "phf-lookup")]
static OS_ITEMS: phf::Map<&'static str, &'static str> = phf_map! {
    "getcwd" => "env::current_dir",
    "environ" => "env::vars",
    "path" => "path::Path",
    "getenv" => "env::var",
};

#[cfg(feature = "phf-lookup")]
static SYS_ITEMS: phf::Map<&'static str, &'static str> = phf_map! {
    "argv" => "env::args",
    "exit" => "process::exit",
    "stdin" => "io::stdin",
    "stdout" => "io::stdout",
    "stderr" => "io::stderr",
};

#[cfg(feature = "phf-lookup")]
static RE_ITEMS: phf::Map<&'static str, &'static str> = phf_map! {
    "compile" => "Regex::new",
    "search" => "Regex::find",
    "match" => "Regex::is_match",
    "findall" => "Regex::find_iter",
    "finditer" => "Regex::find_iter",
    "sub" => "Regex::replace_all",
    "subn" => "Regex::replace_all",
    "split" => "Regex::split",
    "Pattern" => "Regex",
    "IGNORECASE" => "(?i)",
    "I" => "(?i)",
    "MULTILINE" => "(?m)",
    "M" => "(?m)",
};

#[cfg(feature = "phf-lookup")]
static RANDOM_ITEMS: phf::Map<&'static str, &'static str> = phf_map! {
    "random" => "random",
    "randint" => "gen_range",
    "choice" => "choose",
    "shuffle" => "shuffle",
    "uniform" => "gen_range",
    "seed" => "SeedableRng::seed_from_u64",
    "randrange" => "gen_range",
    "sample" => "choose_multiple",
};

#[cfg(feature = "phf-lookup")]
static NUMPY_ITEMS: phf::Map<&'static str, &'static str> = phf_map! {
    "array" => "Vector::from_slice",
    "zeros" => "Vector::zeros",
    "ones" => "Vector::ones",
    "empty" => "Vector::zeros",
    "arange" => "Vector::arange",
    "linspace" => "Vector::linspace",
    "add" => "Vector::add",
    "subtract" => "Vector::sub",
    "multiply" => "Vector::mul",
    "divide" => "Vector::div",
    "sqrt" => "Vector::sqrt",
    "exp" => "Vector::exp",
    "log" => "Vector::ln",
    "sin" => "Vector::sin",
    "cos" => "Vector::cos",
    "abs" => "Vector::abs",
    "dot" => "Vector::dot",
    "matmul" => "Matrix::matmul",
    "sum" => "Vector::sum",
    "mean" => "Vector::mean",
    "max" => "Vector::max",
    "min" => "Vector::min",
    "std" => "Vector::std",
    "var" => "Vector::var",
    "argmax" => "Vector::argmax",
    "argmin" => "Vector::argmin",
};

// ============================================================================
// Public API
// ============================================================================

/// O(1) worst-case module lookup using PHF
///
/// # Example
///
/// ```rust,ignore
/// use depyler_core::module_mapper_phf::get_module_mapping;
///
/// if let Some(mapping) = get_module_mapping("json") {
///     assert_eq!(mapping.rust_path, "serde_json");
///     assert!(mapping.is_external);
/// }
/// ```
#[cfg(feature = "phf-lookup")]
pub fn get_module_mapping(module: &str) -> Option<&'static StaticModuleMapping> {
    MODULE_MAP.get(module)
}

/// O(1) worst-case item lookup within a module using PHF
///
/// # Example
///
/// ```rust,ignore
/// use depyler_core::module_mapper_phf::get_item_mapping;
///
/// if let Some(rust_name) = get_item_mapping("json", "loads") {
///     assert_eq!(rust_name, "from_str");
/// }
/// ```
#[cfg(feature = "phf-lookup")]
pub fn get_item_mapping(module: &str, item: &str) -> Option<&'static str> {
    match module {
        "json" => JSON_ITEMS.get(item).copied(),
        "math" => MATH_ITEMS.get(item).copied(),
        "os" => OS_ITEMS.get(item).copied(),
        "sys" => SYS_ITEMS.get(item).copied(),
        "re" => RE_ITEMS.get(item).copied(),
        "random" => RANDOM_ITEMS.get(item).copied(),
        "numpy" => NUMPY_ITEMS.get(item).copied(),
        _ => None,
    }
}

/// Check if a module is supported by PHF lookup
#[cfg(feature = "phf-lookup")]
pub fn is_module_supported(module: &str) -> bool {
    MODULE_MAP.contains_key(module)
}

/// Get all supported module names (for diagnostics)
#[cfg(feature = "phf-lookup")]
pub fn supported_modules() -> impl Iterator<Item = &'static str> {
    MODULE_MAP.keys().copied()
}

// ============================================================================
// Fallback for non-PHF builds
// ============================================================================

#[cfg(not(feature = "phf-lookup"))]
pub fn get_module_mapping(_module: &str) -> Option<&'static StaticModuleMapping> {
    None // Use HashMap-based ModuleMapper instead
}

#[cfg(not(feature = "phf-lookup"))]
pub fn get_item_mapping(_module: &str, _item: &str) -> Option<&'static str> {
    None // Use HashMap-based ModuleMapper instead
}

#[cfg(not(feature = "phf-lookup"))]
pub fn is_module_supported(_module: &str) -> bool {
    false
}

#[cfg(not(feature = "phf-lookup"))]
pub fn supported_modules() -> impl Iterator<Item = &'static str> {
    std::iter::empty()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod fallback_tests {
    use super::*;

    // Tests for non-PHF fallback mode (always present)
    #[cfg(not(feature = "phf-lookup"))]
    #[test]
    fn test_fallback_get_module_mapping() {
        assert!(get_module_mapping("json").is_none());
        assert!(get_module_mapping("os").is_none());
    }

    #[cfg(not(feature = "phf-lookup"))]
    #[test]
    fn test_fallback_get_item_mapping() {
        assert!(get_item_mapping("json", "loads").is_none());
        assert!(get_item_mapping("math", "sqrt").is_none());
    }

    #[cfg(not(feature = "phf-lookup"))]
    #[test]
    fn test_fallback_is_module_supported() {
        assert!(!is_module_supported("json"));
        assert!(!is_module_supported("os"));
    }

    #[cfg(not(feature = "phf-lookup"))]
    #[test]
    fn test_fallback_supported_modules() {
        assert_eq!(supported_modules().count(), 0);
    }

    // Test StaticModuleMapping and StaticItemMapping structs
    #[test]
    fn test_static_module_mapping_struct() {
        let mapping = StaticModuleMapping {
            rust_path: "serde_json",
            is_external: true,
            version: Some("1.0"),
        };
        assert_eq!(mapping.rust_path, "serde_json");
        assert!(mapping.is_external);
        assert_eq!(mapping.version, Some("1.0"));
    }

    #[test]
    fn test_static_item_mapping_struct() {
        let mapping = StaticItemMapping {
            python_name: "loads",
            rust_name: "from_str",
        };
        assert_eq!(mapping.python_name, "loads");
        assert_eq!(mapping.rust_name, "from_str");
    }
}

#[cfg(test)]
#[cfg(feature = "phf-lookup")]
mod tests {
    use super::*;

    #[test]
    fn test_module_lookup_json() {
        let mapping = get_module_mapping("json").expect("json should be mapped");
        assert_eq!(mapping.rust_path, "serde_json");
        assert!(mapping.is_external);
        assert_eq!(mapping.version, Some("1.0"));
    }

    #[test]
    fn test_module_lookup_stdlib() {
        let mapping = get_module_mapping("os").expect("os should be mapped");
        assert_eq!(mapping.rust_path, "std");
        assert!(!mapping.is_external);
        assert_eq!(mapping.version, None);
    }

    #[test]
    fn test_module_lookup_numpy() {
        let mapping = get_module_mapping("numpy").expect("numpy should be mapped");
        assert_eq!(mapping.rust_path, "trueno");
        assert!(mapping.is_external);
    }

    #[test]
    fn test_module_lookup_sklearn() {
        let mapping = get_module_mapping("sklearn.linear_model")
            .expect("sklearn.linear_model should be mapped");
        assert_eq!(mapping.rust_path, "aprender::linear");
        assert!(mapping.is_external);
    }

    #[test]
    fn test_item_lookup_json() {
        assert_eq!(get_item_mapping("json", "loads"), Some("from_str"));
        assert_eq!(get_item_mapping("json", "dumps"), Some("to_string"));
    }

    #[test]
    fn test_item_lookup_math() {
        assert_eq!(get_item_mapping("math", "sqrt"), Some("sqrt"));
        assert_eq!(get_item_mapping("math", "pi"), Some("consts::PI"));
    }

    #[test]
    fn test_item_lookup_numpy() {
        assert_eq!(
            get_item_mapping("numpy", "array"),
            Some("Vector::from_slice")
        );
        assert_eq!(get_item_mapping("numpy", "sum"), Some("Vector::sum"));
    }

    #[test]
    fn test_unknown_module() {
        assert!(get_module_mapping("unknown_module").is_none());
    }

    #[test]
    fn test_unknown_item() {
        assert!(get_item_mapping("json", "unknown_func").is_none());
    }

    #[test]
    fn test_supported_modules_count() {
        let count = supported_modules().count();
        assert!(
            count >= 30,
            "Should have at least 30 modules, got {}",
            count
        );
    }
}
