#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[doc = "Trivial function - should be inlined."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn square(x: i32) -> i32 {
    x * x
}
#[doc = "Another trivial function."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add_one(n: i32) -> i32 {
    n + 1
}
#[doc = "Should inline the square calls."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn compute_distance_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    square(dx) + square(dy)
}
#[doc = "Called only once - should be inlined."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_single_use(value: i32) -> i32 {
    let _cse_temp_0 = value * 2;
    let temp = _cse_temp_0;
    let result = temp + 10;
    result
}
#[doc = "Main function that uses other functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main_computation(a: i32, b: i32) -> i32 {
    let step1 = process_single_use(a);
    let step2 = add_one(step1);
    let step3 = add_one(b);
    let distance = compute_distance_squared(0, 0, step2, step3);
    distance
}
#[doc = "Recursive function - should NOT be inlined."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn recursive_factorial(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    n * recursive_factorial(n - 1)
}
#[doc = "Contains loop - may not be inlined depending on config."]
#[doc = " Depyler: verified panic-free"]
pub fn has_loop(items: &Vec<DepylerValue>) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for item in items.iter().cloned() {
        total = total + item;
    }
    total
}
#[doc = "Large function - should NOT be inlined."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn large_function(x: i32, y: i32, z: i32) -> i32 {
    let a = x + y;
    let b = y + z;
    let c = z + x;
    let _cse_temp_0 = a * b;
    let d = _cse_temp_0;
    let _cse_temp_1 = (b as f64) * c;
    let e = _cse_temp_1;
    let _cse_temp_2 = c * (a as f64);
    let f = _cse_temp_2;
    let g = d + e;
    let h = e + f;
    let i = f + d;
    let _cse_temp_3 = (g as f64) * (h as f64);
    let j = _cse_temp_3;
    let _cse_temp_4 = h * (i as f64);
    let k = _cse_temp_4;
    let _cse_temp_5 = (i as f64) * g;
    let l = _cse_temp_5;
    let m = (j as f64) + k;
    let n = (k as f64) + (l as f64);
    let o = l + (j as f64);
    let _cse_temp_6 = m + (n as f64) + (o as f64);
    let result = _cse_temp_6;
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_square_examples() {
        assert_eq!(square(0), 0);
        assert_eq!(square(1), 1);
        assert_eq!(square(-1), -1);
    }
    #[test]
    fn test_add_one_examples() {
        assert_eq!(add_one(0), 0);
        assert_eq!(add_one(1), 1);
        assert_eq!(add_one(-1), -1);
    }
    #[test]
    fn test_process_single_use_examples() {
        assert_eq!(process_single_use(0), 0);
        assert_eq!(process_single_use(1), 1);
        assert_eq!(process_single_use(-1), -1);
    }
    #[test]
    fn test_main_computation_examples() {
        assert_eq!(main_computation(0, 0), 0);
        assert_eq!(main_computation(1, 2), 3);
        assert_eq!(main_computation(-1, 1), 0);
    }
    #[test]
    fn test_recursive_factorial_examples() {
        assert_eq!(recursive_factorial(0), 0);
        assert_eq!(recursive_factorial(1), 1);
        assert_eq!(recursive_factorial(-1), -1);
    }
    #[test]
    fn test_has_loop_examples() {
        assert_eq!(has_loop(&vec![]), 0);
        assert_eq!(has_loop(&vec![1]), 1);
        assert_eq!(has_loop(&vec![1, 2, 3]), 3);
    }
}
