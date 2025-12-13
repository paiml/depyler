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
#[doc = "Compute the sum of a list of numbers."]
#[doc = " Depyler: verified panic-free"]
pub fn compute_sum(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers.iter().cloned() {
        total = total + num;
    }
    total
}
#[doc = "Binary search with nested loops and array access."]
pub fn binary_search(arr: &Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut left = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut right = _cse_temp_0 - 1;
    while left <= right {
        let mid = {
            let a = left + right;
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
        };
        if arr.get(mid as usize).cloned().unwrap_or_default() == target {
            return Ok(mid);
        } else {
            if arr.get(mid as usize).cloned().unwrap_or_default() < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    Ok(-1)
}
#[doc = "Process strings with concatenation."]
#[doc = " Depyler: verified panic-free"]
pub fn process_strings(strings: &Vec<String>) -> String {
    let mut result = "".to_string();
    for s in strings.iter().cloned() {
        result = format!("{}{}", result, format!("{}{}", s, " "));
    }
    result.trim().to_string()
}
#[doc = "Function with frequent dictionary lookups."]
#[doc = " Depyler: verified panic-free"]
pub fn lookup_values<'a, 'b>(
    data: &'a std::collections::HashMap<String, i32>,
    keys: &'b Vec<String>,
) -> Vec<i32> {
    let mut results = vec![];
    for key in keys.iter().cloned() {
        if data.get(&key).is_some() {
            results.push(data.get(&key).cloned().unwrap_or_default());
        } else {
            results.push(0);
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_compute_sum_examples() {
        assert_eq!(compute_sum(&vec![]), 0);
        assert_eq!(compute_sum(&vec![1]), 1);
        assert_eq!(compute_sum(&vec![1, 2, 3]), 6);
    }
}
