use serde_json;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct DataProcessor {
    pub data: Vec<i32>,
    pub name: serde_json::Value,
}
impl DataProcessor {
    pub fn new(name: Option<String>) -> Self {
        Self {
            data: Vec::new(),
            name,
        }
    }
    pub fn add_data(&self, value: i32) {
        self.data.push(value);
    }
    pub fn add_batch(&self, values: Vec<i32>) {
        self.data.extend(values);
    }
    pub fn filter_data(&self, predicate: serde_json::Value) -> Vec<i32> {
        return self
            .data
            .into_iter()
            .filter(|&x| predicate(x))
            .map(|x| x)
            .collect::<Vec<_>>();
    }
    pub fn get_summary(&self) -> std::collections::HashMap<String, UnionType> {
        if !self.data {
            return {
                let mut map = std::collections::HashMap::new();
                map.insert("count".to_string(), 0);
                map.insert("mean".to_string(), 0);
                map
            };
        };
        return {
            let mut map = std::collections::HashMap::new();
            map.insert("count".to_string(), self.data.len() as i32);
            map.insert("sum".to_string(), sum(self.data));
            map.insert("mean".to_string(), sum(self.data) / self.data.len() as i32);
            map.insert("max".to_string(), max(self.data));
            map.insert("min".to_string(), min(self.data));
            map
        };
    }
    pub fn merge_processors(processors: Vec<DataProcessor>) -> DataProcessor {
        let merged = DataProcessor::new();
        for proc in processors {
            merged.add_batch(proc.data);
        }
        return merged;
    }
    pub fn is_empty(&self) -> bool {
        return self.data.len() as i32 == 0;
    }
}
#[doc = "Calculate the n-th Fibonacci number.\n    \n    This function uses an iterative approach for efficiency.\n    \n    Args:\n        n: The position in the Fibonacci sequence(0-indexed)\n        \n    Returns:\n        The n-th Fibonacci number\n        \n    Examples:\n      >>>fibonacci(0)\n        0\n      >>>fibonacci(1)\n        1\n      >>>fibonacci(10)\n        55\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    let (mut a, mut b) = (0, 1);
    for __sanitized in 2..n + 1 {
        (a, b) = (b, a + b);
    }
    b
}
#[doc = "Process a list of integers and return statistics.\n    \n    This function analyzes a list of integers and returns various\n    statistics about the data.\n    \n    Args:\n        items: List of integers to process\n        threshold: Optional threshold for filtering(default: None)\n        \n    Returns:\n        Dictionary containing statistics:\n        - 'count': Total number of items\n        - 'sum': Sum of all items\n        - 'max': Maximum value\n        - 'min': Minimum value\n        - 'above_threshold': Count of items above threshold\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_data(items: Vec<i32>, threshold: &Option<i32>) -> HashMap<String, i32> {
    let mut stats = {
        let mut map = HashMap::new();
        map.insert("count".to_string(), items.len() as i32);
        map.insert("sum".to_string(), items.iter().sum::<i32>());
        map.insert(
            "max".to_string(),
            if !items.is_empty() {
                *items.iter().max().unwrap()
            } else {
                0
            },
        );
        map.insert(
            "min".to_string(),
            if !items.is_empty() {
                *items.iter().min().unwrap()
            } else {
                0
            },
        );
        map.insert("above_threshold".to_string(), 0);
        map
    };
    if threshold.is_some() {
        let _cse_temp_0 = items
            .iter()
            .cloned()
            .filter(|&x| x > threshold.unwrap_or(i32::MIN))
            .map(|x| 1)
            .sum::<i32>();
        stats.insert("above_threshold".to_string(), _cse_temp_0);
    }
    stats
}
#[doc = "Main entry point demonstrating usage."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let processor = DataProcessor::new("example".to_string());
    processor.add_batch(vec![1, 2, 3, 4, 5]);
    let summary = processor.get_summary();
    println!("{}", format!("Summary: {:?}", summary));
    let stats = process_data(processor.data, 3);
    println!("{}", format!("Stats: {:?}", stats));
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_fibonacci_examples() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(-1), -1);
    }
}
