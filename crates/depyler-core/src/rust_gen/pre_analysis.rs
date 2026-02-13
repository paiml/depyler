//! Pre-analysis phase for `generate_rust_file_internal`.
//!
//! Populates [`CodeGenContext`] with function and class metadata
//! that must be available *before* individual function/class code generation
//! begins.  This includes result-returning maps, option-returning maps,
//! parameter mutability, vararg detection, class field types, and more.
//!
//! Extracted from `rust_gen.rs` lines 874-1011.

use super::context::CodeGenContext;
use super::{argparse_transform, validator_analysis};
use crate::hir::{AssignTarget, HirClass, HirExpr, HirFunction, HirModule, HirStmt, Type};

/// Populate `ctx` with cross-function / cross-class metadata derived from the
/// HIR module.
///
/// This must run **after** `CodeGenContext` creation and **before** any
/// per-function or per-class code generation.
///
/// # Phases
///
/// 1. String optimization analysis
/// 2. Validator analysis (argparse type= parameters)
/// 3. Argparse subcommand pre-registration
/// 4. Result-returning functions map
/// 5. Result<bool> functions map
/// 6. Option-returning functions and return types
/// 7. Function parameter types for literal coercion
/// 8. Vararg function detection
/// 9. Function param_muts pre-population
/// 10. Optional parameter tracking
/// 11. Class field types
/// 12. Class method return types
pub(super) fn populate_context_metadata(module: &HirModule, ctx: &mut CodeGenContext) {
    // -- String optimization --------------------------------------------------

    // Analyze all functions first for string optimization
    validator_analysis::analyze_string_optimization(ctx, &module.functions);

    // Finalize interned string names (resolve collisions)
    ctx.string_optimizer.finalize_interned_names();

    // -- Validator analysis ---------------------------------------------------

    // DEPYLER-0447: Scan all function bodies and constants for argparse validators
    // Must run BEFORE function conversion so validator parameter types are correct
    validator_analysis::analyze_validators(ctx, &module.functions, &module.constants);

    // -- Argparse subcommand pre-registration ---------------------------------

    // DEPYLER-0789: Pre-register ALL argparse subcommands from ALL functions
    // This ensures cmd_* functions have access to argument type info (e.g., store_true -> bool)
    // even when defined before the main() function that sets up argparse
    for func in &module.functions {
        argparse_transform::preregister_subcommands_from_hir(func, &mut ctx.argparser_tracker);
    }

    // -- Result-returning functions -------------------------------------------

    // DEPYLER-0270: Populate Result-returning functions map
    // All functions that can_fail return Result<T, E> and need unwrapping at call sites
    populate_result_returning(ctx, &module.functions);

    // -- Result<bool> functions -----------------------------------------------

    // DEPYLER-0308: Populate Result<bool> functions map
    // Functions that can_fail and return Bool need unwrapping in boolean contexts
    populate_result_bool(ctx, &module.functions);

    // -- Option-returning functions & return types ----------------------------

    // DEPYLER-0497: Populate Option-returning functions map and function return types
    // Functions that return Option<T> need unwrapping in format! and other Display contexts
    populate_option_returning_and_return_types(ctx, &module.functions);

    // -- Function parameter types for literal coercion ------------------------

    // DEPYLER-0950: Populate function parameter types for literal coercion at call sites
    // When calling add(1, 2.5) where add expects (f64, f64), we need to coerce 1 to 1.0
    populate_param_types(ctx, &module.functions);

    // -- Vararg function detection --------------------------------------------

    // DEPYLER-0648: Pre-populate vararg functions before codegen
    // Python *args functions become fn(args: &[String]) in Rust
    // Call sites need to wrap arguments in &[...] slices
    populate_vararg_functions(ctx, &module.functions);

    // -- Function param_muts pre-population -----------------------------------

    // DEPYLER-99MODE-S9: Pre-populate function_param_muts for all functions
    // This enables correct &mut passing at call sites for forward references
    // (e.g., fibonacci_memo calls fib_helper which is defined later)
    populate_param_muts(ctx, &module.functions);

    // -- Optional parameter tracking ------------------------------------------

    // DEPYLER-0737: Pre-populate Optional parameters for call site wrapping
    // When a parameter has Optional type (from =None default), call sites need Some() wrapping
    populate_optional_params(ctx, &module.functions);

    // -- Class field types ----------------------------------------------------

    // DEPYLER-0720: Pre-populate class field types for self.X attribute access
    // This enables expr_returns_float() to recognize self.balance as float
    populate_class_field_types(ctx, &module.classes);

    // -- Class method return types --------------------------------------------

    // DEPYLER-1007: Pre-populate class method return types for return type inference
    // This enables infer_expr_type_with_env() to recognize p.distance_squared() return type
    populate_class_method_return_types(ctx, &module.classes);
}

// ---------------------------------------------------------------------------
// Helper functions (private)
// ---------------------------------------------------------------------------

fn populate_result_returning(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        if func.properties.can_fail {
            ctx.result_returning_functions.insert(func.name.clone());
        }
    }
}

fn populate_result_bool(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        if func.properties.can_fail && matches!(func.ret_type, Type::Bool) {
            ctx.result_bool_functions.insert(func.name.clone());
        }
    }
}

fn populate_option_returning_and_return_types(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        // Store all function return types for type tracking
        ctx.function_return_types
            .insert(func.name.clone(), func.ret_type.clone());

        // Track Option-returning functions specifically
        if matches!(func.ret_type, Type::Optional(_)) {
            ctx.option_returning_functions.insert(func.name.clone());
        }
    }
}

fn populate_param_types(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        let param_types: Vec<Type> = func.params.iter().map(|p| p.ty.clone()).collect();
        ctx.function_param_types
            .insert(func.name.clone(), param_types);
    }
}

fn populate_vararg_functions(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        if func.params.iter().any(|p| p.is_vararg) {
            ctx.vararg_functions.insert(func.name.clone());
        }
    }
}

fn populate_param_muts(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        let param_muts: Vec<bool> = func
            .params
            .iter()
            .map(|p| {
                // A param needs &mut if: (a) it's a Dict/List type AND (b) body mutates it
                let is_collection = matches!(p.ty, Type::Dict(_, _) | Type::List(_) | Type::Set(_));
                if !is_collection {
                    return false;
                }
                // Check if body has subscript assignment to this param
                param_is_mutated_in_body(&p.name, &func.body)
            })
            .collect();
        if param_muts.iter().any(|&m| m) {
            ctx.function_param_muts
                .insert(func.name.clone(), param_muts);
        }
    }
}

fn populate_optional_params(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        let optionals: Vec<bool> = func
            .params
            .iter()
            .map(|p| matches!(p.ty, Type::Optional(_)))
            .collect();
        // Only track if any param is Optional
        if optionals.iter().any(|&b| b) {
            ctx.function_param_optionals
                .insert(func.name.clone(), optionals);
        }
    }
}

fn populate_class_field_types(ctx: &mut CodeGenContext, classes: &[HirClass]) {
    for class in classes {
        for field in &class.fields {
            ctx.class_field_types
                .insert(field.name.clone(), field.field_type.clone());
        }
    }
}

fn populate_class_method_return_types(ctx: &mut CodeGenContext, classes: &[HirClass]) {
    for class in classes {
        // Track constructor return type: ClassName() -> Type::Custom("ClassName")
        // This enables type inference for expressions like `p = Point(3, 4)`
        ctx.function_return_types
            .insert(class.name.clone(), Type::Custom(class.name.clone()));
        // DEPYLER-99MODE-S9: Register class name so NASA mode type annotations
        // use the struct type instead of HashMap<DepylerValue, DepylerValue>
        ctx.class_names.insert(class.name.clone());

        for method in &class.methods {
            // Skip __init__ and __new__ which don't have meaningful return types for inference
            if method.name == "__init__" || method.name == "__new__" {
                continue;
            }
            // Only track methods with explicit return type annotations
            if !matches!(method.ret_type, Type::Unknown | Type::None) {
                ctx.class_method_return_types.insert(
                    (class.name.clone(), method.name.clone()),
                    method.ret_type.clone(),
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Body mutation analysis (moved from rust_gen.rs `param_is_mutated_in_body`)
// ---------------------------------------------------------------------------

/// Check whether a collection parameter is mutated anywhere in the function body.
///
/// Looks for subscript assignments (`param[key] = val`) and mutating method calls
/// (`param.insert(...)`, `param.append(...)`, etc.) recursively through control flow.
fn param_is_mutated_in_body(param_name: &str, body: &[HirStmt]) -> bool {
    for stmt in body {
        match stmt {
            HirStmt::Assign { target, .. } => {
                if let AssignTarget::Index { base, .. } = target {
                    if let HirExpr::Var(name) = base.as_ref() {
                        if name == param_name {
                            return true;
                        }
                    }
                }
            }
            HirStmt::Expr(HirExpr::MethodCall { object, method, .. }) => {
                if let HirExpr::Var(name) = object.as_ref() {
                    if name == param_name
                        && matches!(
                            method.as_str(),
                            "insert"
                                | "pop"
                                | "remove"
                                | "clear"
                                | "update"
                                | "append"
                                | "extend"
                                | "add"
                                | "discard"
                        )
                    {
                        return true;
                    }
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                if param_is_mutated_in_body(param_name, then_body) {
                    return true;
                }
                if let Some(eb) = else_body {
                    if param_is_mutated_in_body(param_name, eb) {
                        return true;
                    }
                }
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                if param_is_mutated_in_body(param_name, body) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{
        FunctionProperties, HirClass, HirField, HirFunction, HirMethod, HirModule, HirParam,
        Literal,
    };
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    /// Helper to create a minimal HirModule with the given functions and classes.
    fn make_module(functions: Vec<HirFunction>, classes: Vec<HirClass>) -> HirModule {
        HirModule {
            functions,
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes,
            constants: vec![],
            top_level_stmts: vec![],
        }
    }

    /// Helper to create a minimal HirFunction with the given name, params, and return type.
    fn make_func(name: &str, params: Vec<HirParam>, ret_type: Type) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: smallvec::SmallVec::from_vec(params),
            ret_type,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }
    }

    /// Helper to create a minimal HirMethod.
    fn make_method(name: &str, ret_type: Type) -> HirMethod {
        HirMethod {
            name: name.to_string(),
            params: smallvec![],
            ret_type,
            body: vec![],
            is_static: false,
            is_classmethod: false,
            is_property: false,
            is_async: false,
            docstring: None,
        }
    }

    fn make_param(name: &str, ty: Type) -> HirParam {
        HirParam::new(name.to_string(), ty)
    }

    #[test]
    fn test_populate_result_returning_marks_can_fail() {
        let funcs = vec![{
            let mut f = make_func("risky", vec![], Type::Int);
            f.properties.can_fail = true;
            f
        }];
        let module = make_module(funcs, vec![]);
        let mut ctx = CodeGenContext::default();
        populate_context_metadata(&module, &mut ctx);
        assert!(ctx.result_returning_functions.contains("risky"));
    }

    #[test]
    fn test_populate_result_bool_marks_bool_can_fail() {
        let funcs = vec![{
            let mut f = make_func("check", vec![], Type::Bool);
            f.properties.can_fail = true;
            f
        }];
        let module = make_module(funcs, vec![]);
        let mut ctx = CodeGenContext::default();
        populate_context_metadata(&module, &mut ctx);
        assert!(ctx.result_bool_functions.contains("check"));
    }

    #[test]
    fn test_populate_option_returning() {
        let funcs = vec![make_func(
            "maybe",
            vec![],
            Type::Optional(Box::new(Type::Int)),
        )];
        let module = make_module(funcs, vec![]);
        let mut ctx = CodeGenContext::default();
        populate_context_metadata(&module, &mut ctx);
        assert!(ctx.option_returning_functions.contains("maybe"));
        assert_eq!(
            ctx.function_return_types.get("maybe"),
            Some(&Type::Optional(Box::new(Type::Int)))
        );
    }

    #[test]
    fn test_populate_param_types() {
        let funcs = vec![make_func(
            "add",
            vec![make_param("a", Type::Float), make_param("b", Type::Float)],
            Type::Float,
        )];
        let module = make_module(funcs, vec![]);
        let mut ctx = CodeGenContext::default();
        populate_context_metadata(&module, &mut ctx);
        assert_eq!(
            ctx.function_param_types.get("add"),
            Some(&vec![Type::Float, Type::Float])
        );
    }

    #[test]
    fn test_populate_vararg_functions() {
        let funcs = vec![make_func(
            "variadic",
            vec![HirParam {
                name: "args".to_string(),
                ty: Type::List(Box::new(Type::String)),
                default: None,
                is_vararg: true,
            }],
            Type::None,
        )];
        let module = make_module(funcs, vec![]);
        let mut ctx = CodeGenContext::default();
        populate_context_metadata(&module, &mut ctx);
        assert!(ctx.vararg_functions.contains("variadic"));
    }

    #[test]
    fn test_populate_optional_params() {
        let funcs = vec![make_func(
            "with_opt",
            vec![
                make_param("x", Type::Int),
                make_param("y", Type::Optional(Box::new(Type::Int))),
            ],
            Type::None,
        )];
        let module = make_module(funcs, vec![]);
        let mut ctx = CodeGenContext::default();
        populate_context_metadata(&module, &mut ctx);
        let optionals = ctx.function_param_optionals.get("with_opt").unwrap();
        assert_eq!(optionals, &vec![false, true]);
    }

    #[test]
    fn test_populate_class_field_types() {
        let classes = vec![HirClass {
            name: "Account".to_string(),
            base_classes: vec![],
            fields: vec![HirField {
                name: "balance".to_string(),
                field_type: Type::Float,
                default_value: None,
                is_class_var: false,
            }],
            methods: vec![],
            is_dataclass: false,
            docstring: None,
            type_params: vec![],
        }];
        let module = make_module(vec![], classes);
        let mut ctx = CodeGenContext::default();
        populate_context_metadata(&module, &mut ctx);
        assert_eq!(ctx.class_field_types.get("balance"), Some(&Type::Float));
    }

    #[test]
    fn test_populate_class_method_return_types() {
        let classes = vec![HirClass {
            name: "Point".to_string(),
            base_classes: vec![],
            fields: vec![],
            methods: vec![
                make_method("__init__", Type::None),
                make_method("distance", Type::Float),
            ],
            is_dataclass: false,
            docstring: None,
            type_params: vec![],
        }];
        let module = make_module(vec![], classes);
        let mut ctx = CodeGenContext::default();
        populate_context_metadata(&module, &mut ctx);
        // Constructor return type
        assert_eq!(
            ctx.function_return_types.get("Point"),
            Some(&Type::Custom("Point".to_string()))
        );
        // Class name registered
        assert!(ctx.class_names.contains("Point"));
        // __init__ skipped, distance tracked
        assert_eq!(
            ctx.class_method_return_types
                .get(&("Point".to_string(), "distance".to_string())),
            Some(&Type::Float)
        );
        // __init__ NOT tracked
        assert!(ctx
            .class_method_return_types
            .get(&("Point".to_string(), "__init__".to_string()))
            .is_none());
    }

    #[test]
    fn test_param_is_mutated_subscript_assign() {
        // d[key] = value  ->  should detect mutation of `d`
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
            },
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];
        assert!(param_is_mutated_in_body("d", &body));
        assert!(!param_is_mutated_in_body("other", &body));
    }

    #[test]
    fn test_param_is_mutated_method_call() {
        // lst.append(1)  ->  should detect mutation of `lst`
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(param_is_mutated_in_body("lst", &body));
        assert!(!param_is_mutated_in_body("other", &body));
    }

    #[test]
    fn test_param_is_mutated_nested_in_if() {
        // if True: d.insert(k, v)
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("d".to_string())),
                method: "insert".to_string(),
                args: vec![
                    HirExpr::Literal(Literal::String("k".to_string())),
                    HirExpr::Literal(Literal::Int(1)),
                ],
                kwargs: vec![],
            })],
            else_body: None,
        }];
        assert!(param_is_mutated_in_body("d", &body));
    }

    #[test]
    fn test_param_is_mutated_nested_in_for() {
        // for x in items: lst.extend(x)
        let body = vec![HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("lst".to_string())),
                method: "extend".to_string(),
                args: vec![HirExpr::Var("x".to_string())],
                kwargs: vec![],
            })],
        }];
        assert!(param_is_mutated_in_body("lst", &body));
    }

    #[test]
    fn test_param_not_mutated_read_only() {
        // val = d["key"]  (read, not mutate)
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("val".to_string()),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
            },
            type_annotation: None,
        }];
        assert!(!param_is_mutated_in_body("d", &body));
    }
}
