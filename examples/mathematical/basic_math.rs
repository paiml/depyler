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
#[doc = "Calculate factorial using iteration"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn factorial(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    result = 1;
    for i in (2)..(n + 1) {
        result = result * i;
    }
    result
}
#[doc = "Greatest common divisor using Euclidean algorithm"]
pub fn gcd(mut a: i32, mut b: i32) -> Result<i32, Box<dyn std::error::Error>> {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    Ok(a)
}
#[doc = "Check if number is prime"]
pub fn is_prime(n: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let _cse_temp_0 = n < 2;
    if _cse_temp_0 {
        return Ok(false);
    }
    let _cse_temp_1 = n == 2;
    if _cse_temp_1 {
        return Ok(true);
    }
    let _cse_temp_2 = n % 2;
    let _cse_temp_3 = _cse_temp_2 == 0;
    if _cse_temp_3 {
        return Ok(false);
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return Ok(false);
        }
        i = i + 2;
    }
    Ok(true)
}
#[doc = "Calculate sum of squares"]
#[doc = " Depyler: verified panic-free"]
pub fn sum_of_squares(numbers: &Vec<i32>) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for num in numbers.iter().cloned() {
        total = total + num * num;
    }
    total
}
#[doc = "Calculate power using exponentiation by squaring"]
pub fn power(mut base: i32, mut exponent: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = exponent == 0;
    if _cse_temp_0 {
        return Ok(1);
    }
    let _cse_temp_1 = exponent < 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    result = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = result * base;
        }
        base = base * base;
        exponent = {
            let a = exponent;
            let b = 2;
            let q = a / b;
            let r = a % b;
            let r_negative = r < 0;
            let b_negative = b < 0;
            let r_nonzero = r != 0;
            let signs_differ = r_negative != b_negative;
            let needs_adjustment = r_nonzero && signs_differ;
            if needs_adjustment {
                q - 1
            } else {
                q
            }
        };
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_factorial_examples() {
        assert_eq!(factorial(0), 0);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(-1), -1);
    }
    #[test]
    fn test_gcd_examples() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(1, 2), 3);
        assert_eq!(gcd(-1, 1), 0);
    }
    #[test]
    fn test_is_prime_examples() {
        let _ = is_prime(Default::default());
    }
    #[test]
    fn test_sum_of_squares_examples() {
        assert_eq!(sum_of_squares(&vec![]), 0);
        assert_eq!(sum_of_squares(&vec![1]), 1);
        assert_eq!(sum_of_squares(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_power_examples() {
        assert_eq!(power(0, 0), 0);
        assert_eq!(power(1, 2), 3);
        assert_eq!(power(-1, 1), 0);
    }
}
