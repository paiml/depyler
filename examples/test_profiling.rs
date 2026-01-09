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
#[doc = "Recursive Fibonacci - will be identified as hot path."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_recursive(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}
#[doc = "Iterative Fibonacci - more efficient."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_iterative(n: i32) -> i32 {
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
#[doc = "Process a list with nested loops - O(n²) complexity."]
#[doc = " Depyler: proven to terminate"]
pub fn process_list(items: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    total = 0;
    for i in 0..(items.len() as i32) {
        for j in (i)..(items.len() as i32) {
            if items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < items
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                total = total
                    + items
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        * items
                            .get(j as usize)
                            .cloned()
                            .expect("IndexError: list index out of range");
            }
        }
    }
    Ok(total)
}
#[doc = "String concatenation in loop - inefficient pattern."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn string_concatenation_in_loop(n: i32) -> String {
    let mut result: String = Default::default();
    result = "".to_string();
    for i in 0..(n) {
        result = format!("{}{}", result, format!("Item {}, ", i));
    }
    result.to_string()
}
#[doc = "Function with many allocations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn allocate_many_lists(n: i32) -> Vec<Vec<i32>> {
    let mut results = vec![];
    for i in 0..(n) {
        let mut inner_list = vec![];
        for j in 0..(10) {
            inner_list.push(i * j);
        }
        results.push(inner_list);
    }
    results
}
#[doc = "Function with many type checks that Rust can optimize away."]
#[doc = " Depyler: verified panic-free"]
pub fn type_check_heavy(values: &Vec<DepylerValue>) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    for value in values.iter().cloned() {
        if true {
            count = count + value;
        } else {
            if true {
                count = count + value.len() as i32;
            } else {
                if true {
                    count = count + value.len() as i32;
                }
            }
        }
    }
    count
}
#[doc = "Matrix multiplication - triple nested loop."]
#[doc = " Depyler: proven to terminate"]
pub fn matrix_multiply<'a, 'b>(
    a: &'a Vec<Vec<f64>>,
    b: &'b Vec<Vec<f64>>,
) -> Result<Vec<Vec<f64>>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = a.len() as i32;
    let rows_a = _cse_temp_0;
    let cols_a = if !a.is_empty() {
        a.get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .len() as i32
    } else {
        0
    };
    let cols_b = if !b.is_empty() {
        b.get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .len() as i32
    } else {
        0
    };
    let result = (0..(rows_a))
        .into_iter()
        .map(|_| (0..(cols_b)).into_iter().map(|_| 0.0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..(rows_a) {
        for j in 0..(cols_b) {
            for k in 0..(cols_a) {
                result.get_mut(&i).unwrap().insert(
                    (j) as usize,
                    result
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        + a.get(i as usize)
                            .cloned()
                            .expect("IndexError: list index out of range")
                            .get(&k)
                            .cloned()
                            .unwrap_or_default()
                            * b.get(&k)
                                .cloned()
                                .unwrap_or_default()
                                .get(j as usize)
                                .cloned()
                                .expect("IndexError: list index out of range"),
                );
            }
        }
    }
    Ok(result)
}
#[doc = "Simple function for baseline comparison."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_function(x: i32, y: i32) -> i32 {
    x + y
}
#[doc = "Main entry point with various function calls."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..(5) {
        fibonacci_recursive(i);
    }
    fibonacci_iterative(30);
    let test_list = (0..(100)).collect::<Vec<_>>();
    process_list(&test_list);
    string_concatenation_in_loop(100);
    allocate_many_lists(50);
    let mixed_values = vec![
        format!("{:?}", 1),
        format!("{:?}", "hello"),
        format!("{:?}", vec![1, 2, 3]),
        format!("{:?}", 42),
        format!("{:?}", "world"),
    ];
    type_check_heavy(&mixed_values);
    let mat_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let mat_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    matrix_multiply(&mat_a, &mat_b);
    simple_function(10, 20);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_fibonacci_recursive_examples() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(-1), -1);
    }
    #[test]
    fn test_fibonacci_iterative_examples() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(-1), -1);
    }
    #[test]
    fn test_process_list_examples() {
        assert_eq!(process_list(&vec![]), 0);
        assert_eq!(process_list(&vec![1]), 1);
        assert_eq!(process_list(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_type_check_heavy_examples() {
        assert_eq!(type_check_heavy(&vec![]), 0);
        assert_eq!(type_check_heavy(&vec![1]), 1);
        assert_eq!(type_check_heavy(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn quickcheck_simple_function() {
        fn prop(x: i32, y: i32) -> TestResult {
            if (x > 0 && y > i32::MAX - x) || (x < 0 && y < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = simple_function(x.clone(), y.clone());
            let result2 = simple_function(y.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_simple_function_examples() {
        assert_eq!(simple_function(0, 0), 0);
        assert_eq!(simple_function(1, 2), 3);
        assert_eq!(simple_function(-1, 1), 0);
    }
}
