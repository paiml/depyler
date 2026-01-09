#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR_ALICE: &'static str = "Alice";
use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
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
#[derive(Debug, Clone, PartialEq)]
pub enum IntOrStringUnion {
    Integer(i32),
    Text(String),
}
impl From<i32> for IntOrStringUnion {
    fn from(value: i32) -> Self {
        IntOrStringUnion::Integer(value)
    }
}
impl From<String> for IntOrStringUnion {
    fn from(value: String) -> Self {
        IntOrStringUnion::Text(value)
    }
}
impl IntOrStringUnion {
    pub fn is_integer(&self) -> bool {
        matches!(self, IntOrStringUnion::Integer(_))
    }
    pub fn is_text(&self) -> bool {
        matches!(self, IntOrStringUnion::Text(_))
    }
    pub fn as_integer(&self) -> Option<&i32> {
        match self {
            IntOrStringUnion::Integer(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_text(&self) -> Option<&String> {
        match self {
            IntOrStringUnion::Text(value) => Some(value),
            _ => None,
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
#[doc = "Test List type annotation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_typing() -> Vec<i32> {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers
}
#[doc = "Test Dict type annotation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_typing() -> HashMap<String, i32> {
    let ages: std::collections::HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert(STR_ALICE.to_string(), 30);
        map.insert("Bob".to_string(), 25);
        map.insert("Charlie".to_string(), 35);
        map
    };
    ages
}
#[doc = "Test Set type annotation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_typing() -> std::collections::HashSet<String> {
    let colors: std::collections::HashSet<String> = {
        let mut set = std::collections::HashSet::new();
        set.insert("red".to_string());
        set.insert("green".to_string());
        set.insert("blue".to_string());
        set
    };
    colors
}
#[doc = "Test Tuple type annotation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_tuple_typing() -> (String, i32, f64) {
    let person: (String, i32, f64) = (STR_ALICE.to_string(), 30, 5.6);
    person
}
#[doc = "Test Optional return type"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_optional_return(value: i32) -> Option<i32> {
    let _cse_temp_0 = value > 0;
    if _cse_temp_0 {
        return Some(value);
    } else {
        return None;
    }
}
#[doc = "Test Optional parameter"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_optional_parameter(value: &Option<i32>) -> i32 {
    if value.is_some() {
        return value;
    } else {
        return 0;
    }
}
#[doc = "Test Union type"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_union_simple(value: i32) -> IntOrStringUnion {
    let _cse_temp_0 = value > 0;
    if _cse_temp_0 {
        return value;
    } else {
        return "negative".to_string();
    }
}
#[doc = "Test nested List type"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nested_list() -> Vec<Vec<i32>> {
    let matrix: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    matrix
}
#[doc = "Test nested Dict type"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nested_dict() -> HashMap<String, HashMap<String, i32>> {
    let data: std::collections::HashMap<String, std::collections::HashMap<String, i32>> = {
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
    data
}
#[doc = "Test List of Tuples"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_list_of_tuples() -> Vec<(String, i32)> {
    let items: Vec<(String, i32)> = vec![
        ("apple".to_string(), 5),
        ("banana".to_string(), 3),
        ("cherry".to_string(), 8),
    ];
    items
}
#[doc = "Test Dict of Lists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_of_lists() -> HashMap<String, Vec<i32>> {
    let grades: std::collections::HashMap<String, Vec<i32>> = {
        let mut map = HashMap::new();
        map.insert(STR_ALICE.to_string(), vec![85, 90, 88]);
        map.insert("Bob".to_string(), vec![78, 82, 80]);
        map
    };
    grades
}
#[doc = "Test complex function signature"]
#[doc = " Depyler: verified panic-free"]
pub fn process_user_data(
    name: String,
    age: i32,
    scores: &Vec<f64>,
    _metadata: Option<std::collections::HashMap<String, String>>,
) -> Result<(String, f64), Box<dyn std::error::Error>> {
    let mut total: f64 = Default::default();
    total = 0.0;
    for score in scores.iter().cloned() {
        total = total + score;
    }
    let avg_score: f64 = if scores.len() as i32 > 0 {
        ((total) as f64) / (((scores.len() as i32) as f64) as f64)
    } else {
        0.0
    };
    let result: String = format!("{}({})", name, age);
    Ok((result, avg_score))
}
#[doc = "Test Dict parameters and return"]
pub fn merge_data<'a, 'b>(
    dict1: &'a std::collections::HashMap<String, i32>,
    dict2: &'b std::collections::HashMap<String, i32>,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut merged: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    for key in dict1.keys().cloned().collect::<Vec<_>>() {
        merged.insert(
            key.to_string().clone(),
            dict1.get(&key).cloned().unwrap_or_default(),
        );
    }
    for key in dict2.keys().cloned().collect::<Vec<_>>() {
        merged.insert(
            key.to_string().clone(),
            dict2.get(&key).cloned().unwrap_or_default(),
        );
    }
    Ok(merged)
}
#[doc = "Test List processing"]
#[doc = " Depyler: verified panic-free"]
pub fn filter_positive(numbers: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        if num > 0 {
            result.push(num);
        }
    }
    result
}
#[doc = "Test Union types in collections"]
#[doc = " Depyler: verified panic-free"]
pub fn count_by_type(items: &Vec<String>) -> (i32, i32) {
    let mut str_count: i32 = Default::default();
    let mut int_count: i32 = Default::default();
    int_count = 0;
    str_count = 0;
    for item in items.iter().cloned() {
        if true {
            int_count = int_count + 1;
        } else {
            str_count = str_count + 1;
        }
    }
    (int_count, str_count)
}
#[doc = "Get first element or None"]
#[doc = " Depyler: proven to terminate"]
pub fn first_element(items: &Vec<i32>) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        return Ok(Some(
            items
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ));
    } else {
        return Ok(None);
    }
}
#[doc = "Get last element or None"]
#[doc = " Depyler: proven to terminate"]
pub fn last_element(items: &Vec<i32>) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        return Ok(Some({
            let base = &items;
            let idx: i32 = (items.len() as i32).saturating_sub(1);
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        }));
    } else {
        return Ok(None);
    }
}
#[doc = "Safe division returning Optional"]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide(a: i32, b: i32) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(None);
    } else {
        return Ok(Some((((a) as f64) as f64) / (((b) as f64) as f64)));
    }
}
#[doc = "Safe dict access"]
#[doc = " Depyler: proven to terminate"]
pub fn get_value<'b, 'a>(
    data: &'a std::collections::HashMap<String, i32>,
    key: &'b str,
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = data.get(key).is_some();
    if _cse_temp_0 {
        return Ok(Some(data.get(key).cloned().unwrap_or_default()));
    } else {
        return Ok(None);
    }
}
#[doc = "Create point(type alias simulation)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn create_point() -> (f64, f64) {
    let point: (f64, f64) = (3.0, 4.0);
    point
}
#[doc = "Calculate distance using point type"]
#[doc = " Depyler: proven to terminate"]
pub fn distance_between_points(
    p1: (f64, f64),
    p2: (f64, f64),
) -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = p2.0 - p1.0;
    let dx: f64 = _cse_temp_0;
    let dy: f64 = _cse_temp_0;
    let distance: f64 = (dx.mul(&dx).unwrap().add(&dy.mul(&dy).unwrap()).unwrap() as f64).sqrt();
    Ok(distance)
}
#[doc = "Test Any type usage"]
#[doc = " Depyler: verified panic-free"]
pub fn validate_config(config: &std::collections::HashMap<String, DepylerValue>) -> bool {
    let required: Vec<String> = vec![
        "host".to_string(),
        "port".to_string(),
        "timeout".to_string(),
    ];
    for key in required.iter().cloned() {
        if config.get(&key).is_none() {
            return false;
        }
    }
    true
}
#[doc = "Test complex transformation"]
pub fn transform_data(
    data: &Vec<std::collections::HashMap<String, i32>>,
) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
    let mut result: Vec<(String, i32)> = vec![];
    for item in data.iter().cloned() {
        for key in item.keys().cloned().collect::<Vec<_>>() {
            let value: i32 = item.get(&key).cloned().unwrap_or_default();
            let pair: (String, i32) = (key, value);
            result.push(pair);
        }
    }
    Ok(result)
}
#[doc = "Test grouping operation"]
pub fn group_by_first_letter(
    words: &Vec<String>,
) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let mut groups: std::collections::HashMap<String, Vec<String>> = {
        let map: HashMap<String, Vec<String>> = HashMap::new();
        map
    };
    for word in words.iter().cloned() {
        if word.len() as i32 == 0 {
            continue;
        }
        let first_letter: String = {
            let base = &word;
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        };
        if groups.get(&first_letter).is_none() {
            groups.insert(first_letter.to_string().clone(), vec![]);
        }
        groups
            .get(&first_letter)
            .cloned()
            .unwrap_or_default()
            .push(word);
    }
    Ok(groups)
}
#[doc = "Run all typing module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_typing_features() -> Result<(), Box<dyn std::error::Error>> {
    let numbers: Vec<i32> = test_list_typing();
    let ages: std::collections::HashMap<String, i32> = test_dict_typing();
    let colors: std::collections::HashSet<String> = test_set_typing();
    let person: (String, i32, f64) = test_tuple_typing();
    let opt_value: Option<i32> = test_optional_return(5);
    let opt_param: i32 = test_optional_parameter(&Some(10));
    let union_result: String = test_union_simple(-1);
    let matrix: Vec<Vec<i32>> = test_nested_list();
    let nested: std::collections::HashMap<String, std::collections::HashMap<String, i32>> =
        test_nested_dict();
    let tuples: Vec<(String, i32)> = test_list_of_tuples();
    let lists: std::collections::HashMap<String, Vec<i32>> = test_dict_of_lists();
    let scores: Vec<f64> = vec![85.5, 90.0, 88.5];
    let user_result: (String, f64) = process_user_data(STR_ALICE.to_string(), 30, &scores, None)?;
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
    let merged: std::collections::HashMap<String, i32> = merge_data(&d1, &d2)?;
    let nums: Vec<i32> = vec![-1, 2, -3, 4, 5];
    let positive: Vec<i32> = filter_positive(&nums);
    let first: Option<i32> = first_element(&vec![1, 2, 3])?;
    let last: Option<i32> = last_element(&vec![1, 2, 3])?;
    let division: Option<f64> = safe_divide(10, 3)?;
    let data: std::collections::HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("x".to_string(), 10);
        map.insert("y".to_string(), 20);
        map
    };
    let value: Option<i32> = get_value(&data, "x")?;
    let p1: (f64, f64) = create_point();
    let p2: (f64, f64) = (6.0, 8.0);
    let dist: f64 = distance_between_points(p1, p2)?;
    let config: std::collections::HashMap<String, DepylerValue> = {
        let mut map = HashMap::new();
        map.insert(
            "host".to_string(),
            DepylerValue::Str("localhost".to_string()),
        );
        map.insert("port".to_string(), DepylerValue::Int(8080 as i64));
        map.insert("timeout".to_string(), DepylerValue::Int(30 as i64));
        map
    };
    let is_valid: bool = validate_config(&config);
    let dict_list: Vec<std::collections::HashMap<String, i32>> = vec![
        {
            let mut map = HashMap::new();
            map.insert("a".to_string(), 1);
            map
        },
        {
            let mut map = HashMap::new();
            map.insert("b".to_string(), 2);
            map
        },
    ];
    let transformed: Vec<(String, i32)> = transform_data(&dict_list)?;
    let words: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "apricot".to_string(),
        "cherry".to_string(),
    ];
    let grouped: std::collections::HashMap<String, Vec<String>> = group_by_first_letter(&words)?;
    println!("{}", "All typing module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_filter_positive_examples() {
        assert_eq!(filter_positive(vec![]), vec![]);
        assert_eq!(filter_positive(vec![1]), vec![1]);
    }
    #[test]
    fn test_validate_config_examples() {
        let _ = validate_config(Default::default());
    }
    #[test]
    fn test_transform_data_examples() {
        assert_eq!(transform_data(vec![]), vec![]);
        assert_eq!(transform_data(vec![1]), vec![1]);
    }
}
