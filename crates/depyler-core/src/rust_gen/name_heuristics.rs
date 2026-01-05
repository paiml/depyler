//! Name-based heuristics for type inference
//! DEPYLER-COVERAGE-95: Extracted pure functions for testability

/// Check if variable name suggests a string type (heuristic)
pub fn is_string_var_name(name: &str) -> bool {
    let is_singular = !name.ends_with('s');
    name == "text"
        || name == "s"
        || name == "string"
        || name == "line"
        || name == "content"
        || name == "timestamp"
        || name == "message"
        || name == "level"
        || name == "prefix"
        || name == "suffix"
        || name == "pattern"
        || name == "char"
        || name == "delimiter"
        || name == "separator"
        || name == "key"
        || name == "k"
        || name == "name"
        || name == "id"
        || name == "word"
        || (name == "word" && is_singular)
        || (name.starts_with("text") && is_singular)
        || (name.starts_with("str") && is_singular)
        || (name.ends_with("_str") && is_singular)
        || (name.ends_with("_string") && is_singular)
        || name.ends_with("_key")
        || name.ends_with("_name")
}

/// Check if variable name suggests a numeric index (heuristic)
pub fn is_numeric_index_name(name: &str) -> bool {
    name == "i"
        || name == "j"
        || name == "k"
        || name == "idx"
        || name == "index"
        || name.starts_with("idx_")
        || name.ends_with("_idx")
        || name.ends_with("_index")
}

/// Check if variable name suggests a list/collection type (heuristic)
pub fn is_list_var_name(name: &str) -> bool {
    name.ends_with('s') && name.len() > 1  // Plural names
        || name == "items"
        || name == "elements"
        || name == "data"
        || name == "results"
        || name == "values"
        || name == "entries"
        || name.ends_with("_list")
        || name.ends_with("_vec")
        || name.ends_with("_array")
}

/// Check if variable name suggests a dict/map type (heuristic)
pub fn is_dict_var_name(name: &str) -> bool {
    name == "dict"
        || name == "map"
        || name == "config"
        || name == "options"
        || name == "settings"
        || name == "params"
        || name == "kwargs"
        || name.ends_with("_dict")
        || name.ends_with("_map")
}

/// Check if variable name suggests a boolean type (heuristic)
pub fn is_bool_var_name(name: &str) -> bool {
    name.starts_with("is_")
        || name.starts_with("has_")
        || name.starts_with("can_")
        || name.starts_with("should_")
        || name.starts_with("will_")
        || name.starts_with("was_")
        || name.starts_with("did_")
        || name == "found"
        || name == "done"
        || name == "enabled"
        || name == "disabled"
        || name == "valid"
        || name == "success"
        || name == "ok"
        || name == "error"
        || name == "verbose"
        || name == "debug"
        || name == "quiet"
}

/// Check if variable name suggests a path type (heuristic)
pub fn is_path_var_name(name: &str) -> bool {
    name == "path"
        || name == "filepath"
        || name == "filename"
        || name == "dir"
        || name == "directory"
        || name == "folder"
        || name.ends_with("_path")
        || name.ends_with("_file")
        || name.ends_with("_dir")
        || name.starts_with("path_")
        || name.starts_with("file_")
}

#[cfg(test)]
mod tests {
    use super::*;

    // String name tests
    #[test]
    fn test_text_is_string() {
        assert!(is_string_var_name("text"));
    }

    #[test]
    fn test_message_is_string() {
        assert!(is_string_var_name("message"));
    }

    #[test]
    fn test_key_is_string() {
        assert!(is_string_var_name("key"));
    }

    #[test]
    fn test_user_key_is_string() {
        assert!(is_string_var_name("user_key"));
    }

    #[test]
    fn test_items_not_string() {
        assert!(!is_string_var_name("items"));
    }

    // Numeric index tests
    #[test]
    fn test_i_is_numeric() {
        assert!(is_numeric_index_name("i"));
    }

    #[test]
    fn test_idx_is_numeric() {
        assert!(is_numeric_index_name("idx"));
    }

    #[test]
    fn test_row_idx_is_numeric() {
        assert!(is_numeric_index_name("row_idx"));
    }

    #[test]
    fn test_text_not_numeric() {
        assert!(!is_numeric_index_name("text"));
    }

    // List name tests
    #[test]
    fn test_items_is_list() {
        assert!(is_list_var_name("items"));
    }

    #[test]
    fn test_words_is_list() {
        assert!(is_list_var_name("words"));
    }

    #[test]
    fn test_data_list_is_list() {
        assert!(is_list_var_name("data_list"));
    }

    #[test]
    fn test_word_not_list() {
        assert!(!is_list_var_name("word"));
    }

    // Dict name tests
    #[test]
    fn test_config_is_dict() {
        assert!(is_dict_var_name("config"));
    }

    #[test]
    fn test_user_dict_is_dict() {
        assert!(is_dict_var_name("user_dict"));
    }

    #[test]
    fn test_kwargs_is_dict() {
        assert!(is_dict_var_name("kwargs"));
    }

    // Bool name tests
    #[test]
    fn test_is_valid_is_bool() {
        assert!(is_bool_var_name("is_valid"));
    }

    #[test]
    fn test_has_error_is_bool() {
        assert!(is_bool_var_name("has_error"));
    }

    #[test]
    fn test_verbose_is_bool() {
        assert!(is_bool_var_name("verbose"));
    }

    #[test]
    fn test_count_not_bool() {
        assert!(!is_bool_var_name("count"));
    }

    // Path name tests
    #[test]
    fn test_path_is_path() {
        assert!(is_path_var_name("path"));
    }

    #[test]
    fn test_output_path_is_path() {
        assert!(is_path_var_name("output_path"));
    }

    #[test]
    fn test_filename_is_path() {
        assert!(is_path_var_name("filename"));
    }

    #[test]
    fn test_data_not_path() {
        assert!(!is_path_var_name("data"));
    }
}
