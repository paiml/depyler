use serde_json as json;
use std::f64 as math;
pub const CONFIG_FILE: &str = "config.json";
pub const MAX_RETRIES: i32 = 3;
pub const DEBUG_MODE: bool = true;
pub static data_processors: once_cell::sync::Lazy<serde_json::Value> =
    once_cell::sync::Lazy::new(|| {
        serde_json::to_value({
            let mut map = HashMap::new();
            map.insert("double".to_string().to_string(), |x| x * 2);
            map.insert("square".to_string().to_string(), |x| {
                if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
                    ({ x } as i32)
                        .checked_pow({ 2 } as u32)
                        .expect("Power operation overflowed")
                } else {
                    ({ x } as f64).powf({ 2 } as f64) as i32
                }
            });
            map.insert("stringify".to_string().to_string(), |x| (x).to_string());
            map
        })
        .unwrap()
    });
use once_cell::sync::Lazy;
use serde_json;
use std::collections::HashMap;
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
pub struct User {
    pub name: String,
    pub age: i32,
    pub created_at: (),
}
impl User {
    pub fn new(name: String, age: i32) -> Self {
        Self {
            name,
            age,
            created_at: Default::default(),
        }
    }
    pub fn greet(&self) -> String {
        return format!("Hello, I'm {}!", self.name);
    }
    pub fn is_adult(&self) -> bool {
        return self.age >= 18;
    }
}
#[derive(Debug, Clone)]
pub struct AdminUser {
    pub permissions: Vec<String>,
}
impl AdminUser {
    pub fn new(_name: String, _age: i32, permissions: Vec<String>) -> Self {
        Self { permissions }
    }
    pub fn has_permission(&self, permission: String) -> bool {
        return self.permissions.contains_key(&permission);
    }
    pub fn greet(&self) -> String {
        return format!("Hello, I'm Admin {}!", self.name);
    }
}
#[derive(Debug, Clone)]
pub struct FileManager {
    pub filename: String,
    pub mode: String,
    pub file: (),
}
impl FileManager {
    pub fn new(filename: String, mode: String) -> Self {
        Self {
            filename,
            mode,
            file: Default::default(),
        }
    }
    pub fn __enter__(&mut self) {
        self.file = std::fs::File::open(&self.filename).unwrap();
        return self.file;
    }
    pub fn __exit__(
        &self,
        exc_type: serde_json::Value,
        exc_val: serde_json::Value,
        exc_tb: serde_json::Value,
    ) {
        if self.file {
            self.file.close();
        };
    }
}
#[doc = "Calculate the nth Fibonacci number.\n    \n    The Fibonacci sequence is defined as:\n    - F(0) = 0\n    - F(1) = 1\n    - F(n) = F(n-1) + F(n-2) for n>1\n    \n    Args:\n        n: The position in the Fibonacci sequence\n        \n    Returns:\n        The nth Fibonacci number\n        \n    Raises:\n        ValueError: If n is negative\n        \n    Examples:\n       >>>calculate_fibonacci(0)\n        0\n       >>>calculate_fibonacci(1)\n        1\n       >>>calculate_fibonacci(10)\n        55\n    "]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_fibonacci(n: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = n < 0;
    if _cse_temp_0 {
        return Err(Box::new(ValueError::new(
            "n must be non-negative".to_string(),
        )));
    }
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
        return Ok(n);
    }
    let (mut prev, mut curr) = (0, 1);
    for __sanitized in 2..n + 1 {
        (prev, curr) = (curr, prev + curr);
    }
    Ok(curr)
}
#[doc = "Process and categorize users.\n    \n    Args:\n        users: List of User objects to process\n        filter_adults: Whether to filter only adult users\n        \n    Returns:\n        Dictionary with 'adults' and 'minors' keys\n    "]
#[doc = " Depyler: verified panic-free"]
pub fn process_users(users: &Vec<User>, filter_adults: bool) -> HashMap<String, Vec<User>> {
    let result = {
        let mut map = HashMap::new();
        map.insert("adults".to_string(), vec![]);
        map.insert("minors".to_string(), vec![]);
        map
    };
    for user in users.iter().cloned() {
        if user.is_adult() {
            result.get("adults").cloned().unwrap_or_default().push(user);
        } else {
            result.get("minors").cloned().unwrap_or_default().push(user);
        }
    }
    if DEBUG_MODE {
        println!("{}", format!("Processed {} users", users.len() as i32));
    }
    result
}
#[doc = "Create a message handler with a prefix."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn create_handler(prefix: String) -> Box<dyn Fn(String) -> String> {
    let mut handler;
    handler = |message: String| -> String {
        return format!("{}: {}", prefix, message);
    };
    Box::new(handler)
}
#[doc = "Validate if age is in acceptable range."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_age(age: i32) -> bool {
    let MIN_AGE = 0;
    let MAX_AGE = 150;
    let _cse_temp_0 = age < MIN_AGE;
    if _cse_temp_0 {
        return false;
    }
    let _cse_temp_1 = age > MAX_AGE;
    if _cse_temp_1 {
        return false;
    }
    age >= 0
}
#[doc = "Function that might trigger LSP diagnostics."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn problematic_function() -> String {
    let result: i32 = "not an int";
    result.to_string()
}
#[doc = "Process numbers using various transformations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    let doubled = numbers
        .iter()
        .cloned()
        .map(|n| (data_processors.get("double").cloned().unwrap_or_default())(n))
        .collect::<Vec<_>>();
    doubled
}
#[doc = "Decorator to log function calls."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn log_calls(func: serde_json::Value) -> Box<dyn Fn(()) -> ()> {
    let mut wrapper;
    wrapper = |args: ()| {
        println!("{}", format!("Calling {}", func.__name__));
        return func(args);
    };
    Box::new(wrapper)
}
#[doc = "An important operation that should be logged."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn important_operation(value: &str) -> String {
    value.to_uppercase()
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_calculate_fibonacci_examples() {
        assert_eq!(calculate_fibonacci(0), 0);
        assert_eq!(calculate_fibonacci(1), 1);
        assert_eq!(calculate_fibonacci(-1), -1);
    }
    #[test]
    fn test_validate_age_examples() {
        let _ = validate_age(Default::default());
    }
    #[test]
    fn test_process_numbers_examples() {
        assert_eq!(process_numbers(vec![]), vec![]);
        assert_eq!(process_numbers(vec![1]), vec![1]);
    }
}
