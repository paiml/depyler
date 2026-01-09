#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
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
#[doc = "Filter out even numbers from list"]
pub fn filter_even_numbers(numbers: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        if num % 2 == 0 {
            result.push(num);
        }
    }
    Ok(result)
}
#[doc = "Find duplicate numbers in list"]
#[doc = " Depyler: verified panic-free"]
pub fn find_duplicates(numbers: &Vec<i32>) -> Vec<i32> {
    let mut seen: Vec<i32> = vec![];
    let mut duplicates: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        if seen.contains(&num) {
            if !duplicates.contains(&num) {
                duplicates.push(num);
            }
        } else {
            seen.push(num);
        }
    }
    duplicates
}
#[doc = "Merge two sorted lists into one sorted list"]
pub fn merge_sorted_lists<'b, 'a>(
    list1: &'a Vec<i32>,
    list2: &'b Vec<i32>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut i: i32 = Default::default();
    let mut j: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    i = 0;
    j = 0;
    while (i < list1.len() as i32) && (j < list2.len() as i32) {
        if list1
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            <= list2
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        {
            result.push(
                list1
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
            i = i + 1;
        } else {
            result.push(
                list2
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
            j = j + 1;
        }
    }
    while i < list1.len() as i32 {
        result.push(
            list1
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        i = i + 1;
    }
    while j < list2.len() as i32 {
        result.push(
            list2
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        j = j + 1;
    }
    Ok(result)
}
#[doc = "Calculate running sum of list"]
#[doc = " Depyler: verified panic-free"]
pub fn calculate_running_sum(numbers: &Vec<i32>) -> Vec<i32> {
    if numbers.is_empty() {
        return vec![];
    }
    let mut result: Vec<i32> = vec![];
    let mut running_total = 0;
    for num in numbers.iter().cloned() {
        running_total = running_total + num;
        result.push(running_total);
    }
    result
}
#[doc = "Rotate list left by specified positions"]
#[doc = " Depyler: proven to terminate"]
pub fn rotate_list_left(
    numbers: Vec<i32>,
    mut positions: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = positions <= 0;
    let _cse_temp_1 = (numbers.is_empty()) || (_cse_temp_0);
    if _cse_temp_1 {
        return Ok(numbers);
    }
    let _cse_temp_2 = numbers.len() as i32;
    let length = _cse_temp_2;
    let _cse_temp_3 = positions % length;
    positions = _cse_temp_3;
    let mut result: Vec<i32> = vec![];
    for i in (positions)..(length) {
        result.push(
            numbers
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    }
    for i in 0..(positions) {
        result.push(
            numbers
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_filter_even_numbers_examples() {
        assert_eq!(filter_even_numbers(vec![]), vec![]);
        assert_eq!(filter_even_numbers(vec![1]), vec![1]);
    }
    #[test]
    fn test_find_duplicates_examples() {
        assert_eq!(find_duplicates(vec![]), vec![]);
        assert_eq!(find_duplicates(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_merge_sorted_lists() {
        fn prop(list1: Vec<i32>, list2: Vec<i32>) -> TestResult {
            let result = merge_sorted_lists(&list1, &list2);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = list1.clone();
            input_sorted.sort();
            let mut result = merge_sorted_lists(&list1);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>, Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_calculate_running_sum_examples() {
        assert_eq!(calculate_running_sum(vec![]), vec![]);
        assert_eq!(calculate_running_sum(vec![1]), vec![1]);
    }
}
