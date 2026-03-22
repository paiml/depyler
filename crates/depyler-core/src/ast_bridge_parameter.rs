fn infer_optional_type_from_name(name: &str) -> Type {
    let param_lower = name.to_lowercase();

    if is_file_path_param(&param_lower) {
        Type::String
    } else if is_string_like_param(&param_lower) {
        Type::String
    } else if is_numeric_param(&param_lower) {
        Type::Int
    } else if is_boolean_param(&param_lower) {
        Type::Bool
    } else {
        // DEPYLER-0744: Fallback to Unknown for return type unification
        Type::Unknown
    }
}

fn is_file_path_param(param_lower: &str) -> bool {
    param_lower.contains("file")
        || param_lower.contains("path")
        || param_lower.contains("output")
        || param_lower.contains("input")
        || param_lower.contains("dir")
        || param_lower.contains("folder")
}

fn is_string_like_param(param_lower: &str) -> bool {
    param_lower.contains("name")
        || param_lower.contains("title")
        || param_lower.contains("text")
        || param_lower.contains("message")
        || param_lower.contains("content")
        || param_lower.contains("label")
        || param_lower.contains("description")
        || param_lower.contains("prefix")
        || param_lower.contains("suffix")
        || param_lower.contains("format")
        || param_lower == "value"
        || param_lower.contains("column")
        || param_lower.contains("key") && !param_lower.contains("keys")
}

fn is_numeric_param(param_lower: &str) -> bool {
    param_lower.contains("count")
        || param_lower.contains("num")
        || param_lower.contains("index")
        || param_lower.contains("size")
        || param_lower.contains("limit")
        || param_lower.contains("max")
        || param_lower.contains("min")
        || param_lower.contains("port")
        || param_lower.contains("timeout")
        || param_lower.contains("depth")
        || param_lower == "n"
        || param_lower == "i"
        || param_lower == "j"
}

fn is_boolean_param(param_lower: &str) -> bool {
    param_lower.contains("flag")
        || param_lower.contains("enabled")
        || param_lower.contains("verbose")
        || param_lower.contains("debug")
        || param_lower.contains("quiet")
        || param_lower.contains("force")
        || param_lower.starts_with("is_")
        || param_lower.starts_with("has_")
        || param_lower.starts_with("use_")
        || param_lower.starts_with("allow_")
}
