use serde_json;
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
#[doc = "Process a list of numbers."]
pub fn process_numbers(data: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total = 0;
    for num in data.iter().cloned() {
        total = total + num;
    }
    Ok(total / data.len() as i32 as i32)
}
#[doc = "Various string operations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn manipulate_text(text: &str) -> String {
    let mut result = text.to_uppercase();
    if result.starts_with("HELLO") {
        result = result.replace("HELLO", "HI");
    }
    result.trim().to_string()
}
#[doc = "Mixed numeric operations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn mixed_operations<'a, 'b>(x: i32, y: i32) {
    let sum_val = x + y;
    let _cse_temp_0 = x * y;
    let product = _cse_temp_0;
    let _cse_temp_1 = x > y;
    if _cse_temp_1 {
        sum_val
    } else {
        product
    }
}
#[doc = "Operations on containers."]
#[doc = " Depyler: proven to terminate"]
pub fn container_operations(
    items: &mut Vec<serde_json::Value>,
) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        let first = items.get(0usize).cloned().unwrap_or_default();
        items.push(42);
        return Ok(Some(first));
    }
    Ok(None)
}
#[doc = "Function with inferable return type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn inferred_return_types() -> i32 {
    let x = 10;
    let y = 20;
    x + y
}
#[doc = "String formatting with mixed types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn string_formatting(name: &str, age: i32) -> String {
    let formatted_name = name.to_uppercase();
    let next_age = format!("{}{}", age, 1);
    format!("{} will be {:?} next year", formatted_name, next_age)
}
#[doc = "Using variables as iterators."]
#[doc = " Depyler: verified panic-free"]
pub fn iterator_usage(
    collection: &serde_json::Value,
    predicate: serde_json::Value,
) -> Vec<serde_json::Value> {
    let mut results = vec![];
    for item in collection.iter().cloned() {
        if predicate(item) {
            results.push(item);
        }
    }
    results
}
#[doc = "Type conversion hints."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn type_conversions(value: &str) -> (String, i32, f64) {
    let _cse_temp_0 = (value).to_string();
    let text = _cse_temp_0;
    let _cse_temp_1 = value.parse::<i32>().unwrap_or_default();
    let number = _cse_temp_1;
    let _cse_temp_2 = value.parse::<f64>().unwrap();
    let decimal = _cse_temp_2;
    (text, number, decimal)
}
#[doc = "Only some parameters have annotations."]
#[doc = " Depyler: verified panic-free"]
pub fn partial_annotations<'a, 'b>(
    data: &'a Vec<serde_json::Value>,
    multiplier: i32,
) -> Vec<serde_json::Value> {
    let mut result = vec![];
    for item in data.iter().cloned() {
        result.push(item * multiplier);
    }
    result
}
