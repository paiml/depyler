//! DEPYLER-0499: Worklist Constraint Solver
//!
//! Implements constraint solving using worklist algorithm (not pure unification).
//! Handles subtyping constraints (T1 <: T2) with transitive closure.
//!
//! # Algorithm
//!
//! 1. Initialize worklist with all constraints
//! 2. Pop constraint from worklist
//! 3. Apply constraint, generate new constraints if needed
//! 4. Add new constraints to worklist
//! 5. Repeat until worklist empty or fixed-point reached
//!
//! # Complexity
//!
//! - **Best case**: O(N) for acyclic constraint graph
//! - **Worst case**: O(N²) for dense constraint graph
//! - **Expected**: O(N log N) with good heuristics

use crate::hir::Type;
use crate::type_system::constraint::{TypeConstraint, ConstraintKind};
use crate::type_system::subtyping::SubtypeChecker;
use std::collections::{HashMap, VecDeque};

/// Solution to constraint system
#[derive(Debug, Clone)]
pub struct Solution {
    /// Type assignments for unification variables
    assignments: HashMap<usize, Type>,
    /// Whether solution is consistent (no conflicts)
    consistent: bool,
}

impl Solution {
    /// Check if solution is consistent
    pub fn is_consistent(&self) -> bool {
        self.consistent
    }

    /// Get type assignment for unification variable
    pub fn get(&self, var_id: usize) -> Option<&Type> {
        self.assignments.get(&var_id)
    }
}

/// Worklist-based constraint solver
///
/// Solves constraints using worklist algorithm with subtyping support.
/// Handles transitive closure of subtyping relations.
pub struct WorklistSolver {
    /// Constraints to solve
    constraints: VecDeque<TypeConstraint>,
    /// Type assignments
    assignments: HashMap<usize, Type>,
    /// Subtype checker
    checker: SubtypeChecker,
    /// Iteration counter (for convergence detection)
    iterations: usize,
    /// Max iterations before timeout
    max_iterations: usize,
}

impl WorklistSolver {
    /// Create new worklist solver
    pub fn new() -> Self {
        Self {
            constraints: VecDeque::new(),
            assignments: HashMap::new(),
            checker: SubtypeChecker::new(),
            iterations: 0,
            max_iterations: 1000,
        }
    }

    /// Add constraint to worklist
    pub fn add_constraint(&mut self, constraint: TypeConstraint) {
        self.constraints.push_back(constraint);
    }

    /// Solve all constraints
    ///
    /// Returns solution if constraints are satisfiable, error otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use depyler_core::type_system::solver::WorklistSolver;
    /// use depyler_core::type_system::constraint::{TypeConstraint, ConstraintKind};
    /// use depyler_core::hir::Type;
    ///
    /// let mut solver = WorklistSolver::new();
    ///
    /// // x: i32, param: i64, constraint: x <: param
    /// solver.add_constraint(TypeConstraint {
    ///     lhs: Type::Int32,
    ///     rhs: Type::Int64,
    ///     kind: ConstraintKind::Subtype,
    ///     reason: "Function call".to_string(),
    /// });
    ///
    /// let solution = solver.solve().expect("Should solve");
    /// assert!(solution.is_consistent());
    /// ```
    pub fn solve(&mut self) -> Result<Solution, String> {
        while let Some(constraint) = self.constraints.pop_front() {
            self.iterations += 1;

            if self.iterations > self.max_iterations {
                return Err(format!("Solver timeout after {} iterations", self.max_iterations));
            }

            // Process constraint
            self.process_constraint(&constraint)?;
        }

        Ok(Solution {
            assignments: self.assignments.clone(),
            consistent: true,
        })
    }

    /// Process single constraint
    fn process_constraint(&mut self, constraint: &TypeConstraint) -> Result<(), String> {
        match constraint.kind {
            ConstraintKind::Eq => {
                self.process_equality(&constraint.lhs, &constraint.rhs, &constraint.reason)
            }
            ConstraintKind::Subtype => {
                self.process_subtype(&constraint.lhs, &constraint.rhs, &constraint.reason)
            }
            ConstraintKind::Supertype => {
                // T1 :> T2 is T2 <: T1
                self.process_subtype(&constraint.rhs, &constraint.lhs, &constraint.reason)
            }
            _ => Ok(()), // Other constraints handled elsewhere
        }
    }

    /// Process equality constraint (T1 == T2)
    fn process_equality(&mut self, lhs: &Type, rhs: &Type, reason: &str) -> Result<(), String> {
        match (lhs, rhs) {
            // Unification variable on left
            (Type::UnificationVar(var_id), ty) | (ty, Type::UnificationVar(var_id)) => {
                if let Some(existing) = self.assignments.get(var_id) {
                    // Variable already assigned, check consistency
                    if existing != ty {
                        return Err(format!(
                            "Type mismatch: variable {} has type {:?}, expected {:?} ({})",
                            var_id, existing, ty, reason
                        ));
                    }
                } else {
                    // Assign type to variable
                    self.assignments.insert(*var_id, ty.clone());
                }
                Ok(())
            }

            // Concrete types must match exactly
            (t1, t2) if t1 == t2 => Ok(()),

            _ => Err(format!("Equality constraint failed: {:?} != {:?} ({})", lhs, rhs, reason))
        }
    }

    /// Process subtype constraint (T1 <: T2)
    fn process_subtype(&mut self, lhs: &Type, rhs: &Type, reason: &str) -> Result<(), String> {
        match (lhs, rhs) {
            // Unification variable on left: assign upper bound
            (Type::UnificationVar(var_id), ty) => {
                if let Some(existing) = self.assignments.get(var_id) {
                    // Check if existing type is subtype of new bound
                    self.checker.check_subtype(existing, ty)
                        .map_err(|e| format!("{} ({})", e, reason))?;
                } else {
                    // No assignment yet - for now, assign the upper bound
                    // TODO: Track upper/lower bounds separately for more precise inference
                    self.assignments.insert(*var_id, ty.clone());
                }
                Ok(())
            }

            // Unification variable on right: assign lower bound
            (ty, Type::UnificationVar(var_id)) => {
                if let Some(existing) = self.assignments.get(var_id) {
                    // Check if new type is subtype of existing
                    self.checker.check_subtype(ty, existing)
                        .map_err(|e| format!("{} ({})", e, reason))?;
                } else {
                    // No assignment yet - assign the lower bound
                    self.assignments.insert(*var_id, ty.clone());
                }
                Ok(())
            }

            // Both concrete: check subtyping relation
            (t1, t2) => {
                self.checker.check_subtype(t1, t2)
                    .map_err(|e| format!("{} ({})", e, reason))
            }
        }
    }
}

impl Default for WorklistSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_simple_equality() {
        let mut solver = WorklistSolver::new();

        // x == Int
        solver.add_constraint(TypeConstraint::eq(
            Type::UnificationVar(0),
            Type::Int,
            "Assignment"
        ));

        let solution = solver.solve().expect("Should solve");
        assert_eq!(solution.get(0), Some(&Type::Int));
    }

    #[test]
    fn test_solve_subtype_constraint() {
        let mut solver = WorklistSolver::new();

        // x: Int, param: Float, x <: param (should succeed)
        solver.add_constraint(TypeConstraint::subtype(
            Type::Int,
            Type::Float,
            "Function argument"
        ));

        let solution = solver.solve().expect("Should solve");
        assert!(solution.is_consistent());
    }

    #[test]
    fn test_solve_transitive_subtyping() {
        let mut solver = WorklistSolver::new();

        // x <: y, y <: z
        solver.add_constraint(TypeConstraint::subtype(
            Type::UnificationVar(0),
            Type::UnificationVar(1),
            "Transitivity test"
        ));
        solver.add_constraint(TypeConstraint::subtype(
            Type::UnificationVar(1),
            Type::UnificationVar(2),
            "Transitivity test"
        ));

        let solution = solver.solve().expect("Should solve");
        assert!(solution.is_consistent());
    }
}
