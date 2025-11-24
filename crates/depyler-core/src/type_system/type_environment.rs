//! DEPYLER-0499: TypeEnvironment - Single Source of Truth for Type Information
//!
//! Unified type tracking replacing 7 fragmented HashMaps.
//!
//! # Design Principles
//!
//! 1. **Single Source of Truth** (一元管理): One unified structure
//! 2. **O(1) Lookups**: Indexed HashMap access
//! 3. **SSA Form**: Variable reassignments create new versions (x_0, x_1)
//! 4. **Bidirectional Typing**: Synthesis (⇒) and Checking (⇐)
//!
//! # Example
//!
//! ```
//! use depyler_core::type_system::type_environment::TypeEnvironment;
//! use depyler_core::hir::Type;
//!
//! let mut env = TypeEnvironment::new();
//!
//! // Python: x = 5 (x_0: i64)
//! let x_0 = env.bind_var("x", Type::Int64);
//!
//! // Python: x = "hello" (x_1: String) - type change requires new version
//! let x_1 = env.bind_var("x", Type::String);
//!
//! assert_ne!(x_0, x_1, "Different versions have different IDs");
//! ```

use crate::hir::{HirExpr, Literal, Type};
use std::collections::HashMap;

/// Variable ID for O(1) lookup
pub type VarId = usize;

/// Type information for a single variable binding
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// Variable name (e.g., "x")
    pub name: String,
    /// SSA version (e.g., x_0, x_1)
    pub version: usize,
    /// Type of this binding
    pub ty: Type,
}

/// TypeEnvironment - unified type tracking system
///
/// Replaces 7 fragmented HashMaps with single source of truth.
pub struct TypeEnvironment {
    /// All variable bindings: VarId → TypeInfo
    bindings: HashMap<VarId, TypeInfo>,

    /// Index: variable name → current VarId
    current_bindings: HashMap<String, VarId>,

    /// Index: variable name → version counter
    version_counters: HashMap<String, usize>,

    /// Next available VarId
    next_var_id: VarId,
}

impl TypeEnvironment {
    /// Create new type environment
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            current_bindings: HashMap::new(),
            version_counters: HashMap::new(),
            next_var_id: 0,
        }
    }

    /// Bind variable to type (creates new SSA version if type changes)
    ///
    /// # Example
    ///
    /// ```
    /// use depyler_core::type_system::type_environment::TypeEnvironment;
    /// use depyler_core::hir::Type;
    ///
    /// let mut env = TypeEnvironment::new();
    ///
    /// let x_0 = env.bind_var("x", Type::Int64);
    /// let x_1 = env.bind_var("x", Type::String);  // New version for type change
    ///
    /// assert_ne!(x_0, x_1);
    /// ```
    pub fn bind_var(&mut self, name: &str, ty: Type) -> VarId {
        // Check if variable exists with different type
        let needs_new_version = if let Some(&existing_id) = self.current_bindings.get(name) {
            let existing_info = self.bindings.get(&existing_id).unwrap();
            existing_info.ty != ty // Type changed - need new version (SSA)
        } else {
            true // First binding
        };

        if needs_new_version {
            // Get next version number
            let version = *self.version_counters.get(name).unwrap_or(&0);
            self.version_counters.insert(name.to_string(), version + 1);

            // Create new binding
            let var_id = self.next_var_id;
            self.next_var_id += 1;

            self.bindings.insert(
                var_id,
                TypeInfo {
                    name: name.to_string(),
                    version,
                    ty: ty.clone(),
                },
            );

            self.current_bindings.insert(name.to_string(), var_id);

            var_id
        } else {
            // Same type - reuse existing binding
            *self.current_bindings.get(name).unwrap()
        }
    }

    /// Get current type for variable name (O(1))
    pub fn get_var_type(&self, name: &str) -> Option<&Type> {
        let var_id = self.current_bindings.get(name)?;
        let info = self.bindings.get(var_id)?;
        Some(&info.ty)
    }

    /// Get type by VarId (O(1))
    pub fn get_type_by_id(&self, var_id: VarId) -> Option<&Type> {
        self.bindings.get(&var_id).map(|info| &info.ty)
    }

    /// Get current SSA version for variable
    pub fn get_var_version(&self, name: &str) -> Option<usize> {
        let var_id = self.current_bindings.get(name)?;
        let info = self.bindings.get(var_id)?;
        Some(info.version)
    }

    /// Synthesize type from expression (bidirectional typing: ⇒)
    ///
    /// Infers type bottom-up from expression structure.
    pub fn synthesize_type(&mut self, expr: &HirExpr) -> Result<Type, String> {
        match expr {
            HirExpr::Literal(lit) => {
                match lit {
                    Literal::Int(_) => Ok(Type::Int),
                    Literal::Float(_) => Ok(Type::Float),
                    Literal::String(_) => Ok(Type::String),
                    Literal::Bool(_) => Ok(Type::Bool),
                    Literal::None => Ok(Type::Optional(Box::new(Type::Unknown))),
                    Literal::Bytes(_) => Ok(Type::String), // Treat bytes as string for simplicity
                }
            }

            HirExpr::Var(name) => self
                .get_var_type(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),

            _ => Ok(Type::Unknown), // Simplified for now
        }
    }

    /// Check expression against expected type (bidirectional typing: ⇐)
    ///
    /// Verifies expression top-down against known type.
    pub fn check_type(&mut self, expr: &HirExpr, expected: &Type) -> Result<(), String> {
        use crate::type_system::subtyping::SubtypeChecker;

        let inferred = self.synthesize_type(expr)?;
        let checker = SubtypeChecker::new();

        // Check if inferred <: expected (subtyping)
        checker
            .check_subtype(&inferred, expected)
            .map_err(|e| format!("Type check failed: {}", e))
    }
}

impl Default for TypeEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_var_creates_new_id() {
        let mut env = TypeEnvironment::new();
        let id = env.bind_var("x", Type::Int);
        assert_eq!(id, 0, "First variable should have ID 0");
    }

    #[test]
    fn test_get_var_type_o1_lookup() {
        let mut env = TypeEnvironment::new();
        env.bind_var("x", Type::Int);
        assert_eq!(env.get_var_type("x"), Some(&Type::Int));
    }

    #[test]
    fn test_ssa_variable_versioning() {
        let mut env = TypeEnvironment::new();

        // First binding: x_0
        let x_0 = env.bind_var("x", Type::Int);
        assert_eq!(env.get_var_version("x"), Some(0));

        // Type change: x_1
        let x_1 = env.bind_var("x", Type::String);
        assert_eq!(env.get_var_version("x"), Some(1));

        assert_ne!(x_0, x_1, "Type change should create new version");

        // Verify both versions exist
        assert_eq!(env.get_type_by_id(x_0), Some(&Type::Int));
        assert_eq!(env.get_type_by_id(x_1), Some(&Type::String));
    }

    #[test]
    fn test_same_type_reuses_binding() {
        let mut env = TypeEnvironment::new();

        let x_0 = env.bind_var("x", Type::Int);
        let x_same = env.bind_var("x", Type::Int); // Same type

        assert_eq!(x_0, x_same, "Same type should reuse binding");
    }

    #[test]
    fn test_synthesize_int_literal() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::Int(42));

        let ty = env.synthesize_type(&expr).expect("Should infer type");
        assert_eq!(ty, Type::Int, "Int literal should infer to Int");
    }
}
