use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::hir::*;
use crate::string_optimization::StringOptimizer;
use anyhow::Result;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{self, parse_quote};

// Module declarations for rust_gen refactoring (v3.18.0 Phases 2-7)
mod context;
mod error_gen;
mod expr_gen;
mod format;
mod func_gen;
mod generator_gen;
mod import_gen;
mod stmt_gen;
mod type_gen;

// Internal imports
use error_gen::generate_error_type_definitions;
use format::format_rust_code;
use import_gen::process_module_imports;
#[cfg(test)]
use stmt_gen::{
    codegen_assign_attribute, codegen_assign_index, codegen_assign_symbol, codegen_assign_tuple,
    codegen_break_stmt, codegen_continue_stmt, codegen_expr_stmt, codegen_pass_stmt,
    codegen_raise_stmt, codegen_return_stmt, codegen_try_stmt, codegen_while_stmt,
    codegen_with_stmt,
};

// Public re-exports for external modules (union_enum_gen, etc.)
pub use context::{CodeGenContext, RustCodeGen, ToRustExpr};
pub use type_gen::rust_type_to_syn;

// Internal re-exports for cross-module access
pub(crate) use func_gen::return_type_expects_float;

/// Analyze functions for string optimization
///
/// Performs string optimization analysis on all functions.
/// Complexity: 2 (well within ≤10 target)
fn analyze_string_optimization(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        ctx.string_optimizer.analyze_function(func);
    }
}

/// Analyze which variables are reassigned (mutated) in a list of statements
///
/// Populates ctx.mutable_vars with variables that are:
/// 1. Reassigned after declaration (x = 1; x = 2)
/// 2. Mutated via method calls (.push(), .extend(), .insert(), .remove(), .pop(), etc.)
///
/// Complexity: 7 (stmt loop + match + if + expr scan + method match)
fn analyze_mutable_vars(stmts: &[HirStmt], ctx: &mut CodeGenContext) {
    let mut declared = HashSet::new();

    fn analyze_expr_for_mutations(expr: &HirExpr, mutable: &mut HashSet<String>) {
        match expr {
            HirExpr::MethodCall { object, method, args } => {
                // Check if this is a mutating method call
                if is_mutating_method(method) {
                    if let HirExpr::Var(var_name) = &**object {
                        mutable.insert(var_name.clone());
                    }
                }
                // Recursively check nested expressions
                analyze_expr_for_mutations(object, mutable);
                for arg in args {
                    analyze_expr_for_mutations(arg, mutable);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                analyze_expr_for_mutations(left, mutable);
                analyze_expr_for_mutations(right, mutable);
            }
            HirExpr::Unary { operand, .. } => {
                analyze_expr_for_mutations(operand, mutable);
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    analyze_expr_for_mutations(arg, mutable);
                }
            }
            HirExpr::IfExpr { test, body, orelse } => {
                analyze_expr_for_mutations(test, mutable);
                analyze_expr_for_mutations(body, mutable);
                analyze_expr_for_mutations(orelse, mutable);
            }
            HirExpr::List(items) | HirExpr::Tuple(items) | HirExpr::Set(items) | HirExpr::FrozenSet(items) => {
                for item in items {
                    analyze_expr_for_mutations(item, mutable);
                }
            }
            HirExpr::Dict(pairs) => {
                for (key, value) in pairs {
                    analyze_expr_for_mutations(key, mutable);
                    analyze_expr_for_mutations(value, mutable);
                }
            }
            HirExpr::Index { base, index } => {
                analyze_expr_for_mutations(base, mutable);
                analyze_expr_for_mutations(index, mutable);
            }
            HirExpr::Attribute { value, .. } => {
                analyze_expr_for_mutations(value, mutable);
            }
            _ => {}
        }
    }

    fn is_mutating_method(method: &str) -> bool {
        matches!(
            method,
            "append" | "extend" | "insert" | "remove" | "pop" | "clear" | "reverse" | "sort"
        )
    }

    fn analyze_stmt(stmt: &HirStmt, declared: &mut HashSet<String>, mutable: &mut HashSet<String>) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                // Check if the value expression contains method calls that mutate variables
                analyze_expr_for_mutations(value, mutable);

                match target {
                    AssignTarget::Symbol(name) => {
                        if declared.contains(name) {
                            // Variable is being reassigned - mark as mutable
                            mutable.insert(name.clone());
                        } else {
                            // First declaration
                            declared.insert(name.clone());
                        }
                    }
                    AssignTarget::Tuple(targets) => {
                        // Tuple assignment - analyze each element
                        for t in targets {
                            if let AssignTarget::Symbol(name) = t {
                                if declared.contains(name) {
                                    // Variable is being reassigned - mark as mutable
                                    mutable.insert(name.clone());
                                } else {
                                    // First declaration
                                    declared.insert(name.clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            HirStmt::Expr(expr) => {
                // Check standalone expressions for method calls (e.g., numbers.push(4))
                analyze_expr_for_mutations(expr, mutable);
            }
            HirStmt::Return(Some(expr)) => {
                analyze_expr_for_mutations(expr, mutable);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                analyze_expr_for_mutations(condition, mutable);
                for stmt in then_body {
                    analyze_stmt(stmt, declared, mutable);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        analyze_stmt(stmt, declared, mutable);
                    }
                }
            }
            HirStmt::While { condition, body, .. } => {
                analyze_expr_for_mutations(condition, mutable);
                for stmt in body {
                    analyze_stmt(stmt, declared, mutable);
                }
            }
            HirStmt::For { body, .. } => {
                for stmt in body {
                    analyze_stmt(stmt, declared, mutable);
                }
            }
            _ => {}
        }
    }

    for stmt in stmts {
        analyze_stmt(stmt, &mut declared, &mut ctx.mutable_vars);
    }
}

/// Convert Python classes to Rust structs
///
/// Processes all classes and generates token streams.
/// Complexity: 3 (well within ≤10 target)
fn convert_classes_to_rust(
    classes: &[HirClass],
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut class_items = Vec::new();
    for class in classes {
        let items = crate::direct_rules::convert_class_to_struct(class, type_mapper)?;
        for item in items {
            let tokens = item.to_token_stream();
            class_items.push(tokens);
        }
    }
    Ok(class_items)
}

/// Convert HIR functions to Rust token streams
///
/// Processes all functions using the code generation context.
/// Complexity: 2 (well within ≤10 target)
fn convert_functions_to_rust(
    functions: &[HirFunction],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    functions
        .iter()
        .map(|f| f.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()
}

/// Generate conditional imports based on code generation context
///
/// Adds imports for collections and smart pointers as needed.
/// Complexity: 1 (data-driven approach, well within ≤10 target)
fn generate_conditional_imports(ctx: &CodeGenContext) -> Vec<proc_macro2::TokenStream> {
    let mut imports = Vec::new();

    // Define all possible conditional imports
    let conditional_imports = [
        (ctx.needs_hashmap, quote! { use std::collections::HashMap; }),
        (ctx.needs_hashset, quote! { use std::collections::HashSet; }),
        (ctx.needs_vecdeque, quote! { use std::collections::VecDeque; }),
        (ctx.needs_fnv_hashmap, quote! { use fnv::FnvHashMap; }),
        (ctx.needs_ahash_hashmap, quote! { use ahash::AHashMap; }),
        (ctx.needs_arc, quote! { use std::sync::Arc; }),
        (ctx.needs_rc, quote! { use std::rc::Rc; }),
        (ctx.needs_cow, quote! { use std::borrow::Cow; }),
    ];

    // Add imports where needed
    for (needed, import_tokens) in conditional_imports {
        if needed {
            imports.push(import_tokens);
        }
    }

    imports
}

/// Generate import token streams from Python imports
///
/// Maps Python imports to Rust use statements.
/// Complexity: ~7-8 (within ≤10 target)
fn generate_import_tokens(
    imports: &[Import],
    module_mapper: &crate::module_mapper::ModuleMapper,
) -> Vec<proc_macro2::TokenStream> {
    let mut items = Vec::new();
    let mut external_imports = Vec::new();
    let mut std_imports = Vec::new();

    // Categorize imports
    for import in imports {
        let rust_imports = module_mapper.map_import(import);
        for rust_import in rust_imports {
            if rust_import.path.starts_with("//") {
                // Comment for unmapped imports
                let comment = &rust_import.path;
                items.push(quote! { #[doc = #comment] });
            } else if rust_import.is_external {
                external_imports.push(rust_import);
            } else {
                std_imports.push(rust_import);
            }
        }
    }

    // Add external imports
    for import in external_imports {
        let path: syn::Path =
            syn::parse_str(&import.path).unwrap_or_else(|_| parse_quote! { unknown });
        if let Some(alias) = import.alias {
            let alias_ident = syn::Ident::new(&alias, proc_macro2::Span::call_site());
            items.push(quote! { use #path as #alias_ident; });
        } else {
            items.push(quote! { use #path; });
        }
    }

    // Add standard library imports
    for import in std_imports {
        // Skip typing imports as they're handled by the type system
        if import.path.starts_with("::") || import.path.is_empty() {
            continue;
        }
        let path: syn::Path = syn::parse_str(&import.path).unwrap_or_else(|_| parse_quote! { std });
        if let Some(alias) = import.alias {
            let alias_ident = syn::Ident::new(&alias, proc_macro2::Span::call_site());
            items.push(quote! { use #path as #alias_ident; });
        } else {
            items.push(quote! { use #path; });
        }
    }

    items
}

/// Generate interned string constant tokens
///
/// Generates constant definitions for interned strings.
/// Complexity: 2 (well within ≤10 target)
fn generate_interned_string_tokens(optimizer: &StringOptimizer) -> Vec<proc_macro2::TokenStream> {
    let interned_constants = optimizer.generate_interned_constants();
    interned_constants
        .into_iter()
        .filter_map(|constant| constant.parse().ok())
        .collect()
}

/// Generate a complete Rust file from HIR module
pub fn generate_rust_file(
    module: &HirModule,
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<String> {
    let module_mapper = crate::module_mapper::ModuleMapper::new();

    // Process imports to populate the context
    let (imported_modules, imported_items) =
        process_module_imports(&module.imports, &module_mapper);

    let mut ctx = CodeGenContext {
        type_mapper,
        annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(type_mapper.clone()),
        string_optimizer: StringOptimizer::new(),
        union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
        generated_enums: Vec::new(),
        needs_hashmap: false,
        needs_hashset: false,
        needs_vecdeque: false,
        needs_fnv_hashmap: false,
        needs_ahash_hashmap: false,
        needs_arc: false,
        needs_rc: false,
        needs_cow: false,
        declared_vars: vec![HashSet::new()],
        current_function_can_fail: false,
        current_return_type: None,
        module_mapper,
        imported_modules,
        imported_items,
        mutable_vars: HashSet::new(),
        needs_zerodivisionerror: false,
        in_generator: false,
        needs_indexerror: false,
        is_classmethod: false,
        generator_state_vars: HashSet::new(),
    };

    // Analyze all functions first for string optimization
    analyze_string_optimization(&mut ctx, &module.functions);

    // Convert classes first (they might be used by functions)
    let classes = convert_classes_to_rust(&module.classes, ctx.type_mapper)?;

    // Convert all functions to detect what imports we need
    let functions = convert_functions_to_rust(&module.functions, &mut ctx)?;

    // Build items list with all generated code
    let mut items = Vec::new();

    // Add module imports (create new mapper for token generation)
    let import_mapper = crate::module_mapper::ModuleMapper::new();
    items.extend(generate_import_tokens(&module.imports, &import_mapper));

    // Add interned string constants
    items.extend(generate_interned_string_tokens(&ctx.string_optimizer));

    // Add collection imports if needed
    items.extend(generate_conditional_imports(&ctx));

    // Add error type definitions if needed
    items.extend(generate_error_type_definitions(&ctx));

    // Add generated union enums
    items.extend(ctx.generated_enums.clone());

    // Add classes
    items.extend(classes);

    // Add all functions
    items.extend(functions);

    // Generate tests for functions if applicable
    let test_gen = crate::test_generation::TestGenerator::new(Default::default());
    let mut test_modules = Vec::new();

    for func in &module.functions {
        if let Some(test_module) = test_gen.generate_tests(func)? {
            test_modules.push(test_module);
        }
    }

    // Add test modules
    items.extend(test_modules);

    let file = quote! {
        #(#items)*
    };

    Ok(format_rust_code(file.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
    use crate::rust_gen::context::RustCodeGen;
    use crate::type_mapper::TypeMapper;
    use depyler_annotations::TranspilationAnnotations;
    use std::collections::HashSet;
    use crate::rust_gen::type_gen::convert_binop;

    fn create_test_context() -> CodeGenContext<'static> {
        // This is a bit of a hack for testing - in real use, the TypeMapper would have a longer lifetime
        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        CodeGenContext {
            type_mapper,
            annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(
                type_mapper.clone(),
            ),
            string_optimizer: StringOptimizer::new(),
            union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
            generated_enums: Vec::new(),
            needs_hashmap: false,
            needs_hashset: false,
        needs_vecdeque: false,
            needs_fnv_hashmap: false,
            needs_ahash_hashmap: false,
            needs_arc: false,
            needs_rc: false,
            needs_cow: false,
            declared_vars: vec![HashSet::new()],
            current_function_can_fail: false,
            current_return_type: None,
            module_mapper: crate::module_mapper::ModuleMapper::new(),
            imported_modules: std::collections::HashMap::new(),
            imported_items: std::collections::HashMap::new(),
            mutable_vars: HashSet::new(),
            needs_zerodivisionerror: false,
            needs_indexerror: false,
            is_classmethod: false,
            in_generator: false,
            generator_state_vars: HashSet::new(),
        }
    }

    #[test]
    fn test_simple_function_generation() {
        let func = HirFunction {
            name: "add".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let tokens = func.to_rust_tokens(&mut ctx).unwrap();
        let code = tokens.to_string();

        assert!(code.contains("pub fn add"));
        assert!(code.contains("i32"));
        assert!(code.contains("return"));
    }

    #[test]
    fn test_control_flow_generation() {
        let if_stmt = HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String(
                "positive".to_string(),
            ))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(
                Literal::String("negative".to_string()),
            )))]),
        };

        let mut ctx = create_test_context();
        let tokens = if_stmt.to_rust_tokens(&mut ctx).unwrap();
        let code = tokens.to_string();

        assert!(code.contains("if"));
        assert!(code.contains("else"));
        assert!(code.contains("return"));
    }

    #[test]
    fn test_list_generation() {
        // Test literal array generation
        let list_expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        let mut ctx = create_test_context();
        let expr = list_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #expr }.to_string();

        // Small literal lists should generate arrays
        assert!(code.contains("[") && code.contains("]"));
        assert!(code.contains("1"));
        assert!(code.contains("2"));
        assert!(code.contains("3"));

        // Test non-literal list still uses vec!
        let var_list = HirExpr::List(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Var("y".to_string()),
        ]);

        let expr2 = var_list.to_rust_expr(&mut ctx).unwrap();
        let code2 = quote! { #expr2 }.to_string();
        assert!(code2.contains("vec !"));
    }

    #[test]
    fn test_dict_generation_sets_needs_hashmap() {
        let dict_expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);

        let mut ctx = create_test_context();
        assert!(!ctx.needs_hashmap);

        let _ = dict_expr.to_rust_expr(&mut ctx).unwrap();

        assert!(ctx.needs_hashmap);
    }

    #[test]
    fn test_binary_operations() {
        let ops = vec![
            (BinOp::Add, "+"),
            (BinOp::Sub, "-"),
            (BinOp::Mul, "*"),
            (BinOp::Eq, "=="),
            (BinOp::Lt, "<"),
        ];

        for (op, expected) in ops {
            let result = convert_binop(op).unwrap();
            assert_eq!(quote! { #result }.to_string(), expected);
        }
    }

    #[test]
    fn test_unsupported_operators() {
        assert!(convert_binop(BinOp::Pow).is_err());
        assert!(convert_binop(BinOp::In).is_err());
        assert!(convert_binop(BinOp::NotIn).is_err());
    }

    // ========================================================================
    // DEPYLER-0140 Phase 1: Tests for extracted statement handlers
    // ========================================================================

    #[test]
    fn test_codegen_pass_stmt() {
        let result = codegen_pass_stmt().unwrap();
        assert!(result.is_empty(), "Pass statement should generate no code");
    }

    #[test]
    fn test_codegen_break_stmt_simple() {
        let result = codegen_break_stmt(&None).unwrap();
        assert_eq!(result.to_string(), "break ;");
    }

    #[test]
    fn test_codegen_break_stmt_with_label() {
        let result = codegen_break_stmt(&Some("outer".to_string())).unwrap();
        assert_eq!(result.to_string(), "break 'outer ;");
    }

    #[test]
    fn test_codegen_continue_stmt_simple() {
        let result = codegen_continue_stmt(&None).unwrap();
        assert_eq!(result.to_string(), "continue ;");
    }

    #[test]
    fn test_codegen_continue_stmt_with_label() {
        let result = codegen_continue_stmt(&Some("outer".to_string())).unwrap();
        assert_eq!(result.to_string(), "continue 'outer ;");
    }

    #[test]
    fn test_codegen_expr_stmt() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let expr = HirExpr::Literal(Literal::Int(42));

        let result = codegen_expr_stmt(&expr, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "42 ;");
    }

    // ========================================================================
    // DEPYLER-0140 Phase 2: Tests for medium-complexity statement handlers
    // ========================================================================

    #[test]
    fn test_codegen_return_stmt_simple() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let expr = Some(HirExpr::Literal(Literal::Int(42)));

        let result = codegen_return_stmt(&expr, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return 42 ;");
    }

    #[test]
    fn test_codegen_return_stmt_none() {
        let mut ctx = create_test_context();

        let result = codegen_return_stmt(&None, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return ;");
    }

    #[test]
    fn test_codegen_while_stmt() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let condition = HirExpr::Literal(Literal::Bool(true));
        let body = vec![HirStmt::Pass];

        let result = codegen_while_stmt(&condition, &body, &mut ctx).unwrap();
        assert!(result.to_string().contains("while true"));
    }

    #[test]
    fn test_codegen_raise_stmt_with_exception() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let exc = Some(HirExpr::Literal(Literal::String("Error".to_string())));

        let result = codegen_raise_stmt(&exc, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return Err (\"Error\" . to_string ()) ;");
    }

    #[test]
    fn test_codegen_raise_stmt_bare() {
        let mut ctx = create_test_context();

        let result = codegen_raise_stmt(&None, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return Err (\"Exception raised\" . into ()) ;");
    }

    #[test]
    fn test_codegen_with_stmt_with_target() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let context = HirExpr::Literal(Literal::Int(42));
        let target = Some("file".to_string());
        let body = vec![HirStmt::Pass];

        let result = codegen_with_stmt(&context, &target, &body, &mut ctx).unwrap();
        assert!(result.to_string().contains("let mut file"));
    }

    #[test]
    fn test_codegen_with_stmt_no_target() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let context = HirExpr::Literal(Literal::Int(42));
        let body = vec![HirStmt::Pass];

        let result = codegen_with_stmt(&context, &None, &body, &mut ctx).unwrap();
        assert!(result.to_string().contains("let _context"));
    }

    // Phase 3b tests - Assign handler tests
    #[test]
    fn test_codegen_assign_symbol_new_var() {
        let mut ctx = create_test_context();
        let value_expr = syn::parse_quote! { 42 };

        let result = codegen_assign_symbol("x", value_expr, None, &mut ctx).unwrap();
        assert!(result.to_string().contains("let x = 42"));
    }

    #[test]
    fn test_codegen_assign_symbol_with_type() {
        let mut ctx = create_test_context();
        let value_expr = syn::parse_quote! { 42 };
        let type_ann = Some(quote! { : i32 });

        let result = codegen_assign_symbol("x", value_expr, type_ann, &mut ctx).unwrap();
        assert!(result.to_string().contains("let x : i32 = 42"));
    }

    #[test]
    fn test_codegen_assign_symbol_existing_var() {
        let mut ctx = create_test_context();
        ctx.declare_var("x");
        let value_expr = syn::parse_quote! { 100 };

        let result = codegen_assign_symbol("x", value_expr, None, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "x = 100 ;");
    }

    #[test]
    fn test_codegen_assign_index() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let base = HirExpr::Var("dict".to_string());
        let index = HirExpr::Literal(Literal::String("key".to_string()));
        let value_expr = syn::parse_quote! { 42 };

        let result = codegen_assign_index(&base, &index, value_expr, &mut ctx).unwrap();
        assert!(result.to_string().contains("dict . insert"));
    }

    #[test]
    fn test_codegen_assign_attribute() {
        let mut ctx = create_test_context();
        let base = HirExpr::Var("obj".to_string());
        let value_expr = syn::parse_quote! { 42 };

        let result = codegen_assign_attribute(&base, "field", value_expr, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "obj . field = 42 ;");
    }

    #[test]
    fn test_codegen_assign_tuple_new_vars() {
        use crate::hir::AssignTarget;

        let mut ctx = create_test_context();
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ];
        let value_expr = syn::parse_quote! { (1, 2) };

        let result = codegen_assign_tuple(&targets, value_expr, None, &mut ctx).unwrap();
        assert!(result.to_string().contains("let (a , b) = (1 , 2)"));
    }

    // Phase 3b tests - Try handler tests
    #[test]
    fn test_codegen_try_stmt_simple() {
        use crate::hir::ExceptHandler;

        let mut ctx = create_test_context();
        let body = vec![HirStmt::Pass];
        let handlers = vec![ExceptHandler {
            exception_type: None,
            name: None,
            body: vec![HirStmt::Pass],
        }];

        let result = codegen_try_stmt(&body, &handlers, &None, &mut ctx).unwrap();
        let result_str = result.to_string();
        assert!(result_str.contains("let _result"));
        assert!(result_str.contains("if let Err (_e) = _result"));
    }

    #[test]
    fn test_codegen_try_stmt_with_finally() {
        let mut ctx = create_test_context();
        let body = vec![HirStmt::Pass];
        let handlers = vec![];
        let finally = Some(vec![HirStmt::Pass]);

        let result = codegen_try_stmt(&body, &handlers, &finally, &mut ctx).unwrap();
        assert!(!result.to_string().is_empty());
    }

    #[test]
    fn test_codegen_try_stmt_except_and_finally() {
        use crate::hir::ExceptHandler;

        let mut ctx = create_test_context();
        let body = vec![HirStmt::Pass];
        let handlers = vec![ExceptHandler {
            exception_type: None,
            name: Some("e".to_string()),
            body: vec![HirStmt::Pass],
        }];
        let finally = Some(vec![HirStmt::Pass]);

        let result = codegen_try_stmt(&body, &handlers, &finally, &mut ctx).unwrap();
        let result_str = result.to_string();
        assert!(result_str.contains("let _result"));
        assert!(result_str.contains("if let Err (_e) = _result"));
    }

    // Phase 1b/1c tests - Type conversion functions (DEPYLER-0149, DEPYLER-0216)
    #[test]
    fn test_int_cast_conversion() {
        // DEPYLER-0216 FIX: Python: int(x) → Rust: (x) as i32 (always cast variables)
        // Previous behavior (no cast) caused "cannot add bool to bool" errors
        // when x is a bool variable: int(flag1) + int(flag2) → flag1 + flag2 (ERROR!)
        let call_expr = HirExpr::Call {
            func: "int".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        // Should generate cast for variables to prevent bool arithmetic errors
        assert!(code.contains("x"), "Expected 'x', got: {}", code);
        assert!(code.contains("as i32"), "Should contain 'as i32' cast, got: {}", code);
    }

    #[test]
    fn test_float_cast_conversion() {
        // Python: float(x) → Rust: (x) as f64
        let call_expr = HirExpr::Call {
            func: "float".to_string(),
            args: vec![HirExpr::Var("y".to_string())],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(code.contains("as f64"), "Expected '(y) as f64', got: {}", code);
    }

    #[test]
    fn test_str_conversion() {
        // Python: str(x) → Rust: x.to_string()
        let call_expr = HirExpr::Call {
            func: "str".to_string(),
            args: vec![HirExpr::Var("value".to_string())],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(code.contains("to_string"), "Expected 'value.to_string()', got: {}", code);
    }

    #[test]
    fn test_bool_cast_conversion() {
        // Python: bool(x) → Rust: (x) as bool
        let call_expr = HirExpr::Call {
            func: "bool".to_string(),
            args: vec![HirExpr::Var("flag".to_string())],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(code.contains("as bool"), "Expected '(flag) as bool', got: {}", code);
    }

    #[test]
    fn test_int_cast_with_expression() {
        // DEPYLER-0216 FIX: Python: int((low + high) / 2) → Rust: ((low + high) / 2) as i32
        // Previous behavior (no cast) caused "cannot add bool to bool" errors
        // when expression might be bool: int(x > 0) + int(y > 0) → (x > 0) + (y > 0) (ERROR!)
        let division = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("low".to_string())),
                right: Box::new(HirExpr::Var("high".to_string())),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };

        let call_expr = HirExpr::Call {
            func: "int".to_string(),
            args: vec![division],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        // Should generate cast for expressions to prevent bool arithmetic errors
        assert!(code.contains("low"), "Expected 'low' variable, got: {}", code);
        assert!(code.contains("high"), "Expected 'high' variable, got: {}", code);
        assert!(code.contains("as i32"), "Should contain 'as i32' cast, got: {}", code);
    }

    #[test]
    fn test_float_literal_decimal_point() {
        // Regression test for DEPYLER-TBD: Ensure float literals always have decimal point
        // Bug: f64::to_string() for 0.0 produces "0" (no decimal), parsed as integer
        // Fix: Always ensure ".0" suffix for floats without decimal/exponent
        let mut ctx = create_test_context();

        // Test 0.0 → should generate "0.0" not "0"
        let zero_float = HirExpr::Literal(Literal::Float(0.0));
        let result = zero_float.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("0.0") || code.contains("0 ."),
                "Expected '0.0' for float zero, got: {}", code);

        // Test 42.0 → should generate "42.0" not "42"
        let forty_two = HirExpr::Literal(Literal::Float(42.0));
        let result = forty_two.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("42.0") || code.contains("42 ."),
                "Expected '42.0' for float, got: {}", code);

        // Test 1.5 → should preserve "1.5" (already has decimal)
        let one_half = HirExpr::Literal(Literal::Float(1.5));
        let result = one_half.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("1.5"), "Expected '1.5', got: {}", code);

        // Test scientific notation: 1e10 → should preserve (has 'e')
        let scientific = HirExpr::Literal(Literal::Float(1e10));
        let result = scientific.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("e") || code.contains("E") || code.contains("."),
                "Expected scientific notation or decimal, got: {}", code);
    }

    #[test]
    fn test_string_method_return_types() {
        // Regression test for v3.16.0 Phase 1
        // String transformation methods (.upper(), .lower(), .strip()) return owned String
        // Function signatures should reflect this: `fn f(s: &str) -> String` not `-> &str`

        // Test 1: .upper() should generate String return type
        let upper_func = HirFunction {
            name: "to_upper".to_string(),
            params: vec![HirParam::new("text".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "upper".to_string(),
                args: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = upper_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        // Should generate: fn to_upper(text: &str) -> String
        // NOT: fn to_upper<'a>(text: &'a str) -> &'a str
        assert!(code.contains("-> String"),
                "Expected '-> String' for .upper() method, got: {}", code);
        assert!(!code.contains("-> & ") && !code.contains("-> &'"),
                "Should not generate borrowed return for .upper(), got: {}", code);

        // Test 2: .lower() should also generate String return type
        let lower_func = HirFunction {
            name: "to_lower".to_string(),
            params: vec![HirParam::new("text".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "lower".to_string(),
                args: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = lower_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        assert!(code.contains("-> String"),
                "Expected '-> String' for .lower() method, got: {}", code);

        // Test 3: .strip() should also generate String return type
        let strip_func = HirFunction {
            name: "trim_text".to_string(),
            params: vec![HirParam::new("text".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "strip".to_string(),
                args: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = strip_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        assert!(code.contains("-> String"),
                "Expected '-> String' for .strip() method, got: {}", code);
    }

    #[test]
    fn test_int_float_division_semantics() {
        // Regression test for v3.16.0 Phase 2
        // Python's `/` operator always returns float, even with int operands
        // Rust's `/` does integer division with int operands
        // We need to cast to float when the context expects float

        // Test 1: int / int returning float (the main bug)
        let divide_func = HirFunction {
            name: "safe_divide".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ].into(),
            ret_type: Type::Float,  // Expects float return!
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Div,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = divide_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        // Should generate: (a as f64) / (b as f64)
        // NOT: a / b (which would do integer division)
        assert!(code.contains("as f64") || code.contains("as f32"),
                "Expected float cast for int/int division with float return, got: {}", code);
        assert!(code.contains("-> f64") || code.contains("-> f32"),
                "Expected float return type, got: {}", code);

        // Test 2: int // int returning int (floor division - should NOT cast)
        let floor_div_func = HirFunction {
            name: "floor_divide".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ].into(),
            ret_type: Type::Int,  // Expects int return
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::FloorDiv,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = floor_div_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        // Floor division should NOT add float casts
        assert!(code.contains("-> i32") || code.contains("-> i64"),
                "Expected int return type for floor division, got: {}", code);

        // Test 3: float / float should work without changes
        let float_div_func = HirFunction {
            name: "divide_floats".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Float),
                HirParam::new("b".to_string(), Type::Float),
            ].into(),
            ret_type: Type::Float,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Div,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = float_div_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        assert!(code.contains("-> f64") || code.contains("-> f32"),
                "Expected float return type, got: {}", code);
    }
}
