#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
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
    let original: std::collections::HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);
        map
    };
    let mut copied: std::collections::HashMap<String, i32> = (original).clone();
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
    let original: std::collections::HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map
    };
    let mut copied: std::collections::HashMap<String, String> = original.clone();
    copied.insert("key3".to_string(), "value3".to_string());
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
    let copied: Vec<Vec<i32>> = (original).clone();
    let _cse_temp_0 = copied.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        copied
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .push(99);
    }
    copied
}
#[doc = "Test deep copy of nested dictionary"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deep_copy_nested_dict() -> HashMap<String, HashMap<String, i32>> {
    let original: std::collections::HashMap<String, std::collections::HashMap<String, i32>> = {
        let mut map = HashMap::new();
        map.insert(
            "group1".to_string(),
            DepylerValue::Str(format!("{:?}", {
                let mut map = HashMap::new();
                map.insert("a".to_string(), 1);
                map.insert("b".to_string(), 2);
                map
            })),
        );
        map.insert(
            "group2".to_string(),
            DepylerValue::Str(format!("{:?}", {
                let mut map = HashMap::new();
                map.insert("c".to_string(), 3);
                map.insert("d".to_string(), 4);
                map
            })),
        );
        map
    };
    let copied: std::collections::HashMap<String, std::collections::HashMap<String, i32>> =
        (original).clone();
    let _cse_temp_0 = copied.get("group1").is_some();
    if _cse_temp_0 {
        copied
            .get_mut(&"group1")
            .unwrap()
            .insert("e".to_string(), 5);
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
    original: &std::collections::HashMap<String, i32>,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut copied: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    for key in original.keys().cloned().collect::<Vec<_>>() {
        copied.insert(
            key.to_string().clone(),
            original.get(&key).cloned().unwrap_or_default(),
        );
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
    let copied: Vec<i32> = (original).clone();
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
    let copied: Vec<i32> = (original).clone();
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
pub fn clone_list_with_transform(original: &Vec<i32>, multiplier: i32) -> Vec<i32> {
    let mut cloned: Vec<i32> = vec![];
    for item in original.iter().cloned() {
        cloned.push(item * multiplier);
    }
    cloned
}
#[doc = "Clone dictionary with filtering"]
pub fn clone_dict_with_filter(
    original: &std::collections::HashMap<String, i32>,
    threshold: i32,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut filtered: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    for key in original.keys().cloned().collect::<Vec<_>>() {
        let value: i32 = original.get(&key).cloned().unwrap_or_default();
        if value > threshold {
            filtered.insert(key.to_string().clone(), value);
        }
    }
    Ok(filtered)
}
#[doc = "Merge two dictionaries by copying"]
pub fn merge_copied_dicts<'b, 'a>(
    dict1: &'a std::collections::HashMap<String, i32>,
    dict2: &'b std::collections::HashMap<String, i32>,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut merged: std::collections::HashMap<String, i32> = (dict1).clone();
    for key in dict2.keys().cloned().collect::<Vec<_>>() {
        merged.insert(
            key.to_string().clone(),
            dict2.get(&key).cloned().unwrap_or_default(),
        );
    }
    Ok(merged)
}
#[doc = "Test copying empty collections"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_copy_empty_collections() -> (i32, i32) {
    let empty_list: Vec<i32> = vec![];
    let empty_dict: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    let copied_list: Vec<i32> = (empty_list).clone();
    let copied_dict: std::collections::HashMap<String, i32> = (empty_dict).clone();
    (copied_list.len() as i32, copied_dict.len() as i32)
}
#[doc = "Test copying single-element collections"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_copy_single_element() -> (i32, i32) {
    let single_list: Vec<i32> = vec![42];
    let single_dict: std::collections::HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("answer".to_string(), 42);
        map
    };
    let copied_list: Vec<i32> = (single_list).clone();
    let copied_dict: std::collections::HashMap<String, i32> = (single_dict).clone();
    (
        copied_list
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range"),
        copied_dict.get("answer").cloned().unwrap_or_default(),
    )
}
#[doc = "Run all copy module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_copy_features() -> Result<(), Box<dyn std::error::Error>> {
    let list_copy: Vec<i32> = test_shallow_copy_list();
    let dict_copy: std::collections::HashMap<String, i32> = test_shallow_copy_dict();
    let list_method: Vec<i32> = test_list_copy_method();
    let dict_method: std::collections::HashMap<String, String> = test_dict_copy_method();
    let nested_shallow: Vec<Vec<i32>> = test_nested_list_shallow_copy();
    let nested_deep_list: Vec<Vec<i32>> = test_deep_copy_nested_list();
    let nested_deep_dict: std::collections::HashMap<
        String,
        std::collections::HashMap<String, i32>,
    > = test_deep_copy_nested_dict();
    let manual_list: Vec<i32> = manual_shallow_copy_list(&vec![1, 2, 3]);
    let manual_dict: std::collections::HashMap<String, i32> = manual_shallow_copy_dict(&{
        let mut map = HashMap::new();
        map.insert("x".to_string(), 10);
        map.insert("y".to_string(), 20);
        map
    })?;
    let manual_deep: Vec<Vec<i32>> = manual_deep_copy_nested_list(&vec![vec![1, 2], vec![3, 4]]);
    let is_independent: bool = test_copy_with_modification();
    let ref_vs_copy: bool = test_reference_vs_copy();
    let data: Vec<i32> = vec![1, 2, 3, 4, 5];
    let transformed: Vec<i32> = clone_list_with_transform(&data, 2);
    let scores: std::collections::HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), 85);
        map.insert("bob".to_string(), 72);
        map.insert("charlie".to_string(), 95);
        map
    };
    let filtered: std::collections::HashMap<String, i32> = clone_dict_with_filter(&scores, 80)?;
    let d1: std::collections::HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map
    };
    let d2: std::collections::HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("c".to_string(), 3);
        map.insert("d".to_string(), 4);
        map
    };
    let merged: std::collections::HashMap<String, i32> = merge_copied_dicts(&d1, &d2)?;
    let empty_sizes: (i32, i32) = test_copy_empty_collections();
    let single_values: (i32, i32) = test_copy_single_element();
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
