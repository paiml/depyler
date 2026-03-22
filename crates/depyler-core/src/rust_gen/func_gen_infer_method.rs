fn infer_method_call_type_simple(object: &HirExpr, method: &str) -> Type {
    // DEPYLER-0931: Check if this is subprocess module call
    if let HirExpr::Var(obj_var) = object {
        if obj_var == "subprocess" {
            return match method {
                "Popen" => Type::Custom("std::process::Child".to_string()),
                "run" => Type::Custom("CompletedProcess".to_string()),
                "call" | "check_call" => Type::Int,
                "check_output" => Type::String,
                _ => Type::Unknown,
            };
        }
    }
    // DEPYLER-0931: Check if object is a subprocess Child
    if method == "wait" {
        let obj_type = infer_expr_type_simple(object);
        if matches!(&obj_type, Type::Custom(s) if s.contains("Child")) {
            return Type::Int;
        }
    }
    infer_method_return_type(object, method)
}

fn infer_method_return_type(object: &HirExpr, method: &str) -> Type {
    match method {
        "copy" => infer_expr_type_simple(object),
        "wait" => Type::Int,
        "poll" => Type::Optional(Box::new(Type::Int)),
        "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "replace" | "title"
        | "capitalize" | "join" | "format" => Type::String,
        "startswith" | "endswith" | "isdigit" | "isalpha" | "isalnum" | "isspace"
        | "isupper" | "islower" => Type::Bool,
        "find" | "rfind" | "index" | "rindex" | "count" => Type::Int,
        "split" | "splitlines" => Type::List(Box::new(Type::String)),
        "read" | "readline" => Type::String,
        "readlines" => Type::List(Box::new(Type::String)),
        "get" => match infer_expr_type_simple(object) {
            Type::Dict(_, val) => *val,
            Type::List(elem) => *elem,
            _ => Type::Unknown,
        },
        "pop" => match infer_expr_type_simple(object) {
            Type::List(elem) => *elem,
            Type::Dict(_, val) => *val,
            _ => Type::Unknown,
        },
        "keys" => Type::List(Box::new(Type::Unknown)),
        "values" => Type::List(Box::new(Type::Unknown)),
        "items" => Type::List(Box::new(Type::Tuple(vec![Type::Unknown, Type::Unknown]))),
        "loads" | "load" => {
            if let HirExpr::Var(obj_var) = object {
                if obj_var == "json" {
                    return Type::Custom("serde_json::Value".to_string());
                }
            }
            Type::Unknown
        }
        _ => Type::Unknown,
    }
}
