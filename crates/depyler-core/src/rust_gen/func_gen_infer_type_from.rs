fn infer_type_from_object_method(param_name: &str, object: &HirExpr, method: &str) -> Option<Type> {
    // DEPYLER-0525: File I/O methods → File type
    let file_object_methods = [
        "write", "writelines", "read", "readline", "readlines",
        "flush", "close", "seek", "tell", "truncate",
    ];
    if let HirExpr::Var(var_name) = object {
        if var_name == param_name && file_object_methods.contains(&method) {
            return Some(Type::Custom("File".to_string()));
        }
    }

    // DEPYLER-0524: String methods → String type
    let string_object_methods = [
        "strip", "lstrip", "rstrip", "startswith", "endswith", "split", "splitlines",
        "join", "upper", "lower", "title", "capitalize", "replace", "find", "rfind",
        "index", "rindex", "count", "isalpha", "isdigit", "isalnum", "isspace",
        "isupper", "islower", "encode", "format", "center", "ljust", "rjust", "zfill",
        "partition", "rpartition", "expandtabs", "swapcase", "casefold",
    ];
    if let HirExpr::Var(var_name) = object {
        if var_name == param_name && string_object_methods.contains(&method) {
            return Some(Type::String);
        }
    }

    // DEPYLER-0550: Dict methods → Dict type
    let dict_object_methods = [
        "get", "items", "keys", "values", "pop", "popitem",
        "update", "setdefault", "clear", "copy",
    ];
    if let HirExpr::Var(var_name) = object {
        if var_name == param_name && dict_object_methods.contains(&method) {
            return Some(Type::Dict(Box::new(Type::String), Box::new(Type::String)));
        }
    }

    None
}

fn infer_type_from_module_method(
    param_name: &str,
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
    kwargs: &[(String, HirExpr)],
) -> Option<Type> {
    // DEPYLER-0518: Regex module method calls
    if let HirExpr::Var(module_name) = object {
        let regex_modules = ["re", "regex"];
        let regex_methods = ["match", "search", "findall", "sub", "subn", "split", "compile"];

        if regex_modules.contains(&module_name.as_str())
            && regex_methods.contains(&method)
        {
            for arg in args.iter().take(2) {
                if let HirExpr::Var(var_name) = arg {
                    if var_name == param_name {
                        return Some(Type::String);
                    }
                }
            }
        }

        // DEPYLER-0554: datetime.datetime.fromtimestamp(param) → param is f64
        if module_name == "datetime" && method == "fromtimestamp" {
            if let Some(HirExpr::Var(var_name)) = args.first() {
                if var_name == param_name {
                    return Some(Type::Float);
                }
            }
        }
    }

    // DEPYLER-0554: Handle datetime.datetime attribute access → fromtimestamp method
    if let HirExpr::Attribute { value, attr } = object {
        if let HirExpr::Var(module_name) = value.as_ref() {
            if module_name == "datetime" && attr == "datetime" && method == "fromtimestamp" {
                if let Some(HirExpr::Var(var_name)) = args.first() {
                    if var_name == param_name {
                        return Some(Type::Float);
                    }
                }
            }
        }
    }

    // DEPYLER-0737: subprocess.run(..., cwd=param) → param is String (path-like)
    if let HirExpr::Var(module_name) = object {
        if module_name == "subprocess" && method == "run" {
            for (kwarg_name, kwarg_value) in kwargs {
                if kwarg_name == "cwd" {
                    if let HirExpr::Var(var_name) = kwarg_value {
                        if var_name == param_name {
                            return Some(Type::String);
                        }
                    }
                }
            }
        }
    }

    None
}

fn infer_type_from_method_call_usage(
    param_name: &str,
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
    kwargs: &[(String, HirExpr)],
) -> Option<Type> {
    // Check object method dispatch (file, string, dict)
    if let Some(ty) = infer_type_from_object_method(param_name, object, method) {
        return Some(ty);
    }

    // Check module method calls (regex, datetime, subprocess)
    if let Some(ty) = infer_type_from_module_method(param_name, object, method, args, kwargs) {
        return Some(ty);
    }

    // Methods that expect string arguments (for method calls on objects)
    let string_methods = [
        "find", "search", "match", "sub", "replace", "replace_all",
        "is_match", "captures", "find_iter", "split", "strip", "lstrip",
        "rstrip", "startswith", "endswith", "contains", "encode", "decode",
    ];
    if string_methods.contains(&method) {
        for arg in args {
            if let HirExpr::Var(var_name) = arg {
                if var_name == param_name {
                    return Some(Type::String);
                }
            }
        }
    }

    // Recursively check arguments
    for arg in args {
        if let Some(ty) = infer_type_from_expr_usage(param_name, arg) {
            return Some(ty);
        }
    }
    // Also recursively check kwargs values
    for (_, kwarg_value) in kwargs {
        if let Some(ty) = infer_type_from_expr_usage(param_name, kwarg_value) {
            return Some(ty);
        }
    }
    // Also check the object expression
    infer_type_from_expr_usage(param_name, object)
}
