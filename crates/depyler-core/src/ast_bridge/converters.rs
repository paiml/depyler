use super::{
    convert_aug_op, convert_binop, convert_body, convert_cmpop, convert_unaryop,
    extract_assign_target,
};
use crate::hir::*;
use anyhow::{bail, Result};
use rustpython_ast::{self as ast};

#[cfg(test)]
#[path = "converters_tests.rs"]
mod tests;

/// Statement converter to reduce complexity
///
/// # Examples
///
/// ```rust,ignore
/// use depyler_core::ast_bridge::converters::StmtConverter;
/// use rustpython_parser::Parse;
///
/// let stmt = rustpython_parser::parse("x = 42", rustpython_parser::Mode::Module, "<test>").unwrap();
/// // Convert the first statement
/// let hir_stmt = StmtConverter::convert(stmt).unwrap();
/// ```
pub struct StmtConverter;

impl StmtConverter {
    pub fn convert(stmt: ast::Stmt) -> Result<HirStmt> {
        match stmt {
            ast::Stmt::Assign(a) => Self::convert_assign(a),
            ast::Stmt::AnnAssign(a) => Self::convert_ann_assign(a),
            ast::Stmt::AugAssign(a) => Self::convert_aug_assign(a),
            ast::Stmt::Return(r) => Self::convert_return(r),
            ast::Stmt::If(i) => Self::convert_if(i),
            ast::Stmt::While(w) => Self::convert_while(w),
            ast::Stmt::For(f) => Self::convert_for(f),
            ast::Stmt::Expr(e) => Self::convert_expr_stmt(e),
            ast::Stmt::Raise(r) => Self::convert_raise(r),
            ast::Stmt::Break(b) => Self::convert_break(b),
            ast::Stmt::Continue(c) => Self::convert_continue(c),
            ast::Stmt::With(w) => Self::convert_with(w),
            ast::Stmt::Try(t) => Self::convert_try(t),
            ast::Stmt::Assert(a) => Self::convert_assert(a),
            ast::Stmt::Pass(_) => Self::convert_pass(),
            _ => bail!("Statement type not yet supported"),
        }
    }

    fn convert_assign(a: ast::StmtAssign) -> Result<HirStmt> {
        if a.targets.len() != 1 {
            bail!("Multiple assignment targets not supported");
        }
        let target = extract_assign_target(&a.targets[0])?;
        let value = super::convert_expr(*a.value)?;
        Ok(HirStmt::Assign {
            target,
            value,
            type_annotation: None,
        })
    }

    fn convert_ann_assign(a: ast::StmtAnnAssign) -> Result<HirStmt> {
        let target = extract_assign_target(&a.target)?;
        let value = if let Some(v) = a.value {
            super::convert_expr(*v)?
        } else {
            bail!("Annotated assignment without value not supported")
        };

        // Extract type annotation
        let type_annotation = Some(super::type_extraction::TypeExtractor::extract_type(
            &a.annotation,
        )?);

        Ok(HirStmt::Assign {
            target,
            value,
            type_annotation,
        })
    }

    fn convert_return(r: ast::StmtReturn) -> Result<HirStmt> {
        let value = r.value.map(|v| super::convert_expr(*v)).transpose()?;
        Ok(HirStmt::Return(value))
    }

    fn convert_if(i: ast::StmtIf) -> Result<HirStmt> {
        let condition = super::convert_expr(*i.test)?;
        let then_body = convert_body(i.body)?;
        let else_body = if i.orelse.is_empty() {
            None
        } else {
            Some(convert_body(i.orelse)?)
        };
        Ok(HirStmt::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn convert_while(w: ast::StmtWhile) -> Result<HirStmt> {
        let condition = super::convert_expr(*w.test)?;
        let body = convert_body(w.body)?;
        Ok(HirStmt::While { condition, body })
    }

    fn convert_for(f: ast::StmtFor) -> Result<HirStmt> {
        let target = extract_assign_target(&f.target)?;
        let iter = super::convert_expr(*f.iter)?;
        let body = convert_body(f.body)?;
        Ok(HirStmt::For { target, iter, body })
    }

    fn convert_expr_stmt(e: ast::StmtExpr) -> Result<HirStmt> {
        let expr = super::convert_expr(*e.value)?;
        Ok(HirStmt::Expr(expr))
    }

    fn convert_aug_assign(a: ast::StmtAugAssign) -> Result<HirStmt> {
        let target = extract_assign_target(&a.target)?;
        let op = convert_aug_op(&a.op)?;

        // Convert the target to an expression for the left side of the binary op
        let left = match &target {
            AssignTarget::Symbol(s) => Box::new(HirExpr::Var(s.clone())),
            AssignTarget::Attribute { value, attr } => Box::new(HirExpr::Attribute {
                value: value.clone(),
                attr: attr.clone(),
            }),
            AssignTarget::Index { base, index } => Box::new(HirExpr::Index {
                base: base.clone(),
                index: index.clone(),
            }),
            _ => bail!("Augmented assignment not supported for this target type"),
        };

        let right = Box::new(super::convert_expr(*a.value)?);
        let value = HirExpr::Binary { op, left, right };
        Ok(HirStmt::Assign {
            target,
            value,
            type_annotation: None,
        })
    }

    fn convert_raise(r: ast::StmtRaise) -> Result<HirStmt> {
        let exception = r.exc.map(|e| super::convert_expr(*e)).transpose()?;
        let cause = r.cause.map(|c| super::convert_expr(*c)).transpose()?;
        Ok(HirStmt::Raise { exception, cause })
    }

    fn convert_break(_b: ast::StmtBreak) -> Result<HirStmt> {
        // Python's AST doesn't support labeled break directly
        // Labels are handled at a higher level with loop naming
        Ok(HirStmt::Break { label: None })
    }

    fn convert_continue(_c: ast::StmtContinue) -> Result<HirStmt> {
        // Python's AST doesn't support labeled continue directly
        // Labels are handled at a higher level with loop naming
        Ok(HirStmt::Continue { label: None })
    }

    fn convert_with(w: ast::StmtWith) -> Result<HirStmt> {
        // For now, only support single context manager
        if w.items.len() != 1 {
            bail!("Multiple context managers not yet supported");
        }

        let item = &w.items[0];
        let context = super::convert_expr(item.context_expr.clone())?;

        // Extract optional target variable
        let target = item.optional_vars.as_ref().and_then(|vars| {
            match vars.as_ref() {
                ast::Expr::Name(n) => Some(n.id.to_string()),
                _ => None, // Complex targets not supported yet
            }
        });

        // Convert body
        let body = w
            .body
            .into_iter()
            .map(super::convert_stmt)
            .collect::<Result<Vec<_>>>()?;

        Ok(HirStmt::With {
            context,
            target,
            body,
        })
    }

    fn convert_try(t: ast::StmtTry) -> Result<HirStmt> {
        let body = convert_body(t.body)?;

        let mut handlers = Vec::new();
        for handler in t.handlers {
            // Extract the ExceptHandlerExceptHandler from the enum
            let ast::ExceptHandler::ExceptHandler(h) = handler;

            let exception_type = h.type_.as_ref().map(|t| {
                match t.as_ref() {
                    ast::Expr::Name(n) => n.id.to_string(),
                    _ => "Exception".to_string(), // Default to generic exception
                }
            });

            let name = h.name.as_ref().map(|id| id.to_string());
            let handler_body = convert_body(h.body)?;

            handlers.push(crate::hir::ExceptHandler {
                exception_type,
                name,
                body: handler_body,
            });
        }

        let orelse = if t.orelse.is_empty() {
            None
        } else {
            Some(convert_body(t.orelse)?)
        };

        let finalbody = if t.finalbody.is_empty() {
            None
        } else {
            Some(convert_body(t.finalbody)?)
        };

        Ok(HirStmt::Try {
            body,
            handlers,
            orelse,
            finalbody,
        })
    }

    fn convert_assert(a: ast::StmtAssert) -> Result<HirStmt> {
        let test = super::convert_expr(*a.test)?;
        let msg = a.msg.map(|m| super::convert_expr(*m)).transpose()?;
        Ok(HirStmt::Assert { test, msg })
    }

    fn convert_pass() -> Result<HirStmt> {
        Ok(HirStmt::Pass)
    }
}

/// Expression converter to reduce complexity
///
/// # Examples
///
/// ```rust,ignore
/// use depyler_core::ast_bridge::converters::ExprConverter;
/// use rustpython_parser::Parse;
/// use rustpython_ast::Expr;
///
/// let expr = Expr::parse("42", "<test>").unwrap();
/// let hir_expr = ExprConverter::convert(expr).unwrap();
/// ```
pub struct ExprConverter;

impl ExprConverter {
    pub fn convert(expr: ast::Expr) -> Result<HirExpr> {
        match expr {
            ast::Expr::Constant(c) => Self::convert_constant(c),
            ast::Expr::Name(n) => Self::convert_name(n),
            ast::Expr::BinOp(b) => Self::convert_binop_expr(b),
            ast::Expr::UnaryOp(u) => Self::convert_unaryop_expr(u),
            ast::Expr::BoolOp(b) => Self::convert_boolop(b),
            ast::Expr::Call(c) => Self::convert_call(c),
            ast::Expr::Subscript(s) => Self::convert_subscript(s),
            ast::Expr::List(l) => Self::convert_list(l),
            ast::Expr::Dict(d) => Self::convert_dict(d),
            ast::Expr::Tuple(t) => Self::convert_tuple(t),
            ast::Expr::Compare(c) => Self::convert_compare(c),
            ast::Expr::ListComp(lc) => Self::convert_list_comp(lc),
            ast::Expr::SetComp(sc) => Self::convert_set_comp(sc),
            ast::Expr::DictComp(dc) => Self::convert_dict_comp(dc),
            ast::Expr::GeneratorExp(ge) => Self::convert_generator_exp(ge),
            ast::Expr::Lambda(l) => Self::convert_lambda(l),
            ast::Expr::Set(s) => Self::convert_set(s),
            ast::Expr::Attribute(a) => Self::convert_attribute(a),
            ast::Expr::Await(a) => Self::convert_await(a),
            ast::Expr::Yield(y) => Self::convert_yield(y),
            ast::Expr::JoinedStr(js) => Self::convert_fstring(js),
            ast::Expr::IfExp(i) => Self::convert_ifexp(i),
            _ => bail!("Expression type not yet supported"),
        }
    }

    fn convert_constant(c: ast::ExprConstant) -> Result<HirExpr> {
        let lit = match &c.value {
            ast::Constant::Int(i) => {
                // Convert BigInt to i64, with overflow handling
                let int_val = i.try_into().unwrap_or(0i64);
                Literal::Int(int_val)
            }
            ast::Constant::Float(f) => Literal::Float(*f),
            ast::Constant::Str(s) => Literal::String(s.to_string()),
            ast::Constant::Bytes(b) => Literal::Bytes(b.clone()),
            ast::Constant::Bool(b) => Literal::Bool(*b),
            ast::Constant::None => Literal::None,
            _ => bail!("Unsupported constant type"),
        };
        Ok(HirExpr::Literal(lit))
    }

    fn convert_name(n: ast::ExprName) -> Result<HirExpr> {
        Ok(HirExpr::Var(n.id.to_string()))
    }

    fn convert_binop_expr(b: ast::ExprBinOp) -> Result<HirExpr> {
        let op = convert_binop(&b.op)?;
        let left = Box::new(Self::convert(*b.left)?);
        let right = Box::new(Self::convert(*b.right)?);
        Ok(HirExpr::Binary { op, left, right })
    }

    fn convert_unaryop_expr(u: ast::ExprUnaryOp) -> Result<HirExpr> {
        let op = convert_unaryop(&u.op)?;
        let operand = Box::new(Self::convert(*u.operand)?);
        Ok(HirExpr::Unary { op, operand })
    }

    fn convert_call(c: ast::ExprCall) -> Result<HirExpr> {
        // Special handling for sorted() with key parameter
        if let ast::Expr::Name(n) = &*c.func {
            if n.id.as_str() == "sorted" && !c.keywords.is_empty() {
                // Extract key and reverse parameters
                let mut key_lambda = None;
                let mut reverse = false;

                for keyword in &c.keywords {
                    if let Some(arg_name) = &keyword.arg {
                        match arg_name.as_str() {
                            "key" => {
                                if let ast::Expr::Lambda(lambda) = &keyword.value {
                                    key_lambda = Some(lambda.clone());
                                } else {
                                    bail!("sorted() key parameter must be a lambda");
                                }
                            }
                            "reverse" => {
                                // Extract boolean value from reverse parameter
                                if let ast::Expr::Constant(c) = &keyword.value {
                                    if let ast::Constant::Bool(b) = &c.value {
                                        reverse = *b;
                                    } else {
                                        bail!("sorted() reverse parameter must be a boolean");
                                    }
                                } else {
                                    bail!("sorted() reverse parameter must be a constant boolean");
                                }
                            }
                            _ => {} // Ignore other parameters
                        }
                    }
                }

                // If we found a key lambda, create SortByKey
                if let Some(lambda) = key_lambda {
                    // Convert the iterable (first positional arg)
                    if c.args.is_empty() {
                        bail!("sorted() requires at least one argument");
                    }
                    let iterable = Box::new(Self::convert(c.args[0].clone())?);

                    // Extract lambda parameters and body
                    let key_params: Vec<String> = lambda
                        .args
                        .args
                        .iter()
                        .map(|arg| arg.def.arg.to_string())
                        .collect();

                    let key_body = Box::new(Self::convert(*lambda.body.clone())?);

                    return Ok(HirExpr::SortByKey {
                        iterable,
                        key_params,
                        key_body,
                        reverse,
                    });
                }

                // DEPYLER-0307: If reverse=True but no key, create SortByKey with identity function
                // This ensures the reverse parameter is preserved in the HIR
                if reverse {
                    if c.args.is_empty() {
                        bail!("sorted() requires at least one argument");
                    }
                    let iterable = Box::new(Self::convert(c.args[0].clone())?);

                    // Use identity function: lambda x: x
                    let key_params = vec!["x".to_string()];
                    let key_body = Box::new(HirExpr::Var("x".to_string()));

                    return Ok(HirExpr::SortByKey {
                        iterable,
                        key_params,
                        key_body,
                        reverse,
                    });
                }
            }
        }

        let args = c
            .args
            .into_iter()
            .map(Self::convert)
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0364: Extract keyword arguments from Python AST
        let kwargs: Vec<(String, HirExpr)> = c
            .keywords
            .into_iter()
            .filter_map(|kw| {
                // Only process keywords with explicit names (not **kwargs unpacking)
                if let Some(arg_name) = kw.arg {
                    let value = Self::convert(kw.value).ok()?;
                    Some((arg_name.to_string(), value))
                } else {
                    None // Skip **kwargs unpacking for now
                }
            })
            .collect();

        match &*c.func {
            ast::Expr::Name(n) => {
                // Simple function call
                let func = n.id.to_string();
                Ok(HirExpr::Call { func, args, kwargs })
            }
            ast::Expr::Attribute(attr) => {
                // Method call
                let object = Box::new(Self::convert(*attr.value.clone())?);
                let method = attr.attr.to_string();
                Ok(HirExpr::MethodCall {
                    object,
                    method,
                    args,
                    kwargs,
                })
            }
            _ => bail!("Unsupported function call type"),
        }
    }

    fn convert_subscript(s: ast::ExprSubscript) -> Result<HirExpr> {
        let base = Box::new(Self::convert(*s.value)?);

        // Check if the slice is actually a slice expression or a simple index
        match s.slice.as_ref() {
            ast::Expr::Slice(slice_expr) => {
                // Convert slice expression
                let start = slice_expr
                    .lower
                    .as_ref()
                    .map(|e| Self::convert(e.as_ref().clone()))
                    .transpose()?
                    .map(Box::new);
                let stop = slice_expr
                    .upper
                    .as_ref()
                    .map(|e| Self::convert(e.as_ref().clone()))
                    .transpose()?
                    .map(Box::new);
                let step = slice_expr
                    .step
                    .as_ref()
                    .map(|e| Self::convert(e.as_ref().clone()))
                    .transpose()?
                    .map(Box::new);

                Ok(HirExpr::Slice {
                    base,
                    start,
                    stop,
                    step,
                })
            }
            _ => {
                // Regular indexing
                let index = Box::new(Self::convert(*s.slice)?);
                Ok(HirExpr::Index { base, index })
            }
        }
    }

    fn convert_list(l: ast::ExprList) -> Result<HirExpr> {
        let elts = l
            .elts
            .into_iter()
            .map(Self::convert)
            .collect::<Result<Vec<_>>>()?;
        Ok(HirExpr::List(elts))
    }

    fn convert_dict(d: ast::ExprDict) -> Result<HirExpr> {
        let mut items = Vec::new();
        for (k, v) in d.keys.into_iter().zip(d.values.into_iter()) {
            if let Some(key) = k {
                let key_expr = Self::convert(key)?;
                let val_expr = Self::convert(v)?;
                items.push((key_expr, val_expr));
            } else {
                bail!("Dict unpacking not supported");
            }
        }
        Ok(HirExpr::Dict(items))
    }

    fn convert_tuple(t: ast::ExprTuple) -> Result<HirExpr> {
        let elts = t
            .elts
            .into_iter()
            .map(Self::convert)
            .collect::<Result<Vec<_>>>()?;
        Ok(HirExpr::Tuple(elts))
    }

    fn convert_boolop(b: ast::ExprBoolOp) -> Result<HirExpr> {
        // Convert boolean operations (and, or) to binary operations
        if b.values.len() < 2 {
            bail!("BoolOp must have at least 2 values");
        }

        // Convert the operator
        let op = match b.op {
            ast::BoolOp::And => BinOp::And,
            ast::BoolOp::Or => BinOp::Or,
        };

        // Convert values and chain them left-to-right
        let mut result = Self::convert(b.values[0].clone())?;
        for value in b.values.iter().skip(1) {
            let right = Self::convert(value.clone())?;
            result = HirExpr::Binary {
                op,
                left: Box::new(result),
                right: Box::new(right),
            };
        }

        Ok(result)
    }

    fn convert_compare(c: ast::ExprCompare) -> Result<HirExpr> {
        // Handle chained comparisons by desugaring them
        // Example: 0 <= x <= 100 becomes (0 <= x) and (x <= 100)

        if c.ops.is_empty() || c.comparators.is_empty() {
            bail!("Compare expression must have at least one operator and comparator");
        }

        // Special handling for 'is None', 'is True', 'is False' patterns (single comparison only)
        if c.ops.len() == 1
            && c.comparators.len() == 1
            && matches!(c.ops[0], ast::CmpOp::Is | ast::CmpOp::IsNot)
        {
            let comparator = &c.comparators[0];
            // Check if comparing with None
            let is_none_comparison = matches!(comparator, ast::Expr::Constant(ref cons)
                    if matches!(cons.value, ast::Constant::None));

            if is_none_comparison {
                // Convert 'x is None' to x.is_none(), 'x is not None' to x.is_some()
                let object = Box::new(Self::convert(*c.left)?);
                let method = if matches!(c.ops[0], ast::CmpOp::Is) {
                    "is_none".to_string()
                } else {
                    "is_some".to_string()
                };
                return Ok(HirExpr::MethodCall {
                    object,
                    method,
                    args: vec![],
                    kwargs: vec![],
                });
            }

            // Check if comparing with True or False
            let is_bool_comparison = matches!(comparator, ast::Expr::Constant(ref cons)
                    if matches!(cons.value, ast::Constant::Bool(_)));

            if is_bool_comparison {
                // Convert 'x is True' to x == true, 'x is False' to x == false
                // Convert 'x is not True' to x != true, 'x is not False' to x != false
                let left_hir = Box::new(Self::convert(*c.left)?);
                let right_hir = Box::new(Self::convert(comparator.clone())?);
                let op = if matches!(c.ops[0], ast::CmpOp::Is) {
                    BinOp::Eq
                } else {
                    BinOp::NotEq
                };
                return Ok(HirExpr::Binary {
                    op,
                    left: left_hir,
                    right: right_hir,
                });
            }
        }

        // Build chain: a op1 b op2 c becomes (a op1 b) and (b op2 c)
        let mut left_expr = *c.left;
        let mut comparisons = Vec::new();

        for (op, comparator) in c.ops.iter().zip(c.comparators.iter()) {
            let op_hir = convert_cmpop(op)?;
            let left_hir = Box::new(Self::convert(left_expr.clone())?);
            let right_hir = Box::new(Self::convert(comparator.clone())?);

            comparisons.push(HirExpr::Binary {
                op: op_hir,
                left: left_hir,
                right: right_hir,
            });

            // For next iteration, the right side becomes the left side
            left_expr = comparator.clone();
        }

        // If only one comparison, return it directly
        if comparisons.len() == 1 {
            return Ok(comparisons.into_iter().next().unwrap());
        }

        // Chain multiple comparisons with AND
        let mut result = comparisons[0].clone();
        for comparison in comparisons.iter().skip(1) {
            result = HirExpr::Binary {
                op: BinOp::And,
                left: Box::new(result),
                right: Box::new(comparison.clone()),
            };
        }

        Ok(result)
    }

    fn convert_list_comp(lc: ast::ExprListComp) -> Result<HirExpr> {
        // Convert only simple list comprehensions for now
        if lc.generators.len() != 1 {
            bail!("Nested list comprehensions not yet supported");
        }

        let generator = &lc.generators[0];

        // Extract the target variable
        let target = match &generator.target {
            ast::Expr::Name(n) => n.id.to_string(),
            _ => bail!("Complex comprehension targets not yet supported"),
        };

        // Convert the iterator expression
        let iter = Box::new(Self::convert(generator.iter.clone())?);

        // Convert the element expression
        let element = Box::new(Self::convert(*lc.elt)?);

        // Convert the condition if present
        let condition = if generator.ifs.is_empty() {
            None
        } else if generator.ifs.len() == 1 {
            Some(Box::new(Self::convert(generator.ifs[0].clone())?))
        } else {
            bail!("Multiple conditions in list comprehension not yet supported");
        };

        Ok(HirExpr::ListComp {
            element,
            target,
            iter,
            condition,
        })
    }

    fn convert_set_comp(sc: ast::ExprSetComp) -> Result<HirExpr> {
        // Convert only simple set comprehensions for now
        if sc.generators.len() != 1 {
            bail!("Nested set comprehensions not yet supported");
        }

        let generator = &sc.generators[0];

        // Extract the target variable
        let target = match &generator.target {
            ast::Expr::Name(n) => n.id.to_string(),
            _ => bail!("Complex comprehension targets not yet supported"),
        };

        // Convert the iterator expression
        let iter = Box::new(Self::convert(generator.iter.clone())?);

        // Convert the element expression
        let element = Box::new(Self::convert(*sc.elt)?);

        // Convert the condition if present
        let condition = if generator.ifs.is_empty() {
            None
        } else if generator.ifs.len() == 1 {
            Some(Box::new(Self::convert(generator.ifs[0].clone())?))
        } else {
            bail!("Multiple if conditions in set comprehensions not yet supported");
        };

        Ok(HirExpr::SetComp {
            element,
            target,
            iter,
            condition,
        })
    }

    fn convert_dict_comp(dc: ast::ExprDictComp) -> Result<HirExpr> {
        // Convert only simple dict comprehensions for now
        if dc.generators.len() != 1 {
            bail!("Nested dict comprehensions not yet supported");
        }

        let generator = &dc.generators[0];

        // Extract the target variable
        let target = match &generator.target {
            ast::Expr::Name(n) => n.id.to_string(),
            _ => bail!("Complex comprehension targets not yet supported"),
        };

        // Convert the iterator expression
        let iter = Box::new(Self::convert(generator.iter.clone())?);

        // Convert the key and value expressions
        let key = Box::new(Self::convert(*dc.key)?);
        let value = Box::new(Self::convert(*dc.value)?);

        // Convert the condition if present
        let condition = if generator.ifs.is_empty() {
            None
        } else if generator.ifs.len() == 1 {
            Some(Box::new(Self::convert(generator.ifs[0].clone())?))
        } else {
            bail!("Multiple if conditions in dict comprehensions not yet supported");
        };

        Ok(HirExpr::DictComp {
            key,
            value,
            target,
            iter,
            condition,
        })
    }

    fn convert_generator_exp(ge: ast::ExprGeneratorExp) -> Result<HirExpr> {
        // Convert element expression
        let element = Box::new(Self::convert(*ge.elt)?);

        // Convert all generators (support nested)
        let mut generators = Vec::new();
        for gen in ge.generators {
            // Extract target variable(s)
            let target = match &gen.target {
                ast::Expr::Name(n) => n.id.to_string(),
                ast::Expr::Tuple(t) => {
                    // For tuple unpacking like: (x, y) in zip(a, b)
                    // Extract all names and join with commas (simplified for now)
                    let names: Vec<String> = t
                        .elts
                        .iter()
                        .filter_map(|e| {
                            if let ast::Expr::Name(n) = e {
                                Some(n.id.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                    if names.is_empty() {
                        bail!("Complex tuple unpacking in generator expression not yet supported");
                    }
                    // Join with comma for tuple targets
                    format!("({})", names.join(", "))
                }
                _ => bail!("Complex generator targets not yet supported"),
            };

            // Convert iterator expression
            let iter = Box::new(Self::convert(gen.iter.clone())?);

            // Convert all conditions
            let conditions: Vec<crate::hir::HirExpr> = gen
                .ifs
                .iter()
                .map(|if_expr| Self::convert(if_expr.clone()))
                .collect::<Result<Vec<_>>>()?;

            generators.push(crate::hir::HirComprehension {
                target,
                iter,
                conditions,
            });
        }

        Ok(HirExpr::GeneratorExp {
            element,
            generators,
        })
    }

    fn convert_lambda(l: ast::ExprLambda) -> Result<HirExpr> {
        // Extract parameter names
        let params: Vec<String> = l
            .args
            .args
            .iter()
            .map(|arg| arg.def.arg.to_string())
            .collect();

        // Convert body expression
        let body = Box::new(super::convert_expr(*l.body)?);

        Ok(HirExpr::Lambda { params, body })
    }

    fn convert_set(s: ast::ExprSet) -> Result<HirExpr> {
        let elems = s
            .elts
            .into_iter()
            .map(super::convert_expr)
            .collect::<Result<Vec<_>>>()?;
        Ok(HirExpr::Set(elems))
    }

    fn convert_attribute(a: ast::ExprAttribute) -> Result<HirExpr> {
        let value = Box::new(Self::convert(*a.value)?);
        let attr = a.attr.to_string();
        Ok(HirExpr::Attribute { value, attr })
    }

    fn convert_await(a: ast::ExprAwait) -> Result<HirExpr> {
        let value = Box::new(Self::convert(*a.value)?);
        Ok(HirExpr::Await { value })
    }

    fn convert_yield(y: ast::ExprYield) -> Result<HirExpr> {
        let value = y
            .value
            .map(|v| Self::convert(*v))
            .transpose()?
            .map(Box::new);
        Ok(HirExpr::Yield { value })
    }

    fn convert_fstring(js: ast::ExprJoinedStr) -> Result<HirExpr> {
        let mut parts = Vec::new();

        for value in js.values {
            match value {
                // Literal string parts
                ast::Expr::Constant(c) => {
                    if let ast::Constant::Str(s) = c.value {
                        parts.push(FStringPart::Literal(s.to_string()));
                    }
                }
                // Formatted values (expressions to interpolate)
                ast::Expr::FormattedValue(fv) => {
                    let expr = Self::convert(*fv.value)?;
                    parts.push(FStringPart::Expr(Box::new(expr)));
                }
                _ => {
                    // Other expression types in f-strings (rare)
                    let expr = Self::convert(value)?;
                    parts.push(FStringPart::Expr(Box::new(expr)));
                }
            }
        }

        Ok(HirExpr::FString { parts })
    }

    fn convert_ifexp(i: ast::ExprIfExp) -> Result<HirExpr> {
        let test = Box::new(Self::convert(*i.test)?);
        let body = Box::new(Self::convert(*i.body)?);
        let orelse = Box::new(Self::convert(*i.orelse)?);
        Ok(HirExpr::IfExpr { test, body, orelse })
    }
}
