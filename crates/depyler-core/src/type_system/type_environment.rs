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
//! // Python: x = 5 (x_0: int)
//! let x_0 = env.bind_var("x", Type::Int);
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
    /// let x_0 = env.bind_var("x", Type::Int);
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
    fn test_bind_var_increments_ids() {
        let mut env = TypeEnvironment::new();
        let id1 = env.bind_var("x", Type::Int);
        let id2 = env.bind_var("y", Type::String);
        let id3 = env.bind_var("z", Type::Float);
        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);
    }

    #[test]
    fn test_get_var_type_o1_lookup() {
        let mut env = TypeEnvironment::new();
        env.bind_var("x", Type::Int);
        assert_eq!(env.get_var_type("x"), Some(&Type::Int));
    }

    #[test]
    fn test_get_var_type_missing() {
        let env = TypeEnvironment::new();
        assert_eq!(env.get_var_type("nonexistent"), None);
    }

    #[test]
    fn test_get_type_by_id_missing() {
        let env = TypeEnvironment::new();
        assert_eq!(env.get_type_by_id(999), None);
    }

    #[test]
    fn test_get_var_version_missing() {
        let env = TypeEnvironment::new();
        assert_eq!(env.get_var_version("nonexistent"), None);
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
    fn test_ssa_multiple_type_changes() {
        let mut env = TypeEnvironment::new();

        let id0 = env.bind_var("x", Type::Int);
        let id1 = env.bind_var("x", Type::String);
        let id2 = env.bind_var("x", Type::Float);
        let id3 = env.bind_var("x", Type::Bool);

        assert_eq!(env.get_var_version("x"), Some(3));
        assert_eq!(id0, 0);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
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

    #[test]
    fn test_synthesize_float_literal() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::Float(3.15));

        let ty = env.synthesize_type(&expr).expect("Should infer type");
        assert_eq!(ty, Type::Float);
    }

    #[test]
    fn test_synthesize_string_literal() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));

        let ty = env.synthesize_type(&expr).expect("Should infer type");
        assert_eq!(ty, Type::String);
    }

    #[test]
    fn test_synthesize_bool_literal() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::Bool(true));

        let ty = env.synthesize_type(&expr).expect("Should infer type");
        assert_eq!(ty, Type::Bool);
    }

    #[test]
    fn test_synthesize_none_literal() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::None);

        let ty = env.synthesize_type(&expr).expect("Should infer type");
        assert_eq!(ty, Type::Optional(Box::new(Type::Unknown)));
    }

    #[test]
    fn test_synthesize_bytes_literal() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::Bytes(vec![0x41, 0x42]));

        let ty = env.synthesize_type(&expr).expect("Should infer type");
        assert_eq!(ty, Type::String); // Bytes treated as String
    }

    #[test]
    fn test_synthesize_var_defined() {
        let mut env = TypeEnvironment::new();
        env.bind_var("x", Type::Int);

        let expr = HirExpr::Var("x".to_string());
        let ty = env.synthesize_type(&expr).expect("Should infer type");
        assert_eq!(ty, Type::Int);
    }

    #[test]
    fn test_synthesize_var_undefined() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Var("undefined".to_string());

        let result = env.synthesize_type(&expr);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_synthesize_other_expr_returns_unknown() {
        let mut env = TypeEnvironment::new();
        // Use a complex expression that falls through to default
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: crate::hir::BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };

        let ty = env.synthesize_type(&expr).expect("Should infer type");
        assert_eq!(ty, Type::Unknown);
    }

    #[test]
    fn test_check_type_success() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::Int(42));

        // Int should pass check against Int
        let result = env.check_type(&expr, &Type::Int);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_type_subtype_success() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::Int(42));

        // Int <: Float (numeric tower)
        let result = env.check_type(&expr, &Type::Float);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_type_failure() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));

        // String is not a subtype of Int
        let result = env.check_type(&expr, &Type::Int);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type check failed"));
    }

    #[test]
    fn test_check_type_undefined_var() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Var("undefined".to_string());

        // Should fail on synthesize step
        let result = env.check_type(&expr, &Type::Int);
        assert!(result.is_err());
    }

    #[test]
    fn test_default_impl() {
        let env = TypeEnvironment::default();
        assert_eq!(env.get_var_type("x"), None);
    }

    #[test]
    fn test_multiple_variables() {
        let mut env = TypeEnvironment::new();

        env.bind_var("x", Type::Int);
        env.bind_var("y", Type::String);
        env.bind_var("z", Type::Float);

        assert_eq!(env.get_var_type("x"), Some(&Type::Int));
        assert_eq!(env.get_var_type("y"), Some(&Type::String));
        assert_eq!(env.get_var_type("z"), Some(&Type::Float));
    }

    #[test]
    fn test_check_type_option_lifting() {
        let mut env = TypeEnvironment::new();
        let expr = HirExpr::Literal(Literal::Int(42));

        // Int <: Optional<Int>
        let result = env.check_type(&expr, &Type::Optional(Box::new(Type::Int)));
        assert!(result.is_ok());
    }
}
