//! DEPYLER-1024: Truthiness conversion helpers
//!
//! Extracted from stmt_gen.rs to reduce complexity and improve testability.
//! These helpers handle Python truthiness semantics conversion to Rust.

/// Collection variable names that indicate container types needing `.is_empty()` check.
/// These are common names that almost always refer to collections in Python code.
pub const COLLECTION_VAR_NAMES: &[&str] = &[
    "queue",
    "stack",
    "heap",
    "items",
    "elements",
    "nodes",
    "visited",
    "seen",
    "pending",
    "worklist",
    "buffer",
    "entries",
    "results",
    "values",
    "keys",
    "children",
    "neighbors",
    "matches", // DEPYLER-1079: Common name for filtered collections
];

/// Collection variable suffixes that indicate container types.
pub const COLLECTION_VAR_SUFFIXES: &[&str] =
    &["_queue", "_stack", "_heap", "_list", "_items", "_set"];

/// String variable names from tuple unpacking that need `.is_empty()` check.
/// e.g., `let (returncode, stdout, stderr) = run_command(...)`
pub const STRING_VAR_NAMES: &[&str] = &["stdout", "stderr", "output"];

/// String variable suffixes that indicate string types.
pub const STRING_VAR_SUFFIXES: &[&str] = &["_output", "_result", "_str", "_string"];

/// Common string attribute/field names used in classes and structs.
/// These are typically String fields that need `!.is_empty()` for truthiness.
#[allow(dead_code)]
pub const STRING_ATTR_NAMES: &[&str] = &[
    "email",
    "name",
    "text",
    "content",
    "message",
    "title",
    "description",
    "path",
    "url",
    "value",
    "data",
    "body",
    "subject",
    "address",
    "filename",
    "username",
    "password",
    "token",
    "key",
    "secret",
    "label",
    "output",
    "input",
    "stdout",
    "stderr",
    "error",
    "warning",
    "info",
    "debug",
];

/// Dict-like variable names that indicate HashMap/dict access.
pub const DICT_VAR_NAMES: &[&str] = &["info", "data", "config", "options", "result", "response"];

/// Dict-like variable suffixes.
pub const DICT_VAR_SUFFIXES: &[&str] = &["_info", "_data", "_dict"];

/// DEPYLER-1071: Option variable names that indicate regex match results or optional values.
/// These need `.is_some()` for truthiness check.
pub const OPTION_VAR_NAMES: &[&str] = &["m", "match_", "match_result", "match_obj", "found", "hit"];

/// DEPYLER-1071: Option variable suffixes that indicate optional types.
pub const OPTION_VAR_SUFFIXES: &[&str] = &["_match", "_result", "_found"];

/// DEPYLER-1071: Check if a variable name suggests it's an Option type (regex match result).
#[inline]
pub fn is_option_var_name(name: &str) -> bool {
    OPTION_VAR_NAMES.contains(&name) || OPTION_VAR_SUFFIXES.iter().any(|s| name.ends_with(s))
}

/// Collection type names that need `.is_empty()` truthiness check.
pub const COLLECTION_TYPE_NAMES: &[&str] = &[
    "VecDeque",
    "std::collections::VecDeque",
    "BinaryHeap",
    "std::collections::BinaryHeap",
    "LinkedList",
    "std::collections::LinkedList",
    "BTreeSet",
    "BTreeMap",
    "HashSet",
    "HashMap",
];

/// Collection type name fragments to match.
pub const COLLECTION_TYPE_FRAGMENTS: &[&str] = &["Deque", "Queue", "Stack", "Heap"];

/// Check if a variable name suggests it's a collection type.
#[inline]
pub fn is_collection_var_name(name: &str) -> bool {
    COLLECTION_VAR_NAMES.contains(&name)
        || COLLECTION_VAR_SUFFIXES.iter().any(|s| name.ends_with(s))
}

/// Check if a variable name suggests it's a string type.
#[inline]
pub fn is_string_var_name(name: &str) -> bool {
    STRING_VAR_NAMES.contains(&name) || STRING_VAR_SUFFIXES.iter().any(|s| name.ends_with(s))
}

/// Check if an attribute name suggests it's a string field.
#[inline]
#[allow(dead_code)]
pub fn is_string_attr_name(name: &str) -> bool {
    STRING_ATTR_NAMES.contains(&name)
}

/// Check if a variable name suggests it's a dict-like type.
#[inline]
pub fn is_dict_var_name(name: &str) -> bool {
    DICT_VAR_NAMES.contains(&name) || DICT_VAR_SUFFIXES.iter().any(|s| name.ends_with(s))
}

/// Check if a type name is a known collection type.
#[inline]
pub fn is_collection_type_name(type_name: &str) -> bool {
    COLLECTION_TYPE_NAMES
        .iter()
        .any(|n| type_name.starts_with(n))
        || COLLECTION_TYPE_FRAGMENTS
            .iter()
            .any(|f| type_name.contains(f))
}

/// Check if a generic base name is a collection type.
#[inline]
pub fn is_collection_generic_base(base: &str) -> bool {
    matches!(
        base,
        "VecDeque"
            | "BinaryHeap"
            | "LinkedList"
            | "BTreeSet"
            | "BTreeMap"
            | "HashSet"
            | "HashMap"
            | "Vec"
    )
}

/// Collection attribute/field names used in classes and structs.
/// These are typically Vec/HashMap/etc fields that need `.is_empty()` for truthiness.
/// DEPYLER-COVERAGE-95: Extracted from stmt_gen::apply_negated_truthiness
pub const COLLECTION_ATTR_NAMES: &[&str] = &[
    "heap", "stack", "queue", "items", "elements", "data", "values", "list", "array", "nodes",
    "children", "entries", "records",
];

/// Check if an attribute name suggests it's a collection field.
#[inline]
pub fn is_collection_attr_name(name: &str) -> bool {
    COLLECTION_ATTR_NAMES.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_var_names() {
        assert!(is_collection_var_name("queue"));
        assert!(is_collection_var_name("stack"));
        assert!(is_collection_var_name("task_queue"));
        assert!(is_collection_var_name("node_list"));
        assert!(!is_collection_var_name("result"));
        assert!(!is_collection_var_name("x"));
    }

    #[test]
    fn test_string_var_names() {
        assert!(is_string_var_name("stdout"));
        assert!(is_string_var_name("stderr"));
        assert!(is_string_var_name("command_output"));
        assert!(is_string_var_name("name_str"));
        assert!(!is_string_var_name("result"));
        assert!(!is_string_var_name("x"));
    }

    #[test]
    fn test_string_attr_names() {
        assert!(is_string_attr_name("email"));
        assert!(is_string_attr_name("username"));
        assert!(is_string_attr_name("path"));
        assert!(!is_string_attr_name("x"));
        assert!(!is_string_attr_name("count"));
    }

    #[test]
    fn test_dict_var_names() {
        assert!(is_dict_var_name("info"));
        assert!(is_dict_var_name("config"));
        assert!(is_dict_var_name("result")); // "result" IS a dict var name per original code
        assert!(is_dict_var_name("response"));
        assert!(is_dict_var_name("user_data"));
        assert!(is_dict_var_name("cache_dict"));
        assert!(!is_dict_var_name("x"));
        assert!(!is_dict_var_name("count"));
    }

    #[test]
    fn test_collection_type_names() {
        assert!(is_collection_type_name("VecDeque<i32>"));
        assert!(is_collection_type_name("std::collections::VecDeque"));
        assert!(is_collection_type_name("BinaryHeap<Task>"));
        assert!(is_collection_type_name("MyDeque"));
        assert!(is_collection_type_name("TaskQueue"));
        assert!(!is_collection_type_name("String"));
        assert!(!is_collection_type_name("i32"));
    }

    #[test]
    fn test_collection_generic_base() {
        assert!(is_collection_generic_base("VecDeque"));
        assert!(is_collection_generic_base("BinaryHeap"));
        assert!(is_collection_generic_base("Vec"));
        assert!(is_collection_generic_base("HashMap"));
        assert!(!is_collection_generic_base("String"));
        assert!(!is_collection_generic_base("Option"));
    }

    #[test]
    fn test_all_collection_var_names_covered() {
        for name in COLLECTION_VAR_NAMES {
            assert!(is_collection_var_name(name), "Missing: {}", name);
        }
    }

    #[test]
    fn test_all_string_attr_names_covered() {
        for name in STRING_ATTR_NAMES {
            assert!(is_string_attr_name(name), "Missing: {}", name);
        }
    }

    #[test]
    fn test_suffix_patterns() {
        // Test that suffix patterns work
        assert!(is_collection_var_name("my_queue"));
        assert!(is_collection_var_name("data_list"));
        assert!(is_collection_var_name("item_set"));
        assert!(is_string_var_name("cmd_output"));
        assert!(is_string_var_name("value_string"));
    }

    #[test]
    fn test_collection_attr_names() {
        // DEPYLER-COVERAGE-95: Test collection attribute names
        assert!(is_collection_attr_name("heap"));
        assert!(is_collection_attr_name("stack"));
        assert!(is_collection_attr_name("queue"));
        assert!(is_collection_attr_name("items"));
        assert!(is_collection_attr_name("data"));
        assert!(is_collection_attr_name("list"));
        assert!(is_collection_attr_name("nodes"));
        assert!(is_collection_attr_name("children"));
        assert!(!is_collection_attr_name("name"));
        assert!(!is_collection_attr_name("email"));
        assert!(!is_collection_attr_name("x"));
    }

    #[test]
    fn test_all_collection_attr_names_covered() {
        for name in COLLECTION_ATTR_NAMES {
            assert!(is_collection_attr_name(name), "Missing: {}", name);
        }
    }

    #[test]
    fn test_option_var_names() {
        // DEPYLER-1071: Test Option variable names for regex match results
        assert!(is_option_var_name("m"));
        assert!(is_option_var_name("match_"));
        assert!(is_option_var_name("match_result"));
        assert!(is_option_var_name("match_obj"));
        assert!(is_option_var_name("found"));
        assert!(is_option_var_name("hit"));
        // Suffix patterns
        assert!(is_option_var_name("regex_match"));
        assert!(is_option_var_name("pattern_result"));
        assert!(is_option_var_name("is_found"));
        // Negative cases
        assert!(!is_option_var_name("x"));
        assert!(!is_option_var_name("text"));
        assert!(!is_option_var_name("pattern"));
    }

    #[test]
    fn test_all_option_var_names_covered() {
        for name in OPTION_VAR_NAMES {
            assert!(is_option_var_name(name), "Missing: {}", name);
        }
    }
}
