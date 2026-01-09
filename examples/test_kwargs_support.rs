#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
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
#[derive(Debug, Clone)]
pub struct MyObject {
    pub mode: DepylerValue,
    pub timeout: DepylerValue,
    pub retry: DepylerValue,
}
impl MyObject {
    pub fn new(mode: DepylerValue, timeout: DepylerValue, retry: DepylerValue) -> Self {
        Self {
            mode,
            timeout,
            retry,
        }
    }
    pub fn setup(&mut self, mode: String, timeout: i32, retry: bool) {
        self.mode = mode;
        self.timeout = timeout;
        self.retry = retry;
    }
}
#[doc = "Test function calls with keyword arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_function_kwargs() -> (String, i32, std::collections::HashMap<String, DepylerValue>) {
    let result1 = greet("Alice".to_string(), "Hello".to_string());
    let result2 = calculate(10, 20, "add".to_string(), true);
    let result3 = configure(800, 600, "My App".to_string());
    (result1, result2, result3)
}
#[doc = "Test method calls with keyword arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_method_kwargs() -> String {
    let mut obj = MyObject::new();
    obj.setup();
    let text = "hello world";
    let formatted = text.replace("world", "Python");
    formatted.to_string()
}
#[doc = "Test builtin functions with keyword arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_builtin_kwargs() -> Result<HashMap<String, DepylerValue>, std::io::Error> {
    let f = std::fs::File::open("data.txt")?;
    let config = std::collections::HashMap::new();
    Ok(config)
}
#[doc = "Test nested function calls with kwargs"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_nested_kwargs() -> f64 {
    let result = outer(inner(10f64, 20f64), 2.0, Some(inner(5f64, 5f64)));
    result
}
#[doc = "Test kwargs with complex expressions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_complex_kwargs() -> HashMap<String, DepylerValue> {
    let settings = configure(
        100 + 200,
        get_height(),
        (true) && (!false),
        format!("{}{}", "App ".to_string(), (42).to_string()),
    );
    settings
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn greet(name: String, greeting: String) -> String {
    format!("{}, {}!", greeting, name)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate(a: i32, b: i32, operation: &str, verbose: bool) -> i32 {
    let mut result: () = Default::default();
    let _cse_temp_0 = operation == "add";
    if _cse_temp_0 {
        result = a + b;
    } else {
        result = a - b;
    }
    if verbose {
        println!("{}", format!("Result: {}", result));
    }
    result
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn configure(width: i32, height: i32, title: &str) -> HashMap<String, DepylerValue> {
    {
        let mut map = HashMap::new();
        map.insert("width".to_string(), DepylerValue::Int(width as i64));
        map.insert("height".to_string(), DepylerValue::Int(height as i64));
        map.insert(
            std::borrow::Cow::Borrowed("title").to_string(),
            DepylerValue::Str(title.to_string()),
        );
        map
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn outer(inner_result: f64, scale: f64, offset: &Option<DepylerValue>) -> f64 {
    inner_result * scale + offset
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn inner(x: f64, y: f64) -> f64 {
    x + y
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_height() -> i32 {
    600
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_inner() {
        fn prop(x: f64, y: f64) -> TestResult {
            if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                return TestResult::discard();
            }
            let result1 = inner(x.clone(), y.clone());
            let result2 = inner(y.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(f64, f64) -> TestResult);
    }
    #[test]
    fn test_get_height_examples() {
        let _ = get_height();
    }
}
