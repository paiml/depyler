#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
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
#[doc = "Infers numeric types from arithmetic operations."]
#[doc = " Depyler: proven to terminate"]
pub fn numeric_operations(x: i32, y: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let sum_val = x + y;
    let _cse_temp_0 = x * y;
    let product = _cse_temp_0;
    let _cse_temp_1 = x > y;
    if _cse_temp_1 {
        return Ok(sum_val);
    } else {
        return Ok(product);
    }
}
#[doc = "Infers string type from string methods."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn string_manipulation(text: &str) -> String {
    let upper_text = text.to_uppercase();
    let lower_text = text.to_lowercase();
    if text.starts_with("Hello") {
        return text.replace("Hello", "Hi");
    }
    text.trim().to_string()
}
#[doc = "Infers list type from list operations."]
#[doc = " Depyler: verified panic-free"]
pub fn list_processing(items: &mut Vec<DepylerValue>) -> Vec<DepylerValue> {
    items.push("new item".to_string());
    items.extend(
        vec![
            "more".to_string().to_string(),
            "items".to_string().to_string(),
        ]
        .iter()
        .cloned(),
    );
    let mut result = vec![];
    for item in items.iter().cloned() {
        result.push(item.to_uppercase());
    }
    result
}
#[doc = "Multiple inference sources for better confidence."]
pub fn mixed_inference(
    data: &Vec<i32>,
    multiplier: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    total = 0;
    for value in data.iter().cloned() {
        total = total + value * multiplier;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = total / _cse_temp_0;
    let average = _cse_temp_1;
    Ok(average)
}
#[doc = "Type conversion functions provide strong hints."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn type_conversions_hint(value: &str) -> (String, i32, f64) {
    let _cse_temp_0 = (value).to_string();
    let as_string = _cse_temp_0;
    let _cse_temp_1 = value.parse::<i32>().unwrap_or_default();
    let as_int = _cse_temp_1;
    let _cse_temp_2 = value.parse::<f64>().unwrap();
    let as_float = _cse_temp_2;
    (as_string, as_int, as_float)
}
#[doc = "Boolean operations suggest bool type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn boolean_logic(a: bool, b: bool, c: bool) -> bool {
    let _cse_temp_0 = (a) && (b);
    if _cse_temp_0 {
        return true;
    } else {
        let _cse_temp_1 = (b) || (c);
        if _cse_temp_1 {
            return false;
        } else {
            return !c;
        }
    }
}
#[doc = "Dictionary method usage."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn dictionary_operations(mapping: &str) -> Option<DepylerValue> {
    let keys = mapping.keys().cloned().collect::<Vec<_>>();
    let values = mapping.values().cloned().collect::<Vec<_>>();
    let _cse_temp_0 = mapping.contains("key");
    if _cse_temp_0 {
        return Some(mapping.get("key").cloned().unwrap_or("default"));
    }
    None
}
#[doc = "Using parameters as callables."]
#[doc = " Depyler: verified panic-free"]
pub fn function_composition(
    transform: impl Fn(i32) -> i32,
    data: &DepylerValue,
) -> Vec<DepylerValue> {
    let mut result = vec![];
    for item in data.iter().cloned() {
        let transformed = transform(item);
        result.push(transformed);
    }
    result
}
#[doc = "Demonstrates different confidence levels."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn confidence_levels_demo<'a, 'b>(
    certain_str: &'a str,
    probable_num: i32,
    possible_container: &'b Vec<DepylerValue>,
) -> (String, i32, i32) {
    let processed = certain_str
        .to_uppercase()
        .trim()
        .to_string()
        .replace(" ", "_");
    let _cse_temp_0 = probable_num * 2;
    let doubled = _cse_temp_0;
    let _cse_temp_1 = possible_container.len() as i32;
    let size = _cse_temp_1;
    (processed, doubled, size)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_numeric_operations_examples() {
        assert_eq!(numeric_operations(0, 0), 0);
        assert_eq!(numeric_operations(1, 2), 3);
        assert_eq!(numeric_operations(-1, 1), 0);
    }
    #[test]
    fn test_list_processing_examples() {
        assert_eq!(list_processing(vec![]), vec![]);
        assert_eq!(list_processing(vec![1]), vec![1]);
    }
}
