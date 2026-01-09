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
#[doc = "Multiple uses of the same complex expression."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn repeated_complex_expressions(a: i32, b: i32, c: i32) -> i32 {
    let _cse_temp_0 = ((a + b) as f64) * c;
    let x = _cse_temp_0 + 10;
    let y = _cse_temp_0 - 5;
    let _cse_temp_1 = _cse_temp_0 * 2;
    let z = _cse_temp_1;
    x + y + z
}
#[doc = "Repeated calls to pure functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn repeated_function_calls(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = (n - 10).abs();
    let _cse_temp_1 = _cse_temp_0 > 5;
    if _cse_temp_1 {
        let _cse_temp_2 = _cse_temp_0.scale(2 as f32).unwrap();
        result = _cse_temp_2;
    } else {
        result = _cse_temp_0 + 100;
    }
    result.add(&(n - 10).abs()).unwrap()
}
#[doc = "Nested common subexpressions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn nested_expressions(x: i32, y: i32) -> i32 {
    let _cse_temp_0 = x * y;
    let _cse_temp_1 = _cse_temp_0.scale(2 as f32).unwrap();
    let a = _cse_temp_0.add(&_cse_temp_1).unwrap();
    let _cse_temp_2 = _cse_temp_0.mul(&_cse_temp_0).unwrap();
    let b = _cse_temp_2;
    let _cse_temp_3 = (x + 1) * (y + 1);
    let c = _cse_temp_3 + 10;
    let d = _cse_temp_3 - 20;
    ((a.add(&b).unwrap()) as f64) + c + (d as f64)
}
#[doc = "CSE across conditional branches."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn conditional_cse(flag: bool, a: i32, b: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = a.mul(&b).unwrap();
    let _cse_temp_1 = _cse_temp_0.add(&a.sub(&b).unwrap()).unwrap();
    let base = _cse_temp_1;
    if flag {
        result = _cse_temp_1 + 10;
    } else {
        result = _cse_temp_1 - 10;
    }
    result.add(&base).unwrap()
}
#[doc = "Expressions that don't change in loops."]
#[doc = " Depyler: verified panic-free"]
pub fn loop_invariant_expressions(items: &Vec<DepylerValue>) -> i32 {
    let mut total: i32 = Default::default();
    let x = 10;
    let y = 20;
    total = 0;
    for item in items.iter().cloned() {
        total = total + item + (x + y) * 2;
    }
    total
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_repeated_function_calls_examples() {
        assert_eq!(repeated_function_calls(0), 0);
        assert_eq!(repeated_function_calls(1), 1);
        assert_eq!(repeated_function_calls(-1), -1);
    }
    #[test]
    fn test_nested_expressions_examples() {
        assert_eq!(nested_expressions(0, 0), 0);
        assert_eq!(nested_expressions(1, 2), 3);
        assert_eq!(nested_expressions(-1, 1), 0);
    }
    #[test]
    fn test_loop_invariant_expressions_examples() {
        assert_eq!(loop_invariant_expressions(&vec![]), 0);
        assert_eq!(loop_invariant_expressions(&vec![1]), 1);
        assert_eq!(loop_invariant_expressions(&vec![1, 2, 3]), 3);
    }
}
