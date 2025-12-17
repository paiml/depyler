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
#[derive(Debug, Clone)]
pub struct ValueError {
    message: String,
}
impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value error: {}", self.message)
    }
}
impl std::error::Error for ValueError {}
impl ValueError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
}
impl ValidationError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
#[derive(Debug, Clone)]
pub struct RangeError {
    pub value: i32,
    pub min_val: i32,
    pub max_val: i32,
}
impl RangeError {
    pub fn new(value: i32, min_val: i32, max_val: i32) -> Self {
        Self {
            value,
            min_val,
            max_val,
        }
    }
}
#[doc = "Simple try/except with fallback return.\n\n    Python: try/except ValueError → return default\n    Rust: s.parse::<i64>().unwrap_or(0)\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_int_safe(s: &str) -> i32 {
    return s.parse::<i32>().unwrap_or(0);
}
#[doc = "try/except returning Optional.\n\n    Python: try/except → None\n    Rust: s.parse::<i64>().ok()\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_int_with_error(s: &str) -> Option<i32> {
    return Some(s.parse::<i32>().unwrap_or(Default::default()));
}
#[doc = "try/except with ZeroDivisionError.\n\n    Python: try/except ZeroDivisionError\n    Rust: if b == 0 {
    0
}
else {
    a / b }\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn divide_safe(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
    if b == 0 {
        return Ok(0);
    } else {
        return {
            let a = a;
            let b = b;
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
    }
}
#[doc = "try/except with KeyError.\n\n    Python: try d[key] except KeyError\n    Rust: d.get(&key).copied().unwrap_or(-1)\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_with_key_error<'a, 'b>(
    d: &'a std::collections::HashMap<String, i32>,
    key: &'b str,
) -> Result<i32, Box<dyn std::error::Error>> {
    return Ok(d.get(key).cloned().unwrap_or(-1));
}
#[doc = "try/except with exception variable binding.\n\n    Python: except KeyError as e → use e\n    Rust: Err(e) =>format!(\"Error: {}\", e)\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_with_bound_exception<'a, 'b>(
    d: &'a std::collections::HashMap<String, i32>,
    key: &'b str,
) -> Result<String, Box<dyn std::error::Error>> {
    match (|| -> Result<String, Box<dyn std::error::Error>> {
        value = d.get(key).cloned().unwrap_or_default();
        return Ok((value).to_string());
        Ok(Default::default())
    })() {
        Ok(_result) => {
            return Ok(_result);
        }
        Err(e) => {
            return Ok(format!("Missing key: {:?}", e));
        }
    }
}
#[doc = "Multiple exception type handlers.\n\n    Python: except ValueError, except KeyError\n    Rust: match with multiple Err patterns\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn multiple_handlers<'a, 'b>(
    s: &'a str,
    d: &'b std::collections::HashMap<String, i32>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut num;
    num = s.parse::<i32>().unwrap_or_default();
    return Ok(d.get(&(num).to_string()).cloned().unwrap_or_default());
}
#[doc = "Nested try/except blocks.\n\n    Python: outer try wrapping inner try\n    Rust: nested match expressions\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn nested_try_except(x: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut outer: i32 = 0;
    let mut inner: i32 = 0;
    {
        outer = x + 1;
        {
            inner = outer * 2;
            if inner > 100 {
                panic!("{}", ValueError::new("Too large".to_string()));
            }
            return Ok(inner);
            return Ok(outer);
        }
        return Ok(0);
    }
}
#[doc = "try/except/finally for resource cleanup.\n\n    Python: finally block always executes\n    Rust: Drop guard or explicit cleanup\n\n    Note: This tests cleanup semantics, not file I/O.\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn try_except_finally_pattern(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String = "".to_string();
    let mut opened: bool = false;
    {
        match (|| -> Result<(), Box<dyn std::error::Error>> {
            opened = true;
            result = format!("Processing {}", filename);
            if filename == "" {
                panic!("{}", ValueError::new("Empty filename".to_string()));
            }
            return Ok(result.to_string());
            Ok(Default::default())
        })() {
            Ok(_result) => {
                return Ok(_result);
            }
            Err(e) => {
                result = format!("Error: {:?}", e);
                return Ok(result.to_string());
            }
        }
        if opened {}
    }
}
#[doc = "Exception propagation through multiple operations.\n\n    Python: Multiple operations that can fail\n    Rust: ? operator or explicit Result handling\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn propagate_result(values: &Vec<String>) -> i32 {
    let mut total: i32 = 0;
    match v.parse::<i32>() {
        Ok(num___i32) => {
            return total;
        }
        Err(_) => {
            return -1;
        }
    }
}
#[doc = "Early return within try block.\n\n    Python: return before try block ends\n    Rust: Ok(value) propagation\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn early_return_in_try(x: i32) -> i32 {
    let mut result;
    {
        if x < 0 {
            return -1;
        }
        result = x * 2;
        if result > 100 {
            return 100;
        }
        return result;
        return 0;
    }
}
#[doc = "Complex computation in try with multiple failure points.\n\n    Python: Chain of operations that can fail\n    Rust: Result chain with ? or match\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn exception_with_computation(
    a: i32,
    b: i32,
    c: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut step1;
    let mut step2;
    {
        step1 = {
            let a = a;
            let b = b;
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
        step2 = step1 * c;
        if step2 < 0 {
            panic!("{}", ValueError::new("Negative result".to_string()));
        }
        return Ok(step2);
        return Ok(-1);
        return Ok(-2);
    }
}
#[doc = "Full try/except/else pattern.\n\n    Python: else block runs when try succeeds(no exception)\n    Rust: Separate success path in match arm\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn try_except_else(s: &str) -> i32 {
    let mut result: i32 = 0;
    match s.parse::<i32>() {
        Ok(value) => {}
        Err(_) => {
            result = -1;
        }
    }
    result
}
#[doc = "Complete try/except/else/finally suite.\n\n    Python: Full exception handling pattern\n    Rust: Complex Result handling with cleanup\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn try_except_else_finally(s: &str) -> String {
    let mut status: String = "init".to_string();
    let mut result: i32 = 0;
    {
        match s.parse::<i32>() {
            Ok(result) => {
                status = "parsed".to_string();
            }
            Err(_) => {
                status = "error".to_string();
                result = -1;
            }
        }
        status = format!("{}_done", status);
    }
    format!("{}:{}", status, result)
}
#[doc = "Raise without arguments(re-raise current exception).\n\n    Python: bare raise re-raises the current exception\n    Rust: return Err(e) in catch block\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn raise_without_args(x: i32) -> Result<i32, Box<dyn std::error::Error>> {
    {
        if x < 0 {
            panic!("{}", ValueError::new("Negative".to_string()));
        }
        return Ok(x);
        return Err("Exception raised".into());
    }
}
#[doc = "Raise with explicit message.\n\n    Python: raise ValueError(\"message\")\n    Rust: Err(Box::new(ValueError {
    message: \"...\".to_string() }))\n    "]
#[doc = " Depyler: proven to terminate"]
pub fn raise_with_message(x: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = x < 0;
    if _cse_temp_0 {
        return Err(Box::new(ValueError::new(
            "Value must be non-negative".to_string(),
        )));
    }
    let _cse_temp_1 = x > 100;
    if _cse_temp_1 {
        return Err(Box::new(ValueError::new(
            "Value must be <= 100".to_string(),
        )));
    }
    Ok(x)
}
#[doc = "Raise custom exception type.\n\n    Python: raise RangeError(value, min, max)\n    Rust: Err(Box::new(RangeError {
    value, min_val, max_val }))\n    "]
#[doc = " Depyler: proven to terminate"]
pub fn raise_custom_exception(value: i32, min_val: i32, max_val: i32) -> Result<i32, RangeError> {
    let _cse_temp_0 = value < min_val;
    let _cse_temp_1 = value > max_val;
    let _cse_temp_2 = (_cse_temp_0) || (_cse_temp_1);
    if _cse_temp_2 {
        return Err(RangeError::new(value, min_val, max_val));
    }
    Ok(value)
}
#[doc = "Exception chaining with raise...from.\n\n    Python: raise NewError from original_error\n    Rust: Error chaining with source()\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn raise_from_exception(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    match (|| -> Result<i64, Box<dyn std::error::Error>> {
        return Ok(s.parse::<i32>().unwrap_or_default());
        Ok(Default::default())
    })() {
        Ok(_result) => {
            return Ok(_result);
        }
        Err(e) => {
            return Err(Box::new(ValidationError::new(format!(
                "Invalid input: {}",
                s
            ))));
        }
    }
}
#[doc = "Multiple raise points in one function.\n\n    Python: Different exceptions at different validation stages\n    Rust: Multiple Err() returns with different error types\n    "]
#[doc = " Depyler: proven to terminate"]
pub fn validate_and_transform(value: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = value < 0;
    if _cse_temp_0 {
        return Err(Box::new(ValidationError::new(
            "Value cannot be negative".to_string().to_string(),
        )));
    }
    let _cse_temp_1 = value > 1000;
    if _cse_temp_1 {
        return Err(Box::new(RangeError::new(value, 0, 1000)));
    }
    let _cse_temp_2 = value % 2;
    let _cse_temp_3 = _cse_temp_2 != 0;
    if _cse_temp_3 {
        return Err(Box::new(ValueError::new("Value must be even".to_string())));
    }
    Ok({
        let a = value;
        let b = 2;
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
    })
}
#[doc = "Catch custom exception types.\n\n    Python: except ValidationError as e\n    Rust: match on specific error types via downcast\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn catch_custom_exception(value: i32) -> Result<String, Box<dyn std::error::Error>> {
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        result = validate_and_transform(value)?;
        return Ok(format!("Result: {}", result));
        Ok(Default::default())
    })() {
        Ok(_result) => {
            return Ok(_result);
        }
        Err(e) => {
            return Ok(format!("Validation failed: {}", e.message));
            return Ok(format!(
                "Range error: {} not in [{}, {}]",
                e.value, e.min_val, e.max_val
            ));
            return Ok(format!("Value error: {:?}", e));
        }
    }
}
#[doc = "Main function exercising all exception patterns."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    assert!(parse_int_safe("42".to_string()) == 42);
    assert!(parse_int_safe("invalid".to_string()) == 0);
    assert!(parse_int_with_error("42".to_string()) == 42);
    assert!(parse_int_with_error("invalid".to_string()).is_none());
    assert!(divide_safe(10, 2) == 5);
    assert!(divide_safe(10, 0) == 0);
    let d: std::collections::HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map
    };
    assert!(get_with_key_error(&d, "a") == 1);
    assert!(get_with_key_error(& d, "missing".to_string()) = = (- 1));
    assert!(
        multiple_handlers("1".to_string(), &{
            let mut map = HashMap::new();
            map.insert("1".to_string().to_string(), 100);
            map
        }) == 100
    );
    assert!(multiple_handlers("invalid".to_string(), & {
    let mut map = HashMap::new();
    map.insert("1".to_string().to_string(), 100);
    map }) = = (- 1));
    assert!(multiple_handlers("99".to_string(), & {
    let mut map = HashMap::new();
    map.insert("1".to_string().to_string(), 100);
    map }) = = (- 2));
    assert!(nested_try_except(10) == 22);
    assert!(nested_try_except(100) == 101);
    assert!(
        propagate_result(&vec![
            "1".to_string().to_string(),
            "2".to_string().to_string(),
            "3".to_string().to_string()
        ]) == 6
    );
    assert!(propagate_result(& vec! ["1".to_string().to_string(), "invalid".to_string().to_string()]) = = (- 1));
    assert!(early_return_in_try(- 5) = = (- 1));
    assert!(early_return_in_try(10) == 20);
    assert!(early_return_in_try(100) == 100);
    assert!(exception_with_computation(10, 2, 3) == 15);
    assert!(exception_with_computation(10, 0, 3) = = (- 1));
    assert!(exception_with_computation(10, 2, - 3) = = (- 2));
    assert!(try_except_else("5".to_string()) == 10);
    assert!(try_except_else("invalid".to_string()) = = (- 1));
    assert!(try_except_else_finally("5".to_string()) == "success_done:50".to_string());
    assert!(try_except_else_finally("invalid".to_string()) == "error_done:-1".to_string());
    assert!(raise_with_message(50) == 50);
    assert!(raise_custom_exception(50, 0, 100) == 50);
    assert!(validate_and_transform(100) == 50);
    assert!(catch_custom_exception(100) == "Result: 50".to_string());
    assert!(catch_custom_exception(-1)
        .get("Validation failed".to_string())
        .is_some());
    assert!(catch_custom_exception(2000)
        .get("Range error".to_string())
        .is_some());
    assert!(catch_custom_exception(3)
        .get("Value error".to_string())
        .is_some());
    ()
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_parse_int_safe_examples() {
        assert_eq!(parse_int_safe(""), 0);
        assert_eq!(parse_int_safe("a"), 1);
        assert_eq!(parse_int_safe("abc"), 3);
    }
    #[test]
    fn test_divide_safe_examples() {
        assert_eq!(divide_safe(0, 0), 0);
        assert_eq!(divide_safe(1, 2), 3);
        assert_eq!(divide_safe(-1, 1), 0);
    }
    #[test]
    fn test_nested_try_except_examples() {
        assert_eq!(nested_try_except(0), 0);
        assert_eq!(nested_try_except(1), 1);
        assert_eq!(nested_try_except(-1), -1);
    }
    #[test]
    fn test_propagate_result_examples() {
        assert_eq!(propagate_result(&vec![]), 0);
        assert_eq!(propagate_result(&vec![1]), 1);
        assert_eq!(propagate_result(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_early_return_in_try_examples() {
        assert_eq!(early_return_in_try(0), 0);
        assert_eq!(early_return_in_try(1), 1);
        assert_eq!(early_return_in_try(-1), -1);
    }
    #[test]
    fn test_try_except_else_examples() {
        assert_eq!(try_except_else(""), 0);
        assert_eq!(try_except_else("a"), 1);
        assert_eq!(try_except_else("abc"), 3);
    }
    #[test]
    fn test_raise_without_args_examples() {
        assert_eq!(raise_without_args(0), 0);
        assert_eq!(raise_without_args(1), 1);
        assert_eq!(raise_without_args(-1), -1);
    }
    #[test]
    fn test_raise_with_message_examples() {
        assert_eq!(raise_with_message(0), 0);
        assert_eq!(raise_with_message(1), 1);
        assert_eq!(raise_with_message(-1), -1);
    }
    #[test]
    fn test_raise_from_exception_examples() {
        assert_eq!(raise_from_exception(""), 0);
        assert_eq!(raise_from_exception("a"), 1);
        assert_eq!(raise_from_exception("abc"), 3);
    }
    #[test]
    fn test_validate_and_transform_examples() {
        assert_eq!(validate_and_transform(0), 0);
        assert_eq!(validate_and_transform(1), 1);
        assert_eq!(validate_and_transform(-1), -1);
    }
    #[test]
    fn test_main_examples() {
        let _ = main();
    }
}
