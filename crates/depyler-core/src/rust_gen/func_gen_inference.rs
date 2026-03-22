fn infer_call_type_simple(func: &str) -> Type {
    match func {
        "json.load" | "json.loads" => Type::Custom("serde_json::Value".to_string()),
        "json.dump" => Type::None,
        "json.dumps" => Type::String,
        "csv.reader" => Type::List(Box::new(Type::List(Box::new(Type::String)))),
        "csv.writer" => Type::Unknown,
        "csv.DictReader" => {
            Type::List(Box::new(Type::Dict(Box::new(Type::String), Box::new(Type::String))))
        }
        "csv.DictWriter" => Type::Unknown,
        "len" | "int" | "abs" | "ord" | "hash" => Type::Int,
        "float" => Type::Float,
        "str" | "repr" | "chr" | "input" => Type::String,
        "bool" => Type::Bool,
        "list" => Type::List(Box::new(Type::Unknown)),
        "dict" => Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
        "set" => Type::Set(Box::new(Type::Unknown)),
        "tuple" => Type::Tuple(vec![]),
        "range" => Type::List(Box::new(Type::Int)),
        "sum" | "min" | "max" => Type::Int,
        "zeros" | "ones" | "full" => Type::List(Box::new(Type::Int)),
        "open" => Type::Custom("std::fs::File".to_string()),
        "Path" | "PurePath" | "PurePosixPath" | "PureWindowsPath" => {
            Type::Custom("PathBuf".to_string())
        }
        _ => Type::Unknown,
    }
}

fn infer_attribute_type_simple(value: &HirExpr, attr: &str) -> Type {
    // DEPYLER-0517: Check if this is an attribute access on a subprocess result
    if let HirExpr::MethodCall { object, method, .. } = value {
        if let HirExpr::Var(module) = object.as_ref() {
            if module == "subprocess" && method == "run" {
                return match attr {
                    "returncode" => Type::Int,
                    "stdout" | "stderr" => Type::String,
                    _ => Type::Unknown,
                };
            }
        }
    }
    // Common attributes with known types
    match attr {
        "real" | "imag" => Type::Float,
        "returncode" => Type::Int,
        "stdout" | "stderr" => Type::String,
        _ => Type::Unknown,
    }
}

pub(crate) fn infer_expr_type_simple(expr: &HirExpr) -> Type {
    match expr {
        HirExpr::Literal(lit) => literal_to_type(lit),
        HirExpr::Binary { op, left, right } => infer_binary_type_simple(op, left, right),
        HirExpr::Unary { op, operand } => {
            if matches!(op, UnaryOp::Not) {
                Type::Bool
            } else {
                infer_expr_type_simple(operand)
            }
        }
        HirExpr::List(elems) => {
            if elems.is_empty() {
                Type::List(Box::new(Type::Unknown))
            } else {
                Type::List(Box::new(infer_expr_type_simple(&elems[0])))
            }
        }
        HirExpr::Tuple(elems) => {
            let elem_types: Vec<Type> = elems.iter().map(infer_expr_type_simple).collect();
            Type::Tuple(elem_types)
        }
        HirExpr::Set(elems) => {
            if elems.is_empty() {
                Type::Set(Box::new(Type::Unknown))
            } else {
                Type::Set(Box::new(infer_expr_type_simple(&elems[0])))
            }
        }
        HirExpr::Dict(pairs) => {
            if pairs.is_empty() {
                Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))
            } else {
                let key_type = infer_expr_type_simple(&pairs[0].0);
                let first_val_type = infer_expr_type_simple(&pairs[0].1);
                let is_homogeneous = pairs.iter().skip(1).all(|(_, v)| {
                    let v_type = infer_expr_type_simple(v);
                    matches!(v_type, Type::Unknown) || v_type == first_val_type
                });
                let val_type = if is_homogeneous { first_val_type } else { Type::Unknown };
                Type::Dict(Box::new(key_type), Box::new(val_type))
            }
        }
        HirExpr::IfExpr { body, orelse, .. } => {
            let body_type = infer_expr_type_simple(body);
            if !matches!(body_type, Type::Unknown) { body_type } else { infer_expr_type_simple(orelse) }
        }
        HirExpr::Index { base, .. } => match infer_expr_type_simple(base) {
            Type::List(elem) => *elem,
            Type::Tuple(elems) => elems.first().cloned().unwrap_or(Type::Unknown),
            Type::Dict(_, val) => *val,
            Type::String => Type::String,
            _ => Type::Int,
        },
        HirExpr::Slice { base, .. } => infer_expr_type_simple(base),
        HirExpr::FString { .. } => Type::String,
        HirExpr::Call { func, .. } => infer_call_type_simple(func),
        HirExpr::MethodCall { object, method, .. } => infer_method_call_type_simple(object, method),
        HirExpr::ListComp { element, .. } => Type::List(Box::new(infer_expr_type_simple(element))),
        HirExpr::SetComp { element, .. } => Type::Set(Box::new(infer_expr_type_simple(element))),
        HirExpr::DictComp { key, value, .. } => Type::Dict(
            Box::new(infer_expr_type_simple(key)),
            Box::new(infer_expr_type_simple(value)),
        ),
        HirExpr::Attribute { value, attr } => infer_attribute_type_simple(value, attr),
        _ => Type::Unknown,
    }
}

fn literal_to_type(lit: &Literal) -> Type {
    match lit {
        Literal::Int(_) => Type::Int,
        Literal::Float(_) => Type::Float,
        Literal::String(_) => Type::String,
        Literal::Bool(_) => Type::Bool,
        Literal::None => Type::None,
        Literal::Bytes(_) => Type::Unknown, // No direct Bytes type in Type enum
    }
}
