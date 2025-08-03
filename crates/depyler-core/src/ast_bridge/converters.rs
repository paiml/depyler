use super::{
    convert_aug_op, convert_binop, convert_body, convert_cmpop, convert_unaryop,
    extract_assign_target, extract_simple_target,
};
use crate::hir::*;
use anyhow::{bail, Result};
use rustpython_ast::{self as ast};

/// Statement converter to reduce complexity
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
            _ => bail!("Statement type not yet supported"),
        }
    }

    fn convert_assign(a: ast::StmtAssign) -> Result<HirStmt> {
        if a.targets.len() != 1 {
            bail!("Multiple assignment targets not supported");
        }
        let target = extract_assign_target(&a.targets[0])?;
        let value = super::convert_expr(*a.value)?;
        Ok(HirStmt::Assign { target, value })
    }

    fn convert_ann_assign(a: ast::StmtAnnAssign) -> Result<HirStmt> {
        let target = extract_assign_target(&a.target)?;
        let value = if let Some(v) = a.value {
            super::convert_expr(*v)?
        } else {
            bail!("Annotated assignment without value not supported")
        };
        Ok(HirStmt::Assign { target, value })
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
        let target = extract_simple_target(&f.target)?;
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
            AssignTarget::Attribute { value, attr } => {
                Box::new(HirExpr::Attribute {
                    value: value.clone(),
                    attr: attr.clone(),
                })
            }
            _ => bail!("Augmented assignment not supported for this target type"),
        };
        
        let right = Box::new(super::convert_expr(*a.value)?);
        let value = HirExpr::Binary { op, left, right };
        Ok(HirStmt::Assign { target, value })
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
}

/// Expression converter to reduce complexity
pub struct ExprConverter;

impl ExprConverter {
    pub fn convert(expr: ast::Expr) -> Result<HirExpr> {
        match expr {
            ast::Expr::Constant(c) => Self::convert_constant(c),
            ast::Expr::Name(n) => Self::convert_name(n),
            ast::Expr::BinOp(b) => Self::convert_binop_expr(b),
            ast::Expr::UnaryOp(u) => Self::convert_unaryop_expr(u),
            ast::Expr::Call(c) => Self::convert_call(c),
            ast::Expr::Subscript(s) => Self::convert_subscript(s),
            ast::Expr::List(l) => Self::convert_list(l),
            ast::Expr::Dict(d) => Self::convert_dict(d),
            ast::Expr::Tuple(t) => Self::convert_tuple(t),
            ast::Expr::Compare(c) => Self::convert_compare(c),
            ast::Expr::ListComp(lc) => Self::convert_list_comp(lc),
            ast::Expr::SetComp(sc) => Self::convert_set_comp(sc),
            ast::Expr::Lambda(l) => Self::convert_lambda(l),
            ast::Expr::Set(s) => Self::convert_set(s),
            ast::Expr::Attribute(a) => Self::convert_attribute(a),
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
        let args = c
            .args
            .into_iter()
            .map(Self::convert)
            .collect::<Result<Vec<_>>>()?;

        match c.func.as_ref() {
            ast::Expr::Name(n) => {
                // Simple function call
                let func = n.id.to_string();
                Ok(HirExpr::Call { func, args })
            }
            ast::Expr::Attribute(attr) => {
                // Method call
                let object = Box::new(Self::convert(*attr.value.clone())?);
                let method = attr.attr.to_string();
                Ok(HirExpr::MethodCall {
                    object,
                    method,
                    args,
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

    fn convert_compare(c: ast::ExprCompare) -> Result<HirExpr> {
        // Convert simple comparisons to binary ops
        if c.ops.len() != 1 || c.comparators.len() != 1 {
            bail!("Chained comparisons not yet supported");
        }
        let op = convert_cmpop(&c.ops[0])?;
        let left = Box::new(Self::convert(*c.left)?);
        let right = Box::new(Self::convert(c.comparators.into_iter().next().unwrap())?);
        Ok(HirExpr::Binary { op, left, right })
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
}
