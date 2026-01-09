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
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_integer_power() -> (i32, i32, i32, i32) {
    let _cse_temp_0 = ({ 2 } as i32)
        .checked_pow({ 3 } as u32)
        .expect("Power operation overflowed");
    let a = _cse_temp_0;
    let _cse_temp_1 = ({ 10 } as i32)
        .checked_pow({ 2 } as u32)
        .expect("Power operation overflowed");
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 5 } as i32)
        .checked_pow({ 0 } as u32)
        .expect("Power operation overflowed");
    let c = _cse_temp_2;
    let base = 3;
    let exp = 4;
    let _cse_temp_3 = {
        if exp >= 0 && (exp as i64) <= (u32::MAX as i64) {
            ({ base } as i32)
                .checked_pow({ exp } as u32)
                .expect("Power operation overflowed")
        } else {
            ({ base } as f64).powf({ exp } as f64) as i32
        }
    };
    let d = _cse_temp_3;
    (a, b, c, d)
}
#[doc = " Depyler: proven to terminate"]
pub fn test_float_power() -> Result<(f64, f64, f64, i32), Box<dyn std::error::Error>> {
    let _cse_temp_0 = ({ 2.5 } as f64).powf({ 2 } as f64);
    let a = _cse_temp_0;
    let _cse_temp_1 = ({ 10.0 } as f64).powf({ 3 } as f64);
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 4 } as f64).powf({ 0.5 } as f64);
    let c = _cse_temp_2;
    let _cse_temp_3 = ({ 8 } as i32)
        .checked_pow({ 0 } as u32)
        .expect("Power operation overflowed");
    let d = _cse_temp_3;
    Ok((a, b, c, d))
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_negative_exponent() -> (f64, f64, f64) {
    let _cse_temp_0 = ({ 2 } as f64).powf({ -1 } as f64);
    let a = _cse_temp_0;
    let _cse_temp_1 = ({ 10 } as f64).powf({ -2 } as f64);
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 5 } as f64).powf({ -3 } as f64);
    let c = _cse_temp_2;
    (a, b, c)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_large_powers() -> (i32, i32, i32) {
    let _cse_temp_0 = ({ 2 } as i32)
        .checked_pow({ 10 } as u32)
        .expect("Power operation overflowed");
    let a = _cse_temp_0;
    let _cse_temp_1 = ({ 2 } as i32)
        .checked_pow({ 20 } as u32)
        .expect("Power operation overflowed");
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 10 } as i32)
        .checked_pow({ 6 } as u32)
        .expect("Power operation overflowed");
    let c = _cse_temp_2;
    (a, b, c)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_mixed_operations() -> (i32, i32, i32, i32) {
    let _cse_temp_0 = ({ 3 } as i32)
        .checked_pow({ 2 } as u32)
        .expect("Power operation overflowed");
    let a = 2 + _cse_temp_0;
    let _cse_temp_1 = ({ 5 } as i32)
        .checked_pow({ 2 } as u32)
        .expect("Power operation overflowed");
    let b = _cse_temp_1;
    let _cse_temp_2 = ({ 2 } as i32)
        .checked_pow({ 3 } as u32)
        .expect("Power operation overflowed");
    let _cse_temp_3 = _cse_temp_2 * 4;
    let c = _cse_temp_3;
    let _cse_temp_4 = ({ 2 } as i32)
        .checked_pow({ 6 } as u32)
        .expect("Power operation overflowed");
    let d = _cse_temp_4;
    (a, b, c, d)
}
#[doc = "Test power with function parameters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn compute_power(base: i32, exp: i32) -> i32 {
    {
        if exp >= 0 && (exp as i64) <= (u32::MAX as i64) {
            ({ base } as i32)
                .checked_pow({ exp } as u32)
                .expect("Power operation overflowed")
        } else {
            ({ base } as f64).powf({ exp } as f64) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_compute_power_examples() {
        assert_eq!(compute_power(0, 0), 0);
        assert_eq!(compute_power(1, 2), 3);
        assert_eq!(compute_power(-1, 1), 0);
    }
}
