use crate::hir::*;
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};

/// Context for code generation including type mapping and configuration
pub struct CodeGenContext<'a> {
    pub type_mapper: &'a crate::type_mapper::TypeMapper,
    pub needs_hashmap: bool,
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
        needs_hashmap: false,
    };

    // Convert all functions first to detect if we need HashMap
    let functions: Vec<_> = module
        .functions
        .iter()
        .map(|f| f.to_rust_tokens(&mut ctx))
        .collect::<Result<Vec<_>>>()?;

    let mut items = Vec::new();

    // Add imports if needed
    if ctx.needs_hashmap {
        items.push(quote! {
            use std::collections::HashMap;
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

        // Convert parameters
        let params: Vec<_> = self
            .params
            .iter()
            .map(|(param_name, param_type)| {
                let param_ident = syn::Ident::new(param_name, proc_macro2::Span::call_site());
                let rust_type = ctx.type_mapper.map_type(param_type);
                let ty = rust_type_to_syn(&rust_type)?;

                // Use references for non-copy types
                let ty = if ctx.type_mapper.needs_reference(&rust_type) {
                    parse_quote! { &#ty }
                } else {
                    ty
                };

                Ok(quote! { #param_ident: #ty })
            })
            .collect::<Result<Vec<_>>>()?;

        // Convert return type
        let rust_ret_type = ctx.type_mapper.map_return_type(&self.ret_type);
        let return_type = if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
            quote! {}
        } else {
            let ty = rust_type_to_syn(&rust_ret_type)?;
            quote! { -> #ty }
        };

        // Convert body
        let body_stmts: Vec<_> = self
            .body
            .iter()
            .map(|stmt| stmt.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;

        // Add documentation
        let mut attrs = vec![];
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
            pub fn #name(#(#params),*) #return_type {
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
                Ok(quote! { let mut #target_ident = #value_expr; })
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
                let then_stmts: Vec<_> = then_body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;

                if let Some(else_stmts) = else_body {
                    let else_tokens: Vec<_> = else_stmts
                        .iter()
                        .map(|s| s.to_rust_tokens(ctx))
                        .collect::<Result<Vec<_>>>()?;
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
                let body_stmts: Vec<_> = body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                Ok(quote! {
                    while #cond {
                        #(#body_stmts)*
                    }
                })
            }
            HirStmt::For { target, iter, body } => {
                let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
                let iter_expr = iter.to_rust_expr(ctx)?;
                let body_stmts: Vec<_> = body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
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
            3 => bail!("range() with step parameter not yet supported"),
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
}

impl ToRustExpr for HirExpr {
    fn to_rust_expr(&self, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
        let mut converter = ExpressionConverter::new(ctx);

        match self {
            HirExpr::Literal(lit) => Ok(literal_to_rust_expr(lit)),
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

fn literal_to_rust_expr(lit: &Literal) -> syn::Expr {
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
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            parse_quote! { #lit.to_string() }
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
        FloorDiv => Ok(parse_quote! { / }), // TODO: Handle floor division properly
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
        RustType::Unit => parse_quote! { () },
        _ => bail!("Unsupported Rust type: {:?}", rust_type),
    })
}

/// Format Rust code using basic prettification
/// TODO: Replace with proper rustfmt integration
fn format_rust_code(code: String) -> String {
    code.replace(" ; ", ";\n    ")
        .replace(" { ", " {\n    ")
        .replace(" } ", "\n}\n")
        .replace("} ;", "};")
        .replace(
            "use std :: collections :: HashMap ;",
            "use std::collections::HashMap;",
        )
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
            needs_hashmap: false,
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
