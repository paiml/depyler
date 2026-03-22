fn contains_string_concatenation(expr: &HirExpr) -> bool {
    match expr {
        // String concatenation: a + b (Add operator generates format!() for strings)
        HirExpr::Binary { op: BinOp::Add, .. } => {
            // Binary Add on strings generates format!() which returns String
            // We detect this by assuming any Add at top level is string concat
            // (numeric Add is handled differently in code generation)
            true
        }
        // F-strings generate format!() which returns String
        HirExpr::FString { .. } => true,
        // Recursive checks for nested expressions
        HirExpr::Binary { left, right, .. } => {
            contains_string_concatenation(left) || contains_string_concatenation(right)
        }
        HirExpr::Unary { operand, .. } => contains_string_concatenation(operand),
        HirExpr::IfExpr { body, orelse, .. } => {
            contains_string_concatenation(body) || contains_string_concatenation(orelse)
        }
        _ => false,
    }
}

pub(crate) fn function_returns_string_concatenation(func: &HirFunction) -> bool {
    for stmt in &func.body {
        if let HirStmt::Return(Some(expr)) = stmt {
            if contains_string_concatenation(expr) {
                return true;
            }
        }
    }
    false
}

pub(crate) fn return_type_expects_float(ty: &Type) -> bool {
    match ty {
        Type::Float => true,
        Type::Optional(inner) => return_type_expects_float(inner),
        Type::List(inner) => return_type_expects_float(inner),
        Type::Tuple(types) => types.iter().any(return_type_expects_float),
        _ => false,
    }
}

pub(crate) fn return_type_expects_int(ty: &Type) -> bool {
    match ty {
        Type::Int => true,
        Type::Optional(inner) => return_type_expects_int(inner),
        Type::List(inner) => return_type_expects_int(inner),
        Type::Tuple(types) => types.iter().any(return_type_expects_int),
        // DEPYLER-E0282-FIX: Handle Custom Result types
        Type::Custom(s) => {
            // Check for Result<i32, ...> or Result<int, ...> patterns
            s.contains("Result<i32") || s.contains("Result<int")
        }
        _ => false,
    }
}

pub(crate) fn rewrite_adt_child_type(
    ty: &Type,
    child_to_parent: &std::collections::HashMap<String, String>,
) -> Type {
    match ty {
        // Check if Custom type name is an ADT child - rewrite to parent
        Type::Custom(name) => {
            // Extract base name from generics (e.g., "ListIter" from "ListIter[T]")
            let base_name = name.split('[').next().unwrap_or(name);
            if let Some(parent_name) = child_to_parent.get(base_name) {
                // Preserve generic params: "ListIter[T]" → "Iter[T]"
                if let Some(generic_part) = name.strip_prefix(base_name) {
                    Type::Custom(format!("{}{}", parent_name, generic_part))
                } else {
                    Type::Custom(parent_name.clone())
                }
            } else {
                ty.clone()
            }
        }
        // DEPYLER-0936: Handle Generic type with base name that's an ADT child
        // Example: Generic { base: "ListIter", params: [T] } → Generic { base: "Iter", params: [T] }
        Type::Generic { base, params } => {
            if let Some(parent_name) = child_to_parent.get(base) {
                // Rewrite base to parent, keep params with recursive rewriting
                Type::Generic {
                    base: parent_name.clone(),
                    params: params
                        .iter()
                        .map(|t| rewrite_adt_child_type(t, child_to_parent))
                        .collect(),
                }
            } else {
                // No rewrite needed, but still recursively process params
                Type::Generic {
                    base: base.clone(),
                    params: params
                        .iter()
                        .map(|t| rewrite_adt_child_type(t, child_to_parent))
                        .collect(),
                }
            }
        }
        // Recursively handle container types
        Type::List(inner) => Type::List(Box::new(rewrite_adt_child_type(inner, child_to_parent))),
        Type::Optional(inner) => {
            Type::Optional(Box::new(rewrite_adt_child_type(inner, child_to_parent)))
        }
        Type::Tuple(types) => {
            Type::Tuple(types.iter().map(|t| rewrite_adt_child_type(t, child_to_parent)).collect())
        }
        Type::Dict(k, v) => Type::Dict(
            Box::new(rewrite_adt_child_type(k, child_to_parent)),
            Box::new(rewrite_adt_child_type(v, child_to_parent)),
        ),
        Type::Union(types) => {
            Type::Union(types.iter().map(|t| rewrite_adt_child_type(t, child_to_parent)).collect())
        }
        // Other types pass through unchanged
        _ => ty.clone(),
    }
}

    fn transpile(python_code: &str) -> String {
        use crate::ast_bridge::AstBridge;
        use crate::rust_gen::generate_rust_file;
        use crate::type_mapper::TypeMapper;
        use rustpython_parser::{parse, Mode};
        let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
        let (module, _) =
            AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
        let tm = TypeMapper::default();
        let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
        result
    }
