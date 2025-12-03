use serde_json;
#[doc = "// NOTE: Map Python module 'copy'(tracked in DEPYLER-0424)"]
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Test shallow copy of list"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_shallow_copy_list() -> Vec<i32> {
    let original: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut copied: Vec<i32> = (original).clone();
    copied.push(6);
    copied
}
#[doc = "Test shallow copy of dictionary"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_shallow_copy_dict() -> HashMap<String, i32> {
    let original: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);
        map
    };
    let mut copied: HashMap<String, i32> = (original).clone();
    copied.insert("d".to_string(), 4);
    copied
}
#[doc = "Test list.copy() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_copy_method() -> Vec<i32> {
    let original: Vec<i32> = vec![10, 20, 30];
    let mut copied: Vec<i32> = original.clone();
    copied.insert((0) as usize, 99);
    copied
}
#[doc = "Test dict.copy() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_copy_method() -> HashMap<String, String> {
    let original: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1");
        map.insert("key2".to_string(), "value2");
        map
    };
    let mut copied: HashMap<String, String> = original.clone();
    copied.insert("key3".to_string().to_string(), "value3".to_string());
    copied
}
#[doc = "Test shallow copy behavior with nested lists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nested_list_shallow_copy() -> Vec<Vec<i32>> {
    let original: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let mut copied: Vec<Vec<i32>> = (original).clone();
    copied.push(vec![7, 8]);
    copied
}
#[doc = "Test deep copy of nested list"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deep_copy_nested_list() -> Vec<Vec<i32>> {
    let original: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let mut copied: Vec<Vec<i32>> = (original).clone();
    let _cse_temp_0 = copied.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        copied.get(0usize).cloned().unwrap_or_default().push(99);
    }
    copied
}
#[doc = "Test deep copy of nested dictionary"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deep_copy_nested_dict() -> HashMap<String, HashMap<String, i32>> {
    let original: HashMap<String, HashMap<String, i32>> = {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "group1".to_string(),
            serde_json::json!(serde_json::json!({ "a": 1, "b": 2 })),
        );
        map.insert(
            "group2".to_string(),
            serde_json::json!(serde_json::json!({ "c": 3, "d": 4 })),
        );
        map
    };
    let mut copied: HashMap<String, HashMap<String, i32>> = (original).clone();
    let _cse_temp_0 = copied.get("group1").is_some();
    if _cse_temp_0 {
        copied
            .get_mut(&"group1")
            .unwrap()
            .insert("e".to_string().to_string(), 5);
    }
    copied
}
#[doc = "Manual implementation of shallow list copy"]
#[doc = " Depyler: verified panic-free"]
pub fn manual_shallow_copy_list(original: &Vec<i32>) -> Vec<i32> {
    let mut copied: Vec<i32> = vec![];
    for item in original.iter().cloned() {
        copied.push(item);
    }
    copied
}
#[doc = "Manual implementation of shallow dict copy"]
pub fn manual_shallow_copy_dict(
    original: &HashMap<String, i32>,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut copied: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    for key in original.keys().cloned().collect::<Vec<_>>() {
        copied.insert(key, original.get(&key).cloned().unwrap_or_default());
    }
    Ok(copied)
}
#[doc = "Manual implementation of deep copy for nested lists"]
#[doc = " Depyler: verified panic-free"]
pub fn manual_deep_copy_nested_list(original: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut copied: Vec<Vec<i32>> = vec![];
    for sublist in original.iter().cloned() {
        let mut new_sublist: Vec<i32> = vec![];
        for item in sublist.iter().cloned() {
            new_sublist.push(item);
        }
        copied.push(new_sublist);
    }
    copied
}
#[doc = "Test that copy creates independent object"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_copy_with_modification() -> bool {
    let mut original: Vec<i32> = vec![1, 2, 3];
    let mut copied: Vec<i32> = (original).clone();
    original.push(4);
    let _cse_temp_0 = copied.len() as i32;
    let _cse_temp_1 = original.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let is_independent: bool = _cse_temp_2;
    is_independent
}
#[doc = "Test difference between reference and copy"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_reference_vs_copy() -> bool {
    let mut original: Vec<i32> = vec![1, 2, 3];
    let mut copied: Vec<i32> = (original).clone();
    let reference: Vec<i32> = original;
    original.push(4);
    let _cse_temp_0 = copied.len() as i32;
    let _cse_temp_1 = original.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let copy_different: bool = _cse_temp_2;
    let _cse_temp_3 = reference.len() as i32;
    let _cse_temp_4 = _cse_temp_3 == _cse_temp_1;
    let reference_same: bool = _cse_temp_4;
    (copy_different) && (reference_same)
}
#[doc = "Clone list and apply transformation"]
#[doc = " Depyler: verified panic-free"]
pub fn clone_list_with_transform(original: &mut Vec<i32>, multiplier: i32) -> Vec<i32> {
    let mut cloned: Vec<i32> = vec![];
    for item in original.iter().cloned() {
        cloned.push(item * multiplier);
    }
    cloned
}
#[doc = "Clone dictionary with filtering"]
pub fn clone_dict_with_filter(
    original: &mut HashMap<String, i32>,
    threshold: i32,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut filtered: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    for key in original.keys().cloned().collect::<Vec<_>>() {
        let value: i32 = original.get(&key).cloned().unwrap_or_default();
        if value > threshold {
            filtered.insert(key, value);
        }
    }
    Ok(filtered)
}
#[doc = "Merge two dictionaries by copying"]
pub fn merge_copied_dicts<'b, 'a>(
    dict1: &'a HashMap<String, i32>,
    dict2: &'b HashMap<String, i32>,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut merged: HashMap<String, i32> = (dict1).clone();
    for key in dict2.keys().cloned().collect::<Vec<_>>() {
        merged.insert(key, dict2.get(&key).cloned().unwrap_or_default());
    }
    Ok(merged)
}
#[doc = "Test copying empty collections"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_copy_empty_collections() -> () {
    let empty_list: Vec<i32> = vec![];
    let empty_dict: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    let copied_list: Vec<i32> = (empty_list).clone();
    let copied_dict: HashMap<String, i32> = (empty_dict).clone();
    (copied_list.len() as i32, copied_dict.len() as i32)
}
#[doc = "Test copying single-element collections"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_copy_single_element() -> () {
    let single_list: Vec<i32> = vec![42];
    let single_dict: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("answer".to_string(), 42);
        map
    };
    let copied_list: Vec<i32> = (single_list).clone();
    let copied_dict: HashMap<String, i32> = (single_dict).clone();
    (
        copied_list.get(0usize).cloned().unwrap_or_default(),
        copied_dict.get("answer").cloned().unwrap_or_default(),
    )
}
#[doc = "Run all copy module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_copy_features() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 2, 3, 4, 5];
    let scores: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), 85);
        map.insert("bob".to_string(), 72);
        map.insert("charlie".to_string(), 95);
        map
    };
    let d1: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map
    };
    let d2: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("c".to_string(), 3);
        map.insert("d".to_string(), 4);
        map
    };
    println!("{}", "All copy module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_manual_shallow_copy_list_examples() {
        assert_eq!(manual_shallow_copy_list(vec![]), vec![]);
        assert_eq!(manual_shallow_copy_list(vec![1]), vec![1]);
    }
    #[test]
    fn test_manual_deep_copy_nested_list_examples() {
        assert_eq!(manual_deep_copy_nested_list(vec![]), vec![]);
        assert_eq!(manual_deep_copy_nested_list(vec![1]), vec![1]);
    }
}
