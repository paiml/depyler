#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[derive(Debug, Clone)]
pub struct DataProcessor {
    pub threshold: i32,
}
impl DataProcessor {
    pub fn new(threshold: i32) -> Self {
        Self { threshold }
    }
    pub fn filter_data(&self, data: Vec<i32>) -> Vec<i32> {
        return data
            .into_iter()
            .filter(|x| {
                let x = x.clone();
                x > self.threshold.clone()
            })
            .map(|x| x)
            .collect::<Vec<_>>();
    }
    pub fn transform_data(&self, data: Vec<i32>) -> Vec<i32> {
        return data.into_iter().map(|x| x * 2 + 1).collect::<Vec<_>>();
    }
}
#[doc = "Calculate fibonacci number recursively."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
#[doc = "Sort array using quicksort algorithm."]
#[doc = " Depyler: proven to terminate"]
pub fn quicksort(arr: Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 <= 1;
    if _cse_temp_1 {
        return Ok(arr);
    }
    let pivot = arr
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let less = {
        let base = &arr;
        let start_idx = 1 as isize;
        let start = if start_idx < 0 {
            (base.len() as isize + start_idx).max(0) as usize
        } else {
            start_idx as usize
        };
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    }
    .into_iter()
    .filter(|x| {
        let x = x.clone();
        x < pivot
    })
    .map(|x| x)
    .collect::<Vec<_>>();
    let greater = {
        let base = &arr;
        let start_idx = 1 as isize;
        let start = if start_idx < 0 {
            (base.len() as isize + start_idx).max(0) as usize
        } else {
            start_idx as usize
        };
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    }
    .into_iter()
    .filter(|x| {
        let x = x.clone();
        x >= pivot
    })
    .map(|x| x)
    .collect::<Vec<_>>();
    Ok(quicksort(less)?
        .iter()
        .chain(vec![pivot].iter())
        .cloned()
        .collect::<Vec<_>>()
        .iter()
        .chain(quicksort(greater)?.iter())
        .cloned()
        .collect::<Vec<_>>())
}
#[doc = "Process data using functional pipeline style."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_data(numbers: &Vec<i32>) -> Vec<i32> {
    let result = numbers
        .as_slice()
        .iter()
        .cloned()
        .filter(|x| {
            let x = x.clone();
            x > 0
        })
        .map(|x| x * 2)
        .collect::<Vec<_>>();
    result
}
#[doc = "Create a greeting with optional title."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn greet(name: String, title: &Option<String>) -> String {
    if let Some(ref title_val) = title {
        format!("Hello, {:?} {}!", title_val, name)
    } else {
        format!("Hello, {}!", name)
    }
}
#[doc = "Async function that will map to Ruchy's async support."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fetch_and_process(url: &str) -> String {
    let data = fetch_data(url.to_string());
    let processed = process_text(&data);
    processed.to_string()
}
#[doc = "Simulate fetching data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fetch_data(url: String) -> String {
    format!("Data from {}", url)
}
#[doc = "Process text data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_text(text: &str) -> String {
    text.to_uppercase()
}
#[doc = "Example that could be transformed to match expression."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn pattern_matching_example(value: &str) -> String {
    if true {
        return format!("Integer: {}", value);
    } else {
        if true {
            return format!("String: {}", value);
        } else {
            if true {
                return format!("List with {} items", value.len() as i32);
            } else {
                return "Unknown type".to_string();
            }
        }
    }
}
#[doc = "Main entry point."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", format!("Fibonacci(10) = {}", fibonacci(10)));
    let arr = vec![64, 34, 25, 12, 22, 11, 90];
    let sorted_arr = quicksort(arr)?;
    println!("{}", format!("Sorted array: {:?}", sorted_arr));
    let numbers = vec![1, -2, 3, -4, 5];
    let processed = process_data(&numbers);
    println!("{}", format!("Processed: {:?}", processed));
    println!("{}", greet("Alice".to_string(), &None));
    println!("{}", greet("Bob".to_string(), &Some("Dr.".to_string())));
    let processor = DataProcessor::new(10);
    let data = vec![5, 10, 15, 20, 25];
    let filtered = processor.filter_data(data);
    let transformed = processor.transform_data(filtered);
    println!("{}", format!("Filtered and transformed: {:?}", transformed));
    println!("{}", pattern_matching_example(42));
    println!("{}", pattern_matching_example("hello"));
    println!("{}", pattern_matching_example(&vec![1, 2, 3]));
    Ok(())
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
    #[test]
    fn quickcheck_quicksort() {
        fn prop(arr: Vec<i32>) -> TestResult {
            let input_len = arr.len();
            let result = quicksort(&arr);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = quicksort(&arr);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = arr.clone();
            input_sorted.sort();
            let mut result = quicksort(&arr);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_quicksort_examples() {
        assert_eq!(quicksort(vec![]), vec![]);
        assert_eq!(quicksort(vec![1]), vec![1]);
    }
    #[test]
    fn test_process_data_examples() {
        assert_eq!(process_data(vec![]), vec![]);
        assert_eq!(process_data(vec![1]), vec![1]);
    }
}
