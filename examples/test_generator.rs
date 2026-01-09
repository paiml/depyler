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
#[doc = "A simple generator that yields numbers"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Generator state struct"]
#[derive(Debug)]
struct SimpleGeneratorState {
    state: usize,
    i: i32,
    n: i32,
}
#[doc = " Generator function - returns Iterator"]
pub fn simple_generator(n: i32) -> impl Iterator<Item = i32> {
    SimpleGeneratorState {
        state: 0,
        i: 0,
        n: n,
    }
}
impl Iterator for SimpleGeneratorState {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.i = 0;
                self.state = 1;
                self.next()
            }
            1 => {
                if self.i < self.n {
                    let result = self.i;
                    self.i = self.i + 1;
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
#[doc = "Generate Fibonacci numbers"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Generator state struct"]
#[derive(Debug)]
struct FibonacciGeneratorState {
    state: usize,
    a: i32,
    b: i32,
    count: i32,
    n: i32,
}
#[doc = " Generator function - returns Iterator"]
pub fn fibonacci_generator(n: i32) -> impl Iterator<Item = i32> {
    FibonacciGeneratorState {
        state: 0,
        a: 0,
        b: 0,
        count: 0,
        n: n,
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
                if self.count < self.n {
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
#[doc = "Test generator usage"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_generator() -> i32 {
    42
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_simple_generator_examples() {
        assert_eq!(simple_generator(0), 0);
        assert_eq!(simple_generator(1), 1);
        assert_eq!(simple_generator(-1), -1);
    }
    #[test]
    fn test_fibonacci_generator_examples() {
        assert_eq!(fibonacci_generator(0), 0);
        assert_eq!(fibonacci_generator(1), 1);
        assert_eq!(fibonacci_generator(-1), -1);
    }
    #[test]
    fn test_test_generator_examples() {
        let _ = test_generator();
    }
}
