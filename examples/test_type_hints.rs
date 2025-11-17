use serde_json;
#[derive(Debug, Clone)] pub struct ZeroDivisionError {
    message: String ,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "division by zero: {}", self.message)
}
} impl std::error::Error for ZeroDivisionError {
   
}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
    Self {
    message: message.into()
}
}
}
#[derive(Debug, Clone)] pub struct IndexError {
    message: String ,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "index out of range: {}", self.message)
}
} impl std::error::Error for IndexError {
   
}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = "Process a list of numbers."] pub fn process_numbers(data: & serde_json::Value) -> Result <(), ZeroDivisionError>{
    let mut total = 0;
    for num in data.iter().cloned() {
    total = total + num;
   
}
Ok(total / data.len() as i32)
}
#[doc = "Various string operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn manipulate_text(text: & str) {
    let mut result = text.to_uppercase();
    if result.starts_with("HELLO") {
    result = result.replace("HELLO", "HI");
   
}
result.trim().to_string()
}
#[doc = "Mixed numeric operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn mixed_operations<'a, 'b>(x: & 'a serde_json::Value, y: & 'b serde_json::Value) {
    let sum_val = x + y;
    let _cse_temp_0 = x * y;
    let product = _cse_temp_0;
    let _cse_temp_1 = x>y;
    if _cse_temp_1 {
    sum_val
}
else {
    product
}
} #[doc = "Operations on containers."] #[doc = " Depyler: proven to terminate"] pub fn container_operations(items: &mut serde_json::Value) -> Result <(), IndexError>{
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0>0;
    if _cse_temp_1 {
    let first = items.get(0usize).cloned().unwrap_or_default();
    items.push(42);
    return;
   
}
() Ok(())
}
#[doc = "Function with inferable return type."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn inferred_return_types() {
    let x = 10;
    let y = 20;
    x + y
}
#[doc = "String formatting with mixed types."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn string_formatting<'b, 'a>(name: & 'a str, age: & 'b serde_json::Value) {
    let formatted_name = name.to_uppercase();
    let next_age = age + 1;
    format!("{:?} will be {:?} next year", formatted_name, next_age)
}
#[doc = "Using variables as iterators."] #[doc = " Depyler: verified panic-free"] pub fn iterator_usage(collection: & serde_json::Value, predicate: serde_json::Value) -> Vec<serde_json::Value>{
    let mut results = vec! [];
    for item in collection.iter().cloned() {
    if predicate(item) {
    results.push(item);
   
}
} results
}
#[doc = "Type conversion hints."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn type_conversions(value: serde_json::Value) {
    let _cse_temp_0 = value.to_string();
    let text = _cse_temp_0;
    let _cse_temp_1 = (value) as i32;
    let number = _cse_temp_1;
    let _cse_temp_2 = (value) as f64;
    let decimal = _cse_temp_2;
  (text, number, decimal)
}
#[doc = "Only some parameters have annotations."] #[doc = " Depyler: verified panic-free"] pub fn partial_annotations<'b, 'a>(data: & 'a Vec<serde_json::Value>, multiplier: & 'b serde_json::Value) -> Vec<serde_json::Value>{
    let mut result = vec! [];
    for item in data.iter().cloned() {
    result.push(item * multiplier);
   
}
result }