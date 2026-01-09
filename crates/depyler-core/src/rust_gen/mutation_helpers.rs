//! Mutation analysis helpers for code generation
//!
//! This module provides utilities to identify mutating methods and attributes
//! in Python code to properly generate Rust code with correct mutability.

/// Check if a method call is a mutating operation that requires `&mut self`
///
/// Returns `true` if the method modifies the object it's called on.
pub fn is_mutating_method(method: &str) -> bool {
    matches!(
        method,
        // List methods
        "append" | "extend" | "insert" | "remove" | "pop" | "clear" | "reverse" | "sort" |
        // Dict methods
        "update" | "setdefault" | "popitem" |
        // Set methods
        "add" | "discard" | "difference_update" | "intersection_update" |
        // DEPYLER-0529: File I/O methods that require mutable access
        "write" | "write_all" | "writelines" | "flush" | "seek" | "truncate" |
        // DEPYLER-0549: CSV reader/writer methods that require mutable access
        // csv::Reader requires &mut self for headers(), records(), deserialize()
        // csv::Writer requires &mut self for write_record(), serialize()
        "headers" | "records" | "deserialize" | "serialize" | "write_record" |
        "writeheader" | "writerow" |
        // DEPYLER-1002: Hashlib digest methods that require &mut self for finalize_reset()
        "hexdigest" | "digest" | "finalize" | "finalize_reset"
    )
}

/// DEPYLER-0835: Python attributes that translate to mutating method calls in Rust
pub fn is_mutating_attribute(attr: &str) -> bool {
    matches!(
        attr,
        // csv.DictReader.fieldnames -> reader.headers() requires &mut self
        "fieldnames"
    )
}

/// Check if a method is a non-mutating list method (returns new value)
pub fn is_non_mutating_list_method(method: &str) -> bool {
    matches!(
        method,
        "copy" | "count" | "index" | "__len__" | "__contains__" | "__getitem__"
    )
}

/// Check if a method is a non-mutating dict method
pub fn is_non_mutating_dict_method(method: &str) -> bool {
    matches!(
        method,
        "get" | "keys" | "values" | "items" | "copy" | "__len__" | "__contains__" | "__getitem__"
    )
}

/// Check if a method is a non-mutating set method
pub fn is_non_mutating_set_method(method: &str) -> bool {
    matches!(
        method,
        "copy" | "difference" | "intersection" | "union" | "symmetric_difference" |
        "issubset" | "issuperset" | "isdisjoint" | "__len__" | "__contains__"
    )
}

/// Check if a method is a file read operation (non-mutating in Python sense)
pub fn is_file_read_method(method: &str) -> bool {
    matches!(method, "read" | "readline" | "readlines" | "read_text" | "read_bytes")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ is_mutating_method tests ============

    #[test]
    fn test_is_mutating_method_list_append() {
        assert!(is_mutating_method("append"));
    }

    #[test]
    fn test_is_mutating_method_list_extend() {
        assert!(is_mutating_method("extend"));
    }

    #[test]
    fn test_is_mutating_method_list_insert() {
        assert!(is_mutating_method("insert"));
    }

    #[test]
    fn test_is_mutating_method_list_remove() {
        assert!(is_mutating_method("remove"));
    }

    #[test]
    fn test_is_mutating_method_list_pop() {
        assert!(is_mutating_method("pop"));
    }

    #[test]
    fn test_is_mutating_method_list_clear() {
        assert!(is_mutating_method("clear"));
    }

    #[test]
    fn test_is_mutating_method_list_reverse() {
        assert!(is_mutating_method("reverse"));
    }

    #[test]
    fn test_is_mutating_method_list_sort() {
        assert!(is_mutating_method("sort"));
    }

    #[test]
    fn test_is_mutating_method_dict_update() {
        assert!(is_mutating_method("update"));
    }

    #[test]
    fn test_is_mutating_method_dict_setdefault() {
        assert!(is_mutating_method("setdefault"));
    }

    #[test]
    fn test_is_mutating_method_dict_popitem() {
        assert!(is_mutating_method("popitem"));
    }

    #[test]
    fn test_is_mutating_method_set_add() {
        assert!(is_mutating_method("add"));
    }

    #[test]
    fn test_is_mutating_method_set_discard() {
        assert!(is_mutating_method("discard"));
    }

    #[test]
    fn test_is_mutating_method_set_difference_update() {
        assert!(is_mutating_method("difference_update"));
    }

    #[test]
    fn test_is_mutating_method_set_intersection_update() {
        assert!(is_mutating_method("intersection_update"));
    }

    #[test]
    fn test_is_mutating_method_file_write() {
        assert!(is_mutating_method("write"));
    }

    #[test]
    fn test_is_mutating_method_file_write_all() {
        assert!(is_mutating_method("write_all"));
    }

    #[test]
    fn test_is_mutating_method_file_writelines() {
        assert!(is_mutating_method("writelines"));
    }

    #[test]
    fn test_is_mutating_method_file_flush() {
        assert!(is_mutating_method("flush"));
    }

    #[test]
    fn test_is_mutating_method_file_seek() {
        assert!(is_mutating_method("seek"));
    }

    #[test]
    fn test_is_mutating_method_file_truncate() {
        assert!(is_mutating_method("truncate"));
    }

    #[test]
    fn test_is_mutating_method_csv_headers() {
        assert!(is_mutating_method("headers"));
    }

    #[test]
    fn test_is_mutating_method_csv_records() {
        assert!(is_mutating_method("records"));
    }

    #[test]
    fn test_is_mutating_method_csv_deserialize() {
        assert!(is_mutating_method("deserialize"));
    }

    #[test]
    fn test_is_mutating_method_csv_serialize() {
        assert!(is_mutating_method("serialize"));
    }

    #[test]
    fn test_is_mutating_method_csv_write_record() {
        assert!(is_mutating_method("write_record"));
    }

    #[test]
    fn test_is_mutating_method_csv_writeheader() {
        assert!(is_mutating_method("writeheader"));
    }

    #[test]
    fn test_is_mutating_method_csv_writerow() {
        assert!(is_mutating_method("writerow"));
    }

    #[test]
    fn test_is_mutating_method_non_mutating() {
        assert!(!is_mutating_method("get"));
        assert!(!is_mutating_method("len"));
        assert!(!is_mutating_method("keys"));
        assert!(!is_mutating_method("values"));
        assert!(!is_mutating_method("items"));
        assert!(!is_mutating_method("copy"));
        assert!(!is_mutating_method("count"));
        assert!(!is_mutating_method("index"));
        assert!(!is_mutating_method("read"));
        assert!(!is_mutating_method("custom_method"));
    }

    // ============ is_mutating_attribute tests ============

    #[test]
    fn test_is_mutating_attribute_fieldnames() {
        assert!(is_mutating_attribute("fieldnames"));
    }

    #[test]
    fn test_is_mutating_attribute_non_mutating() {
        assert!(!is_mutating_attribute("value"));
        assert!(!is_mutating_attribute("name"));
        assert!(!is_mutating_attribute("data"));
        assert!(!is_mutating_attribute("result"));
        assert!(!is_mutating_attribute("custom_attr"));
    }

    // ============ is_non_mutating_list_method tests ============

    #[test]
    fn test_is_non_mutating_list_method_copy() {
        assert!(is_non_mutating_list_method("copy"));
    }

    #[test]
    fn test_is_non_mutating_list_method_count() {
        assert!(is_non_mutating_list_method("count"));
    }

    #[test]
    fn test_is_non_mutating_list_method_index() {
        assert!(is_non_mutating_list_method("index"));
    }

    #[test]
    fn test_is_non_mutating_list_method_len() {
        assert!(is_non_mutating_list_method("__len__"));
    }

    #[test]
    fn test_is_non_mutating_list_method_contains() {
        assert!(is_non_mutating_list_method("__contains__"));
    }

    #[test]
    fn test_is_non_mutating_list_method_getitem() {
        assert!(is_non_mutating_list_method("__getitem__"));
    }

    #[test]
    fn test_is_non_mutating_list_method_mutating() {
        assert!(!is_non_mutating_list_method("append"));
        assert!(!is_non_mutating_list_method("pop"));
        assert!(!is_non_mutating_list_method("sort"));
    }

    // ============ is_non_mutating_dict_method tests ============

    #[test]
    fn test_is_non_mutating_dict_method_get() {
        assert!(is_non_mutating_dict_method("get"));
    }

    #[test]
    fn test_is_non_mutating_dict_method_keys() {
        assert!(is_non_mutating_dict_method("keys"));
    }

    #[test]
    fn test_is_non_mutating_dict_method_values() {
        assert!(is_non_mutating_dict_method("values"));
    }

    #[test]
    fn test_is_non_mutating_dict_method_items() {
        assert!(is_non_mutating_dict_method("items"));
    }

    #[test]
    fn test_is_non_mutating_dict_method_copy() {
        assert!(is_non_mutating_dict_method("copy"));
    }

    #[test]
    fn test_is_non_mutating_dict_method_mutating() {
        assert!(!is_non_mutating_dict_method("update"));
        assert!(!is_non_mutating_dict_method("setdefault"));
        assert!(!is_non_mutating_dict_method("popitem"));
    }

    // ============ is_non_mutating_set_method tests ============

    #[test]
    fn test_is_non_mutating_set_method_copy() {
        assert!(is_non_mutating_set_method("copy"));
    }

    #[test]
    fn test_is_non_mutating_set_method_difference() {
        assert!(is_non_mutating_set_method("difference"));
    }

    #[test]
    fn test_is_non_mutating_set_method_intersection() {
        assert!(is_non_mutating_set_method("intersection"));
    }

    #[test]
    fn test_is_non_mutating_set_method_union() {
        assert!(is_non_mutating_set_method("union"));
    }

    #[test]
    fn test_is_non_mutating_set_method_symmetric_difference() {
        assert!(is_non_mutating_set_method("symmetric_difference"));
    }

    #[test]
    fn test_is_non_mutating_set_method_issubset() {
        assert!(is_non_mutating_set_method("issubset"));
    }

    #[test]
    fn test_is_non_mutating_set_method_issuperset() {
        assert!(is_non_mutating_set_method("issuperset"));
    }

    #[test]
    fn test_is_non_mutating_set_method_isdisjoint() {
        assert!(is_non_mutating_set_method("isdisjoint"));
    }

    #[test]
    fn test_is_non_mutating_set_method_mutating() {
        assert!(!is_non_mutating_set_method("add"));
        assert!(!is_non_mutating_set_method("discard"));
        assert!(!is_non_mutating_set_method("difference_update"));
    }

    // ============ is_file_read_method tests ============

    #[test]
    fn test_is_file_read_method_read() {
        assert!(is_file_read_method("read"));
    }

    #[test]
    fn test_is_file_read_method_readline() {
        assert!(is_file_read_method("readline"));
    }

    #[test]
    fn test_is_file_read_method_readlines() {
        assert!(is_file_read_method("readlines"));
    }

    #[test]
    fn test_is_file_read_method_read_text() {
        assert!(is_file_read_method("read_text"));
    }

    #[test]
    fn test_is_file_read_method_read_bytes() {
        assert!(is_file_read_method("read_bytes"));
    }

    #[test]
    fn test_is_file_read_method_write_not_read() {
        assert!(!is_file_read_method("write"));
        assert!(!is_file_read_method("flush"));
        assert!(!is_file_read_method("seek"));
    }
}
