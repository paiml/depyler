use crate::hir::*;
use anyhow::{bail, Result};
use rustpython_ast::{self as ast};
use super::{convert_aug_op, convert_binop, convert_cmpop, convert_unaryop, extract_assign_target, convert_body};

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
        let left = Box::new(HirExpr::Var(target.clone()));
        let right = Box::new(super::convert_expr(*a.value)?);
        let value = HirExpr::Binary { op, left, right };
        Ok(HirStmt::Assign { target, value })
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
        let func = if let ast::Expr::Name(n) = c.func.as_ref() {
            n.id.to_string()
        } else {
            bail!("Only simple function calls supported")
        };
        let args = c
            .args
            .into_iter()
            .map(Self::convert)
            .collect::<Result<Vec<_>>>()?;
        Ok(HirExpr::Call { func, args })
    }

    fn convert_subscript(s: ast::ExprSubscript) -> Result<HirExpr> {
        let base = Box::new(Self::convert(*s.value)?);
        let index = Box::new(Self::convert(*s.slice)?);
        Ok(HirExpr::Index { base, index })
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
}