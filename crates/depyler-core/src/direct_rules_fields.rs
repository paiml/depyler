fn infer_method_return_type(body: &[HirStmt], fields: &[HirField]) -> Option<Type> {
    let mut return_types = Vec::new();
    collect_method_return_types(body, fields, &mut return_types);

    if return_types.is_empty() {
        return None;
    }

    // If all return types are the same (ignoring Unknown), use that type
    let first_known = return_types.iter().find(|t| !matches!(t, Type::Unknown));
    if let Some(first) = first_known {
        if return_types.iter().all(|t| matches!(t, Type::Unknown) || t == first) {
            return Some(first.clone());
        }
    }

    // Mixed types - return first known
    first_known.cloned()
}

fn collect_method_return_types(stmts: &[HirStmt], fields: &[HirField], types: &mut Vec<Type>) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                types.push(infer_expr_type_with_fields(expr, fields));
            }
            HirStmt::Return(None) => {
                types.push(Type::None);
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_method_return_types(then_body, fields, types);
                if let Some(else_stmts) = else_body {
                    collect_method_return_types(else_stmts, fields, types);
                }
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                collect_method_return_types(body, fields, types);
            }
            _ => {}
        }
    }
}

fn infer_expr_type_with_fields(expr: &HirExpr, fields: &[HirField]) -> Type {
    match expr {
        HirExpr::Literal(lit) => match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
            Literal::None => Type::None,
            Literal::Bytes(_) => Type::Unknown,
        },
        HirExpr::Binary { op, left, right } => {
            // Comparison operators return bool
            if matches!(
                op,
                BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn
            ) {
                return Type::Bool;
            }
            // For arithmetic, infer from operands
            let left_type = infer_expr_type_with_fields(left, fields);
            if !matches!(left_type, Type::Unknown) {
                left_type
            } else {
                infer_expr_type_with_fields(right, fields)
            }
        }
        HirExpr::Unary { op, operand } => {
            if matches!(op, UnaryOp::Not) {
                Type::Bool
            } else {
                infer_expr_type_with_fields(operand, fields)
            }
        }
        HirExpr::List(elems) => {
            if elems.is_empty() {
                Type::List(Box::new(Type::Unknown))
            } else {
                Type::List(Box::new(infer_expr_type_with_fields(&elems[0], fields)))
            }
        }
        HirExpr::Tuple(elems) => {
            let elem_types: Vec<Type> =
                elems.iter().map(|e| infer_expr_type_with_fields(e, fields)).collect();
            Type::Tuple(elem_types)
        }
        // DEPYLER-0696: Handle attribute access like self.field
        HirExpr::Attribute { value, attr } => {
            // Check if this is self.field access
            if let HirExpr::Var(var_name) = value.as_ref() {
                if var_name == "self" {
                    // Look up field type
                    if let Some(field) = fields.iter().find(|f| f.name == *attr) {
                        return field.field_type.clone();
                    }
                }
            }
            Type::Unknown
        }
        // DEPYLER-0736: Handle constructor calls like Point(...) or Self::new(...)
        // In static methods, return ClassName(...) should infer return type as ClassName
        HirExpr::Call { func, .. } => {
            // Check if func looks like a constructor (capitalized name or Self::new)
            let class_name = if func.starts_with("Self::") || func == "Self" {
                // Self::new() or Self() - return Self type
                // Note: We can't know the actual class name here, but Type::Custom("Self")
                // will be mapped correctly when generating code
                Some("Self".to_string())
            } else if func.chars().next().is_some_and(|c| c.is_uppercase()) {
                // Capitalized name like Point, MyClass - likely a constructor
                // Handle both Point and Point::new patterns
                let base_name = func.split("::").next().unwrap_or(func);
                Some(base_name.to_string())
            } else if func == "cls" {
                // cls() in classmethod - return Self
                Some("Self".to_string())
            } else {
                None
            };

            if let Some(name) = class_name {
                Type::Custom(name)
            } else {
                Type::Unknown
            }
        }
        _ => Type::Unknown,
    }
}
