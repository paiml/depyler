#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct DataProcessor {
    pub data: Vec<i32>,
    pub name: DepylerValue,
}
impl DataProcessor {
    pub fn new(name: Option<String>) -> Self {
        Self {
            data: Vec::new(),
            name,
        }
    }
    pub fn add_data(&mut self, value: i32) {
        self.data.push(value);
    }
    pub fn add_batch(&mut self, values: Vec<i32>) {
        self.data.extend(values);
    }
    pub fn filter_data(&self, _predicate: ()) -> Vec<i32> {
        return self
            .data
            .clone()
            .into_iter()
            .filter(|x| {
                let x = x.clone();
                predicate(x)
            })
            .map(|x| x)
            .collect::<Vec<_>>();
    }
    pub fn get_summary(&self) -> std::collections::HashMap<String, String> {
        if self.data.clone().is_empty() {
            return {
                let mut map = std::collections::HashMap::new();
                map.insert("count".to_string(), 0);
                map.insert("mean".to_string(), 0.0);
                map
            };
        };
        return {
            let mut map = std::collections::HashMap::new();
            map.insert("count".to_string(), self.data.clone().len() as i32);
            map.insert("sum".to_string(), self.data.clone().iter().sum::<i32>());
            map.insert(
                "mean".to_string(),
                self.data.clone().iter().sum::<i32>() / (self.data.clone().len() as i32),
            );
            map.insert("max".to_string(), max(self.data.clone()));
            map.insert("min".to_string(), min(self.data.clone()));
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
        return (self.data.clone().len() as i32) == 0;
    }
}
#[doc = "Calculate the n-th Fibonacci number.\n    \n    This function uses an iterative approach for efficiency.\n    \n    Args:\n        n: The position in the Fibonacci sequence(0-indexed)\n        \n    Returns:\n        The n-th Fibonacci number\n        \n    Examples:\n       >>>fibonacci(0)\n        0\n       >>>fibonacci(1)\n        1\n       >>>fibonacci(10)\n        55\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci(n: i32) -> i32 {
    let mut b: i32 = Default::default();
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    let (mut a, mut b) = (0, 1);
    for __sanitized in (2)..(n + 1) {
        (a, b) = (b, a + b);
    }
    b
}
#[doc = "Process a list of integers and return statistics.\n    \n    This function analyzes a list of integers and returns various\n    statistics about the data.\n    \n    Args:\n        items: List of integers to process\n        threshold: Optional threshold for filtering(default: None)\n        \n    Returns:\n        Dictionary containing statistics:\n        - 'count': Total number of items\n        - 'sum': Sum of all items\n        - 'max': Maximum value\n        - 'min': Minimum value\n        - 'above_threshold': Count of items above threshold\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_data<'a, 'b>(
    items: &'a Vec<i32>,
    threshold: &'b Option<i32>,
) -> HashMap<String, i32> {
    let mut stats = {
        let mut map = HashMap::new();
        map.insert(
            "count".to_string(),
            DepylerValue::Str(format!("{:?}", items.len() as i32)),
        );
        map.insert(
            "sum".to_string(),
            DepylerValue::Str(format!("{:?}", items.iter().sum::<i32>())),
        );
        map.insert(
            "max".to_string(),
            DepylerValue::Str(format!(
                "{:?}",
                if !items.is_empty() {
                    *items.iter().max().unwrap()
                } else {
                    0
                }
            )),
        );
        map.insert(
            "min".to_string(),
            DepylerValue::Str(format!(
                "{:?}",
                if !items.is_empty() {
                    *items.iter().min().unwrap()
                } else {
                    0
                }
            )),
        );
        map.insert("above_threshold".to_string(), DepylerValue::Int(0 as i64));
        map
    };
    if threshold.is_some() {
        let _cse_temp_0 = items
            .as_slice()
            .iter()
            .cloned()
            .filter(|x| x > threshold.unwrap_or(i32::MIN))
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
    let mut processor = DataProcessor::new("example".to_string());
    processor.add_batch(vec![1, 2, 3, 4, 5]);
    let summary = processor.get_summary();
    println!("{}", format!("Summary: {:?}", summary));
    let stats = process_data(processor.data, &Some(3));
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
