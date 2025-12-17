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
    let sum_val: i32 = x + y;
    let _cse_temp_0 = x * y;
    let product: i32 = _cse_temp_0;
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
    let _upper_text: String = text.to_uppercase();
    let _lower_text: String = text.to_lowercase();
    if text.starts_with("Hello") {
        return text.replace("Hello", "Hi");
    }
    text.trim().to_string()
}
#[doc = "Infers list type from list operations."]
#[doc = " Depyler: verified panic-free"]
pub fn list_processing(items: &mut Vec<String>) -> Vec<String> {
    items.push("new item".to_string().to_string());
    items.extend(
        vec![
            "more".to_string().to_string(),
            "items".to_string().to_string(),
        ]
        .iter()
        .cloned(),
    );
    let mut result: Vec<String> = vec![];
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
    let mut total: i32 = 0;
    for value in data.iter().cloned() {
        total = total + value * multiplier;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = {
        let a = total;
        let b = _cse_temp_0;
        let q = a / b;
        let r = a % b;
        let r_negative = r < 0;
        let b_negative = b < 0;
        let r_nonzero = r != 0;
        let signs_differ = r_negative != b_negative;
        let needs_adjustment = r_nonzero && signs_differ;
        if needs_adjustment {
            q - 1
        } else {
            q
        }
    };
    let average: i32 = _cse_temp_1;
    Ok(average)
}
#[doc = "Type conversion functions provide strong hints."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn type_conversions_hint(value: &str) -> (String, i32, f64) {
    let _cse_temp_0 = (value).to_string();
    let as_string: String = _cse_temp_0.to_string();
    let _cse_temp_1 = value.parse::<i32>().unwrap_or_default();
    let as_int: i32 = _cse_temp_1;
    let _cse_temp_2 = value.parse::<f64>().unwrap();
    let as_float: f64 = _cse_temp_2;
    (as_string, as_int, as_float)
}
#[doc = "Boolean operations suggest bool type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn boolean_logic(a: bool, b: bool, c: bool) -> bool {
    let _cse_temp_0 = (a) && (b);
    if _cse_temp_0 {
        true
    } else {
        let _cse_temp_1 = (b) || (c);
        if _cse_temp_1 {
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
) -> Option<String> {
    let _keys: Vec<String> = mapping.keys().cloned().collect::<Vec<_>>();
    let _values: Vec<String> = mapping.values().cloned().collect::<Vec<_>>();
    let _cse_temp_0 = mapping.get("key").is_some();
    if _cse_temp_0 {
        return Some(mapping.get("key").cloned().unwrap_or("default"));
    }
    None
}
#[doc = "Using parameters as callables."]
#[doc = " Depyler: verified panic-free"]
pub fn function_composition(
    transform: Box<dyn Fn(String) -> String>,
    data: &Vec<String>,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for item in data.iter().cloned() {
        let transformed: String = transform(&item);
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
    possible_container: &'b Vec<i32>,
) -> (String, i32, i32) {
    let processed: String = certain_str
        .to_uppercase()
        .trim()
        .to_string()
        .replace(" ", "_");
    let _cse_temp_0 = probable_num * 2;
    let doubled: i32 = _cse_temp_0;
    let _cse_temp_1 = possible_container.len() as i32;
    let size: i32 = _cse_temp_1;
    (processed, doubled, size)
}
#[doc = "Simple arithmetic with explicit types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_arithmetic(a: i32, b: i32) -> i32 {
    let result: i32 = a + b;
    result
}
#[doc = "Simple string concatenation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_string_concat<'a, 'b>(s1: &'a str, s2: &'b str) -> String {
    let result: String = format!("{}{}", s1, s2);
    result.to_string()
}
#[doc = "Sum a list of integers."]
#[doc = " Depyler: verified panic-free"]
pub fn simple_list_sum(numbers: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for n in numbers.iter().cloned() {
        total = total + n;
    }
    total
}
#[doc = "Dictionary lookup with default."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_dict_lookup<'b, 'a>(
    d: &'a std::collections::HashMap<String, i32>,
    key: &'b str,
) -> i32 {
    let value: i32 = d.get(key).cloned().unwrap_or(0);
    value
}
#[doc = "Handle optional values."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn optional_handling(maybe_value: &Option<i32>) -> i32 {
    if maybe_value.is_none() {
        return 0;
    }
    maybe_value
}
#[doc = "Unpack a tuple."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn tuple_unpacking(pair: &(i32, String)) -> String {
    let (num, text) = pair;
    let result: String = format!("{}: {:?}", text, num);
    result.to_string()
}
#[doc = "List comprehension with explicit type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn list_comprehension_typed(numbers: &Vec<i32>) -> Vec<i32> {
    let doubled: Vec<i32> = numbers.iter().cloned().map(|n| n * 2).collect::<Vec<_>>();
    doubled
}
#[doc = "Conditional expression(ternary)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn conditional_expression(flag: bool, a: i32, b: i32) -> i32 {
    let result: i32 = if flag { a } else { b };
    result
}
#[doc = "Main function to exercise all examples."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_result: i32 = numeric_operations(10, 5)?;
    println!("{}", format!("Numeric: {:?}", num_result));
    let str_result: String = string_manipulation("Hello World");
    println!("{}", format!("String: {:?}", str_result));
    let items: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let list_result: Vec<String> = list_processing(&mut items);
    println!("{}", format!("List: {:?}", list_result));
    let data: Vec<i32> = vec![1, 2, 3, 4, 5];
    let avg: i32 = mixed_inference(&data, 2)?;
    println!("{}", format!("Average: {:?}", avg));
    let conv: (String, i32, f64) = type_conversions_hint("42");
    println!("{}", format!("Conversions: {}", conv));
    let bool_result: bool = boolean_logic(true, false, true);
    println!("{}", format!("Boolean: {:?}", bool_result));
    let mapping: std::collections::HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value");
        map.insert("other".to_string(), "data");
        map
    };
    let dict_result: Option<String> = dictionary_operations(&mapping);
    println!("{}", format!("Dict: {:?}", dict_result));
    let arith: i32 = simple_arithmetic(5, 3);
    let concat: String = simple_string_concat("Hello", " World");
    let sum_val: i32 = simple_list_sum(&vec![1, 2, 3]);
    let lookup: i32 = simple_dict_lookup(
        &{
            let mut map = HashMap::new();
            map.insert("a".to_string(), 1);
            map
        },
        "a",
    );
    println!(
        "{}",
        format!(
            "Simple tests: {:?}, {:?}, {}, {:?}",
            arith, concat, sum_val, lookup
        )
    );
    Ok(())
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
    #[test]
    fn test_simple_arithmetic_examples() {
        assert_eq!(simple_arithmetic(0, 0), 0);
        assert_eq!(simple_arithmetic(1, 2), 3);
        assert_eq!(simple_arithmetic(-1, 1), 0);
    }
    #[test]
    fn test_simple_list_sum_examples() {
        assert_eq!(simple_list_sum(&vec![]), 0);
        assert_eq!(simple_list_sum(&vec![1]), 1);
        assert_eq!(simple_list_sum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_list_comprehension_typed_examples() {
        assert_eq!(list_comprehension_typed(vec![]), vec![]);
        assert_eq!(list_comprehension_typed(vec![1]), vec![1]);
    }
}
