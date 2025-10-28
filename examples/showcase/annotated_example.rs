use fnv::FnvHashMap;
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
#[doc = "Compute sum with parallel processing hints."]
#[doc = " Depyler: verified panic-free"]
pub fn parallel_sum(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers.iter().cloned() {
        total = total + num;
        total = total + num;
        total = total + num;
        total = total + num;
    }
    total
}
#[doc = "Process text with zero-copy string strategy."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_text(text: &str) -> String {
    text.to_uppercase()
}
#[doc = "Count word frequencies with FNV hash strategy."]
pub fn count_words(text: &str) -> Result<FnvHashMap<String, i32>, IndexError> {
    let mut word_count = {
        let mut map = HashMap::new();
        map
    };
    let words = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    for word in words.iter().cloned() {
        if word_count.contains_key(&word) {
            word_count.insert(word, word_count.get(&word).cloned().unwrap_or_default() + 1);
        } else {
            word_count.insert(word, 1);
        }
    }
    Ok(word_count)
}
#[doc = "Safe division with Result type."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide(a: i32, b: i32) -> Result<Option<f64>, ZeroDivisionError> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(None);
    }
    Ok(Some((a as f64) / (b as f64)))
}
#[doc = "Compute dot product with SIMD hints."]
#[doc = " Depyler: proven to terminate"]
pub fn dot_product<'b, 'a>(v1: &'a Vec<f64>, v2: &'b Vec<f64>) -> Result<f64, IndexError> {
    let mut result = 0.0;
    for i in 0..v1.len() as i32 {
        result = result + {
            let base = v1;
            let idx = i;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub((-idx) as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        } * {
            let base = v2;
            let idx = i;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub((-idx) as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        };
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_parallel_sum_examples() {
        assert_eq!(parallel_sum(0), 0);
        assert_eq!(parallel_sum(1), 1);
        assert_eq!(parallel_sum(-1), -1);
    }
}
