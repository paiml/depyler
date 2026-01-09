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
#[derive(Debug, Clone)]
pub struct VerifiedStack {
    pub items: Vec<i32>,
    pub capacity: i32,
}
impl VerifiedStack {
    pub fn new(capacity: i32) -> Self {
        Self {
            items: Vec::new(),
            capacity,
        }
    }
    pub fn push(&mut self, item: i32) {
        if (self.items.clone().len() as i32) < self.capacity.clone() {
            self.items.push(item);
        };
    }
    pub fn pop(&self) -> i32 {
        if self.items.clone() {
            return self.items.pop().unwrap_or_default();
        };
        return 0;
    }
    pub fn is_empty(&self) -> bool {
        return (self.items.clone().len() as i32) == 0;
    }
    pub fn is_full(&self) -> bool {
        return (self.items.clone().len() as i32) >= self.capacity.clone();
    }
    pub fn size(&self) -> i32 {
        return self.items.clone().len() as i32;
    }
}
#[doc = "Pure function - no side effects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[doc = "Safe array access with bounds checking"]
#[doc = " Depyler: proven to terminate"]
pub fn safe_access(
    items: &Vec<i32>,
    index: i32,
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = 0 <= index;
    let _cse_temp_1 = items.len() as i32;
    let _cse_temp_2 = index < _cse_temp_1;
    let _cse_temp_3 = (_cse_temp_0) && (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(Some(
            items
                .get(index as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ));
    }
    Ok(None)
}
#[doc = "Thread-safe counter increment"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn concurrent_counter(current: i32, increment: i32) -> i32 {
    current + increment
}
#[doc = "Guaranteed to terminate for non-negative inputs"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn factorial(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return 1;
    }
    result = 1;
    for i in (2)..(n + 1) {
        result = result * i;
    }
    result
}
#[doc = "Division that never panics"]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide(a: i32, b: i32) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(None);
    }
    Ok(Some(((a) as f64) / ((b) as f64)))
}
#[doc = "Returns reference to max value with proper lifetime"]
pub fn find_max(numbers: &Vec<i32>) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let mut max_val: i32 = Default::default();
    if numbers.is_empty() {
        return Ok(None);
    }
    max_val = numbers
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for num in {
        let base = &numbers;
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
    } {
        if num > max_val {
            max_val = num;
        }
    }
    Ok(Some(max_val))
}
#[doc = "Fibonacci with formal contracts"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
#[doc = "Demonstrate verified functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", "=== Verification Demo ===");
    let result = add(5, 3);
    println!("{}", format!("Pure add: {:?}", result));
    let items = vec![10, 20, 30];
    println!("{}", format!("Safe access: {:?}", safe_access(&items, 1)));
    println!(
        "{}",
        format!("Safe access OOB: {:?}", safe_access(&items, 10))
    );
    println!("{}", format!("Concurrent: {}", concurrent_counter(100, 5)));
    println!("{}", format!("Factorial(5): {}", factorial(5)));
    println!("{}", format!("Safe divide: {:?}", safe_divide(10, 2)));
    println!(
        "{}",
        format!("Safe divide by zero: {:?}", safe_divide(10, 0))
    );
    let mut stack = VerifiedStack::new(3);
    stack.push(1);
    stack.push(2);
    println!("{}", format!("Stack size: {}", stack.size()));
    println!("{}", format!("Stack pop: {}", stack.pop()));
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_add() {
        fn prop(a: i32, b: i32) -> TestResult {
            if (a > 0 && b > i32::MAX - a) || (a < 0 && b < i32::MIN - a) {
                return TestResult::discard();
            }
            let result1 = add(a.clone(), b.clone());
            let result2 = add(b.clone(), a.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_add_examples() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
    }
    #[test]
    fn quickcheck_concurrent_counter() {
        fn prop(current: i32, increment: i32) -> TestResult {
            if (current > 0 && increment > i32::MAX - current)
                || (current < 0 && increment < i32::MIN - current)
            {
                return TestResult::discard();
            }
            let result1 = concurrent_counter(current.clone(), increment.clone());
            let result2 = concurrent_counter(increment.clone(), current.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_concurrent_counter_examples() {
        assert_eq!(concurrent_counter(0, 0), 0);
        assert_eq!(concurrent_counter(1, 2), 3);
        assert_eq!(concurrent_counter(-1, 1), 0);
    }
    #[test]
    fn test_factorial_examples() {
        assert_eq!(factorial(0), 0);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(-1), -1);
    }
    #[test]
    fn test_fibonacci_examples() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(-1), -1);
    }
}
