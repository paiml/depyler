//! DEPYLER-0499: TypeEnvironment with Subtyping Constraints
//!
//! RED Phase: Failing tests for subtyping-aware type system
//!
//! Tests verify:
//! 1. Numeric tower subtyping (Int <: Float)
//! 2. Container variance (List<T> <: List<U> if T <: U)
//! 3. Option lifting (T <: Option<T>)
//! 4. Transitivity (T <: U, U <: V → T <: V)

use depyler_core::type_system::subtyping::SubtypeChecker;
use depyler_core::hir::Type;

#[test]
fn test_subtype_int_to_float() {
    let checker = SubtypeChecker::new();

    // Int <: Float (can safely widen)
    let result = checker.check_subtype(&Type::Int, &Type::Float);
    assert!(result.is_ok(), "Int should be subtype of Float");
}

#[test]
fn test_subtype_not_reflexive_for_narrowing() {
    let checker = SubtypeChecker::new();

    // Float NOT <: Int (narrowing loses information)
    let result = checker.check_subtype(&Type::Float, &Type::Int);
    assert!(result.is_err(), "Float should NOT be subtype of Int");
}

#[test]
fn test_subtype_reflexive_for_same_type() {
    let checker = SubtypeChecker::new();

    // Int <: Int (reflexivity)
    let result = checker.check_subtype(&Type::Int, &Type::Int);
    assert!(result.is_ok(), "Type should be subtype of itself");
}

#[test]
fn test_subtype_option_lift() {
    let checker = SubtypeChecker::new();

    // T <: Option<T> (value lifting)
    let result = checker.check_subtype(
        &Type::Int,
        &Type::Optional(Box::new(Type::Int))
    );
    assert!(result.is_ok(), "T should be subtype of Option<T>");
}

#[test]
fn test_subtype_option_covariance() {
    let checker = SubtypeChecker::new();

    // Option<Int> <: Option<Float> (container covariance)
    let result = checker.check_subtype(
        &Type::Optional(Box::new(Type::Int)),
        &Type::Optional(Box::new(Type::Float))
    );
    assert!(result.is_ok(), "Option<Int> should be subtype of Option<Float>");
}

#[test]
fn test_subtype_list_covariance() {
    let checker = SubtypeChecker::new();

    // List<Int> <: List<Float> (read-only covariance)
    let result = checker.check_subtype(
        &Type::List(Box::new(Type::Int)),
        &Type::List(Box::new(Type::Float))
    );
    assert!(result.is_ok(), "List<Int> should be subtype of List<Float>");
}

#[test]
fn test_subtype_incompatible_types() {
    let checker = SubtypeChecker::new();

    // String NOT <: Int (unrelated types)
    let result = checker.check_subtype(&Type::String, &Type::Int);
    assert!(result.is_err(), "String should NOT be subtype of Int");
}

#[test]
fn test_constraint_subtype_vs_equality() {
    use depyler_core::type_system::constraint::TypeConstraint;
    use depyler_core::type_system::constraint::ConstraintKind;

    // Subtype constraint: arg: Int when param expects Float (should succeed)
    let subtype_constraint = TypeConstraint {
        lhs: Type::Int,
        rhs: Type::Float,
        kind: ConstraintKind::Subtype,
        reason: "Function argument".to_string(),
    };

    let checker = SubtypeChecker::new();
    let result = checker.check_constraint(&subtype_constraint);
    assert!(result.is_ok(), "Subtype constraint Int <: Float should succeed");

    // Equality constraint: arg: Int when param expects Float (should fail)
    let equality_constraint = TypeConstraint {
        lhs: Type::Int,
        rhs: Type::Float,
        kind: ConstraintKind::Eq,
        reason: "Function argument".to_string(),
    };

    let result = checker.check_constraint(&equality_constraint);
    assert!(result.is_err(), "Equality constraint Int == Float should fail");
}

#[test]
fn test_worklist_solver_propagates_bounds() {
    use depyler_core::type_system::solver::WorklistSolver;
    use depyler_core::type_system::constraint::{TypeConstraint, ConstraintKind};

    let mut solver = WorklistSolver::new();

    // Add constraints: x <: y, y <: z
    solver.add_constraint(TypeConstraint {
        lhs: Type::UnificationVar(0), // x
        rhs: Type::UnificationVar(1), // y
        kind: ConstraintKind::Subtype,
        reason: "Assignment".to_string(),
    });

    solver.add_constraint(TypeConstraint {
        lhs: Type::UnificationVar(1), // y
        rhs: Type::UnificationVar(2), // z
        kind: ConstraintKind::Subtype,
        reason: "Assignment".to_string(),
    });

    // Solve: should infer x <: z (transitivity)
    let solution = solver.solve().expect("Solver should converge");

    // Verify transitivity: if x = i32, y = i64, z = f64, then x <: z
    assert!(solution.is_consistent(), "Solution should be consistent");
}

#[test]
fn test_ssa_variable_versioning() {
    use depyler_core::type_system::type_environment::TypeEnvironment;

    let mut env = TypeEnvironment::new();

    // Python: x = 5 (x_0: i64)
    let x_0 = env.bind_var("x", Type::Int);
    assert_eq!(env.get_var_version("x"), Some(0), "First binding should be version 0");

    // Python: x = "hello" (x_1: String) - type change requires new version
    let x_1 = env.bind_var("x", Type::String);
    assert_eq!(env.get_var_version("x"), Some(1), "Type change should create version 1");

    assert_ne!(x_0, x_1, "Different versions should have different IDs");

    // Verify both versions exist
    assert_eq!(env.get_type_by_id(x_0), Some(&Type::Int));
    assert_eq!(env.get_type_by_id(x_1), Some(&Type::String));
}

#[test]
fn test_bidirectional_checking_synthesis() {
    use depyler_core::type_system::type_environment::TypeEnvironment;
    use depyler_core::hir::HirExpr;

    let mut env = TypeEnvironment::new();

    // Synthesis: infer type from literal
    let expr = HirExpr::Literal(depyler_core::hir::Literal::Int(42));
    let inferred = env.synthesize_type(&expr).expect("Should infer i32 from small literal");

    assert_eq!(inferred, Type::Int, "Small int literal should synthesize to i32");
}

#[test]
fn test_bidirectional_checking_check() {
    use depyler_core::type_system::type_environment::TypeEnvironment;
    use depyler_core::hir::HirExpr;

    let mut env = TypeEnvironment::new();

    // Checking: verify expression against expected type
    let expr = HirExpr::Literal(depyler_core::hir::Literal::Int(42));
    let result = env.check_type(&expr, &Type::Int);

    // Should succeed: i32 literal <: i64 expected
    assert!(result.is_ok(), "i32 literal should check against i64 (subtyping)");
}
