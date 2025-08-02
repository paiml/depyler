use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::hir::*;
use crate::lifetime_analysis::LifetimeInference;
use crate::string_optimization::{StringContext, StringOptimizer};
use anyhow::{bail, Result};
use quote::quote;
use std::collections::HashSet;
use syn::{self, parse_quote};

/// Context for code generation including type mapping and configuration
pub struct CodeGenContext<'a> {
    pub type_mapper: &'a crate::type_mapper::TypeMapper,
    pub annotation_aware_mapper: AnnotationAwareTypeMapper,
    pub string_optimizer: StringOptimizer,
    pub needs_hashmap: bool,
    pub needs_fnv_hashmap: bool,
    pub needs_ahash_hashmap: bool,
    pub needs_arc: bool,
    pub needs_rc: bool,
    pub needs_cow: bool,
    pub declared_vars: Vec<HashSet<String>>,
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
        needs_hashmap: false,
        needs_fnv_hashmap: false,
        needs_ahash_hashmap: false,
        needs_arc: false,
        needs_rc: false,
        needs_cow: false,
        declared_vars: vec![HashSet::new()],
    };

    // Analyze all functions first for string optimization
    for func in &module.functions {
        ctx.string_optimizer.analyze_function(func);
    }

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

    // Add all functions
    items.extend(functions);

    let file = quote! {
        #(#items)*
    };

    Ok(format_rust_code(file.to_string()))
}

impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        // Perform lifetime analysis
        let mut lifetime_inference = LifetimeInference::new();
        let lifetime_result = lifetime_inference.analyze_function(self, ctx.type_mapper);

        // Generate lifetime parameters if needed
        let lifetime_params = if lifetime_result.lifetime_params.is_empty() {
            quote! {}
        } else {
            let lifetimes: Vec<_> = lifetime_result
                .lifetime_params
                .iter()
                .map(|lt| {
                    let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
                    quote! { #lt_ident }
                })
                .collect();
            quote! { <#(#lifetimes),*> }
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

                // Get the inferred parameter info
                if let Some(inferred) = lifetime_result.param_lifetimes.get(param_name) {
                    let rust_type = &inferred.rust_type;
                    update_import_needs(ctx, rust_type);
                    let mut ty = rust_type_to_syn(rust_type)?;

                    // Apply borrowing if needed
                    if inferred.should_borrow {
                        if let Some(ref lifetime) = inferred.lifetime {
                            let lt = syn::Lifetime::new(lifetime, proc_macro2::Span::call_site());
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

                    Ok(quote! { #param_ident: #ty })
                } else {
                    // Fallback to original mapping
                    let rust_type = ctx
                        .annotation_aware_mapper
                        .map_type_with_annotations(param_type, &self.annotations);
                    update_import_needs(ctx, &rust_type);
                    let ty = rust_type_to_syn(&rust_type)?;
                    Ok(quote! { #param_ident: #ty })
                }
            })
            .collect::<Result<Vec<_>>>()?;

        // Convert return type using annotation-aware mapping
        let rust_ret_type = ctx
            .annotation_aware_mapper
            .map_return_type_with_annotations(&self.ret_type, &self.annotations);
        let return_type = if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
            quote! {}
        } else {
            let mut ty = rust_type_to_syn(&rust_ret_type)?;

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

            quote! { -> #ty }
        };

        // Enter function scope and declare parameters
        ctx.enter_scope();
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
            pub fn #name #lifetime_params(#(#params),*) #return_type #where_clause {
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
                let iter_expr = iter.to_rust_expr(ctx)?;
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
                Ok(parse_quote! { (#start..#end).step_by(#step as usize) })
            }
            _ => bail!("Invalid number of arguments for range()"),
        }
    }

    fn convert_generic_call(&self, func: &str, args: &[syn::Expr]) -> Result<syn::Expr> {
        let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
        Ok(parse_quote! { #func_ident(#(#args),*) })
    }

    fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;
        let index_expr = index.to_rust_expr(self.ctx)?;
        // V1: Safe indexing with bounds checking
        Ok(parse_quote! {
            #base_expr.get(#index_expr as usize).copied().unwrap_or_default()
        })
    }

    fn convert_list(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| e.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
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
                let expr = literal_to_rust_expr(lit, &ctx.string_optimizer, &ctx.needs_cow);
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
            HirExpr::Index { base, index } => converter.convert_index(base, index),
            HirExpr::List(elts) => converter.convert_list(elts),
            HirExpr::Dict(items) => converter.convert_dict(items),
            HirExpr::Tuple(elts) => converter.convert_tuple(elts),
            HirExpr::Attribute { value, attr } => converter.convert_attribute(value, attr),
            HirExpr::Borrow { expr, mutable } => converter.convert_borrow(expr, *mutable),
        }
    }
}

fn literal_to_rust_expr(
    lit: &Literal,
    string_optimizer: &StringOptimizer,
    _needs_cow: &bool,
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
                        // Use Cow for flexible ownership
                        parse_quote! { std::borrow::Cow::Borrowed(#lit) }
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

fn rust_type_to_syn(rust_type: &crate::type_mapper::RustType) -> Result<syn::Type> {
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
            needs_hashmap: false,
            needs_fnv_hashmap: false,
            needs_ahash_hashmap: false,
            needs_arc: false,
            needs_rc: false,
            needs_cow: false,
            declared_vars: vec![HashSet::new()],
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
        let list_expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        let mut ctx = create_test_context();
        let expr = list_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #expr }.to_string();

        assert!(code.contains("vec !"));
        assert!(code.contains("1"));
        assert!(code.contains("2"));
        assert!(code.contains("3"));
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
