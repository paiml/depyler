fn infer_collection_element_type(elems: &[HirExpr]) -> Type {
    infer_collection_element_type_with_ctx(elems, &HashMap::new())
}

fn infer_collection_element_type_with_ctx(
    elems: &[HirExpr],
    var_types: &HashMap<String, Type>,
) -> Type {
    if elems.is_empty() {
        // Empty collection defaults to Unknown (will use DepylerValue)
        return Type::Unknown;
    }

    // Helper to get element type from expression (literal, variable, or nested collection)
    fn get_elem_type_recursive(e: &HirExpr, var_types: &HashMap<String, Type>) -> Option<Type> {
        match e {
            HirExpr::Literal(Literal::Int(_)) => Some(Type::Int),
            HirExpr::Literal(Literal::Float(_)) => Some(Type::Float),
            HirExpr::Literal(Literal::String(_)) => Some(Type::String),
            HirExpr::Literal(Literal::Bool(_)) => Some(Type::Bool),
            // DEPYLER-0467: Look up variable types
            HirExpr::Var(name) => var_types.get(name).cloned(),
            // DEPYLER-1313: Handle nested lists - recursively infer inner element type
            HirExpr::List(inner_elems) => {
                let inner_type = infer_collection_element_type_with_ctx(inner_elems, var_types);
                Some(Type::List(Box::new(inner_type)))
            }
            // DEPYLER-1313: Handle tuples - infer element types
            HirExpr::Tuple(inner_elems) => {
                let tuple_types: Vec<Type> = inner_elems
                    .iter()
                    .map(|elem| get_elem_type_recursive(elem, var_types).unwrap_or(Type::Unknown))
                    .collect();
                Some(Type::Tuple(tuple_types))
            }
            // DEPYLER-1313: Handle dicts - infer key/value types
            HirExpr::Dict(items) => {
                if items.is_empty() {
                    Some(Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)))
                } else {
                    let key_type =
                        get_elem_type_recursive(&items[0].0, var_types).unwrap_or(Type::Unknown);
                    let val_type =
                        get_elem_type_recursive(&items[0].1, var_types).unwrap_or(Type::Unknown);
                    Some(Type::Dict(Box::new(key_type), Box::new(val_type)))
                }
            }
            _ => None,
        }
    }

    // Use the recursive helper
    let get_elem_type = |e: &HirExpr| -> Option<Type> { get_elem_type_recursive(e, var_types) };

    // Get type of first element
    let first_type = match get_elem_type(&elems[0]) {
        Some(t) => t,
        None => return Type::Unknown,
    };

    // Verify all elements have the same type
    let all_same = elems.iter().all(|e| {
        match get_elem_type(e) {
            Some(t) => {
                // Int and Float can coexist - promote to Float
                match (&first_type, &t) {
                    (Type::Float, Type::Int) | (Type::Int, Type::Float) => true,
                    _ => t == first_type,
                }
            }
            None => false, // Unknown element type breaks homogeneity
        }
    });

    if all_same {
        // Check if we need to promote int to float
        let has_float = elems.iter().any(|e| matches!(get_elem_type(e), Some(Type::Float)));
        if has_float && matches!(first_type, Type::Int) {
            Type::Float
        } else {
            first_type
        }
    } else {
        Type::Unknown // Heterogeneous
    }
}
