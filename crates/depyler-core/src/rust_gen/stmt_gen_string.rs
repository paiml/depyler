fn type_to_rust_string(ty: &Type) -> String {
    match ty {
        Type::Int => "i32".to_string(),
        Type::Float => "f64".to_string(),
        Type::String => "String".to_string(),
        Type::Bool => "bool".to_string(),
        Type::None => "()".to_string(),
        Type::List(inner) => format!("Vec<{}>", type_to_rust_string(inner)),
        Type::Optional(inner) => format!("Option<{}>", type_to_rust_string(inner)),
        Type::Tuple(types) => {
            let inner: Vec<_> = types.iter().map(type_to_rust_string).collect();
            format!("({})", inner.join(", "))
        }
        Type::Dict(k, v) => format!(
            "std::collections::HashMap<{}, {}>",
            type_to_rust_string(k),
            type_to_rust_string(v)
        ),
        Type::Set(inner) => format!("std::collections::HashSet<{}>", type_to_rust_string(inner)),
        Type::Custom(name) => name.clone(),
        Type::Generic { base, params } if !params.is_empty() => {
            let inner: Vec<_> = params.iter().map(type_to_rust_string).collect();
            format!("{}<{}>", base, inner.join(", "))
        }
        _ => "DepylerValue".to_string(), // Fallback for Unknown, etc.
    }
}

fn track_deque_constructor(
    var_name: &str,
    func: &str,
    type_annotation: &Option<Type>,
    ctx: &mut CodeGenContext,
) {
    if func == "deque" || func == "collections.deque" || func == "Deque" {
        let elem_type_str = get_deque_elem_type(type_annotation);
        ctx.var_types.insert(
            var_name.to_string(),
            Type::Custom(format!("std::collections::VecDeque<{}>", elem_type_str)),
        );
    } else if func == "Queue" || func == "LifoQueue" || func == "PriorityQueue" {
        ctx.var_types.insert(
            var_name.to_string(),
            Type::Custom("std::collections::VecDeque<i32>".to_string()),
        );
    } else if func == "heappush" || func == "heapify" {
        ctx.var_types.insert(
            var_name.to_string(),
            Type::Custom("std::collections::BinaryHeap<i32>".to_string()),
        );
    }
}

fn get_deque_elem_type(type_annotation: &Option<Type>) -> String {
    if let Some(Type::Generic { base, params }) = type_annotation {
        if (base == "deque" || base == "collections.deque" || base == "Deque") && !params.is_empty()
        {
            return type_to_rust_string(&params[0]);
        }
    }
    if let Some(Type::List(elem)) = type_annotation {
        return type_to_rust_string(elem);
    }
    "i32".to_string()
}
