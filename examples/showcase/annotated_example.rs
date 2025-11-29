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
pub fn count_words(text: &str) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut word_count = {
        let map = HashMap::new();
        map
    };
    let words = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    for word in words.iter().cloned() {
        if word_count.get(&word).is_some() {
            {
                let _key = word;
                let _old_val = word_count.get(&_key).cloned().unwrap_or_default();
                word_count.insert(_key, _old_val + 1);
            }
        } else {
            word_count.insert(word, 1);
        }
    }
    Ok(word_count)
}
#[doc = "Safe division with Result type."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide(a: i32, b: i32) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(None);
    }
    Ok(Some((a as f64) / (b as f64)))
}
#[doc = "Compute dot product with SIMD hints."]
#[doc = " Depyler: proven to terminate"]
pub fn dot_product<'b, 'a>(
    v1: &'a Vec<f64>,
    v2: &'b Vec<f64>,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut result = 0.0;
    for i in 0..v1.len() as i32 {
        result = result
            + v1.get(i as usize).cloned().unwrap_or_default()
                * v2.get(i as usize).cloned().unwrap_or_default();
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_parallel_sum_examples() {
        assert_eq!(parallel_sum(&vec![]), 0);
        assert_eq!(parallel_sum(&vec![1]), 1);
        assert_eq!(parallel_sum(&vec![1, 2, 3]), 6);
    }
}
