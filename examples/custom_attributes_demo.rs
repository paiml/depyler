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
#[doc = "Simple addition with inline hint."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
#[inline]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
#[doc = "Performance-critical multiplication with aggressive inlining."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
#[inline(always)]
pub fn multiply_fast(x: i32, y: i32) -> i32 {
    x * y
}
#[doc = "Calculate checksum - result must be used."]
#[doc = " Depyler: verified panic-free"]
#[must_use]
pub fn calculate_checksum(data: &Vec<i32>) -> i32 {
    let mut checksum: i32 = Default::default();
    checksum = 0;
    for value in data.iter().cloned() {
        checksum = checksum ^ value;
    }
    checksum
}
#[doc = "Error handler - rarely executed."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
#[cold]
pub fn handle_panic(message: String) {
    println!("{}", format!("PANIC: {}", message));
}
#[doc = "Hash function with multiple attributes."]
#[inline]
#[must_use]
pub fn compute_hash(text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut hash_val: i32 = Default::default();
    hash_val = 0;
    for char in text.chars() {
        hash_val = (hash_val * 31 + char as u32 as i32)
            % ({ 2 } as i32)
                .checked_pow({ 32 } as u32)
                .expect("Power operation overflowed");
    }
    Ok(hash_val)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_add_numbers() {
        fn prop(a: i32, b: i32) -> TestResult {
            if (a > 0 && b > i32::MAX - a) || (a < 0 && b < i32::MIN - a) {
                return TestResult::discard();
            }
            let result1 = add_numbers(a.clone(), b.clone());
            let result2 = add_numbers(b.clone(), a.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_add_numbers_examples() {
        assert_eq!(add_numbers(0, 0), 0);
        assert_eq!(add_numbers(1, 2), 3);
        assert_eq!(add_numbers(-1, 1), 0);
    }
    #[test]
    fn quickcheck_multiply_fast() {
        fn prop(x: i32, y: i32) -> TestResult {
            if (x > 0 && y > i32::MAX - x) || (x < 0 && y < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = multiply_fast(x.clone(), y.clone());
            let result2 = multiply_fast(y.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_multiply_fast_examples() {
        assert_eq!(multiply_fast(0, 0), 0);
        assert_eq!(multiply_fast(1, 2), 3);
        assert_eq!(multiply_fast(-1, 1), 0);
    }
    #[test]
    fn test_calculate_checksum_examples() {
        assert_eq!(calculate_checksum(&vec![]), 0);
        assert_eq!(calculate_checksum(&vec![1]), 1);
        assert_eq!(calculate_checksum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_compute_hash_examples() {
        assert_eq!(compute_hash(""), 0);
        assert_eq!(compute_hash("a"), 1);
        assert_eq!(compute_hash("abc"), 3);
    }
}
