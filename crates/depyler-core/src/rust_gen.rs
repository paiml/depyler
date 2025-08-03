use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::hir::*;
use crate::lifetime_analysis::LifetimeInference;
use crate::string_optimization::{StringContext, StringOptimizer};
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{self, parse_quote};

/// Context for code generation including type mapping and configuration
pub struct CodeGenContext<'a> {
    pub type_mapper: &'a crate::type_mapper::TypeMapper,
    pub annotation_aware_mapper: AnnotationAwareTypeMapper,
    pub string_optimizer: StringOptimizer,
    pub union_enum_generator: crate::union_enum_gen::UnionEnumGenerator,
    pub generated_enums: Vec<proc_macro2::TokenStream>,
    pub needs_hashmap: bool,
    pub needs_fnv_hashmap: bool,
    pub needs_ahash_hashmap: bool,
    pub needs_arc: bool,
    pub needs_rc: bool,
    pub needs_cow: bool,
    pub declared_vars: Vec<HashSet<String>>,
    pub current_function_can_fail: bool,
    pub current_return_type: Option<Type>,
}

impl<'a> CodeGenContext<'a> {
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

    /// Process a Union type and generate an enum if needed
    pub fn process_union_type(&mut self, types: &[crate::hir::Type]) -> String {
        let (enum_name, enum_def) = self.union_enum_generator.generate_union_enum(types);
        if !enum_def.is_empty() {
            self.generated_enums.push(enum_def);
        }
        enum_name
    }
}

/// Trait for converting HIR elements to Rust tokens
pub trait RustCodeGen {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream>;
}

/// Generate a complete Rust file from HIR module
pub fn generate_rust_file(
    module: &HirModule,
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<String> {
    let mut ctx = CodeGenContext {
        type_mapper,
        annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(type_mapper.clone()),
        string_optimizer: StringOptimizer::new(),
        union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
        generated_enums: Vec::new(),
        needs_hashmap: false,
        needs_fnv_hashmap: false,
        needs_ahash_hashmap: false,
        needs_arc: false,
        needs_rc: false,
        needs_cow: false,
        declared_vars: vec![HashSet::new()],
        current_function_can_fail: false,
        current_return_type: None,
    };

    // Analyze all functions first for string optimization
    for func in &module.functions {
        ctx.string_optimizer.analyze_function(func);
    }

    // Convert classes first (they might be used by functions)
    // Always use direct_rules for classes for now
    let mut class_items = Vec::new();
    for class in &module.classes {
        let items = crate::direct_rules::convert_class_to_struct(class, ctx.type_mapper)?;
        for item in items {
            let tokens = item.to_token_stream();
            class_items.push(tokens);
        }
    }
    let classes = class_items;

    // Convert all functions to detect what imports we need
    let functions: Vec<_> = module
        .functions
        .iter()
        .map(|f| f.to_rust_tokens(&mut ctx))
        .collect::<Result<Vec<_>>>()?;

    let mut items = Vec::new();

    // Add interned string constants at the top
    let interned_constants = ctx.string_optimizer.generate_interned_constants();
    for constant in interned_constants {
        let tokens: proc_macro2::TokenStream = constant.parse().unwrap_or_default();
        items.push(tokens);
    }

    // Add imports if needed
    if ctx.needs_hashmap {
        items.push(quote! {
            use std::collections::HashMap;
        });
    }

    if ctx.needs_fnv_hashmap {
        items.push(quote! {
            use fnv::FnvHashMap;
        });
    }

    if ctx.needs_ahash_hashmap {
        items.push(quote! {
            use ahash::AHashMap;
        });
    }

    if ctx.needs_arc {
        items.push(quote! {
            use std::sync::Arc;
        });
    }

    if ctx.needs_rc {
        items.push(quote! {
            use std::rc::Rc;
        });
    }

    if ctx.needs_cow {
        items.push(quote! {
            use std::borrow::Cow;
        });
    }

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

impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        // Perform generic type inference
        let mut generic_registry = crate::generic_inference::TypeVarRegistry::new();
        let type_params = generic_registry.infer_function_generics(self)?;

        // Perform lifetime analysis
        let mut lifetime_inference = LifetimeInference::new();
        let lifetime_result = lifetime_inference.analyze_function(self, ctx.type_mapper);

        // Generate combined generic parameters (lifetimes + type params)
        let generic_params = if type_params.is_empty() && lifetime_result.lifetime_params.is_empty()
        {
            quote! {}
        } else {
            let mut all_params = Vec::new();

            // Add lifetime parameters first
            for lt in &lifetime_result.lifetime_params {
                let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
                all_params.push(quote! { #lt_ident });
            }

            // Add type parameters with their bounds
            for type_param in &type_params {
                let param_name = syn::Ident::new(&type_param.name, proc_macro2::Span::call_site());
                if type_param.bounds.is_empty() {
                    all_params.push(quote! { #param_name });
                } else {
                    let bounds: Vec<_> = type_param
                        .bounds
                        .iter()
                        .map(|b| {
                            let bound: syn::Path =
                                syn::parse_str(b).unwrap_or_else(|_| parse_quote! { Clone });
                            quote! { #bound }
                        })
                        .collect();
                    all_params.push(quote! { #param_name: #(#bounds)+* });
                }
            }

            quote! { <#(#all_params),*> }
        };

        // Generate lifetime bounds
        let where_clause = if lifetime_result.lifetime_bounds.is_empty() {
            quote! {}
        } else {
            let bounds: Vec<_> = lifetime_result
                .lifetime_bounds
                .iter()
                .map(|(from, to)| {
                    let from_lt = syn::Lifetime::new(from, proc_macro2::Span::call_site());
                    let to_lt = syn::Lifetime::new(to, proc_macro2::Span::call_site());
                    quote! { #from_lt: #to_lt }
                })
                .collect();
            quote! { where #(#bounds),* }
        };

        // Convert parameters using lifetime analysis results
        let params: Vec<_> = self
            .params
            .iter()
            .map(|(param_name, param_type)| {
                let param_ident = syn::Ident::new(param_name, proc_macro2::Span::call_site());

                // Check if parameter is mutated
                let is_param_mutated = matches!(
                    lifetime_result.borrowing_strategies.get(param_name),
                    Some(crate::borrowing_context::BorrowingStrategy::TakeOwnership)
                ) && self.body.iter().any(
                    |stmt| matches!(stmt, HirStmt::Assign { target, .. } if target == param_name),
                );

                // Get the inferred parameter info
                if let Some(inferred) = lifetime_result.param_lifetimes.get(param_name) {
                    let rust_type = &inferred.rust_type;

                    // Check if this is a placeholder Union enum that needs proper generation
                    let actual_rust_type =
                        if let crate::type_mapper::RustType::Enum { name, variants: _ } = rust_type
                        {
                            if name == "UnionType" {
                                // Generate a proper enum name and definition from the original Union type
                                if let Type::Union(types) = param_type {
                                    let enum_name = ctx.process_union_type(types);
                                    crate::type_mapper::RustType::Custom(enum_name)
                                } else {
                                    rust_type.clone()
                                }
                            } else {
                                rust_type.clone()
                            }
                        } else {
                            rust_type.clone()
                        };

                    update_import_needs(ctx, &actual_rust_type);
                    let mut ty = rust_type_to_syn(&actual_rust_type)?;

                    // Check if we're dealing with a string that should use Cow
                    if let Some(strategy) = lifetime_result.borrowing_strategies.get(param_name) {
                        match strategy {
                            crate::borrowing_context::BorrowingStrategy::UseCow { lifetime } => {
                                ctx.needs_cow = true;
                                let lt =
                                    syn::Lifetime::new(lifetime, proc_macro2::Span::call_site());
                                ty = parse_quote! { Cow<#lt, str> };
                            }
                            _ => {
                                // Apply normal borrowing if needed
                                if inferred.should_borrow {
                                    // Special case for strings: use &str instead of &String
                                    if matches!(rust_type, crate::type_mapper::RustType::String) {
                                        if let Some(ref lifetime) = inferred.lifetime {
                                            let lt = syn::Lifetime::new(
                                                lifetime,
                                                proc_macro2::Span::call_site(),
                                            );
                                            ty = if inferred.needs_mut {
                                                parse_quote! { &#lt mut str }
                                            } else {
                                                parse_quote! { &#lt str }
                                            };
                                        } else {
                                            ty = if inferred.needs_mut {
                                                parse_quote! { &mut str }
                                            } else {
                                                parse_quote! { &str }
                                            };
                                        }
                                    } else {
                                        // Non-string types
                                        if let Some(ref lifetime) = inferred.lifetime {
                                            let lt = syn::Lifetime::new(
                                                lifetime,
                                                proc_macro2::Span::call_site(),
                                            );
                                            ty = if inferred.needs_mut {
                                                parse_quote! { &#lt mut #ty }
                                            } else {
                                                parse_quote! { &#lt #ty }
                                            };
                                        } else {
                                            ty = if inferred.needs_mut {
                                                parse_quote! { &mut #ty }
                                            } else {
                                                parse_quote! { &#ty }
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Fallback to normal borrowing
                        if inferred.should_borrow {
                            // Special case for strings: use &str instead of &String
                            if matches!(rust_type, crate::type_mapper::RustType::String) {
                                if let Some(ref lifetime) = inferred.lifetime {
                                    let lt = syn::Lifetime::new(
                                        lifetime,
                                        proc_macro2::Span::call_site(),
                                    );
                                    ty = if inferred.needs_mut {
                                        parse_quote! { &#lt mut str }
                                    } else {
                                        parse_quote! { &#lt str }
                                    };
                                } else {
                                    ty = if inferred.needs_mut {
                                        parse_quote! { &mut str }
                                    } else {
                                        parse_quote! { &str }
                                    };
                                }
                            } else {
                                // Non-string types
                                if let Some(ref lifetime) = inferred.lifetime {
                                    let lt = syn::Lifetime::new(
                                        lifetime,
                                        proc_macro2::Span::call_site(),
                                    );
                                    ty = if inferred.needs_mut {
                                        parse_quote! { &#lt mut #ty }
                                    } else {
                                        parse_quote! { &#lt #ty }
                                    };
                                } else {
                                    ty = if inferred.needs_mut {
                                        parse_quote! { &mut #ty }
                                    } else {
                                        parse_quote! { &#ty }
                                    };
                                }
                            }
                        }
                    }

                    if is_param_mutated {
                        Ok(quote! { mut #param_ident: #ty })
                    } else {
                        Ok(quote! { #param_ident: #ty })
                    }
                } else {
                    // Fallback to original mapping
                    let rust_type = ctx
                        .annotation_aware_mapper
                        .map_type_with_annotations(param_type, &self.annotations);
                    update_import_needs(ctx, &rust_type);
                    let ty = rust_type_to_syn(&rust_type)?;
                    if is_param_mutated {
                        Ok(quote! { mut #param_ident: #ty })
                    } else {
                        Ok(quote! { #param_ident: #ty })
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;

        // Convert return type using annotation-aware mapping
        let mapped_ret_type = ctx
            .annotation_aware_mapper
            .map_return_type_with_annotations(&self.ret_type, &self.annotations);

        // Check if this is a placeholder Union enum that needs proper generation
        let rust_ret_type =
            if let crate::type_mapper::RustType::Enum { name, .. } = &mapped_ret_type {
                if name == "UnionType" {
                    // Generate a proper enum name and definition from the original Union type
                    if let Type::Union(types) = &self.ret_type {
                        let enum_name = ctx.process_union_type(types);
                        crate::type_mapper::RustType::Custom(enum_name)
                    } else {
                        mapped_ret_type
                    }
                } else {
                    mapped_ret_type
                }
            } else {
                mapped_ret_type
            };

        // Check if function can fail and needs Result wrapper
        let can_fail = self.properties.can_fail;
        let error_type_str = if can_fail && !self.properties.error_types.is_empty() {
            // Use first error type or generic for mixed types
            if self.properties.error_types.len() == 1 {
                self.properties.error_types[0].clone()
            } else {
                "Box<dyn std::error::Error>".to_string()
            }
        } else {
            "Box<dyn std::error::Error>".to_string()
        };

        let return_type = if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
            if can_fail {
                let error_type: syn::Type = syn::parse_str(&error_type_str)
                    .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });
                quote! { -> Result<(), #error_type> }
            } else {
                quote! {}
            }
        } else {
            let mut ty = rust_type_to_syn(&rust_ret_type)?;

            // Check if any parameter escapes through return and uses Cow
            let mut uses_cow_return = false;
            for (param_name, _) in &self.params {
                if let Some(strategy) = lifetime_result.borrowing_strategies.get(param_name) {
                    if matches!(
                        strategy,
                        crate::borrowing_context::BorrowingStrategy::UseCow { .. }
                    ) {
                        if let Some(_usage) = lifetime_result.param_lifetimes.get(param_name) {
                            // If a Cow parameter escapes, return type should also be Cow
                            if matches!(self.ret_type, crate::hir::Type::String) {
                                uses_cow_return = true;
                                break;
                            }
                        }
                    }
                }
            }

            if uses_cow_return {
                // Use the same Cow type for return
                ctx.needs_cow = true;
                if let Some(ref return_lt) = lifetime_result.return_lifetime {
                    let lt = syn::Lifetime::new(return_lt, proc_macro2::Span::call_site());
                    ty = parse_quote! { Cow<#lt, str> };
                } else {
                    ty = parse_quote! { Cow<'static, str> };
                }
            } else {
                // Apply return lifetime if needed
                if let Some(ref return_lt) = lifetime_result.return_lifetime {
                    // Check if the return type needs lifetime substitution
                    if matches!(
                        rust_ret_type,
                        crate::type_mapper::RustType::Str { .. }
                            | crate::type_mapper::RustType::Reference { .. }
                    ) {
                        let lt = syn::Lifetime::new(return_lt, proc_macro2::Span::call_site());
                        match rust_ret_type {
                            crate::type_mapper::RustType::Str { .. } => {
                                ty = parse_quote! { &#lt str };
                            }
                            crate::type_mapper::RustType::Reference { mutable, inner, .. } => {
                                let inner_ty = rust_type_to_syn(&inner)?;
                                ty = if mutable {
                                    parse_quote! { &#lt mut #inner_ty }
                                } else {
                                    parse_quote! { &#lt #inner_ty }
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }

            if can_fail {
                let error_type: syn::Type = syn::parse_str(&error_type_str)
                    .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });
                quote! { -> Result<#ty, #error_type> }
            } else {
                quote! { -> #ty }
            }
        };

        // Enter function scope and declare parameters
        ctx.enter_scope();
        ctx.current_function_can_fail = can_fail;
        ctx.current_return_type = Some(self.ret_type.clone());
        for (param_name, _) in &self.params {
            ctx.declare_var(param_name);
        }

        // Convert body
        let body_stmts: Vec<_> = self
            .body
            .iter()
            .map(|stmt| stmt.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;

        ctx.exit_scope();
        ctx.current_function_can_fail = false;
        ctx.current_return_type = None;

        // Add documentation
        let mut attrs = vec![];

        // Add docstring as documentation if present
        if let Some(docstring) = &self.docstring {
            attrs.push(quote! {
                #[doc = #docstring]
            });
        }

        if self.properties.panic_free {
            attrs.push(quote! {
                #[doc = " Depyler: verified panic-free"]
            });
        }
        if self.properties.always_terminates {
            attrs.push(quote! {
                #[doc = " Depyler: proven to terminate"]
            });
        }

        Ok(quote! {
            #(#attrs)*
            pub fn #name #generic_params(#(#params),*) #return_type #where_clause {
                #(#body_stmts)*
            }
        })
    }
}

impl RustCodeGen for HirStmt {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        match self {
            HirStmt::Assign { target, value } => {
                let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
                let value_expr = value.to_rust_expr(ctx)?;

                if ctx.is_declared(target) {
                    // Variable already exists, just assign
                    Ok(quote! { #target_ident = #value_expr; })
                } else {
                    // First declaration, use let mut
                    ctx.declare_var(target);
                    Ok(quote! { let mut #target_ident = #value_expr; })
                }
            }
            HirStmt::Return(expr) => {
                if let Some(e) = expr {
                    let expr_tokens = e.to_rust_expr(ctx)?;
                    if ctx.current_function_can_fail {
                        Ok(quote! { return Ok(#expr_tokens); })
                    } else {
                        Ok(quote! { return #expr_tokens; })
                    }
                } else if ctx.current_function_can_fail {
                    Ok(quote! { return Ok(()); })
                } else {
                    Ok(quote! { return; })
                }
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond = condition.to_rust_expr(ctx)?;
                ctx.enter_scope();
                let then_stmts: Vec<_> = then_body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();

                if let Some(else_stmts) = else_body {
                    ctx.enter_scope();
                    let else_tokens: Vec<_> = else_stmts
                        .iter()
                        .map(|s| s.to_rust_tokens(ctx))
                        .collect::<Result<Vec<_>>>()?;
                    ctx.exit_scope();
                    Ok(quote! {
                        if #cond {
                            #(#then_stmts)*
                        } else {
                            #(#else_tokens)*
                        }
                    })
                } else {
                    Ok(quote! {
                        if #cond {
                            #(#then_stmts)*
                        }
                    })
                }
            }
            HirStmt::While { condition, body } => {
                let cond = condition.to_rust_expr(ctx)?;
                ctx.enter_scope();
                let body_stmts: Vec<_> = body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();
                Ok(quote! {
                    while #cond {
                        #(#body_stmts)*
                    }
                })
            }
            HirStmt::For { target, iter, body } => {
                let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
                let mut iter_expr = iter.to_rust_expr(ctx)?;

                // Check if we're iterating over a borrowed collection
                // If iter is a simple variable that refers to a borrowed collection (e.g., &Vec<T>),
                // we need to add .iter() to properly iterate over it
                if let HirExpr::Var(_var_name) = iter {
                    // This is a simple heuristic: if the expression is just a variable name,
                    // it's likely a parameter or local var that might be borrowed
                    // The generated code already has the variable as borrowed (e.g., data: &Vec<T>)
                    // so we need to call .iter() on it
                    iter_expr = parse_quote! { #iter_expr.iter() };
                }

                ctx.enter_scope();
                ctx.declare_var(target); // for loop variable is declared in the loop scope
                let body_stmts: Vec<_> = body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();
                Ok(quote! {
                    for #target_ident in #iter_expr {
                        #(#body_stmts)*
                    }
                })
            }
            HirStmt::Expr(expr) => {
                let expr_tokens = expr.to_rust_expr(ctx)?;
                Ok(quote! { #expr_tokens; })
            }
            HirStmt::Raise {
                exception,
                cause: _,
            } => {
                // For V1, we'll implement basic error handling
                if let Some(exc) = exception {
                    let exc_expr = exc.to_rust_expr(ctx)?;
                    Ok(quote! { return Err(#exc_expr); })
                } else {
                    // Re-raise or bare raise - use generic error
                    Ok(quote! { return Err("Exception raised".into()); })
                }
            }
        }
    }
}

/// Extension trait for converting expressions to Rust
trait ToRustExpr {
    fn to_rust_expr(&self, ctx: &mut CodeGenContext) -> Result<syn::Expr>;
}

/// Expression converter to reduce complexity
struct ExpressionConverter<'a, 'b> {
    ctx: &'a mut CodeGenContext<'b>,
}

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    fn new(ctx: &'a mut CodeGenContext<'b>) -> Self {
        Self { ctx }
    }

    fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
        Ok(parse_quote! { #ident })
    }

    fn convert_binary(&mut self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
        let left_expr = left.to_rust_expr(self.ctx)?;
        let right_expr = right.to_rust_expr(self.ctx)?;

        match op {
            BinOp::In => {
                // Convert "x in dict" to "dict.contains_key(&x)" for dicts
                // For now, assume it's a dict/hashmap
                Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
            }
            BinOp::NotIn => {
                // Convert "x not in dict" to "!dict.contains_key(&x)"
                Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
            }
            BinOp::Add => {
                // Special handling for string concatenation
                // Only use format! if we're certain at least one operand is a string
                let is_definitely_string = matches!(left, HirExpr::Literal(Literal::String(_)))
                    || matches!(right, HirExpr::Literal(Literal::String(_)));

                if is_definitely_string {
                    // This is string concatenation - use format! to handle references properly
                    Ok(parse_quote! { format!("{}{}", #left_expr, #right_expr) })
                } else {
                    // Regular arithmetic addition or unknown types
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { (#left_expr #rust_op #right_expr) })
                }
            }
            BinOp::FloorDiv => {
                // Python floor division semantics differ from Rust integer division
                // Python: rounds towards negative infinity (floor)
                // Rust: truncates towards zero
                // For now, we generate code that works for integers with proper floor semantics
                Ok(parse_quote! {
                    {
                        let a = #left_expr;
                        let b = #right_expr;
                        let q = a / b;
                        let r = a % b;
                        if (r != 0) && ((r < 0) != (b < 0)) { q - 1 } else { q }
                    }
                })
            }
            BinOp::Sub => {
                // Check if we're subtracting from a .len() call to prevent underflow
                if self.is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    Ok(parse_quote! { #left_expr.saturating_sub(#right_expr) })
                } else {
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { (#left_expr #rust_op #right_expr) })
                }
            }
            BinOp::Mul => {
                // Special case: [value] * n or n * [value] creates an array
                match (left, right) {
                    // Pattern: [x] * n
                    (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size))) 
                        if elts.len() == 1 && *size > 0 && *size <= 32 => {
                        let elem = elts[0].to_rust_expr(self.ctx)?;
                        let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Pattern: n * [x]
                    (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                        if elts.len() == 1 && *size > 0 && *size <= 32 => {
                        let elem = elts[0].to_rust_expr(self.ctx)?;
                        let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Default multiplication
                    _ => {
                        let rust_op = convert_binop(op)?;
                        Ok(parse_quote! { (#left_expr #rust_op #right_expr) })
                    }
                }
            }
            _ => {
                let rust_op = convert_binop(op)?;
                Ok(parse_quote! { (#left_expr #rust_op #right_expr) })
            }
        }
    }

    fn convert_unary(&mut self, op: &UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
        let operand_expr = operand.to_rust_expr(self.ctx)?;
        match op {
            UnaryOp::Not => Ok(parse_quote! { !#operand_expr }),
            UnaryOp::Neg => Ok(parse_quote! { -#operand_expr }),
            UnaryOp::Pos => Ok(operand_expr), // No +x in Rust
            UnaryOp::BitNot => Ok(parse_quote! { !#operand_expr }),
        }
    }

    fn convert_call(&mut self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        match func {
            "len" => self.convert_len_call(&arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            "zeros" | "ones" | "full" => self.convert_array_init_call(func, args, &arg_exprs),
            _ => self.convert_generic_call(func, &arg_exprs),
        }
    }

    fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("len() requires exactly one argument");
        }
        let arg = &args[0];
        Ok(parse_quote! { #arg.len() })
    }

    fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        match args.len() {
            1 => {
                let end = &args[0];
                Ok(parse_quote! { 0..#end })
            }
            2 => {
                let start = &args[0];
                let end = &args[1];
                Ok(parse_quote! { #start..#end })
            }
            3 => {
                let start = &args[0];
                let end = &args[1];
                let step = &args[2];

                // Check if step is negative by looking at the expression
                let is_negative_step = if let syn::Expr::Unary(unary) = step {
                    matches!(unary.op, syn::UnOp::Neg(_))
                } else {
                    false
                };

                if is_negative_step {
                    // For negative steps, we need to reverse the range
                    // Python: range(10, 0, -1) → Rust: (0..10).rev()
                    // But we also need to handle step sizes > 1
                    Ok(parse_quote! {
                        {
                            let step = (#step).abs() as usize;
                            if step == 0 {
                                panic!("range() arg 3 must not be zero");
                            }
                            if step == 1 {
                                (#end..#start).rev()
                            } else {
                                (#end..#start).rev().step_by(step)
                            }
                        }
                    })
                } else {
                    // Positive step - check for zero
                    Ok(parse_quote! {
                        {
                            let step = #step as usize;
                            if step == 0 {
                                panic!("range() arg 3 must not be zero");
                            }
                            (#start..#end).step_by(step)
                        }
                    })
                }
            }
            _ => bail!("Invalid number of arguments for range()"),
        }
    }

    fn convert_array_init_call(&mut self, func: &str, args: &[HirExpr], _arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        // Handle zeros(n), ones(n), full(n, value) patterns
        if args.is_empty() {
            bail!("{} requires at least one argument", func);
        }
        
        // Extract size from first argument if it's a literal
        if let HirExpr::Literal(Literal::Int(size)) = &args[0] {
            if *size > 0 && *size <= 32 {
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                match func {
                    "zeros" => Ok(parse_quote! { [0; #size_lit] }),
                    "ones" => Ok(parse_quote! { [1; #size_lit] }),
                    "full" => {
                        if args.len() >= 2 {
                            let value = args[1].to_rust_expr(self.ctx)?;
                            Ok(parse_quote! { [#value; #size_lit] })
                        } else {
                            bail!("full() requires a value argument");
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                // For large arrays or dynamic sizes, fall back to vec!
                match func {
                    "zeros" => {
                        let size_expr = args[0].to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { vec![0; #size_expr as usize] })
                    }
                    "ones" => {
                        let size_expr = args[0].to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { vec![1; #size_expr as usize] })
                    }
                    "full" => {
                        if args.len() >= 2 {
                            let size_expr = args[0].to_rust_expr(self.ctx)?;
                            let value = args[1].to_rust_expr(self.ctx)?;
                            Ok(parse_quote! { vec![#value; #size_expr as usize] })
                        } else {
                            bail!("full() requires a value argument");
                        }
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            // Dynamic size - use vec!
            let size_expr = args[0].to_rust_expr(self.ctx)?;
            match func {
                "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
                "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
                "full" => {
                    if args.len() >= 2 {
                        let value = args[1].to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { vec![#value; #size_expr as usize] })
                    } else {
                        bail!("full() requires a value argument");
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn convert_generic_call(&self, func: &str, args: &[syn::Expr]) -> Result<syn::Expr> {
        let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
        Ok(parse_quote! { #func_ident(#(#args),*) })
    }

    fn convert_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Map Python collection methods to Rust equivalents
        match method {
            // List methods
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            "extend" => {
                if arg_exprs.len() != 1 {
                    bail!("extend() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.extend(#arg) })
            }
            "pop" => {
                if arg_exprs.is_empty() {
                    Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
                } else {
                    bail!("pop() with index not supported in V1");
                }
            }
            "insert" => {
                if arg_exprs.len() != 2 {
                    bail!("insert() requires exactly two arguments");
                }
                let index = &arg_exprs[0];
                let value = &arg_exprs[1];
                Ok(parse_quote! { #object_expr.insert(#index as usize, #value) })
            }
            "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                // This is a simplified version - real implementation would need to find and remove
                Ok(parse_quote! {
                    if let Some(pos) = #object_expr.iter().position(|x| x == &#value) {
                        #object_expr.remove(pos)
                    } else {
                        panic!("ValueError: list.remove(x): x not in list")
                    }
                })
            }

            // Dict methods
            "get" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Ok(parse_quote! { #object_expr.get(&#key).cloned() })
                } else if arg_exprs.len() == 2 {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    Ok(parse_quote! { #object_expr.get(&#key).cloned().unwrap_or(#default) })
                } else {
                    bail!("get() requires 1 or 2 arguments");
                }
            }
            "keys" => {
                if !arg_exprs.is_empty() {
                    bail!("keys() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.keys().cloned().collect::<Vec<_>>() })
            }
            "values" => {
                if !arg_exprs.is_empty() {
                    bail!("values() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.values().cloned().collect::<Vec<_>>() })
            }
            "items" => {
                if !arg_exprs.is_empty() {
                    bail!("items() takes no arguments");
                }
                Ok(
                    parse_quote! { #object_expr.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() },
                )
            }
            "update" => {
                if arg_exprs.len() != 1 {
                    bail!("update() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! {
                    for (k, v) in #arg {
                        #object_expr.insert(k, v);
                    }
                })
            }

            // String methods
            "upper" => {
                if !arg_exprs.is_empty() {
                    bail!("upper() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_uppercase() })
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_lowercase() })
            }
            "strip" => {
                if !arg_exprs.is_empty() {
                    bail!("strip() with arguments not supported in V1");
                }
                Ok(parse_quote! { #object_expr.trim().to_string() })
            }
            "startswith" => {
                if arg_exprs.len() != 1 {
                    bail!("startswith() requires exactly one argument");
                }
                let prefix = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.starts_with(#prefix) })
            }
            "endswith" => {
                if arg_exprs.len() != 1 {
                    bail!("endswith() requires exactly one argument");
                }
                let suffix = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.ends_with(#suffix) })
            }
            "split" => {
                if arg_exprs.is_empty() {
                    Ok(
                        parse_quote! { #object_expr.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 1 {
                    let sep = &arg_exprs[0];
                    Ok(
                        parse_quote! { #object_expr.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("split() with maxsplit not supported in V1");
                }
            }
            "join" => {
                if arg_exprs.len() != 1 {
                    bail!("join() requires exactly one argument");
                }
                let iterable = &arg_exprs[0];
                Ok(parse_quote! { #iterable.join(#object_expr) })
            }

            // Generic method call fallback
            _ => {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                Ok(parse_quote! { #object_expr.#method_ident(#(#arg_exprs),*) })
            }
        }
    }

    fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;
        let index_expr = index.to_rust_expr(self.ctx)?;
        // V1: Safe indexing with bounds checking
        Ok(parse_quote! {
            #base_expr.get(#index_expr as usize).copied().unwrap_or_default()
        })
    }

    fn convert_slice(
        &mut self,
        base: &HirExpr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;

        // Convert slice parameters
        let start_expr = if let Some(s) = start {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        let stop_expr = if let Some(s) = stop {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        let step_expr = if let Some(s) = step {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        // Generate slice code based on the parameters
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: base[::step]
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        let step = #step;
                        if step == 1 {
                            #base_expr.clone()
                        } else if step > 0 {
                            #base_expr.iter().step_by(step as usize).cloned().collect::<Vec<_>>()
                        } else if step == -1 {
                            #base_expr.iter().rev().cloned().collect::<Vec<_>>()
                        } else {
                            // Negative step with abs value
                            let abs_step = (-step) as usize;
                            #base_expr.iter().rev().step_by(abs_step).cloned().collect::<Vec<_>>()
                        }
                    }
                })
            }

            // Start and stop: base[start:stop]
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    let start = (#start).max(0) as usize;
                    let stop = (#stop).max(0) as usize;
                    if start < #base_expr.len() {
                        #base_expr[start..stop.min(#base_expr.len())].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Start only: base[start:]
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    let start = (#start).max(0) as usize;
                    if start < #base_expr.len() {
                        #base_expr[start..].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop only: base[:stop]
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    let stop = (#stop).max(0) as usize;
                    #base_expr[..stop.min(#base_expr.len())].to_vec()
                }
            }),

            // Full slice: base[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.clone() }),

            // Start, stop, and step: base[start:stop:step]
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        let start = (#start).max(0) as usize;
                        let stop = (#stop).max(0) as usize;
                        let step = #step;

                        if step == 1 {
                            if start < #base_expr.len() {
                                #base_expr[start..stop.min(#base_expr.len())].to_vec()
                            } else {
                                Vec::new()
                            }
                        } else if step > 0 {
                            #base_expr[start..stop.min(#base_expr.len())]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            // Negative step - slice in reverse
                            let abs_step = (-step) as usize;
                            if start < #base_expr.len() {
                                #base_expr[start..stop.min(#base_expr.len())]
                                    .iter()
                                    .rev()
                                    .step_by(abs_step)
                                    .cloned()
                                    .collect::<Vec<_>>()
                            } else {
                                Vec::new()
                            }
                        }
                    }
                })
            }

            // Start and step: base[start::step]
            (Some(start), None, Some(step)) => Ok(parse_quote! {
                {
                    let start = (#start).max(0) as usize;
                    let step = #step;

                    if start < #base_expr.len() {
                        if step == 1 {
                            #base_expr[start..].to_vec()
                        } else if step > 0 {
                            #base_expr[start..]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else if step == -1 {
                            #base_expr[start..]
                                .iter()
                                .rev()
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            let abs_step = (-step) as usize;
                            #base_expr[start..]
                                .iter()
                                .rev()
                                .step_by(abs_step)
                                .cloned()
                                .collect::<Vec<_>>()
                        }
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop and step: base[:stop:step]
            (None, Some(stop), Some(step)) => Ok(parse_quote! {
                {
                    let stop = (#stop).max(0) as usize;
                    let step = #step;

                    if step == 1 {
                        #base_expr[..stop.min(#base_expr.len())].to_vec()
                    } else if step > 0 {
                        #base_expr[..stop.min(#base_expr.len())]
                            .iter()
                            .step_by(step as usize)
                            .cloned()
                            .collect::<Vec<_>>()
                    } else if step == -1 {
                        #base_expr[..stop.min(#base_expr.len())]
                            .iter()
                            .rev()
                            .cloned()
                            .collect::<Vec<_>>()
                    } else {
                        let abs_step = (-step) as usize;
                        #base_expr[..stop.min(#base_expr.len())]
                            .iter()
                            .rev()
                            .step_by(abs_step)
                            .cloned()
                            .collect::<Vec<_>>()
                    }
                }
            }),
        }
    }

    fn convert_list(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| e.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        
        // Check if this list should be an array
        if !elts.is_empty() && elts.len() <= 32 {
            // Check if all elements are literals (good candidate for array)
            let all_literals = elts.iter().all(|e| matches!(e, HirExpr::Literal(_)));
            
            if all_literals {
                // Generate array literal instead of vec!
                Ok(parse_quote! { [#(#elt_exprs),*] })
            } else {
                Ok(parse_quote! { vec![#(#elt_exprs),*] })
            }
        } else {
            Ok(parse_quote! { vec![#(#elt_exprs),*] })
        }
    }

    fn convert_dict(&mut self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        self.ctx.needs_hashmap = true;
        let mut insert_stmts = Vec::new();
        for (key, value) in items {
            let key_expr = key.to_rust_expr(self.ctx)?;
            let val_expr = value.to_rust_expr(self.ctx)?;
            insert_stmts.push(quote! { map.insert(#key_expr, #val_expr); });
        }
        Ok(parse_quote! {
            {
                let mut map = HashMap::new();
                #(#insert_stmts)*
                map
            }
        })
    }

    fn convert_tuple(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| e.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    fn convert_attribute(&mut self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        let value_expr = value.to_rust_expr(self.ctx)?;
        let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
        Ok(parse_quote! { #value_expr.#attr_ident })
    }

    fn convert_borrow(&mut self, expr: &HirExpr, mutable: bool) -> Result<syn::Expr> {
        let expr_tokens = expr.to_rust_expr(self.ctx)?;
        if mutable {
            Ok(parse_quote! { &mut #expr_tokens })
        } else {
            Ok(parse_quote! { &#expr_tokens })
        }
    }

    fn convert_list_comp(
        &mut self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let iter_expr = iter.to_rust_expr(self.ctx)?;
        let element_expr = element.to_rust_expr(self.ctx)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_ident| #cond_expr)
                    .map(|#target_ident| #element_expr)
                    .collect::<Vec<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_ident| #element_expr)
                    .collect::<Vec<_>>()
            })
        }
    }

    fn convert_lambda(
        &mut self,
        params: &[String],
        body: &HirExpr,
    ) -> Result<syn::Expr> {
        // Convert parameters to pattern identifiers
        let param_pats: Vec<syn::Pat> = params
            .iter()
            .map(|p| {
                let ident = syn::Ident::new(p, proc_macro2::Span::call_site());
                parse_quote! { #ident }
            })
            .collect();
        
        // Convert body expression
        let body_expr = body.to_rust_expr(self.ctx)?;
        
        // Generate closure
        if params.is_empty() {
            // No parameters
            Ok(parse_quote! { || #body_expr })
        } else if params.len() == 1 {
            // Single parameter
            let param = &param_pats[0];
            Ok(parse_quote! { |#param| #body_expr })
        } else {
            // Multiple parameters
            Ok(parse_quote! { |#(#param_pats),*| #body_expr })
        }
    }

    /// Check if an expression is a len() call
    fn is_len_call(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Call { func, args } if func == "len" && args.len() == 1)
    }
}

impl ToRustExpr for HirExpr {
    fn to_rust_expr(&self, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
        let mut converter = ExpressionConverter::new(ctx);

        match self {
            HirExpr::Literal(lit) => {
                let expr = literal_to_rust_expr(lit, &ctx.string_optimizer, &ctx.needs_cow, ctx);
                if let Literal::String(s) = lit {
                    let context = StringContext::Literal(s.clone());
                    if matches!(
                        ctx.string_optimizer.get_optimal_type(&context),
                        crate::string_optimization::OptimalStringType::CowStr
                    ) {
                        ctx.needs_cow = true;
                    }
                }
                Ok(expr)
            }
            HirExpr::Var(name) => converter.convert_variable(name),
            HirExpr::Binary { op, left, right } => converter.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => converter.convert_unary(op, operand),
            HirExpr::Call { func, args } => converter.convert_call(func, args),
            HirExpr::MethodCall {
                object,
                method,
                args,
            } => converter.convert_method_call(object, method, args),
            HirExpr::Index { base, index } => converter.convert_index(base, index),
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => converter.convert_slice(base, start, stop, step),
            HirExpr::List(elts) => converter.convert_list(elts),
            HirExpr::Dict(items) => converter.convert_dict(items),
            HirExpr::Tuple(elts) => converter.convert_tuple(elts),
            HirExpr::Attribute { value, attr } => converter.convert_attribute(value, attr),
            HirExpr::Borrow { expr, mutable } => converter.convert_borrow(expr, *mutable),
            HirExpr::ListComp {
                element,
                target,
                iter,
                condition,
            } => converter.convert_list_comp(element, target, iter, condition),
            HirExpr::Lambda { params, body } => converter.convert_lambda(params, body),
        }
    }
}

fn literal_to_rust_expr(
    lit: &Literal,
    string_optimizer: &StringOptimizer,
    _needs_cow: &bool,
    ctx: &CodeGenContext,
) -> syn::Expr {
    match lit {
        Literal::Int(n) => {
            let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::Float(f) => {
            let lit = syn::LitFloat::new(&f.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::String(s) => {
            // Check if this string should be interned
            if let Some(interned_name) = string_optimizer.get_interned_name(s) {
                let ident = syn::Ident::new(&interned_name, proc_macro2::Span::call_site());
                parse_quote! { #ident }
            } else {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());

                // Use string optimizer to determine if we need .to_string()
                let context = StringContext::Literal(s.clone());
                match string_optimizer.get_optimal_type(&context) {
                    crate::string_optimization::OptimalStringType::StaticStr => {
                        // For read-only strings, just use the literal
                        parse_quote! { #lit }
                    }
                    crate::string_optimization::OptimalStringType::BorrowedStr { .. } => {
                        // Use &'static str for literals that can be borrowed
                        parse_quote! { #lit }
                    }
                    crate::string_optimization::OptimalStringType::CowStr => {
                        // Check if we're in a context where String is required
                        if let Some(Type::String) = &ctx.current_return_type {
                            // Function returns String, so convert to owned
                            parse_quote! { #lit.to_string() }
                        } else {
                            // Use Cow for flexible ownership
                            parse_quote! { std::borrow::Cow::Borrowed(#lit) }
                        }
                    }
                    crate::string_optimization::OptimalStringType::OwnedString => {
                        // Only use .to_string() when absolutely necessary
                        parse_quote! { #lit.to_string() }
                    }
                }
            }
        }
        Literal::Bool(b) => {
            let lit = syn::LitBool::new(*b, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::None => parse_quote! { () },
    }
}

fn convert_binop(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;

    match op {
        // Arithmetic operators
        Add => Ok(parse_quote! { + }),
        Sub => Ok(parse_quote! { - }),
        Mul => Ok(parse_quote! { * }),
        Div => Ok(parse_quote! { / }),
        Mod => Ok(parse_quote! { % }),

        // Special arithmetic cases
        FloorDiv => {
            // Floor division needs explicit conversion for negative values
            bail!("Floor division requires custom implementation for Python semantics")
        }
        Pow => bail!("Power operator not directly supported in Rust"),

        // Comparison operators
        Eq => Ok(parse_quote! { == }),
        NotEq => Ok(parse_quote! { != }),
        Lt => Ok(parse_quote! { < }),
        LtEq => Ok(parse_quote! { <= }),
        Gt => Ok(parse_quote! { > }),
        GtEq => Ok(parse_quote! { >= }),

        // Logical operators
        And => Ok(parse_quote! { && }),
        Or => Ok(parse_quote! { || }),

        // Bitwise operators
        BitAnd => Ok(parse_quote! { & }),
        BitOr => Ok(parse_quote! { | }),
        BitXor => Ok(parse_quote! { ^ }),
        LShift => Ok(parse_quote! { << }),
        RShift => Ok(parse_quote! { >> }),

        // Special membership operators handled in convert_binary
        In | NotIn => bail!("in/not in operators should be handled by convert_binary"),
    }
}

pub fn rust_type_to_syn(rust_type: &crate::type_mapper::RustType) -> Result<syn::Type> {
    use crate::type_mapper::RustType;

    Ok(match rust_type {
        RustType::Primitive(p) => {
            let ident = syn::Ident::new(p.to_rust_string(), proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::String => parse_quote! { String },
        RustType::Str { lifetime } => {
            if let Some(lt) = lifetime {
                let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
                parse_quote! { &#lt_ident str }
            } else {
                parse_quote! { &str }
            }
        }
        RustType::Cow { lifetime } => {
            let lt_ident = syn::Lifetime::new(lifetime, proc_macro2::Span::call_site());
            parse_quote! { Cow<#lt_ident, str> }
        }
        RustType::Vec(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Vec<#inner_ty> }
        }
        RustType::HashMap(k, v) => {
            let key_ty = rust_type_to_syn(k)?;
            let val_ty = rust_type_to_syn(v)?;
            parse_quote! { HashMap<#key_ty, #val_ty> }
        }
        RustType::Option(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Option<#inner_ty> }
        }
        RustType::Result(ok, err) => {
            let ok_ty = rust_type_to_syn(ok)?;
            let err_ty = rust_type_to_syn(err)?;
            parse_quote! { Result<#ok_ty, #err_ty> }
        }
        RustType::Reference {
            lifetime,
            mutable,
            inner,
        } => {
            let inner_ty = rust_type_to_syn(inner)?;
            if *mutable {
                if let Some(lt) = lifetime {
                    let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
                    parse_quote! { &#lt_ident mut #inner_ty }
                } else {
                    parse_quote! { &mut #inner_ty }
                }
            } else if let Some(lt) = lifetime {
                let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
                parse_quote! { &#lt_ident #inner_ty }
            } else {
                parse_quote! { &#inner_ty }
            }
        }
        RustType::Tuple(types) => {
            let tys: Vec<_> = types
                .iter()
                .map(rust_type_to_syn)
                .collect::<Result<Vec<_>>>()?;
            parse_quote! { (#(#tys),*) }
        }
        RustType::Unit => parse_quote! { () },
        RustType::Custom(name) => {
            let ty: syn::Type = syn::parse_str(name)?;
            ty
        }
        RustType::Unsupported(reason) => bail!("Unsupported Rust type: {}", reason),
        RustType::TypeParam(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::Generic { base, params } => {
            let base_ident = syn::Ident::new(base, proc_macro2::Span::call_site());
            let param_types: Vec<_> = params
                .iter()
                .map(rust_type_to_syn)
                .collect::<Result<Vec<_>>>()?;
            parse_quote! { #base_ident<#(#param_types),*> }
        }
        RustType::Enum { name, .. } => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::Array { element_type, size } => {
            let element = rust_type_to_syn(element_type)?;
            match size {
                crate::type_mapper::RustConstGeneric::Literal(n) => {
                    let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                    parse_quote! { [#element; #size_lit] }
                }
                crate::type_mapper::RustConstGeneric::Parameter(name) => {
                    let param_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                    parse_quote! { [#element; #param_ident] }
                }
                crate::type_mapper::RustConstGeneric::Expression(expr) => {
                    let expr_tokens: proc_macro2::TokenStream = expr.parse().unwrap_or_else(|_| {
                        quote! { /* invalid const expression */ }
                    });
                    parse_quote! { [#element; #expr_tokens] }
                }
            }
        }
    })
}

/// Format Rust code using basic prettification
/// Note: This is a simple formatter for V1. rustfmt integration planned for V2.
fn format_rust_code(code: String) -> String {
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
        .replace(":: ", "::")
        // Fix attribute spacing
        .replace("# [", "#[")
        // Fix type annotations
        .replace(" : ", ": ")
        // Fix parameter spacing
        .replace(" , ", ", ")
        // Fix assignment operator spacing issues
        .replace("=(", " = (")
        .replace("= (", " = (")
        .replace("  =", " =") // Fix multiple spaces before =
        .replace("   =", " =") // Fix even more spaces
        // Fix generic type spacing
        .replace("Vec < ", "Vec<")
        .replace(" < ", "<")
        .replace(" > ", ">")
        .replace("> ", ">")
        .replace("< ", "<")
        .replace(" >", ">") // Fix trailing space before closing bracket
        // Fix return type spacing
        .replace("->", " -> ")
        .replace(" ->  ", " -> ")
        .replace(" ->   ", " -> ")
        // Fix range spacing
        .replace(" .. ", "..")
        .replace(" ..", "..")
        .replace(".. ", "..")
        // Fix 'in' keyword spacing
        .replace("in(", "in (")
        .replace(";\n    }", "\n}")
}

/// Updates the import needs based on the rust type being used
fn update_import_needs(ctx: &mut CodeGenContext, rust_type: &crate::type_mapper::RustType) {
    match rust_type {
        crate::type_mapper::RustType::HashMap(_, _) => ctx.needs_hashmap = true,
        crate::type_mapper::RustType::Cow { .. } => ctx.needs_cow = true,
        crate::type_mapper::RustType::Custom(name) => {
            if name.contains("FnvHashMap") {
                ctx.needs_fnv_hashmap = true;
            } else if name.contains("AHashMap") {
                ctx.needs_ahash_hashmap = true;
            } else if name.contains("Arc<") {
                ctx.needs_arc = true;
            } else if name.contains("Rc<") {
                ctx.needs_rc = true;
            } else if name.contains("HashMap<")
                && !name.contains("FnvHashMap")
                && !name.contains("AHashMap")
            {
                ctx.needs_hashmap = true;
            }
        }
        crate::type_mapper::RustType::Reference { inner, .. } => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Vec(inner) => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Option(inner) => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Result(ok, err) => {
            update_import_needs(ctx, ok);
            update_import_needs(ctx, err);
        }
        crate::type_mapper::RustType::Tuple(types) => {
            for t in types {
                update_import_needs(ctx, t);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_mapper::TypeMapper;
    use depyler_annotations::TranspilationAnnotations;

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
            needs_fnv_hashmap: false,
            needs_ahash_hashmap: false,
            needs_arc: false,
            needs_rc: false,
            needs_cow: false,
            declared_vars: vec![HashSet::new()],
            current_function_can_fail: false,
            current_return_type: None,
        }
    }

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
}
