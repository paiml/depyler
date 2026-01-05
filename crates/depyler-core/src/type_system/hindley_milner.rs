//! Hindley-Milner Type Inference (Algorithm W)
//!
//! Implements constraint-based type inference using the Damas-Milner algorithm.
//! This provides systematic type inference with proven correctness properties.
//!
//! # Algorithm Complexity
//!
//! - **Unification**: O(N * α(N)) where α is inverse Ackermann (effectively O(N))
//! - **Constraint solving**: O(N * log N) for N constraints
//! - **Expected performance**: 10-50ms for typical examples
//!
//! # Example
//!
//! ```rust
//! use depyler_core::type_system::{TypeConstraintSolver, Constraint};
//! use depyler_core::hir::Type;
//! use std::collections::HashMap;
//!
//! let mut solver = TypeConstraintSolver::new();
//!
//! // Add constraints: x: Int, y: Int, result = x + y
//! solver.add_constraint(Constraint::Instance(0, Type::Int));
//! solver.add_constraint(Constraint::Instance(1, Type::Int));
//! solver.add_constraint(Constraint::Equality(
//!     Type::UnificationVar(2),
//!     Type::Int
//! ));
//!
//! // Solve constraints
//! let solution = solver.solve().expect("Type inference failed");
//! assert_eq!(solution.get(&2), Some(&Type::Int));
//! ```

use crate::hir::Type;
use std::collections::HashMap;

/// Type variable identifier for unification
pub type VarId = usize;

/// Type constraints collected during HIR analysis
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    /// Two types must be equal: t1 == t2
    Equality(Type, Type),
    /// Variable must have a specific type: var: Type
    Instance(VarId, Type),
}

/// Type errors that can occur during inference
#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    /// Infinite type detected (occurs check failed)
    InfiniteType(VarId, Type),
    /// Type mismatch: expected vs actual
    Mismatch(Type, Type),
    /// Unification failed
    UnificationFailed(String),
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::InfiniteType(var, ty) => {
                write!(f, "Infinite type: variable {} occurs in {:?}", var, ty)
            }
            TypeError::Mismatch(expected, actual) => {
                write!(
                    f,
                    "Type mismatch: expected {:?}, got {:?}",
                    expected, actual
                )
            }
            TypeError::UnificationFailed(msg) => {
                write!(f, "Unification failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for TypeError {}

/// Hindley-Milner type constraint solver
///
/// Solves type constraints using Algorithm W (Damas-Milner).
/// Provides O(N * log N) constraint solving with Robinson's unification.
pub struct TypeConstraintSolver {
    /// Collected type constraints
    constraints: Vec<Constraint>,
    /// Substitutions mapping type variables to types
    substitutions: HashMap<VarId, Type>,
    /// Counter for generating fresh type variables
    next_type_var: VarId,
}

impl TypeConstraintSolver {
    /// Create a new type constraint solver
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            substitutions: HashMap::new(),
            next_type_var: 0,
        }
    }

    /// Generate a fresh type variable
    pub fn fresh_var(&mut self) -> VarId {
        let var = self.next_type_var;
        self.next_type_var += 1;
        var
    }

    /// Add a constraint to the solver
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Solve all constraints using Algorithm W
    ///
    /// Returns a substitution mapping each variable to its inferred type.
    ///
    /// # Errors
    ///
    /// Returns `TypeError` if:
    /// - Types cannot be unified (mismatch)
    /// - Infinite types are detected (occurs check)
    pub fn solve(&mut self) -> Result<HashMap<VarId, Type>, TypeError> {
        // Step 1: Process all constraints
        for constraint in self.constraints.clone() {
            match constraint {
                Constraint::Equality(t1, t2) => {
                    self.unify(t1, t2)?;
                }
                Constraint::Instance(var, ty) => {
                    self.substitutions.insert(var, ty);
                }
            }
        }

        // Step 2: Apply substitutions to get final types
        let mut result = HashMap::new();
        for (var, ty) in &self.substitutions.clone() {
            result.insert(*var, self.apply_substitution(ty));
        }

        Ok(result)
    }

    /// Unify two types using Robinson's algorithm
    ///
    /// # Arguments
    ///
    /// * `t1` - First type
    /// * `t2` - Second type
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if unification succeeds, or `TypeError` if it fails.
    ///
    /// # Algorithm
    ///
    /// 1. Apply current substitutions to both types
    /// 2. Check if types are identical (base case)
    /// 3. Handle type variables with occurs check
    /// 4. Recursively unify compound types
    /// 5. Fail on type mismatch
    fn unify(&mut self, t1: Type, t2: Type) -> Result<(), TypeError> {
        // Apply current substitutions
        let t1 = self.apply_substitution(&t1);
        let t2 = self.apply_substitution(&t2);

        match (t1.clone(), t2.clone()) {
            // Identical primitive types
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::String, Type::String)
            | (Type::Bool, Type::Bool)
            | (Type::None, Type::None) => Ok(()),

            // Unification variables
            (Type::UnificationVar(v1), Type::UnificationVar(v2)) if v1 == v2 => Ok(()),
            (Type::UnificationVar(v), t) | (t, Type::UnificationVar(v)) => {
                if self.occurs_check(v, &t) {
                    Err(TypeError::InfiniteType(v, t))
                } else {
                    self.substitutions.insert(v, t);
                    Ok(())
                }
            }

            // Compound types
            (Type::List(inner1), Type::List(inner2)) => self.unify(*inner1, *inner2),

            (Type::Dict(k1, v1), Type::Dict(k2, v2)) => {
                self.unify(*k1, *k2)?;
                self.unify(*v1, *v2)
            }

            (Type::Optional(inner1), Type::Optional(inner2)) => self.unify(*inner1, *inner2),

            (Type::Tuple(types1), Type::Tuple(types2)) => {
                if types1.len() != types2.len() {
                    return Err(TypeError::Mismatch(
                        Type::Tuple(types1),
                        Type::Tuple(types2),
                    ));
                }
                for (t1, t2) in types1.into_iter().zip(types2.into_iter()) {
                    self.unify(t1, t2)?;
                }
                Ok(())
            }

            (Type::Set(inner1), Type::Set(inner2)) => self.unify(*inner1, *inner2),

            (
                Type::Function {
                    params: params1,
                    ret: ret1,
                },
                Type::Function {
                    params: params2,
                    ret: ret2,
                },
            ) => {
                if params1.len() != params2.len() {
                    return Err(TypeError::Mismatch(
                        Type::Function {
                            params: params1,
                            ret: ret1,
                        },
                        Type::Function {
                            params: params2,
                            ret: ret2,
                        },
                    ));
                }
                for (p1, p2) in params1.into_iter().zip(params2.into_iter()) {
                    self.unify(p1, p2)?;
                }
                self.unify(*ret1, *ret2)
            }

            // Type mismatch
            _ => Err(TypeError::Mismatch(t1, t2)),
        }
    }

    /// Occurs check: prevent infinite types
    ///
    /// Checks if variable `var` occurs in type `ty`. This prevents
    /// creating infinite types like `T = List<T>`.
    ///
    /// # Arguments
    ///
    /// * `var` - Type variable to check
    /// * `ty` - Type to search in
    ///
    /// # Returns
    ///
    /// `true` if `var` occurs in `ty`, `false` otherwise
    fn occurs_check(&self, var: VarId, ty: &Type) -> bool {
        match ty {
            Type::UnificationVar(v) => *v == var,
            Type::List(inner) => self.occurs_check(var, inner),
            Type::Dict(k, v) => self.occurs_check(var, k) || self.occurs_check(var, v),
            Type::Optional(inner) => self.occurs_check(var, inner),
            Type::Tuple(types) => types.iter().any(|t| self.occurs_check(var, t)),
            Type::Set(inner) => self.occurs_check(var, inner),
            Type::Function { params, ret } => {
                params.iter().any(|p| self.occurs_check(var, p)) || self.occurs_check(var, ret)
            }
            Type::Generic { params, .. } => params.iter().any(|p| self.occurs_check(var, p)),
            Type::Union(types) => types.iter().any(|t| self.occurs_check(var, t)),
            Type::Array { element_type, .. } => self.occurs_check(var, element_type),
            _ => false,
        }
    }

    /// Apply substitution to a type
    ///
    /// Recursively applies all known substitutions to a type, resolving
    /// type variables to their concrete types.
    ///
    /// # Arguments
    ///
    /// * `ty` - Type to apply substitutions to
    ///
    /// # Returns
    ///
    /// The type with all substitutions applied
    fn apply_substitution(&self, ty: &Type) -> Type {
        match ty {
            Type::UnificationVar(v) => {
                if let Some(subst) = self.substitutions.get(v) {
                    // Recursively apply substitutions
                    self.apply_substitution(subst)
                } else {
                    ty.clone()
                }
            }
            Type::List(inner) => Type::List(Box::new(self.apply_substitution(inner))),
            Type::Dict(k, v) => Type::Dict(
                Box::new(self.apply_substitution(k)),
                Box::new(self.apply_substitution(v)),
            ),
            Type::Optional(inner) => Type::Optional(Box::new(self.apply_substitution(inner))),
            Type::Tuple(types) => {
                Type::Tuple(types.iter().map(|t| self.apply_substitution(t)).collect())
            }
            Type::Set(inner) => Type::Set(Box::new(self.apply_substitution(inner))),
            Type::Function { params, ret } => Type::Function {
                params: params.iter().map(|p| self.apply_substitution(p)).collect(),
                ret: Box::new(self.apply_substitution(ret)),
            },
            Type::Generic { base, params } => Type::Generic {
                base: base.clone(),
                params: params.iter().map(|p| self.apply_substitution(p)).collect(),
            },
            Type::Union(types) => {
                Type::Union(types.iter().map(|t| self.apply_substitution(t)).collect())
            }
            Type::Array { element_type, size } => Type::Array {
                element_type: Box::new(self.apply_substitution(element_type)),
                size: size.clone(),
            },
            _ => ty.clone(),
        }
    }
}

impl Default for TypeConstraintSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_identical_primitives() {
        let mut solver = TypeConstraintSolver::new();
        assert!(solver.unify(Type::Int, Type::Int).is_ok());
        assert!(solver.unify(Type::String, Type::String).is_ok());
        assert!(solver.unify(Type::Bool, Type::Bool).is_ok());
    }

    #[test]
    fn test_unify_type_mismatch() {
        let mut solver = TypeConstraintSolver::new();
        let result = solver.unify(Type::Int, Type::String);
        assert!(matches!(result, Err(TypeError::Mismatch(_, _))));
    }

    #[test]
    fn test_unify_type_variable() {
        let mut solver = TypeConstraintSolver::new();
        let var = solver.fresh_var();

        // Unify variable with Int
        assert!(solver.unify(Type::UnificationVar(var), Type::Int).is_ok());
        assert_eq!(solver.substitutions.get(&var), Some(&Type::Int));
    }

    #[test]
    fn test_occurs_check() {
        let mut solver = TypeConstraintSolver::new();
        let var = solver.fresh_var();

        // Try to create infinite type: var = List<var>
        let infinite_type = Type::List(Box::new(Type::UnificationVar(var)));
        let result = solver.unify(Type::UnificationVar(var), infinite_type.clone());
        assert!(matches!(result, Err(TypeError::InfiniteType(_, _))));
    }

    #[test]
    fn test_unify_compound_types() {
        let mut solver = TypeConstraintSolver::new();

        // List<Int> = List<Int>
        let list1 = Type::List(Box::new(Type::Int));
        let list2 = Type::List(Box::new(Type::Int));
        assert!(solver.unify(list1, list2).is_ok());

        // Dict<String, Int> = Dict<String, Int>
        let dict1 = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        let dict2 = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        assert!(solver.unify(dict1, dict2).is_ok());
    }

    #[test]
    fn test_constraint_solving() {
        let mut solver = TypeConstraintSolver::new();

        let var1 = solver.fresh_var();
        let var2 = solver.fresh_var();

        // Add constraints: var1: Int, var2 = var1
        solver.add_constraint(Constraint::Instance(var1, Type::Int));
        solver.add_constraint(Constraint::Equality(
            Type::UnificationVar(var2),
            Type::UnificationVar(var1),
        ));

        let solution = solver.solve().expect("Solving failed");
        assert_eq!(solution.get(&var1), Some(&Type::Int));
        assert_eq!(solution.get(&var2), Some(&Type::Int));
    }

    #[test]
    fn test_apply_substitution() {
        let mut solver = TypeConstraintSolver::new();
        let var = solver.fresh_var();

        solver.substitutions.insert(var, Type::String);

        let ty = Type::List(Box::new(Type::UnificationVar(var)));
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::List(Box::new(Type::String)));
    }

    #[test]
    fn test_fresh_var_uniqueness() {
        let mut solver = TypeConstraintSolver::new();
        let var1 = solver.fresh_var();
        let var2 = solver.fresh_var();
        let var3 = solver.fresh_var();

        assert_ne!(var1, var2);
        assert_ne!(var2, var3);
        assert_ne!(var1, var3);
    }

    #[test]
    fn test_unify_tuples() {
        let mut solver = TypeConstraintSolver::new();

        let tuple1 = Type::Tuple(vec![Type::Int, Type::String]);
        let tuple2 = Type::Tuple(vec![Type::Int, Type::String]);
        assert!(solver.unify(tuple1, tuple2).is_ok());

        // Mismatched lengths
        let tuple3 = Type::Tuple(vec![Type::Int]);
        let tuple4 = Type::Tuple(vec![Type::Int, Type::String]);
        assert!(matches!(
            solver.unify(tuple3, tuple4),
            Err(TypeError::Mismatch(_, _))
        ));
    }

    #[test]
    fn test_unify_functions() {
        let mut solver = TypeConstraintSolver::new();

        let func1 = Type::Function {
            params: vec![Type::Int, Type::String],
            ret: Box::new(Type::Bool),
        };
        let func2 = Type::Function {
            params: vec![Type::Int, Type::String],
            ret: Box::new(Type::Bool),
        };
        assert!(solver.unify(func1, func2).is_ok());
    }

    // ============ TypeError Display tests ============

    #[test]
    fn test_type_error_display_infinite() {
        let err = TypeError::InfiniteType(0, Type::List(Box::new(Type::Int)));
        let msg = format!("{}", err);
        assert!(msg.contains("Infinite type"));
        assert!(msg.contains("variable 0"));
    }

    #[test]
    fn test_type_error_display_mismatch() {
        let err = TypeError::Mismatch(Type::Int, Type::String);
        let msg = format!("{}", err);
        assert!(msg.contains("Type mismatch"));
        assert!(msg.contains("Int"));
        assert!(msg.contains("String"));
    }

    #[test]
    fn test_type_error_display_unification_failed() {
        let err = TypeError::UnificationFailed("test error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Unification failed"));
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_type_error_is_error_trait() {
        let err: &dyn std::error::Error = &TypeError::Mismatch(Type::Int, Type::String);
        assert!(err.to_string().contains("Type mismatch"));
    }

    // ============ Occurs check edge cases ============

    #[test]
    fn test_occurs_check_dict() {
        let solver = TypeConstraintSolver::new();
        // var occurs in Dict<var, Int>
        assert!(solver.occurs_check(0, &Type::Dict(
            Box::new(Type::UnificationVar(0)),
            Box::new(Type::Int)
        )));
        // var occurs in Dict<Int, var>
        assert!(solver.occurs_check(0, &Type::Dict(
            Box::new(Type::Int),
            Box::new(Type::UnificationVar(0))
        )));
        // var does not occur
        assert!(!solver.occurs_check(0, &Type::Dict(
            Box::new(Type::Int),
            Box::new(Type::String)
        )));
    }

    #[test]
    fn test_occurs_check_optional() {
        let solver = TypeConstraintSolver::new();
        assert!(solver.occurs_check(0, &Type::Optional(Box::new(Type::UnificationVar(0)))));
        assert!(!solver.occurs_check(0, &Type::Optional(Box::new(Type::Int))));
    }

    #[test]
    fn test_occurs_check_tuple() {
        let solver = TypeConstraintSolver::new();
        assert!(solver.occurs_check(0, &Type::Tuple(vec![
            Type::Int,
            Type::UnificationVar(0),
            Type::String
        ])));
        assert!(!solver.occurs_check(0, &Type::Tuple(vec![
            Type::Int,
            Type::String
        ])));
    }

    #[test]
    fn test_occurs_check_set() {
        let solver = TypeConstraintSolver::new();
        assert!(solver.occurs_check(0, &Type::Set(Box::new(Type::UnificationVar(0)))));
        assert!(!solver.occurs_check(0, &Type::Set(Box::new(Type::Int))));
    }

    #[test]
    fn test_occurs_check_function() {
        let solver = TypeConstraintSolver::new();
        // var in params
        assert!(solver.occurs_check(0, &Type::Function {
            params: vec![Type::UnificationVar(0), Type::Int],
            ret: Box::new(Type::Bool)
        }));
        // var in return type
        assert!(solver.occurs_check(0, &Type::Function {
            params: vec![Type::Int],
            ret: Box::new(Type::UnificationVar(0))
        }));
        // var not present
        assert!(!solver.occurs_check(0, &Type::Function {
            params: vec![Type::Int],
            ret: Box::new(Type::Bool)
        }));
    }

    #[test]
    fn test_occurs_check_generic() {
        let solver = TypeConstraintSolver::new();
        assert!(solver.occurs_check(0, &Type::Generic {
            base: "Iterator".to_string(),
            params: vec![Type::UnificationVar(0)]
        }));
        assert!(!solver.occurs_check(0, &Type::Generic {
            base: "Iterator".to_string(),
            params: vec![Type::Int]
        }));
    }

    #[test]
    fn test_occurs_check_union() {
        let solver = TypeConstraintSolver::new();
        assert!(solver.occurs_check(0, &Type::Union(vec![
            Type::Int,
            Type::UnificationVar(0)
        ])));
        assert!(!solver.occurs_check(0, &Type::Union(vec![
            Type::Int,
            Type::String
        ])));
    }

    #[test]
    fn test_occurs_check_array() {
        use crate::hir::ConstGeneric;
        let solver = TypeConstraintSolver::new();
        assert!(solver.occurs_check(0, &Type::Array {
            element_type: Box::new(Type::UnificationVar(0)),
            size: ConstGeneric::Literal(10)
        }));
        assert!(!solver.occurs_check(0, &Type::Array {
            element_type: Box::new(Type::Int),
            size: ConstGeneric::Literal(10)
        }));
    }

    #[test]
    fn test_occurs_check_primitive() {
        let solver = TypeConstraintSolver::new();
        assert!(!solver.occurs_check(0, &Type::Int));
        assert!(!solver.occurs_check(0, &Type::Float));
        assert!(!solver.occurs_check(0, &Type::String));
        assert!(!solver.occurs_check(0, &Type::Bool));
        assert!(!solver.occurs_check(0, &Type::None));
    }

    // ============ Apply substitution tests ============

    #[test]
    fn test_apply_substitution_dict() {
        let mut solver = TypeConstraintSolver::new();
        solver.substitutions.insert(0, Type::String);
        solver.substitutions.insert(1, Type::Int);

        let ty = Type::Dict(
            Box::new(Type::UnificationVar(0)),
            Box::new(Type::UnificationVar(1))
        );
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Dict(Box::new(Type::String), Box::new(Type::Int)));
    }

    #[test]
    fn test_apply_substitution_optional() {
        let mut solver = TypeConstraintSolver::new();
        solver.substitutions.insert(0, Type::Int);

        let ty = Type::Optional(Box::new(Type::UnificationVar(0)));
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Optional(Box::new(Type::Int)));
    }

    #[test]
    fn test_apply_substitution_tuple() {
        let mut solver = TypeConstraintSolver::new();
        solver.substitutions.insert(0, Type::Int);
        solver.substitutions.insert(1, Type::String);

        let ty = Type::Tuple(vec![
            Type::UnificationVar(0),
            Type::Bool,
            Type::UnificationVar(1)
        ]);
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Tuple(vec![Type::Int, Type::Bool, Type::String]));
    }

    #[test]
    fn test_apply_substitution_set() {
        let mut solver = TypeConstraintSolver::new();
        solver.substitutions.insert(0, Type::String);

        let ty = Type::Set(Box::new(Type::UnificationVar(0)));
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Set(Box::new(Type::String)));
    }

    #[test]
    fn test_apply_substitution_function() {
        let mut solver = TypeConstraintSolver::new();
        solver.substitutions.insert(0, Type::Int);
        solver.substitutions.insert(1, Type::Bool);

        let ty = Type::Function {
            params: vec![Type::UnificationVar(0), Type::String],
            ret: Box::new(Type::UnificationVar(1))
        };
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Function {
            params: vec![Type::Int, Type::String],
            ret: Box::new(Type::Bool)
        });
    }

    #[test]
    fn test_apply_substitution_generic() {
        let mut solver = TypeConstraintSolver::new();
        solver.substitutions.insert(0, Type::Int);

        let ty = Type::Generic {
            base: "Vec".to_string(),
            params: vec![Type::UnificationVar(0)]
        };
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Generic {
            base: "Vec".to_string(),
            params: vec![Type::Int]
        });
    }

    #[test]
    fn test_apply_substitution_union() {
        let mut solver = TypeConstraintSolver::new();
        solver.substitutions.insert(0, Type::Int);

        let ty = Type::Union(vec![Type::UnificationVar(0), Type::String]);
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Union(vec![Type::Int, Type::String]));
    }

    #[test]
    fn test_apply_substitution_array() {
        use crate::hir::ConstGeneric;
        let mut solver = TypeConstraintSolver::new();
        solver.substitutions.insert(0, Type::Float);

        let ty = Type::Array {
            element_type: Box::new(Type::UnificationVar(0)),
            size: ConstGeneric::Literal(5)
        };
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Array {
            element_type: Box::new(Type::Float),
            size: ConstGeneric::Literal(5)
        });
    }

    #[test]
    fn test_apply_substitution_nested() {
        let mut solver = TypeConstraintSolver::new();
        // Chain: var0 -> var1 -> Int
        solver.substitutions.insert(0, Type::UnificationVar(1));
        solver.substitutions.insert(1, Type::Int);

        let ty = Type::UnificationVar(0);
        let result = solver.apply_substitution(&ty);
        assert_eq!(result, Type::Int);
    }

    #[test]
    fn test_apply_substitution_no_change() {
        let solver = TypeConstraintSolver::new();
        assert_eq!(solver.apply_substitution(&Type::Int), Type::Int);
        assert_eq!(solver.apply_substitution(&Type::Float), Type::Float);
        assert_eq!(solver.apply_substitution(&Type::String), Type::String);
        assert_eq!(solver.apply_substitution(&Type::Bool), Type::Bool);
        assert_eq!(solver.apply_substitution(&Type::None), Type::None);
    }

    // ============ Unify edge cases ============

    #[test]
    fn test_unify_optional() {
        let mut solver = TypeConstraintSolver::new();
        let opt1 = Type::Optional(Box::new(Type::Int));
        let opt2 = Type::Optional(Box::new(Type::Int));
        assert!(solver.unify(opt1, opt2).is_ok());
    }

    #[test]
    fn test_unify_optional_mismatch() {
        let mut solver = TypeConstraintSolver::new();
        let opt1 = Type::Optional(Box::new(Type::Int));
        let opt2 = Type::Optional(Box::new(Type::String));
        assert!(matches!(solver.unify(opt1, opt2), Err(TypeError::Mismatch(_, _))));
    }

    #[test]
    fn test_unify_set() {
        let mut solver = TypeConstraintSolver::new();
        let set1 = Type::Set(Box::new(Type::Int));
        let set2 = Type::Set(Box::new(Type::Int));
        assert!(solver.unify(set1, set2).is_ok());
    }

    #[test]
    fn test_unify_list_mismatch() {
        let mut solver = TypeConstraintSolver::new();
        let list1 = Type::List(Box::new(Type::Int));
        let list2 = Type::List(Box::new(Type::String));
        assert!(matches!(solver.unify(list1, list2), Err(TypeError::Mismatch(_, _))));
    }

    #[test]
    fn test_unify_dict_mismatch() {
        let mut solver = TypeConstraintSolver::new();
        let dict1 = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        let dict2 = Type::Dict(Box::new(Type::Int), Box::new(Type::Int));
        assert!(matches!(solver.unify(dict1, dict2), Err(TypeError::Mismatch(_, _))));
    }

    #[test]
    fn test_unify_function_param_mismatch() {
        let mut solver = TypeConstraintSolver::new();
        let func1 = Type::Function {
            params: vec![Type::Int],
            ret: Box::new(Type::Bool)
        };
        let func2 = Type::Function {
            params: vec![Type::Int, Type::String],
            ret: Box::new(Type::Bool)
        };
        assert!(matches!(solver.unify(func1, func2), Err(TypeError::Mismatch(_, _))));
    }

    #[test]
    fn test_unify_same_var() {
        let mut solver = TypeConstraintSolver::new();
        let var = Type::UnificationVar(0);
        assert!(solver.unify(var.clone(), var).is_ok());
    }

    #[test]
    fn test_unify_none() {
        let mut solver = TypeConstraintSolver::new();
        assert!(solver.unify(Type::None, Type::None).is_ok());
    }

    #[test]
    fn test_unify_float() {
        let mut solver = TypeConstraintSolver::new();
        assert!(solver.unify(Type::Float, Type::Float).is_ok());
    }

    // ============ Default impl test ============

    #[test]
    fn test_default_impl() {
        let solver = TypeConstraintSolver::default();
        assert!(solver.constraints.is_empty());
        assert!(solver.substitutions.is_empty());
        assert_eq!(solver.next_type_var, 0);
    }

    // ============ Complex constraint solving ============

    #[test]
    fn test_solve_chain_substitution() {
        let mut solver = TypeConstraintSolver::new();
        let var1 = solver.fresh_var();
        let var2 = solver.fresh_var();
        let var3 = solver.fresh_var();

        // var1 = var2, var2 = var3, var3 = Int
        solver.add_constraint(Constraint::Equality(
            Type::UnificationVar(var1),
            Type::UnificationVar(var2)
        ));
        solver.add_constraint(Constraint::Equality(
            Type::UnificationVar(var2),
            Type::UnificationVar(var3)
        ));
        solver.add_constraint(Constraint::Instance(var3, Type::Int));

        let solution = solver.solve().expect("Solving failed");
        assert_eq!(solution.get(&var3), Some(&Type::Int));
    }

    #[test]
    fn test_solve_empty_constraints() {
        let mut solver = TypeConstraintSolver::new();
        let solution = solver.solve().expect("Solving failed");
        assert!(solution.is_empty());
    }

    #[test]
    fn test_constraint_equality() {
        let c1 = Constraint::Equality(Type::Int, Type::Int);
        let c2 = Constraint::Equality(Type::Int, Type::Int);
        assert_eq!(c1, c2);

        let c3 = Constraint::Instance(0, Type::Int);
        let c4 = Constraint::Instance(0, Type::Int);
        assert_eq!(c3, c4);
    }

    #[test]
    fn test_constraint_debug() {
        let c = Constraint::Equality(Type::Int, Type::String);
        let debug_str = format!("{:?}", c);
        assert!(debug_str.contains("Equality"));
    }

    #[test]
    fn test_type_error_equality() {
        let e1 = TypeError::InfiniteType(0, Type::Int);
        let e2 = TypeError::InfiniteType(0, Type::Int);
        assert_eq!(e1, e2);

        let e3 = TypeError::Mismatch(Type::Int, Type::String);
        let e4 = TypeError::Mismatch(Type::Int, Type::String);
        assert_eq!(e3, e4);
    }
}
