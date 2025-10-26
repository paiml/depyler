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
#[doc = "Calculate nth Fibonacci number iteratively."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_iterative(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n as i32;
    }
    let mut a = 0;
    let mut b = 1;
    for i in 2..n + 1 {
        let c = a + b;
        a = b;
        b = c;
    }
    return b as i32;
}
#[doc = "Sum first 'limit' Fibonacci numbers."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn sum_fibonacci_numbers(limit: i32) -> i32 {
    let mut total = 0;
    for i in 0..limit {
        total = total + fibonacci_iterative(i);
    }
    return total as i32;
}
#[doc = "Calculate basic statistics on a list of numbers."]
pub fn calculate_statistics<'a>(numbers: &'a Vec<i32>) -> Result<HashMap<String, i32>, IndexError> {
    if !numbers {
        return Ok({
            let mut map = HashMap::new();
            map.insert("count".to_string(), 0);
            map.insert("sum".to_string(), 0);
            map.insert("min".to_string(), 0);
            map.insert("max".to_string(), 0);
            map
        });
    }
    let _cse_temp_0 = numbers.len() as i32;
    let count = _cse_temp_0;
    let mut total = 0;
    let mut min_val = {
        let base = numbers;
        let idx = 0;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub((-idx) as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).copied().unwrap_or_default()
    };
    let mut max_val = {
        let base = numbers;
        let idx = 0;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub((-idx) as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).copied().unwrap_or_default()
    };
    for num in numbers.iter() {
        total = total + num;
        if num < min_val {
            min_val = num;
        }
        if num > max_val {
            max_val = num;
        }
    }
    return Ok({
        let mut map = HashMap::new();
        map.insert("count".to_string(), count);
        map.insert("sum".to_string(), total);
        map.insert("min".to_string(), min_val);
        map.insert("max".to_string(), max_val);
        map
    });
}
#[doc = "Run benchmark with different limits."]
#[doc = " Depyler: verified panic-free"]
pub fn main() -> serde_json::Value {
    let limits = vec![25, 30, 35];
    for limit in limits.iter() {
        let result = sum_fibonacci_numbers(limit);
        let mut fib_sequence = vec![];
        for i in 0..limit {
            fib_sequence.push(fibonacci_iterative(i));
        }
        let stats = calculate_statistics(fib_sequence);
        println!(
            "{}",
            format!(
                "Limit: {} | Sum: {} | Count: {} | Max: {}",
                limit,
                result,
                stats.get("count").cloned().unwrap_or_default(),
                stats.get("max").cloned().unwrap_or_default()
            )
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_fibonacci_iterative_examples() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(-1), -1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_sum_fibonacci_numbers_examples() {
        assert_eq!(sum_fibonacci_numbers(0), 0);
        assert_eq!(sum_fibonacci_numbers(1), 1);
        assert_eq!(sum_fibonacci_numbers(-1), -1);
    }
}
