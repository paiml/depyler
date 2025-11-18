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
#[doc = "Calculate sum of numbers."]
#[doc = " Depyler: verified panic-free"]
pub fn calculate_sum(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers.iter().cloned() {
        total = total + num;
    }
    total
}
#[doc = "Find maximum value."]
pub fn find_max(values: &Vec<i32>) -> Result<i32, IndexError> {
    if values.is_empty() {
        return Ok(0);
    }
    let mut max_val = values.get(0usize).cloned().unwrap_or_default();
    for val in values.iter().cloned() {
        if val > max_val {
            max_val = val;
        }
    }
    Ok(max_val)
}
#[doc = "Process data and return statistics."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_data(data: Vec<i32>) -> HashMap<String, i32> {
    let result = {
        let mut map = HashMap::new();
        map.insert("sum".to_string(), calculate_sum(&data));
        map.insert("max".to_string(), find_max(&data));
        map.insert("count".to_string(), data.len() as i32);
        map
    };
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_calculate_sum_examples() {
        assert_eq!(calculate_sum(&vec![]), 0);
        assert_eq!(calculate_sum(&vec![1]), 1);
        assert_eq!(calculate_sum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_find_max_examples() {
        assert_eq!(find_max(&vec![]), 0);
        assert_eq!(find_max(&vec![1]), 1);
        assert_eq!(find_max(&vec![1, 2, 3]), 3);
    }
}
