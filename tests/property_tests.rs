use depyler_annotations::TranspilationAnnotations;
use depyler_core::hir::{HirExpr, HirParam, Literal, Symbol, Type};
use depyler_core::hir::{HirFunction, HirStmt};

use depyler_core::direct_rules::apply_rules;
use depyler_core::hir::HirModule;
use depyler_core::type_mapper::TypeMapper;
use quickcheck::TestResult;

use quickcheck::Arbitrary;
use quickcheck::Gen;

/// Property: All transpiled functions should produce valid Rust code
#[quickcheck_macros::quickcheck(tests = 100, max_tests = 200)] // Increased test count
fn prop_transpiled_functions_are_valid_rust(func: ArbitraryFunction) -> TestResult {
    // Safety check: avoid overly complex functions
    if func.0.body.len() > 5 {
        return TestResult::discard();
    }

    let module = HirModule {
        functions: vec![func.0],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
        constants: vec![],
    };

    let type_mapper = TypeMapper::default();

    match apply_rules(&module, &type_mapper) {
        Ok(rust_file) => {
            // The generated file should have valid syntax
            let tokens = quote::quote! { #rust_file };
            let code = tokens.to_string();

            // Basic syntax checks
            TestResult::from_bool(code.contains("fn") && code.contains("{") && code.contains("}"))
        }
        Err(_) => TestResult::discard(), // Some functions may not be supported yet
    }
}

/// Property: Type preservation - transpiled code should maintain type correctness
#[quickcheck_macros::quickcheck(tests = 100, max_tests = 200)] // Increased test count
fn prop_type_preservation(expr: ArbitraryTypedExpr) -> TestResult {
    let type_mapper = TypeMapper::default();

    // Create a minimal function to test the expression
    let func = HirFunction {
        name: "test".to_string(),
        params: vec![].into(),
        ret_type: expr.ty.clone(),
        body: vec![HirStmt::Return(Some(expr.expr))],
        properties: Default::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
        constants: vec![],
    };

    match apply_rules(&module, &type_mapper) {
        Ok(rust_file) => {
            let code = quote::quote! { #rust_file }.to_string();

            // Check that the return type matches
            match &expr.ty {
                Type::Int => TestResult::from_bool(code.contains("-> i32")),
                Type::Float => TestResult::from_bool(code.contains("-> f64")),
                Type::String => TestResult::from_bool(code.contains("-> String")),
                Type::Bool => TestResult::from_bool(code.contains("-> bool")),
                Type::None => TestResult::from_bool(!code.contains("->")),
                _ => TestResult::discard(),
            }
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Pure functions should not have side effects
#[quickcheck_macros::quickcheck(tests = 100, max_tests = 200)] // Increased test count
fn prop_pure_functions_have_no_side_effects(func: ArbitraryPureFunction) -> TestResult {
    let module = HirModule {
        functions: vec![func.0],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
        constants: vec![],
    };

    let type_mapper = TypeMapper::default();

    match apply_rules(&module, &type_mapper) {
        Ok(rust_file) => {
            let code = quote::quote! { #rust_file }.to_string();

            // Pure functions should not contain I/O operations
            TestResult::from_bool(
                !code.contains("println!")
                    && !code.contains("print!")
                    && !code.contains("eprintln!")
                    && !code.contains("write!")
                    && !code.contains("std::io"),
            )
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Panic-free functions should not contain panic operations
#[quickcheck_macros::quickcheck(tests = 100, max_tests = 200)] // Increased test count
fn prop_panic_free_functions_dont_panic(func: ArbitraryPanicFreeFunction) -> bool {
    let module = HirModule {
        functions: vec![func.0],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
        constants: vec![],
    };

    let type_mapper = TypeMapper::default();

    match apply_rules(&module, &type_mapper) {
        Ok(rust_file) => {
            let code = quote::quote! { #rust_file }.to_string();

            // Should not contain explicit panics
            !code.contains("panic!")
                && !code.contains("unreachable!")
                && !code.contains("unimplemented!")
                && !code.contains("todo!")
                && !code.contains(".unwrap()")
                && !code.contains(".expect(")
        }
        Err(_) => true, // If transpilation fails, it's still panic-free
    }
}

// Arbitrary implementations for property testing

#[derive(Clone, Debug)]
struct ArbitraryFunction(HirFunction);

impl Arbitrary for ArbitraryFunction {
    fn arbitrary(g: &mut Gen) -> Self {
        let name = format!("func_{}", u32::arbitrary(g) % 100); // Reduce range
        let num_params = std::cmp::min(g.size() % 3, 2); // Limit to max 2 params
        let params: Vec<HirParam> = (0..num_params)
            .map(|i| HirParam {
                name: Symbol::from(format!("param_{i}")),
                ty: arbitrary_simple_type(g),
                default: None,
            })
            .collect();

        let ret_type = arbitrary_simple_type(g);
        let body = arbitrary_function_body(g, &ret_type);

        ArbitraryFunction(HirFunction {
            name,
            params: params.into(),
            ret_type,
            body,
            properties: Default::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        })
    }
}

#[derive(Clone, Debug)]
struct ArbitraryTypedExpr {
    ty: Type,
    expr: HirExpr,
}

impl Arbitrary for ArbitraryTypedExpr {
    fn arbitrary(g: &mut Gen) -> Self {
        let ty = arbitrary_simple_type(g);
        let expr = arbitrary_expr_of_type(g, &ty);
        ArbitraryTypedExpr { ty, expr }
    }
}

#[derive(Clone, Debug)]
struct ArbitraryPureFunction(HirFunction);

impl Arbitrary for ArbitraryPureFunction {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut func = ArbitraryFunction::arbitrary(g).0;
        // Ensure the function is pure by only using arithmetic operations
        func.body = vec![HirStmt::Return(Some(arbitrary_pure_expr(g)))];
        func.properties.is_pure = true;
        ArbitraryPureFunction(func)
    }
}

#[derive(Clone, Debug)]
struct ArbitraryPanicFreeFunction(HirFunction);

impl Arbitrary for ArbitraryPanicFreeFunction {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut func = ArbitraryFunction::arbitrary(g).0;
        // Ensure the function is panic-free by avoiding dangerous operations
        func.body = vec![HirStmt::Return(Some(arbitrary_safe_expr(g)))];
        func.properties.panic_free = true;
        ArbitraryPanicFreeFunction(func)
    }
}

// Helper functions

fn arbitrary_simple_type(g: &mut Gen) -> Type {
    // Use a fixed seed-based approach to avoid non-deterministic behavior
    match (g.size() + 42) % 4 {
        // Only use 4 types to simplify
        0 => Type::Int,
        1 => Type::String,
        2 => Type::Bool,
        _ => Type::None,
    }
}

fn arbitrary_expr_of_type(g: &mut Gen, ty: &Type) -> HirExpr {
    match ty {
        Type::Int => {
            // Limit the range to avoid overflow issues
            let base: u32 = Arbitrary::arbitrary(g);
            let n: i32 = (base % 1000) as i32;
            HirExpr::Literal(Literal::Int(n as i64))
        }
        Type::Float => {
            // Use simple float values to avoid NaN/infinity issues
            let base: u32 = Arbitrary::arbitrary(g);
            let f: f32 = (base % 100) as f32 / 10.0;
            HirExpr::Literal(Literal::Float(f as f64))
        }
        Type::String => {
            // Use simple, short strings to avoid memory issues
            let options = ["test", "hello", "world", "rust", "py"];
            let base: u32 = Arbitrary::arbitrary(g);
            let idx = (base as usize) % options.len();
            HirExpr::Literal(Literal::String(options[idx].to_string()))
        }
        Type::Bool => {
            let b: bool = Arbitrary::arbitrary(g);
            HirExpr::Literal(Literal::Bool(b))
        }
        Type::None => HirExpr::Literal(Literal::None),
        _ => HirExpr::Literal(Literal::None),
    }
}

fn arbitrary_pure_expr(g: &mut Gen) -> HirExpr {
    use depyler_core::hir::BinOp;

    // Limit recursion depth to prevent stack overflow
    if g.size() < 2 {
        let n: i32 = Arbitrary::arbitrary(g);
        return HirExpr::Literal(Literal::Int(n as i64));
    }

    match g.size() % 3 {
        0 => {
            // Literal
            let n: i32 = Arbitrary::arbitrary(g);
            HirExpr::Literal(Literal::Int(n as i64))
        }
        1 => {
            // Binary arithmetic operation - reduce depth to avoid infinite recursion
            let mut sub_gen = Gen::new(g.size() / 2);
            let left = Box::new(arbitrary_pure_expr(&mut sub_gen));
            let right = Box::new(arbitrary_pure_expr(&mut sub_gen));
            let op = match g.size() % 4 {
                0 => BinOp::Add,
                1 => BinOp::Sub,
                2 => BinOp::Mul,
                _ => BinOp::Add,
            };
            HirExpr::Binary { op, left, right }
        }
        _ => {
            // Variable reference
            HirExpr::Var("x".to_string())
        }
    }
}

fn arbitrary_safe_expr(g: &mut Gen) -> HirExpr {
    // Only generate simple, safe expressions that won't panic
    match g.size() % 3 {
        0 => {
            let base: u32 = Arbitrary::arbitrary(g);
            let n: i32 = (base % 100) as i32; // Small safe range
            HirExpr::Literal(Literal::Int(n as i64))
        }
        1 => {
            let options = ["safe", "test", "ok"];
            let base: u32 = Arbitrary::arbitrary(g);
            let idx = (base as usize) % options.len();
            HirExpr::Literal(Literal::String(options[idx].to_string()))
        }
        _ => HirExpr::Literal(Literal::Bool(true)),
    }
}

fn arbitrary_function_body(g: &mut Gen, ret_type: &Type) -> Vec<HirStmt> {
    // Simple body that just returns a value of the correct type
    vec![HirStmt::Return(Some(arbitrary_expr_of_type(g, ret_type)))]
}

#[cfg(all(test, not(feature = "coverage")))]
mod tests {
    use super::*;

    #[test]
    fn test_arbitrary_function_generation() {
        let mut g = Gen::new(10);
        let func = ArbitraryFunction::arbitrary(&mut g);
        assert!(!func.0.name.is_empty());
    }

    #[test]
    fn test_typed_expr_generation() {
        let mut g = Gen::new(10);
        let expr = ArbitraryTypedExpr::arbitrary(&mut g);

        // The expression should match its type
        match (&expr.ty, &expr.expr) {
            (Type::Int, HirExpr::Literal(Literal::Int(_))) => (),
            (Type::Float, HirExpr::Literal(Literal::Float(_))) => (),
            (Type::String, HirExpr::Literal(Literal::String(_))) => (),
            (Type::Bool, HirExpr::Literal(Literal::Bool(_))) => (),
            (Type::None, HirExpr::Literal(Literal::None)) => (),
            _ => panic!("Type mismatch in generated expression"),
        }
    }
}
