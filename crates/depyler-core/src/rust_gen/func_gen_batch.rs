fn infer_binary_type_with_env(
    op: &BinOp,
    left: &HirExpr,
    right: &HirExpr,
    var_types: &std::collections::HashMap<String, Type>,
) -> Type {
    if matches!(
        op,
        BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq
            | BinOp::In | BinOp::NotIn
    ) {
        return Type::Bool;
    }
    if matches!(op, BinOp::Mul) {
        match (left, right) {
            (HirExpr::List(elems), HirExpr::Literal(Literal::Int(n)))
                if elems.len() == 1 && *n > 0 =>
            {
                let elem_type = infer_expr_type_with_env(&elems[0], var_types);
                return Type::List(Box::new(elem_type));
            }
            (HirExpr::Literal(Literal::Int(n)), HirExpr::List(elems))
                if elems.len() == 1 && *n > 0 =>
            {
                let elem_type = infer_expr_type_with_env(&elems[0], var_types);
                return Type::List(Box::new(elem_type));
            }
            _ => {}
        }
    }
    if matches!(op, BinOp::Pow) && is_negative_int_expr(right) {
        return Type::Float;
    }
    let left_type = infer_expr_type_with_env(left, var_types);
    let right_type = infer_expr_type_with_env(right, var_types);
    if matches!(left_type, Type::Float) || matches!(right_type, Type::Float) {
        Type::Float
    } else if !matches!(left_type, Type::Unknown) {
        left_type
    } else {
        right_type
    }
}

fn infer_binary_type_simple(op: &BinOp, left: &HirExpr, right: &HirExpr) -> Type {
    // Comparison operators always return bool
    if matches!(
        op,
        BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq
            | BinOp::In | BinOp::NotIn
    ) {
        return Type::Bool;
    }

    // DEPYLER-0420/1132: Detect list repeat patterns: [elem] * n or n * [elem]
    if matches!(op, BinOp::Mul) {
        match (left, right) {
            (HirExpr::List(elems), HirExpr::Literal(Literal::Int(n)))
                if elems.len() == 1 && *n > 0 =>
            {
                let elem_type = infer_expr_type_simple(&elems[0]);
                return Type::List(Box::new(elem_type));
            }
            (HirExpr::Literal(Literal::Int(n)), HirExpr::List(elems))
                if elems.len() == 1 && *n > 0 =>
            {
                let elem_type = infer_expr_type_simple(&elems[0]);
                return Type::List(Box::new(elem_type));
            }
            _ => {}
        }
    }

    // DEPYLER-0808: Power with negative exponent always returns float
    if matches!(op, BinOp::Pow) && is_negative_int_expr(right) {
        return Type::Float;
    }

    // For arithmetic, infer from operands
    let left_type = infer_expr_type_simple(left);
    let right_type = infer_expr_type_simple(right);
    if matches!(left_type, Type::Float) || matches!(right_type, Type::Float) {
        Type::Float
    } else if !matches!(left_type, Type::Unknown) {
        left_type
    } else {
        right_type
    }
}

fn is_negative_int_expr(expr: &HirExpr) -> bool {
    match expr {
        // Direct negative literal: -1, -2, etc.
        HirExpr::Literal(Literal::Int(n)) => *n < 0,
        // Unary negation: -(1), -(x)
        HirExpr::Unary { op, operand } => {
            matches!(op, UnaryOp::Neg)
                && matches!(operand.as_ref(), HirExpr::Literal(Literal::Int(n)) if *n > 0)
        }
        _ => false,
    }
}
