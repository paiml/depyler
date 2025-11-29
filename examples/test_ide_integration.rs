pub const result: serde_json::Value = calculate_fibonacci(10);
pub const math_util: serde_json::Value = MathUtils::new();
pub const rounded: serde_json::Value = math_util.round_number(3.14159);
use serde_json;
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
#[derive(Debug, Clone)]
pub struct MathUtils {
    pub precision: i32,
}
impl MathUtils {
    pub fn new() -> Self {
        Self { precision: 0 }
    }
    pub fn round_number(&self, value: f64) -> f64 {
        return round(value, self.precision);
    }
    pub fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        };
        for i in 2..(n as f64).powf(0.5 as f64).parse::<i32>().unwrap_or(0) + 1 {
            if n % i == 0 {
                return false;
            };
        }
        return true;
    }
}
#[doc = "Calculate the nth Fibonacci number.\n    \n    Uses recursion with memoization for efficiency.\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
}
#[doc = "Process a list of integers and return statistics."]
pub fn process_data(items: Vec<i32>) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let utils = MathUtils::new();
    let mut stats = {
        let mut map = HashMap::new();
        map.insert("count".to_string(), items.len() as i32);
        map.insert("sum".to_string(), items.iter().sum::<i32>());
        map.insert("primes".to_string(), 0);
        map
    };
    for item in items.iter().cloned() {
        if utils.is_prime(item) {
            stats.insert(
                "primes".to_string(),
                stats.get("primes").cloned().unwrap_or_default() + 1,
            );
        }
    }
    Ok(stats)
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
}
