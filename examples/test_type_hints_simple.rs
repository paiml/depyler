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
#[doc = "Add two numbers - should infer numeric types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add_numbers<'b, 'a>(a: i32, b: i32) {
    a + b
}
#[doc = "Process text - should infer string type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_text(text: &str) -> String {
    let result = text.to_uppercase();
    result.to_string()
}
#[doc = "Calculate average - should infer list of numbers."]
pub fn calculate_average(numbers: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total = 0;
    let mut count = 0;
    for num in numbers.iter().cloned() {
        total = total + num;
        count = count + 1;
    }
    let _cse_temp_0 = count > 0;
    if _cse_temp_0 {
        return Ok(total / count);
    }
    Ok(0)
}
#[doc = "Check string properties."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn string_checker(s: &str) -> bool {
    if s.starts_with("hello") {
        return true;
    }
    false
}
#[doc = "Perform list operations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn list_operations(items: &mut Vec<serde_json::Value>) -> i32 {
    items.push(42);
    items.len() as i32 as i32
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_add_numbers() {
        fn prop(a: (), b: ()) -> TestResult {
            let result1 = add_numbers(a.clone(), b.clone());
            let result2 = add_numbers(b.clone(), a.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn((), ()) -> TestResult);
    }
    #[test]
    fn test_calculate_average_examples() {
        assert_eq!(calculate_average(&vec![]), 0);
        assert_eq!(calculate_average(&vec![1]), 1);
        assert_eq!(calculate_average(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_string_checker_examples() {
        let _ = string_checker(Default::default());
    }
}
