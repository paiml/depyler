use crate::hir::*;
use anyhow::Result;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn;

pub fn generate_rust(file: syn::File) -> Result<String> {
    let tokens = file.to_token_stream();
    let rust_code = tokens.to_string();

    // Format the code (in a real implementation, we'd use rustfmt)
    Ok(prettify_rust_code(rust_code))
}

pub fn hir_to_rust(hir: &HirModule) -> Result<String> {
    let mut rust_items = Vec::new();

    // Add necessary imports
    if needs_std_collections(hir) {
        rust_items.push(quote! { use std::collections::HashMap; });
    }

    // Convert each function
    for func in &hir.functions {
        let rust_func = convert_function_to_rust(func)?;
        rust_items.push(rust_func);
    }

    let file = quote! {
        #(#rust_items)*
    };

    Ok(prettify_rust_code(file.to_string()))
}

fn needs_std_collections(hir: &HirModule) -> bool {
    hir.functions.iter().any(|f| {
        f.params.iter().any(|(_, ty)| uses_hashmap(ty))
            || uses_hashmap(&f.ret_type)
            || function_body_uses_hashmap(&f.body)
    })
}

fn uses_hashmap(ty: &Type) -> bool {
    match ty {
        Type::Dict(_, _) => true,
        Type::List(inner) | Type::Optional(inner) => uses_hashmap(inner),
        Type::Tuple(types) => types.iter().any(uses_hashmap),
        Type::Function { params, ret } => params.iter().any(uses_hashmap) || uses_hashmap(ret),
        _ => false,
    }
}

fn function_body_uses_hashmap(body: &[HirStmt]) -> bool {
    body.iter().any(stmt_uses_hashmap)
}

fn stmt_uses_hashmap(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { value, .. } => expr_uses_hashmap(value),
        HirStmt::Return(Some(expr)) => expr_uses_hashmap(expr),
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_uses_hashmap(condition)
                || function_body_uses_hashmap(then_body)
                || else_body
                    .as_ref()
                    .is_some_and(|body| function_body_uses_hashmap(body))
        }
        HirStmt::While { condition, body } => {
            expr_uses_hashmap(condition) || function_body_uses_hashmap(body)
        }
        HirStmt::For { iter, body, .. } => {
            expr_uses_hashmap(iter) || function_body_uses_hashmap(body)
        }
        HirStmt::Expr(expr) => expr_uses_hashmap(expr),
        _ => false,
    }
}

fn expr_uses_hashmap(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Dict(_) => true,
        HirExpr::Binary { left, right, .. } => expr_uses_hashmap(left) || expr_uses_hashmap(right),
        HirExpr::Unary { operand, .. } => expr_uses_hashmap(operand),
        HirExpr::Call { args, .. } => args.iter().any(expr_uses_hashmap),
        HirExpr::Index { base, index } => expr_uses_hashmap(base) || expr_uses_hashmap(index),
        HirExpr::List(items) | HirExpr::Tuple(items) => items.iter().any(expr_uses_hashmap),
        _ => false,
    }
}

struct ScopeTracker {
    declared_vars: Vec<HashSet<String>>,
}

impl ScopeTracker {
    fn new() -> Self {
        Self {
            declared_vars: vec![HashSet::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.declared_vars.push(HashSet::new());
    }

    fn exit_scope(&mut self) {
        self.declared_vars.pop();
    }

    fn is_declared(&self, var_name: &str) -> bool {
        self.declared_vars
            .iter()
            .any(|scope| scope.contains(var_name))
    }

    fn declare_var(&mut self, var_name: &str) {
        if let Some(current_scope) = self.declared_vars.last_mut() {
            current_scope.insert(var_name.to_string());
        }
    }
}

fn convert_function_to_rust(func: &HirFunction) -> Result<proc_macro2::TokenStream> {
    let name = syn::Ident::new(&func.name, proc_macro2::Span::call_site());

    // Convert parameters
    let params: Vec<_> = func
        .params
        .iter()
        .map(|(param_name, param_type)| {
            let param_ident = syn::Ident::new(param_name, proc_macro2::Span::call_site());
            let rust_type = type_to_rust_type(param_type);
            quote! { #param_ident: #rust_type }
        })
        .collect();

    // Convert return type
    let return_type = type_to_rust_type(&func.ret_type);

    // Convert body with scope tracking
    let mut scope_tracker = ScopeTracker::new();

    // Declare function parameters in the scope
    for (param_name, _) in &func.params {
        scope_tracker.declare_var(param_name);
    }

    let body_stmts: Vec<_> = func
        .body
        .iter()
        .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, &mut scope_tracker))
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        pub fn #name(#(#params),*) -> #return_type {
            #(#body_stmts)*
        }
    })
}

fn type_to_rust_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Int => quote! { i32 },
        Type::Float => quote! { f64 },
        Type::String => quote! { String },
        Type::Bool => quote! { bool },
        Type::None => quote! { () },
        Type::List(inner) => {
            let inner_type = type_to_rust_type(inner);
            quote! { Vec<#inner_type> }
        }
        Type::Dict(key, value) => {
            let key_type = type_to_rust_type(key);
            let value_type = type_to_rust_type(value);
            quote! { HashMap<#key_type, #value_type> }
        }
        Type::Tuple(types) => {
            let rust_types: Vec<_> = types.iter().map(type_to_rust_type).collect();
            quote! { (#(#rust_types),*) }
        }
        Type::Optional(inner) => {
            let inner_type = type_to_rust_type(inner);
            quote! { Option<#inner_type> }
        }
        Type::Function { params, ret } => {
            let param_types: Vec<_> = params.iter().map(type_to_rust_type).collect();
            let ret_type = type_to_rust_type(ret);
            quote! { fn(#(#param_types),*) -> #ret_type }
        }
        Type::Custom(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
        Type::Unknown => quote! { () },
        Type::TypeVar(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
        Type::Generic { base, params } => {
            let base_ident = syn::Ident::new(base, proc_macro2::Span::call_site());
            let param_types: Vec<_> = params.iter().map(type_to_rust_type).collect();
            quote! { #base_ident<#(#param_types),*> }
        }
        Type::Union(_) => quote! { UnionType }, // Placeholder, will be handled by enum generation
        Type::Array { element_type, size } => {
            let element = type_to_rust_type(element_type);
            match size {
                crate::hir::ConstGeneric::Literal(n) => {
                    let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                    quote! { [#element; #size_lit] }
                }
                crate::hir::ConstGeneric::Parameter(name) => {
                    let param_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                    quote! { [#element; #param_ident] }
                }
                crate::hir::ConstGeneric::Expression(expr) => {
                    // For expressions, parse them as token streams
                    let expr_tokens: proc_macro2::TokenStream = expr.parse().unwrap_or_else(|_| {
                        quote! { /* invalid const expression */ }
                    });
                    quote! { [#element; #expr_tokens] }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn stmt_to_rust_tokens(stmt: &HirStmt) -> Result<proc_macro2::TokenStream> {
    // Legacy function - delegate to the new scope-aware version with a throwaway scope
    let mut scope_tracker = ScopeTracker::new();
    stmt_to_rust_tokens_with_scope(stmt, &mut scope_tracker)
}

fn stmt_to_rust_tokens_with_scope(
    stmt: &HirStmt,
    scope_tracker: &mut ScopeTracker,
) -> Result<proc_macro2::TokenStream> {
    match stmt {
        HirStmt::Assign { target, value } => {
            let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let value_tokens = expr_to_rust_tokens(value)?;

            if scope_tracker.is_declared(target) {
                // Variable already exists, just assign
                Ok(quote! { #target_ident = #value_tokens; })
            } else {
                // First declaration, use let mut
                scope_tracker.declare_var(target);
                Ok(quote! { let mut #target_ident = #value_tokens; })
            }
        }
        HirStmt::Return(expr_opt) => {
            if let Some(expr) = expr_opt {
                let expr_tokens = expr_to_rust_tokens(expr)?;
                Ok(quote! { return #expr_tokens; })
            } else {
                Ok(quote! { return; })
            }
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond_tokens = expr_to_rust_tokens(condition)?;
            scope_tracker.enter_scope();
            let then_stmts: Vec<_> = then_body
                .iter()
                .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
                .collect::<Result<Vec<_>>>()?;
            scope_tracker.exit_scope();

            if let Some(else_stmts) = else_body {
                scope_tracker.enter_scope();
                let else_tokens: Vec<_> = else_stmts
                    .iter()
                    .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
                    .collect::<Result<Vec<_>>>()?;
                scope_tracker.exit_scope();
                Ok(quote! {
                    if #cond_tokens {
                        #(#then_stmts)*
                    } else {
                        #(#else_tokens)*
                    }
                })
            } else {
                Ok(quote! {
                    if #cond_tokens {
                        #(#then_stmts)*
                    }
                })
            }
        }
        HirStmt::While { condition, body } => {
            let cond_tokens = expr_to_rust_tokens(condition)?;
            scope_tracker.enter_scope();
            let body_stmts: Vec<_> = body
                .iter()
                .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
                .collect::<Result<Vec<_>>>()?;
            scope_tracker.exit_scope();
            Ok(quote! {
                while #cond_tokens {
                    #(#body_stmts)*
                }
            })
        }
        HirStmt::For { target, iter, body } => {
            let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let iter_tokens = expr_to_rust_tokens(iter)?;
            scope_tracker.enter_scope();
            scope_tracker.declare_var(target); // for loop variable is declared in the loop scope
            let body_stmts: Vec<_> = body
                .iter()
                .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
                .collect::<Result<Vec<_>>>()?;
            scope_tracker.exit_scope();
            Ok(quote! {
                for #target_ident in #iter_tokens {
                    #(#body_stmts)*
                }
            })
        }
        HirStmt::Expr(expr) => {
            let expr_tokens = expr_to_rust_tokens(expr)?;
            Ok(quote! { #expr_tokens; })
        }
        HirStmt::Raise {
            exception,
            cause: _,
        } => {
            // Simple error handling for codegen - just generate a panic for now
            if let Some(exc) = exception {
                let exc_tokens = expr_to_rust_tokens(exc)?;
                Ok(quote! { panic!("Exception: {}", #exc_tokens); })
            } else {
                Ok(quote! { panic!("Exception raised"); })
            }
        }
    }
}

fn expr_to_rust_tokens(expr: &HirExpr) -> Result<proc_macro2::TokenStream> {
    match expr {
        HirExpr::Literal(lit) => literal_to_rust_tokens(lit),
        HirExpr::Var(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            Ok(quote! { #ident })
        }
        HirExpr::Binary { op, left, right } => {
            let left_tokens = expr_to_rust_tokens(left)?;
            let right_tokens = expr_to_rust_tokens(right)?;

            // Special handling for specific operators
            match op {
                BinOp::Sub if is_len_call(left) => {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    Ok(quote! { #left_tokens.saturating_sub(#right_tokens) })
                }
                BinOp::FloorDiv => {
                    // Python floor division semantics
                    // For now, assume numeric types and use the integer floor division formula
                    Ok(quote! {
                        {
                            let a = #left_tokens;
                            let b = #right_tokens;
                            let q = a / b;
                            let r = a % b;
                            if (r != 0) && ((r < 0) != (b < 0)) { q - 1 } else { q }
                        }
                    })
                }
                _ => {
                    let op_tokens = binop_to_rust_tokens(op);
                    Ok(quote! { (#left_tokens #op_tokens #right_tokens) })
                }
            }
        }
        HirExpr::Unary { op, operand } => {
            let operand_tokens = expr_to_rust_tokens(operand)?;
            let op_tokens = unaryop_to_rust_tokens(op);
            Ok(quote! { (#op_tokens #operand_tokens) })
        }
        HirExpr::Call { func, args } => {
            let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
            let arg_tokens: Vec<_> = args
                .iter()
                .map(expr_to_rust_tokens)
                .collect::<Result<Vec<_>>>()?;
            Ok(quote! { #func_ident(#(#arg_tokens),*) })
        }
        HirExpr::Index { base, index } => {
            let base_tokens = expr_to_rust_tokens(base)?;
            let index_tokens = expr_to_rust_tokens(index)?;
            Ok(quote! { #base_tokens[#index_tokens] })
        }
        HirExpr::List(items) => {
            let item_tokens: Vec<_> = items
                .iter()
                .map(expr_to_rust_tokens)
                .collect::<Result<Vec<_>>>()?;
            Ok(quote! { vec![#(#item_tokens),*] })
        }
        HirExpr::Dict(items) => {
            let mut entries = Vec::new();
            for (key, value) in items {
                let key_tokens = expr_to_rust_tokens(key)?;
                let value_tokens = expr_to_rust_tokens(value)?;
                entries.push(quote! { (#key_tokens, #value_tokens) });
            }
            Ok(quote! {
                {
                    let mut map = HashMap::new();
                    #(map.insert #entries;)*
                    map
                }
            })
        }
        HirExpr::Tuple(items) => {
            let item_tokens: Vec<_> = items
                .iter()
                .map(expr_to_rust_tokens)
                .collect::<Result<Vec<_>>>()?;
            Ok(quote! { (#(#item_tokens),*) })
        }
        HirExpr::Attribute { value, attr } => {
            let value_tokens = expr_to_rust_tokens(value)?;
            let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
            Ok(quote! { #value_tokens.#attr_ident })
        }
        HirExpr::Borrow { expr, mutable } => {
            let expr_tokens = expr_to_rust_tokens(expr)?;
            if *mutable {
                Ok(quote! { &mut #expr_tokens })
            } else {
                Ok(quote! { &#expr_tokens })
            }
        }
        HirExpr::MethodCall {
            object,
            method,
            args,
        } => {
            let obj_tokens = expr_to_rust_tokens(object)?;
            let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
            let arg_tokens: Vec<_> = args
                .iter()
                .map(expr_to_rust_tokens)
                .collect::<Result<Vec<_>>>()?;
            Ok(quote! { #obj_tokens.#method_ident(#(#arg_tokens),*) })
        }
        HirExpr::Slice {
            base,
            start,
            stop,
            step,
        } => {
            let base_tokens = expr_to_rust_tokens(base)?;
            // Simple codegen - just use slice notation where possible
            match (start, stop, step) {
                (None, None, None) => Ok(quote! { #base_tokens.clone() }),
                (Some(start), Some(stop), None) => {
                    let start_tokens = expr_to_rust_tokens(start)?;
                    let stop_tokens = expr_to_rust_tokens(stop)?;
                    Ok(quote! { #base_tokens[#start_tokens..#stop_tokens].to_vec() })
                }
                (Some(start), None, None) => {
                    let start_tokens = expr_to_rust_tokens(start)?;
                    Ok(quote! { #base_tokens[#start_tokens..].to_vec() })
                }
                (None, Some(stop), None) => {
                    let stop_tokens = expr_to_rust_tokens(stop)?;
                    Ok(quote! { #base_tokens[..#stop_tokens].to_vec() })
                }
                _ => {
                    // For complex cases with step, fall back to method call
                    Ok(quote! { slice_complex(#base_tokens) })
                }
            }
        }
        HirExpr::ListComp {
            element,
            target,
            iter,
            condition,
        } => {
            let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let iter_tokens = expr_to_rust_tokens(iter)?;
            let element_tokens = expr_to_rust_tokens(element)?;

            if let Some(cond) = condition {
                // With condition: iter().filter().map().collect()
                let cond_tokens = expr_to_rust_tokens(cond)?;
                Ok(quote! {
                    #iter_tokens
                        .into_iter()
                        .filter(|#target_ident| #cond_tokens)
                        .map(|#target_ident| #element_tokens)
                        .collect::<Vec<_>>()
                })
            } else {
                // Without condition: iter().map().collect()
                Ok(quote! {
                    #iter_tokens
                        .into_iter()
                        .map(|#target_ident| #element_tokens)
                        .collect::<Vec<_>>()
                })
            }
        }
    }
}

fn literal_to_rust_tokens(lit: &Literal) -> Result<proc_macro2::TokenStream> {
    match lit {
        Literal::Int(i) => Ok(quote! { #i }),
        Literal::Float(f) => Ok(quote! { #f }),
        Literal::String(s) => Ok(quote! { #s.to_string() }),
        Literal::Bool(b) => Ok(quote! { #b }),
        Literal::None => Ok(quote! { None }),
    }
}

fn binop_to_rust_tokens(op: &BinOp) -> proc_macro2::TokenStream {
    match op {
        BinOp::Add => quote! { + },
        BinOp::Sub => quote! { - },
        BinOp::Mul => quote! { * },
        BinOp::Div => quote! { / },
        BinOp::FloorDiv => quote! { / }, // Note: not exact equivalent
        BinOp::Mod => quote! { % },
        BinOp::Pow => quote! { .pow }, // Special handling needed
        BinOp::Eq => quote! { == },
        BinOp::NotEq => quote! { != },
        BinOp::Lt => quote! { < },
        BinOp::LtEq => quote! { <= },
        BinOp::Gt => quote! { > },
        BinOp::GtEq => quote! { >= },
        BinOp::And => quote! { && },
        BinOp::Or => quote! { || },
        BinOp::BitAnd => quote! { & },
        BinOp::BitOr => quote! { | },
        BinOp::BitXor => quote! { ^ },
        BinOp::LShift => quote! { << },
        BinOp::RShift => quote! { >> },
        BinOp::In => quote! { .contains }, // Special handling needed
        BinOp::NotIn => quote! { .not_contains }, // Special handling needed
    }
}

fn unaryop_to_rust_tokens(op: &UnaryOp) -> proc_macro2::TokenStream {
    match op {
        UnaryOp::Not => quote! { ! },
        UnaryOp::Neg => quote! { - },
        UnaryOp::Pos => quote! { + },
        UnaryOp::BitNot => quote! { ! },
    }
}

fn prettify_rust_code(code: String) -> String {
    // Very basic formatting - in production, use rustfmt
    code.replace(" ; ", ";\n    ")
        .replace(" { ", " {\n    ")
        .replace(" } ", "\n}\n")
        .replace("} ;", "};")
        .replace(
            "use std :: collections :: HashMap ;",
            "use std::collections::HashMap;",
        )
        // Fix method call spacing
        .replace(" . ", ".")
        .replace(" (", "(")
        .replace(" )", ")")
        // Fix specific common patterns
        .replace(".len ()", ".len()")
        .replace(".push (", ".push(")
        .replace(".insert (", ".insert(")
        .replace(".get (", ".get(")
        .replace(".contains_key (", ".contains_key(")
        .replace(".to_string ()", ".to_string()")
        // Fix spacing around operators in some contexts
        .replace(" ::", "::")
        // Fix attribute spacing
        .replace("# [", "#[")
        // Fix type annotations
        .replace(" : ", ": ")
        .replace(";\n    }", "\n}")
}

/// Check if an expression is a len() call
fn is_len_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, args } if func == "len" && args.len() == 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_annotations::TranspilationAnnotations;

    #[test]
    fn test_simple_function_generation() {
        let func = HirFunction {
            name: "add".to_string(),
            params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
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

        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
        };

        let rust_code = hir_to_rust(&module).unwrap();
        assert!(rust_code.contains("pub fn add"));
        assert!(rust_code.contains("i32"));
        assert!(rust_code.contains("return(a + b)"));
    }

    #[test]
    fn test_type_conversion() {
        assert_eq!(type_to_rust_type(&Type::Int).to_string(), "i32");
        assert_eq!(type_to_rust_type(&Type::String).to_string(), "String");
        assert_eq!(
            type_to_rust_type(&Type::List(Box::new(Type::Int))).to_string(),
            "Vec < i32 >"
        );
        assert_eq!(
            type_to_rust_type(&Type::Optional(Box::new(Type::String))).to_string(),
            "Option < String >"
        );
    }

    #[test]
    fn test_literal_conversion() {
        let int_lit = literal_to_rust_tokens(&Literal::Int(42)).unwrap();
        assert_eq!(int_lit.to_string(), "42i64");

        let str_lit = literal_to_rust_tokens(&Literal::String("hello".to_string())).unwrap();
        assert_eq!(str_lit.to_string(), "\"hello\" . to_string ()");

        let bool_lit = literal_to_rust_tokens(&Literal::Bool(true)).unwrap();
        assert_eq!(bool_lit.to_string(), "true");
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

        let tokens = stmt_to_rust_tokens(&if_stmt).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("if"));
        assert!(code.contains("else"));
        assert!(code.contains("return"));
    }

    #[test]
    fn test_needs_std_collections() {
        let module_with_dict = HirModule {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: vec![(
                    "data".to_string(),
                    Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
                )]
                .into(),
                ret_type: Type::None,
                body: vec![],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
        };

        assert!(needs_std_collections(&module_with_dict));

        let module_without_dict = HirModule {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: vec![("x".to_string(), Type::Int)].into(),
                ret_type: Type::Int,
                body: vec![],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
        };

        assert!(!needs_std_collections(&module_without_dict));
    }

    #[test]
    fn test_assignment_generation() {
        let assign = HirStmt::Assign {
            target: "x".to_string(),
            value: HirExpr::Literal(Literal::Int(42)),
        };

        let tokens = stmt_to_rust_tokens(&assign).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("let mut x = 42"));
    }

    #[test]
    fn test_function_call_generation() {
        let call = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))])],
        };

        let tokens = expr_to_rust_tokens(&call).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("len"));
        assert!(code.contains("vec !") || code.contains("vec!"));
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
            let tokens = binop_to_rust_tokens(&op);
            assert_eq!(tokens.to_string(), expected);
        }
    }

    #[test]
    fn test_floor_division_codegen() {
        // Test floor division with positive integers
        let floor_div = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(7))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };

        let tokens = expr_to_rust_tokens(&floor_div).unwrap();
        let code = tokens.to_string();
        // Should generate the floor division block
        assert!(code.contains("let a"));
        assert!(code.contains("let b"));
        assert!(code.contains("let q = a / b"));
        assert!(code.contains("let r = a % b"));

        // Test with negative operands
        let neg_floor_div = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(-7))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };

        let tokens = expr_to_rust_tokens(&neg_floor_div).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("if (r != 0) && ((r < 0) != (b < 0))"));
    }
}
