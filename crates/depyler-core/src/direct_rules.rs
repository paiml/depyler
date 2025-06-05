use crate::hir::*;
use crate::type_mapper::{RustType, TypeMapper};
use anyhow::{bail, Result};
use syn::{self, parse_quote};

pub fn apply_rules(module: &HirModule, type_mapper: &TypeMapper) -> Result<syn::File> {
    let mut items = Vec::new();

    // Add standard imports
    items.push(parse_quote! {
        use std::collections::HashMap;
    });

    // Convert functions
    for func in &module.functions {
        let rust_func = convert_function(func, type_mapper)?;
        items.push(syn::Item::Fn(rust_func));
    }

    Ok(syn::File {
        shebang: None,
        attrs: vec![],
        items,
    })
}

fn convert_function(func: &HirFunction, type_mapper: &TypeMapper) -> Result<syn::ItemFn> {
    let name = syn::Ident::new(&func.name, proc_macro2::Span::call_site());

    // Convert parameters
    let mut inputs = Vec::new();
    for (param_name, param_type) in &func.params {
        let rust_type = type_mapper.map_type(param_type);
        let ty = rust_type_to_syn(&rust_type)?;
        let pat = syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: syn::Ident::new(param_name, proc_macro2::Span::call_site()),
            subpat: None,
        });

        // Use references for non-copy types
        let ty = if type_mapper.needs_reference(&rust_type) {
            parse_quote! { &#ty }
        } else {
            ty
        };

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(pat),
            colon_token: Default::default(),
            ty: Box::new(ty),
        }));
    }

    // Convert return type
    let rust_ret_type = type_mapper.map_return_type(&func.ret_type);
    let output = if matches!(rust_ret_type, RustType::Unit) {
        syn::ReturnType::Default
    } else {
        let ty = rust_type_to_syn(&rust_ret_type)?;
        syn::ReturnType::Type(Default::default(), Box::new(ty))
    };

    // Convert body
    let body_stmts = convert_body(&func.body, type_mapper)?;
    let block = syn::Block {
        brace_token: Default::default(),
        stmts: body_stmts,
    };

    // Add documentation
    let mut attrs = vec![];
    if func.properties.panic_free {
        attrs.push(parse_quote! {
            #[doc = " Depyler: verified panic-free"]
        });
    }
    if func.properties.always_terminates {
        attrs.push(parse_quote! {
            #[doc = " Depyler: proven to terminate"]
        });
    }

    Ok(syn::ItemFn {
        attrs,
        vis: syn::Visibility::Public(Default::default()),
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: name,
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: inputs.into_iter().collect(),
            variadic: None,
            output,
        },
        block: Box::new(block),
    })
}

fn rust_type_to_syn(rust_type: &RustType) -> Result<syn::Type> {
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

fn convert_body(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<Vec<syn::Stmt>> {
    stmts
        .iter()
        .map(|stmt| convert_stmt(stmt, type_mapper))
        .collect()
}

fn convert_stmt(stmt: &HirStmt, type_mapper: &TypeMapper) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value } => {
            let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let value_expr = convert_expr(value, type_mapper)?;

            let stmt = syn::Stmt::Local(syn::Local {
                attrs: vec![],
                let_token: Default::default(),
                pat: syn::Pat::Ident(syn::PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: Some(Default::default()),
                    ident: target_ident,
                    subpat: None,
                }),
                init: Some(syn::LocalInit {
                    eq_token: Default::default(),
                    expr: Box::new(value_expr),
                    diverge: None,
                }),
                semi_token: Default::default(),
            });
            Ok(stmt)
        }
        HirStmt::Return(expr) => {
            let ret_expr = if let Some(e) = expr {
                convert_expr(e, type_mapper)?
            } else {
                parse_quote! { () }
            };
            Ok(syn::Stmt::Expr(
                parse_quote! { return #ret_expr },
                Some(Default::default()),
            ))
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond = convert_expr(condition, type_mapper)?;
            let then_block = convert_block(then_body, type_mapper)?;

            let if_expr = if let Some(else_stmts) = else_body {
                let else_block = convert_block(else_stmts, type_mapper)?;
                parse_quote! {
                    if #cond #then_block else #else_block
                }
            } else {
                parse_quote! {
                    if #cond #then_block
                }
            };

            Ok(syn::Stmt::Expr(if_expr, Some(Default::default())))
        }
        HirStmt::While { condition, body } => {
            let cond = convert_expr(condition, type_mapper)?;
            let body_block = convert_block(body, type_mapper)?;

            let while_expr = parse_quote! {
                while #cond #body_block
            };

            Ok(syn::Stmt::Expr(while_expr, Some(Default::default())))
        }
        HirStmt::For { target, iter, body } => {
            let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let iter_expr = convert_expr(iter, type_mapper)?;
            let body_block = convert_block(body, type_mapper)?;

            let for_expr = parse_quote! {
                for #target_ident in #iter_expr #body_block
            };

            Ok(syn::Stmt::Expr(for_expr, Some(Default::default())))
        }
        HirStmt::Expr(expr) => {
            // Skip string literals (likely docstrings)
            if let HirExpr::Literal(Literal::String(_)) = expr {
                // Convert to comment instead of expression
                Ok(syn::Stmt::Expr(
                    parse_quote! { () },
                    Some(Default::default()),
                ))
            } else {
                let rust_expr = convert_expr(expr, type_mapper)?;
                Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
            }
        }
    }
}

fn convert_block(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<syn::Block> {
    let rust_stmts = convert_body(stmts, type_mapper)?;
    Ok(syn::Block {
        brace_token: Default::default(),
        stmts: rust_stmts,
    })
}

/// Convert HIR expressions to Rust expressions using strategy pattern
fn convert_expr(expr: &HirExpr, type_mapper: &TypeMapper) -> Result<syn::Expr> {
    let converter = ExprConverter::new(type_mapper);
    converter.convert(expr)
}

/// Expression converter using strategy pattern to reduce complexity
struct ExprConverter<'a> {
    #[allow(dead_code)]
    type_mapper: &'a TypeMapper,
}

impl<'a> ExprConverter<'a> {
    fn new(type_mapper: &'a TypeMapper) -> Self {
        Self { type_mapper }
    }

    fn convert(&self, expr: &HirExpr) -> Result<syn::Expr> {
        match expr {
            HirExpr::Literal(lit) => self.convert_literal(lit),
            HirExpr::Var(name) => self.convert_variable(name),
            HirExpr::Binary { op, left, right } => self.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => self.convert_unary(*op, operand),
            HirExpr::Call { func, args } => self.convert_call(func, args),
            HirExpr::Index { base, index } => self.convert_index(base, index),
            HirExpr::List(elts) => self.convert_list(elts),
            HirExpr::Dict(items) => self.convert_dict(items),
            HirExpr::Tuple(elts) => self.convert_tuple(elts),
            _ => bail!("Expression type not yet supported: {:?}", expr),
        }
    }

    fn convert_literal(&self, lit: &Literal) -> Result<syn::Expr> {
        Ok(convert_literal(lit))
    }

    fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
        Ok(parse_quote! { #ident })
    }

    fn convert_binary(&self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
        let left_expr = self.convert(left)?;
        let right_expr = self.convert(right)?;
        let rust_op = convert_binop(op)?;
        Ok(parse_quote! { #left_expr #rust_op #right_expr })
    }

    fn convert_unary(&self, op: UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
        let operand_expr = self.convert(operand)?;
        match op {
            UnaryOp::Not => Ok(parse_quote! { !#operand_expr }),
            UnaryOp::Neg => Ok(parse_quote! { -#operand_expr }),
            UnaryOp::Pos => Ok(operand_expr), // No +x in Rust
            UnaryOp::BitNot => Ok(parse_quote! { !#operand_expr }),
        }
    }

    fn convert_call(&self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
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
                // TODO: Handle step parameter
                bail!("range() with step parameter not yet supported")
            }
            _ => bail!("Invalid number of arguments for range()"),
        }
    }

    fn convert_generic_call(&self, func: &str, args: &[syn::Expr]) -> Result<syn::Expr> {
        let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
        Ok(parse_quote! { #func_ident(#(#args),*) })
    }

    fn convert_index(&self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;
        let index_expr = self.convert(index)?;

        // V1: Direct indexing for simplicity (matches Python behavior)
        Ok(parse_quote! {
            #base_expr[#index_expr as usize]
        })
    }

    fn convert_list(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| self.convert(e))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    fn convert_dict(&self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        let insert_exprs: Vec<syn::Expr> = items
            .iter()
            .map(|(k, v)| {
                let key = self.convert(k)?;
                let val = self.convert(v)?;
                Ok(parse_quote! { map.insert(#key, #val) })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(parse_quote! {
            {
                let mut map = HashMap::new();
                #(#insert_exprs;)*
                map
            }
        })
    }

    fn convert_tuple(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| self.convert(e))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }
}

fn convert_literal(lit: &Literal) -> syn::Expr {
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

/// Convert HIR binary operators to Rust binary operators
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

        // Special membership operators (require special handling)
        In | NotIn => bail!("in/not in operators require special handling"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_mapper::TypeMapper;

    fn create_test_type_mapper() -> TypeMapper {
        TypeMapper::default()
    }

    #[test]
    fn test_expr_converter_literal() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let lit_expr = HirExpr::Literal(Literal::Int(42));
        let result = converter.convert(&lit_expr).unwrap();

        // Should generate a literal integer expression
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_expr_converter_variable() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let var_expr = HirExpr::Var("x".to_string());
        let result = converter.convert(&var_expr).unwrap();

        // Should generate a path expression (variable reference)
        assert!(matches!(result, syn::Expr::Path(_)));
    }

    #[test]
    fn test_expr_converter_binary() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let binary_expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };

        let result = converter.convert(&binary_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));
    }

    #[test]
    fn test_expr_converter_len_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let call_expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
        };

        let result = converter.convert(&call_expr).unwrap();
        // Should generate a method call expression
        assert!(matches!(result, syn::Expr::MethodCall(_)));
    }

    #[test]
    fn test_expr_converter_range_call_single_arg() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let call_expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
        };

        let result = converter.convert(&call_expr).unwrap();
        // Should generate a range expression
        assert!(matches!(result, syn::Expr::Range(_)));
    }

    #[test]
    fn test_expr_converter_range_call_two_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let call_expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(10)),
            ],
        };

        let result = converter.convert(&call_expr).unwrap();
        assert!(matches!(result, syn::Expr::Range(_)));
    }

    #[test]
    fn test_expr_converter_list() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let list_expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        let result = converter.convert(&list_expr).unwrap();
        // Should generate a macro call expression (vec![...])
        assert!(matches!(result, syn::Expr::Macro(_)));
    }

    #[test]
    fn test_expr_converter_dict() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let dict_expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);

        let result = converter.convert(&dict_expr).unwrap();
        // Should generate a block expression
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_expr_converter_tuple() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let tuple_expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("hello".to_string())),
        ]);

        let result = converter.convert(&tuple_expr).unwrap();
        // Should generate a tuple expression
        assert!(matches!(result, syn::Expr::Tuple(_)));
    }

    #[test]
    fn test_convert_binop_arithmetic() {
        // Test arithmetic operators
        assert!(convert_binop(BinOp::Add).is_ok());
        assert!(convert_binop(BinOp::Sub).is_ok());
        assert!(convert_binop(BinOp::Mul).is_ok());
        assert!(convert_binop(BinOp::Div).is_ok());
        assert!(convert_binop(BinOp::Mod).is_ok());
    }

    #[test]
    fn test_convert_binop_comparison() {
        // Test comparison operators
        assert!(convert_binop(BinOp::Eq).is_ok());
        assert!(convert_binop(BinOp::NotEq).is_ok());
        assert!(convert_binop(BinOp::Lt).is_ok());
        assert!(convert_binop(BinOp::LtEq).is_ok());
        assert!(convert_binop(BinOp::Gt).is_ok());
        assert!(convert_binop(BinOp::GtEq).is_ok());
    }

    #[test]
    fn test_convert_binop_logical() {
        // Test logical operators
        assert!(convert_binop(BinOp::And).is_ok());
        assert!(convert_binop(BinOp::Or).is_ok());
    }

    #[test]
    fn test_convert_binop_bitwise() {
        // Test bitwise operators
        assert!(convert_binop(BinOp::BitAnd).is_ok());
        assert!(convert_binop(BinOp::BitOr).is_ok());
        assert!(convert_binop(BinOp::BitXor).is_ok());
        assert!(convert_binop(BinOp::LShift).is_ok());
        assert!(convert_binop(BinOp::RShift).is_ok());
    }

    #[test]
    fn test_convert_binop_unsupported() {
        // Test unsupported operators
        assert!(convert_binop(BinOp::Pow).is_err());
        assert!(convert_binop(BinOp::In).is_err());
        assert!(convert_binop(BinOp::NotIn).is_err());
    }

    #[test]
    fn test_convert_literal() {
        // Test integer literal
        let int_lit = convert_literal(&Literal::Int(42));
        assert!(matches!(int_lit, syn::Expr::Lit(_)));

        // Test float literal
        let float_lit = convert_literal(&Literal::Float(1.234)); // Use arbitrary float for test
        assert!(matches!(float_lit, syn::Expr::Lit(_)));

        // Test string literal
        let string_lit = convert_literal(&Literal::String("hello".to_string()));
        assert!(matches!(string_lit, syn::Expr::MethodCall(_)));

        // Test bool literal
        let bool_lit = convert_literal(&Literal::Bool(true));
        assert!(matches!(bool_lit, syn::Expr::Lit(_)));

        // Test None literal
        let none_lit = convert_literal(&Literal::None);
        assert!(matches!(none_lit, syn::Expr::Tuple(_)));
    }

    #[test]
    fn test_convert_function_with_documentation() {
        let type_mapper = create_test_type_mapper();

        let func = HirFunction {
            name: "test_func".to_string(),
            params: vec![("x".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties {
                is_pure: true,
                always_terminates: true,
                panic_free: true,
                max_stack_depth: Some(1),
            },
        };

        let result = convert_function(&func, &type_mapper).unwrap();

        // Should have documentation attributes
        assert!(!result.attrs.is_empty());
        assert_eq!(result.sig.ident.to_string(), "test_func");
    }

    #[test]
    fn test_apply_rules() {
        let type_mapper = create_test_type_mapper();

        let module = HirModule {
            functions: vec![HirFunction {
                name: "add".to_string(),
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: FunctionProperties::default(),
            }],
            imports: vec![],
        };

        let result = apply_rules(&module, &type_mapper).unwrap();

        // Should have at least one import and one function
        assert!(result.items.len() >= 2);

        // First item should be an import
        assert!(matches!(result.items[0], syn::Item::Use(_)));

        // Second item should be a function
        assert!(matches!(result.items[1], syn::Item::Fn(_)));
    }
}
