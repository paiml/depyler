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
#[doc = "Calculate nth Fibonacci number recursively."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_recursive(n: i32) -> i32 {
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return 0;
    } else {
        let _cse_temp_1 = n == 1;
        if _cse_temp_1 {
            return 1;
        } else {
            return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
        }
    }
}
#[doc = "Calculate nth Fibonacci number iteratively."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_iterative(n: i32) -> i32 {
    let mut curr: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return 0;
    } else {
        let _cse_temp_1 = n == 1;
        if _cse_temp_1 {
            return 1;
        }
    }
    let (mut prev, mut curr) = (0, 1);
    for __sanitized in (2)..(n + 1) {
        (prev, curr) = (curr, prev + curr);
    }
    curr
}
#[doc = "Generate Fibonacci sequence up to n terms."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_sequence(limit: i32) -> Vec<i32> {
    let _cse_temp_0 = limit <= 0;
    if _cse_temp_0 {
        return vec![];
    }
    let mut sequence: Vec<i32> = vec![];
    let (mut a, mut b) = (0, 1);
    for __sanitized in 0..(limit) {
        sequence.push(a);
        (a, b) = (b, a + b);
    }
    sequence
}
#[doc = "Generate Fibonacci numbers as an iterator."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Generator state struct"]
#[derive(Debug)]
struct FibonacciGeneratorState {
    state: usize,
    a: i32,
    b: i32,
    count: i32,
    limit: Option<i32>,
}
#[doc = " Generator function - returns Iterator"]
pub fn fibonacci_generator(limit: &Option<i32>) -> impl Iterator<Item = i32> {
    FibonacciGeneratorState {
        state: 0,
        a: 0,
        b: 0,
        count: 0,
        limit: *limit,
    }
}
impl Iterator for FibonacciGeneratorState {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                let _tuple_temp = (0, 1);
                self.a = _tuple_temp.0;
                self.b = _tuple_temp.1;
                self.count = 0;
                self.state = 1;
                self.next()
            }
            1 => {
                if (self.limit.is_none()) || (self.count < self.limit.unwrap_or(i32::MAX)) {
                    let result = self.a;
                    let _tuple_temp = (self.b, self.a + self.b);
                    self.a = _tuple_temp.0;
                    self.b = _tuple_temp.1;
                    self.count = self.count + 1;
                    return Some(result);
                } else {
                    self.state = 2;
                    None
                }
            }
            _ => None,
        }
    }
}
#[doc = "Calculate Fibonacci with memoization."]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_memoized(
    n: i32,
    memo: &mut Option<std::collections::HashMap<String, DepylerValue>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    if memo.is_none() {
        *memo = Some({
            let map: HashMap<String, String> = HashMap::new();
            map
        });
    }
    let _cse_temp_0 = memo.as_ref().unwrap().get(&n).is_some();
    if _cse_temp_0 {
        return Ok(memo.as_ref().unwrap().get(&n).cloned().unwrap_or_default());
    }
    let _cse_temp_1 = n <= 0;
    if _cse_temp_1 {
        return Ok(0);
    } else {
        let _cse_temp_2 = n == 1;
        if _cse_temp_2 {
            return Ok(1);
        }
    }
    let _cse_temp_3 = fibonacci_memoized(n - 1, memo)? + fibonacci_memoized(n - 2, memo)?;
    let result = _cse_temp_3;
    memo.as_mut().unwrap().insert(n.clone(), result);
    Ok(result)
}
#[doc = "Find the index of a target value in Fibonacci sequence."]
#[doc = " Depyler: verified panic-free"]
pub fn find_fibonacci_index(target: i32) -> Option<i32> {
    let mut index: i32 = Default::default();
    let mut a: i32 = Default::default();
    let _cse_temp_0 = target < 0;
    if _cse_temp_0 {
        return None;
    }
    let (mut a, mut b) = (0, 1);
    index = 0;
    while a < target {
        (a, b) = (b, a + b);
        index = index + 1;
    }
    if a == target {
        Some(index)
    } else {
        None
    }
}
#[doc = "Check if a number is in the Fibonacci sequence."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_fibonacci_number(num: i32) -> bool {
    let _cse_temp_0 = num < 0;
    if _cse_temp_0 {
        return false;
    }
    let is_perfect_square = move |x: i32| -> bool {
        let root = (({ x } as f64).powf({ 0.5 } as f64)) as i32;
        return root * root == x;
    };
    (is_perfect_square(5 * num * num + 4)) || (is_perfect_square(5 * num * num - 4))
}
#[doc = "Test the Fibonacci functions."]
#[doc = " Depyler: verified panic-free"]
pub fn main() {
    let n = 10;
    println!(
        "{}",
        format!("Fibonacci({}) recursive: {}", n, fibonacci_recursive(n))
    );
    println!(
        "{}",
        format!("Fibonacci({}) iterative: {}", n, fibonacci_iterative(n))
    );
    println!(
        "{}",
        format!(
            "Fibonacci({}) memoized: {:?}",
            n,
            fibonacci_memoized(n, &None)
        )
    );
    println!(
        "{}",
        format!("\nFirst {} Fibonacci numbers: {}", n, fibonacci_sequence(n))
    );
    println!("{}", "\nUsing generator:");
    for (i, fib) in fibonacci_generator(&Some(n))
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| (i as i32, x))
    {
        let i = i as i32;
        println!("{}", format!("  F({:?}) = {:?}", i, fib));
    }
    let target = 21;
    let index = find_fibonacci_index(target);
    if index.is_some() {
        println!(
            "{}",
            format!("\n{} is at index {:?} in Fibonacci sequence", target, index)
        );
    } else {
        println!("{}", format!("\n{} is not in Fibonacci sequence", target));
    }
    let test_nums = vec![0, 1, 2, 3, 4, 5, 8, 13, 20, 21];
    println!("{}", "\nFibonacci number check:");
    for num in test_nums.iter().cloned() {
        println!("{}", format!("  {}: {}", num, is_fibonacci_number(num)));
    }
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
    fn test_is_fibonacci_number_examples() {
        let _ = is_fibonacci_number(Default::default());
    }
}
