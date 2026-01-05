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

/// Check if variable name suggests a float type (ML/scientific parameters)
/// DEPYLER-0582, DEPYLER-0694: Used for type coercion in binary operations
pub fn is_float_var_name(name: &str) -> bool {
    let name_lower = name.to_lowercase();

    // Common ML/scientific parameter names
    if name_lower.contains("beta")
        || name_lower.contains("alpha")
        || name_lower.contains("lr")
        || name_lower.contains("eps")
        || name_lower.contains("rate")
        || name_lower.contains("momentum")
        || name_lower.contains("gamma")
        || name_lower.contains("lambda")
        || name_lower.contains("sigma")
        || name_lower.contains("theta")
        || name_lower.contains("weight")
        || name_lower.contains("bias")
    {
        return true;
    }

    // Common float suffixes
    if name_lower.ends_with("_f")
        || name_lower.ends_with("_float")
        || name_lower.ends_with("_ratio")
        || name_lower.ends_with("_percent")
        || name_lower.ends_with("_prob")
        || name_lower.ends_with("_probability")
    {
        return true;
    }

    false
}

/// Check if single-letter variable name suggests a float (color channels)
/// DEPYLER-0950: Heuristic for colorsys color channel variables
/// Single-letter names from hsv_to_rgb(), rgb_to_hsv(), rgb_to_hls() etc.
///
/// Note: a, b, x, y are intentionally excluded (too generic, DEPYLER-0954)
pub fn is_color_channel_name(name: &str) -> bool {
    matches!(
        name,
        "r" | "g" | "h" | "s" | "v" | "l" | "c" | "m" | "k"
    )
}

/// Check if variable name suggests an integer count/size type
pub fn is_count_var_name(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    name_lower.contains("count")
        || name_lower.contains("num")
        || name_lower.contains("size")
        || name_lower.contains("length")
        || name_lower.contains("len")
        || name_lower.contains("offset")
        || name_lower.contains("position")
        || name_lower.ends_with("_n")
        || name_lower.ends_with("_i")
        || name_lower.ends_with("_int")
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

    // Float name tests
    #[test]
    fn test_beta_is_float() {
        assert!(is_float_var_name("beta"));
        assert!(is_float_var_name("beta1"));
        assert!(is_float_var_name("my_beta"));
    }

    #[test]
    fn test_alpha_is_float() {
        assert!(is_float_var_name("alpha"));
        assert!(is_float_var_name("alpha_value"));
    }

    #[test]
    fn test_lr_is_float() {
        assert!(is_float_var_name("lr"));
        assert!(is_float_var_name("learning_lr"));
    }

    #[test]
    fn test_eps_is_float() {
        assert!(is_float_var_name("eps"));
        assert!(is_float_var_name("epsilon"));
    }

    #[test]
    fn test_rate_is_float() {
        assert!(is_float_var_name("rate"));
        assert!(is_float_var_name("learning_rate"));
        assert!(is_float_var_name("drop_rate"));
    }

    #[test]
    fn test_momentum_is_float() {
        assert!(is_float_var_name("momentum"));
    }

    #[test]
    fn test_greek_letters_are_float() {
        assert!(is_float_var_name("gamma"));
        assert!(is_float_var_name("lambda_val"));
        assert!(is_float_var_name("sigma"));
        assert!(is_float_var_name("theta"));
    }

    #[test]
    fn test_ml_params_are_float() {
        assert!(is_float_var_name("weight"));
        assert!(is_float_var_name("bias"));
        assert!(is_float_var_name("hidden_weight"));
    }

    #[test]
    fn test_float_suffixes() {
        assert!(is_float_var_name("value_f"));
        assert!(is_float_var_name("x_float"));
        assert!(is_float_var_name("scale_ratio"));
        assert!(is_float_var_name("completion_percent"));
        assert!(is_float_var_name("success_prob"));
        assert!(is_float_var_name("event_probability"));
    }

    #[test]
    fn test_non_float_names() {
        assert!(!is_float_var_name("count"));
        assert!(!is_float_var_name("index"));
        assert!(!is_float_var_name("name"));
        assert!(!is_float_var_name("x"));
        assert!(!is_float_var_name("data"));
    }

    // Color channel name tests
    #[test]
    fn test_rgb_channels() {
        assert!(is_color_channel_name("r"));
        assert!(is_color_channel_name("g"));
    }

    #[test]
    fn test_hsv_channels() {
        assert!(is_color_channel_name("h"));
        assert!(is_color_channel_name("s"));
        assert!(is_color_channel_name("v"));
    }

    #[test]
    fn test_hsl_channel() {
        assert!(is_color_channel_name("l"));
    }

    #[test]
    fn test_cmyk_channels() {
        assert!(is_color_channel_name("c"));
        assert!(is_color_channel_name("m"));
        assert!(is_color_channel_name("k"));
    }

    #[test]
    fn test_excluded_generic_letters() {
        // These are too generic and excluded per DEPYLER-0954
        assert!(!is_color_channel_name("a"));
        assert!(!is_color_channel_name("b"));
        assert!(!is_color_channel_name("x"));
        assert!(!is_color_channel_name("y"));
    }

    #[test]
    fn test_multichar_not_color_channel() {
        // Only single letters match
        assert!(!is_color_channel_name("red"));
        assert!(!is_color_channel_name("hue"));
    }

    // Count var name tests
    #[test]
    fn test_count_is_count() {
        assert!(is_count_var_name("count"));
        assert!(is_count_var_name("word_count"));
        assert!(is_count_var_name("item_count"));
    }

    #[test]
    fn test_num_is_count() {
        assert!(is_count_var_name("num"));
        assert!(is_count_var_name("num_items"));
    }

    #[test]
    fn test_size_is_count() {
        assert!(is_count_var_name("size"));
        assert!(is_count_var_name("buffer_size"));
        assert!(is_count_var_name("length"));
        assert!(is_count_var_name("len"));
    }

    #[test]
    fn test_position_is_count() {
        assert!(is_count_var_name("offset"));
        assert!(is_count_var_name("position"));
    }

    #[test]
    fn test_count_suffixes() {
        assert!(is_count_var_name("total_n"));
        assert!(is_count_var_name("value_i"));
        assert!(is_count_var_name("x_int"));
    }

    #[test]
    fn test_non_count_names() {
        assert!(!is_count_var_name("beta"));
        assert!(!is_count_var_name("name"));
        assert!(!is_count_var_name("is_valid"));
        assert!(!is_count_var_name("data"));
    }
}
