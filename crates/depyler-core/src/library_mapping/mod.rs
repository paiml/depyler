//! DEPYLER-0903: Enterprise Library Mapping System
//!
//! Deterministic, extensible system for mapping Python external libraries to Rust equivalents.
//! Uses O(1) hash table lookup with priority chain: overrides > extensions > core.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │                  MappingRegistry                     │
//! ├─────────────────────────────────────────────────────┤
//! │  Priority 1: User Overrides (highest)               │
//! │  Priority 2: Enterprise Extensions                  │
//! │  Priority 3: Core Mappings (shipped with depyler)   │
//! └─────────────────────────────────────────────────────┘
//! ```
//!
//! # References
//!
//! - [1] CLRS: O(1) hash table lookup
//! - [26] Parnas: Information hiding principle
//! - [27] Fredman et al.: Perfect hashing

// Allow deprecated Template variant for backwards compatibility
#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod toml_plugin;

#[cfg(test)]
mod tests;

// ============================================================================
// Core Data Structures (Section 2.1 of spec)
// ============================================================================

/// A deterministic mapping from Python library to Rust equivalent.
///
/// This is a pure function: f(python_module, python_item) → rust_equivalent
/// No randomness, no learning, no approximation.
///
/// Design follows Parnas's information hiding principle [26].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LibraryMapping {
    /// Python module path (e.g., "pandas", "numpy.linalg")
    pub python_module: String,

    /// Rust crate and module path (e.g., "polars", "ndarray::linalg")
    pub rust_crate: String,

    /// Python version requirement (e.g., ">=3.8" or "*")
    pub python_version_req: String,

    /// Rust crate version constraint (semver)
    pub rust_crate_version: String,

    /// Item-level mappings: Python name → Rust mapping
    pub items: HashMap<String, ItemMapping>,

    /// Required Cargo.toml features
    pub features: Vec<String>,

    /// Mapping confidence level
    pub confidence: MappingConfidence,

    /// Source of mapping (documentation URL, RFC, etc.)
    pub provenance: String,
}

/// Individual item mapping within a library
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemMapping {
    /// Rust equivalent name
    pub rust_name: String,

    /// Transformation pattern
    pub pattern: TransformPattern,

    /// Type signature transformation (optional)
    pub type_transform: Option<TypeTransform>,
}

/// Transformation patterns for Python→Rust mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(tag = "type")]
pub enum TransformPattern {
    /// Direct 1:1 rename
    #[default]
    Direct,

    /// Method call with extra arguments
    MethodCall { extra_args: Vec<String> },

    /// Property to method conversion
    PropertyToMethod,

    /// Constructor pattern (e.g., DataFrame() → DataFrame::new())
    Constructor { method: String },

    /// Argument reordering [31]
    ReorderArgs { indices: Vec<usize> },

    /// Type-safe template with validation [32, 33]
    TypedTemplate {
        pattern: String,
        params: Vec<String>,
        param_types: Vec<ParamType>,
    },

    /// Legacy template (deprecated)
    #[deprecated(note = "Use TypedTemplate for type-safe templates")]
    Template { template: String },
}

/// Parameter types for TypedTemplate validation (Poka-Yoke)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParamType {
    Expr,
    String,
    Number,
    Bytes,
    Bool,
    Path,
    List,
    Dict,
}

/// Type transformation hints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeTransform {
    /// Python type hint
    pub python_type: String,
    /// Rust type equivalent
    pub rust_type: String,
}

/// Confidence level for mappings [36]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MappingConfidence {
    /// Verified against official documentation and tests
    Verified,
    /// Community-contributed, reviewed
    Community,
    /// Experimental, may have edge cases
    #[default]
    Experimental,
}

// ============================================================================
// Registry Architecture (Section 2.2 of spec)
// ============================================================================

/// Enterprise-extensible mapping registry with O(1) lookup [1, 27]
///
/// Priority chain: overrides > extensions > core
#[derive(Debug, Default)]
pub struct MappingRegistry {
    /// Core mappings (shipped with depyler)
    core: HashMap<String, LibraryMapping>,

    /// Enterprise extensions (loaded from plugins)
    extensions: HashMap<String, LibraryMapping>,

    /// User overrides (highest priority)
    overrides: HashMap<String, LibraryMapping>,
}

impl MappingRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Create registry with default core mappings
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register_core_defaults();
        registry
    }

    /// Lookup item with priority: overrides > extensions > core
    ///
    /// Complexity: O(1) amortized [1]
    pub fn lookup(&self, module: &str, item: &str) -> Option<&ItemMapping> {
        self.overrides
            .get(module)
            .or_else(|| self.extensions.get(module))
            .or_else(|| self.core.get(module))
            .and_then(|m| m.items.get(item))
    }

    /// Lookup full library mapping with priority
    pub fn lookup_module(&self, module: &str) -> Option<&LibraryMapping> {
        self.overrides
            .get(module)
            .or_else(|| self.extensions.get(module))
            .or_else(|| self.core.get(module))
    }

    /// Register a core mapping (lowest priority)
    pub fn register_core(&mut self, mapping: LibraryMapping) {
        self.core.insert(mapping.python_module.clone(), mapping);
    }

    /// Register an extension mapping (medium priority)
    pub fn register_extension(&mut self, mapping: LibraryMapping) {
        self.extensions
            .insert(mapping.python_module.clone(), mapping);
    }

    /// Register a user override (highest priority)
    pub fn register_override(&mut self, mapping: LibraryMapping) {
        self.overrides
            .insert(mapping.python_module.clone(), mapping);
    }

    /// Get count of all registered modules
    pub fn module_count(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        for key in self.core.keys() {
            seen.insert(key.as_str());
        }
        for key in self.extensions.keys() {
            seen.insert(key.as_str());
        }
        for key in self.overrides.keys() {
            seen.insert(key.as_str());
        }
        seen.len()
    }

    /// Register default core mappings (Section 3 of spec)
    fn register_core_defaults(&mut self) {
        // json → serde_json
        self.register_core(LibraryMapping {
            python_module: "json".to_string(),
            rust_crate: "serde_json".to_string(),
            python_version_req: "*".to_string(),
            rust_crate_version: "1.0".to_string(),
            items: HashMap::from([
                (
                    "loads".to_string(),
                    ItemMapping {
                        rust_name: "from_str".to_string(),
                        pattern: TransformPattern::Direct,
                        type_transform: None,
                    },
                ),
                (
                    "dumps".to_string(),
                    ItemMapping {
                        rust_name: "to_string".to_string(),
                        pattern: TransformPattern::Direct,
                        type_transform: None,
                    },
                ),
            ]),
            features: vec![],
            confidence: MappingConfidence::Verified,
            provenance: "https://docs.rs/serde_json/".to_string(),
        });

        // os → std
        self.register_core(LibraryMapping {
            python_module: "os".to_string(),
            rust_crate: "std".to_string(),
            python_version_req: "*".to_string(),
            rust_crate_version: "*".to_string(),
            items: HashMap::from([
                (
                    "getcwd".to_string(),
                    ItemMapping {
                        rust_name: "env::current_dir".to_string(),
                        pattern: TransformPattern::Direct,
                        type_transform: None,
                    },
                ),
                (
                    "getenv".to_string(),
                    ItemMapping {
                        rust_name: "env::var".to_string(),
                        pattern: TransformPattern::Direct,
                        type_transform: None,
                    },
                ),
            ]),
            features: vec![],
            confidence: MappingConfidence::Verified,
            provenance: "https://doc.rust-lang.org/std/".to_string(),
        });

        // re → regex
        self.register_core(LibraryMapping {
            python_module: "re".to_string(),
            rust_crate: "regex".to_string(),
            python_version_req: "*".to_string(),
            rust_crate_version: "1.0".to_string(),
            items: HashMap::from([
                (
                    "compile".to_string(),
                    ItemMapping {
                        rust_name: "Regex::new".to_string(),
                        pattern: TransformPattern::Constructor {
                            method: "new".to_string(),
                        },
                        type_transform: None,
                    },
                ),
                (
                    "match".to_string(),
                    ItemMapping {
                        rust_name: "is_match".to_string(),
                        pattern: TransformPattern::MethodCall { extra_args: vec![] },
                        type_transform: None,
                    },
                ),
            ]),
            features: vec![],
            confidence: MappingConfidence::Verified,
            provenance: "https://docs.rs/regex/".to_string(),
        });
    }
}

// ============================================================================
// Plugin Architecture (Section 2.3 of spec)
// ============================================================================

/// Enterprise plugin interface for custom library mappings
pub trait MappingPlugin: Send + Sync {
    /// Plugin identifier (e.g., "netflix-internal", "google-cloud")
    fn id(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Register mappings into the registry
    fn register(&self, registry: &mut MappingRegistry);

    /// Optional: Validate that mappings are correct
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

/// Validation error for plugin mappings
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub mapping: Option<String>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation error: {}", self.message)
    }
}

impl std::error::Error for ValidationError {}

// ============================================================================
// Transform Pattern Utilities
// ============================================================================

impl TransformPattern {
    /// Validate a ReorderArgs pattern
    ///
    /// Indices must be a valid permutation (0..n where n = indices.len())
    pub fn validate_reorder_args(indices: &[usize]) -> Result<(), ValidationError> {
        let n = indices.len();
        let mut seen = vec![false; n];

        for &idx in indices {
            if idx >= n {
                return Err(ValidationError {
                    message: format!("Index {} out of bounds for {} args", idx, n),
                    mapping: None,
                });
            }
            if seen[idx] {
                return Err(ValidationError {
                    message: format!("Duplicate index {} in permutation", idx),
                    mapping: None,
                });
            }
            seen[idx] = true;
        }

        Ok(())
    }

    /// Validate a TypedTemplate pattern
    ///
    /// Params must match placeholders in pattern, and lengths must match
    pub fn validate_typed_template(
        pattern: &str,
        params: &[String],
        param_types: &[ParamType],
    ) -> Result<(), ValidationError> {
        // Check param/type count match
        if params.len() != param_types.len() {
            return Err(ValidationError {
                message: format!(
                    "Param count {} != type count {}",
                    params.len(),
                    param_types.len()
                ),
                mapping: None,
            });
        }

        // Check all params appear in pattern
        for param in params {
            let placeholder = format!("{{{}}}", param);
            if !pattern.contains(&placeholder) {
                return Err(ValidationError {
                    message: format!("Param '{}' not found in pattern", param),
                    mapping: None,
                });
            }
        }

        Ok(())
    }
}

