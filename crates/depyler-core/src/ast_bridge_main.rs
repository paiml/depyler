    fn try_convert_if_main(&mut self, if_stmt: &ast::StmtIf) -> Result<Option<HirFunction>> {
        // Check if condition is `__name__ == "__main__"`
        if !self.is_main_guard(&if_stmt.test) {
            return Ok(None);
        }

        // Convert the body of the if statement to HirStmts
        let body: Vec<HirStmt> =
            if_stmt.body.iter().filter_map(|stmt| convert_stmt(stmt.clone()).ok()).collect();

        // Create a main function with the body
        Ok(Some(HirFunction {
            name: "main".to_string(),
            params: smallvec::smallvec![],
            ret_type: Type::None, // Rust main() returns ()
            body,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }))
    }

    fn is_main_guard(&self, expr: &ast::Expr) -> bool {
        match expr {
            ast::Expr::Compare(compare) => {
                // Check left side is `__name__`
                let left_is_name = match compare.left.as_ref() {
                    ast::Expr::Name(name) => name.id.as_str() == "__name__",
                    _ => false,
                };

                // Check operator is `==`
                let op_is_eq = compare.ops.len() == 1 && matches!(compare.ops[0], ast::CmpOp::Eq);

                // Check right side is `"__main__"`
                let right_is_main = compare.comparators.len() == 1
                    && match &compare.comparators[0] {
                        ast::Expr::Constant(c) => match &c.value {
                            ast::Constant::Str(s) => s.as_str() == "__main__",
                            _ => false,
                        },
                        _ => false,
                    };

                left_is_name && op_is_eq && right_is_main
            }
            _ => false,
        }
    }

fn convert_stmt(stmt: ast::Stmt) -> Result<HirStmt> {
    StmtConverter::convert(stmt)
}

pub(crate) fn extract_assign_target(expr: &ast::Expr) -> Result<AssignTarget> {
    use crate::ast_bridge::converters::ExprConverter;
    match expr {
        ast::Expr::Name(n) => Ok(AssignTarget::Symbol(n.id.to_string())),
        ast::Expr::Subscript(s) => {
            let base = Box::new(ExprConverter::convert(s.value.as_ref().clone())?);
            let index = Box::new(ExprConverter::convert(s.slice.as_ref().clone())?);
            Ok(AssignTarget::Index { base, index })
        }
        ast::Expr::Attribute(a) => {
            let value = Box::new(ExprConverter::convert(a.value.as_ref().clone())?);
            Ok(AssignTarget::Attribute { value, attr: a.attr.to_string() })
        }
        ast::Expr::Tuple(t) => {
            let targets = t.elts.iter().map(extract_assign_target).collect::<Result<Vec<_>>>()?;
            Ok(AssignTarget::Tuple(targets))
        }
        _ => bail!("Unsupported assignment target"),
    }
}

pub(crate) fn convert_expr(expr: ast::Expr) -> Result<HirExpr> {
    ExprConverter::convert(expr)
}

pub(crate) fn convert_binop(op: &ast::Operator) -> Result<BinOp> {
    Ok(match op {
        ast::Operator::Add => BinOp::Add,
        ast::Operator::Sub => BinOp::Sub,
        ast::Operator::Mult => BinOp::Mul,
        ast::Operator::Div => BinOp::Div,
        ast::Operator::FloorDiv => BinOp::FloorDiv,
        ast::Operator::Mod => BinOp::Mod,
        ast::Operator::Pow => BinOp::Pow,
        ast::Operator::BitAnd => BinOp::BitAnd,
        ast::Operator::BitOr => BinOp::BitOr,
        ast::Operator::BitXor => BinOp::BitXor,
        ast::Operator::LShift => BinOp::LShift,
        ast::Operator::RShift => BinOp::RShift,
        _ => bail!("Unsupported binary operator"),
    })
}

pub(crate) fn convert_aug_op(op: &ast::Operator) -> Result<BinOp> {
    // Augmented assignment operators map to the same binary operators
    convert_binop(op)
}

pub(crate) fn convert_unaryop(op: &ast::UnaryOp) -> Result<UnaryOp> {
    Ok(match op {
        ast::UnaryOp::Not => UnaryOp::Not,
        ast::UnaryOp::UAdd => UnaryOp::Pos,
        ast::UnaryOp::USub => UnaryOp::Neg,
        ast::UnaryOp::Invert => UnaryOp::BitNot,
    })
}

pub(crate) fn convert_cmpop(op: &ast::CmpOp) -> Result<BinOp> {
    Ok(match op {
        ast::CmpOp::Eq => BinOp::Eq,
        ast::CmpOp::NotEq => BinOp::NotEq,
        ast::CmpOp::Lt => BinOp::Lt,
        ast::CmpOp::LtE => BinOp::LtEq,
        ast::CmpOp::Gt => BinOp::Gt,
        ast::CmpOp::GtE => BinOp::GtEq,
        ast::CmpOp::In => BinOp::In,
        ast::CmpOp::NotIn => BinOp::NotIn,
        // DEPYLER-0188: Python 'is' checks identity (same object), but in transpiled code
        // we use value equality since Rust doesn't have Python's object identity concept.
        // This is correct for: x is None, x is True/False, small integers, interned strings
        ast::CmpOp::Is => BinOp::Eq,
        ast::CmpOp::IsNot => BinOp::NotEq,
    })
}
