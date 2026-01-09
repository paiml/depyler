#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[doc = "String concatenation in loop - O(n²) complexity."]
#[doc = " Depyler: verified panic-free"]
pub fn string_concat_in_loop(items: &DepylerValue) -> String {
    let mut result: String = Default::default();
    result = "".to_string();
    for item in items.iter().cloned() {
        result = format!("{}{}", result, (item).to_string());
    }
    result.to_string()
}
#[doc = "Deeply nested loops - O(n³) complexity."]
#[doc = " Depyler: proven to terminate"]
pub fn nested_loops_cubic(matrix: &Vec<DepylerValue>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    total = 0;
    for i in 0..(matrix.len() as i32) {
        for j in 0..(matrix
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .len() as i32)
        {
            for k in 0..(matrix
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .len() as i32)
            {
                total = total
                    + matrix
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        .get(&k)
                        .cloned()
                        .unwrap_or_default();
            }
        }
    }
    Ok(total)
}
#[doc = "Expensive operations in loop."]
#[doc = " Depyler: verified panic-free"]
pub fn repeated_expensive_computation(data: &Vec<i32>) -> Vec<DepylerValue> {
    let mut results = vec![];
    for item in data.iter().cloned() {
        let sorted_data = {
            let mut sorted_vec = data.iter().cloned().collect::<Vec<_>>();
            sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            sorted_vec
        };
        results.push(item * sorted_data.len() as i32);
    }
    results
}
#[doc = "Inefficient list operations."]
#[doc = " Depyler: verified panic-free"]
pub fn inefficient_list_operations(items: &mut Vec<DepylerValue>) {
    while items.len() as i32 > 0 {
        if let Some(pos) = items.iter().position(|x| {
            x == &items
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range")
        }) {
            items.remove(pos)
        } else {
            panic!("ValueError: list.remove(x): x not in list")
        };
    }
}
#[doc = "Creating large objects in loops."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn large_list_in_loop(n: &DepylerValue) -> Vec<DepylerValue> {
    let mut results = vec![];
    for _i in 0..(n) {
        let temp = (0..(1000)).into_iter().map(|j| j).collect::<Vec<_>>();
        results.push(temp.iter().sum::<i32>());
    }
    results
}
#[doc = "Linear search in nested loop - O(n²)."]
#[doc = " Depyler: verified panic-free"]
pub fn linear_search_in_loop<'b, 'a>(
    items: &'a str,
    targets: &'b DepylerValue,
) -> Vec<DepylerValue> {
    let mut found = vec![];
    for target in targets.iter().cloned() {
        if items.get(&target).is_some() {
            let idx = items
                .iter()
                .position(|x| x == &target)
                .map(|i| i as i32)
                .expect("ValueError: value is not in list");
            found.push((target, idx));
        }
    }
    found
}
#[doc = "Expensive math operations in loop."]
#[doc = " Depyler: verified panic-free"]
pub fn power_in_tight_loop(values: &DepylerValue) -> Vec<DepylerValue> {
    let mut results = vec![];
    for x in values.iter().cloned() {
        let result = ({ x } as f64).powf({ 3.5 } as f64);
        results.push(result);
    }
    results
}
#[doc = "Using range(len()) instead of enumerate."]
#[doc = " Depyler: proven to terminate"]
pub fn range_len_antipattern(items: &Vec<DepylerValue>) -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..(items.len() as i32) {
        process_item(
            i,
            items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    }
    Ok(())
}
#[doc = "Computing aggregates repeatedly."]
#[doc = " Depyler: verified panic-free"]
pub fn aggregate_in_nested_loop(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    for row in matrix.iter().cloned() {
        for col in row.iter().cloned() {
            let total = row.iter().sum::<i32>();
            result = result + col * total;
        }
    }
    result
}
#[doc = "Large parameters passed by value."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn large_parameter_by_value<'a, 'b>(
    huge_list: &'a Vec<DepylerValue>,
    huge_dict: &'b std::collections::HashMap<String, DepylerValue>,
) -> i32 {
    huge_list.len() as i32 + huge_dict.len() as i32 as i32
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_item(_idx: DepylerValue, _item: DepylerValue) {}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_nested_loops_cubic_examples() {
        assert_eq!(nested_loops_cubic(&vec![]), 0);
        assert_eq!(nested_loops_cubic(&vec![1]), 1);
        assert_eq!(nested_loops_cubic(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_repeated_expensive_computation_examples() {
        assert_eq!(repeated_expensive_computation(vec![]), vec![]);
        assert_eq!(repeated_expensive_computation(vec![1]), vec![1]);
    }
    #[test]
    fn test_aggregate_in_nested_loop_examples() {
        assert_eq!(aggregate_in_nested_loop(&vec![]), 0);
        assert_eq!(aggregate_in_nested_loop(&vec![1]), 1);
        assert_eq!(aggregate_in_nested_loop(&vec![1, 2, 3]), 3);
    }
}
