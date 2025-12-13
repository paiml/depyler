use serde_json;
use std::collections::HashMap;
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
#[doc = "Infers numeric types from arithmetic operations."]
#[doc = " Depyler: proven to terminate"]
pub fn numeric_operations(x: i32, y: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let sum_val = x + y;
    let _cse_temp_0 = x * y;
    let product = _cse_temp_0;
    let _cse_temp_1 = x > y;
    if _cse_temp_1 {
        Ok(sum_val)
    } else {
        Ok(product)
    }
}
#[doc = "Infers string type from string methods."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn string_manipulation(text: &str) -> String {
    let _upper_text = text.to_uppercase();
    let _lower_text = text.to_lowercase();
    if text.starts_with("Hello") {
        return text.replace("Hello", "Hi");
    }
    text.trim().to_string()
}
#[doc = "Infers list type from list operations."]
#[doc = " Depyler: verified panic-free"]
pub fn list_processing(items: &mut Vec<serde_json::Value>) -> Vec<serde_json::Value> {
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
pub fn mixed_inference<'a, 'b>(
    data: &'a Vec<i32>,
    multiplier: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total = 0;
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
pub fn boolean_logic<'c, 'a, 'b>(a: bool, b: bool, c: bool) -> bool {
    let _cse_temp_0 = (a) && (b);
    if !_cse_temp_0.is_empty() {
        true
    } else {
        let _cse_temp_1 = (b) || (c);
        if _cse_temp_1 != 0 {
            false
        } else {
            !c
        }
    }
}
#[doc = "Dictionary method usage."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn dictionary_operations(
    mapping: &std::collections::HashMap<String, String>,
) -> Option<serde_json::Value> {
    let _keys = mapping.keys().cloned().collect::<Vec<_>>();
    let _values = mapping.values().cloned().collect::<Vec<_>>();
    let _cse_temp_0 = mapping.get("key").is_some();
    if _cse_temp_0 {
        return Some(mapping.get("key").cloned().unwrap_or("default"));
    }
    None
}
#[doc = "Using parameters as callables."]
#[doc = " Depyler: verified panic-free"]
pub fn function_composition(
    transform: serde_json::Value,
    data: &serde_json::Value,
) -> Vec<serde_json::Value> {
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
    possible_container: &'b Vec<serde_json::Value>,
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
    fn test_list_processing_examples() {
        assert_eq!(list_processing(vec![]), vec![]);
        assert_eq!(list_processing(vec![1]), vec![1]);
    }
}
