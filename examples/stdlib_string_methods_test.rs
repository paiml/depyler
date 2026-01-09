#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR_HELLO: &'static str = "hello";
const STR_HELLO_WORLD: &'static str = "hello world";
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
#[doc = "Test str.upper() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_upper() -> String {
    let text = STR_HELLO_WORLD;
    let result = text.to_uppercase();
    result.to_string()
}
#[doc = "Test str.lower() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_lower() -> String {
    let text = "HELLO WORLD";
    let result = text.to_lowercase();
    result.to_string()
}
#[doc = "Test str.strip() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_strip() -> String {
    let text = "  hello world  ";
    let result = text.trim().to_string();
    result.to_string()
}
#[doc = "Test str.startswith() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_startswith() -> bool {
    let text = STR_HELLO_WORLD;
    let result = text.starts_with("hello");
    result
}
#[doc = "Test str.startswith() returns False"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_startswith_false() -> bool {
    let text = STR_HELLO_WORLD;
    let result = text.starts_with("world");
    result
}
#[doc = "Test str.endswith() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_endswith() -> bool {
    let text = STR_HELLO_WORLD;
    let result = text.ends_with("world");
    result
}
#[doc = "Test str.endswith() returns False"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_endswith_false() -> bool {
    let text = STR_HELLO_WORLD;
    let result = text.ends_with("hello");
    result
}
#[doc = "Test str.split() with default whitespace"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_split_whitespace() -> i32 {
    let text = "hello world foo bar";
    let parts = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    parts.len() as i32 as i32
}
#[doc = "Test str.split(sep) with custom separator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_split_separator() -> i32 {
    let text = "hello,world,foo,bar";
    let parts = text
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    parts.len() as i32 as i32
}
#[doc = "Test str.join () method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_join() -> String {
    let parts = vec![STR_HELLO.to_string(), "world".to_string()];
    let result = parts.join(",");
    result.to_string()
}
#[doc = "Test str.join () with space separator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_join_space() -> String {
    let parts = vec![
        STR_HELLO.to_string(),
        "world".to_string(),
        "foo".to_string(),
    ];
    let result = parts.join(" ");
    result.to_string()
}
#[doc = "Test str.find() when substring exists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_find_found() -> i32 {
    let text = STR_HELLO_WORLD;
    let pos = text.find("world").map(|i| i as i32).unwrap_or(-1);
    pos
}
#[doc = "Test str.find() when substring doesn't exist"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_find_not_found() -> i32 {
    let text = STR_HELLO_WORLD;
    let pos = text.find("xyz").map(|i| i as i32).unwrap_or(-1);
    pos
}
#[doc = "Test str.replace() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_replace() -> String {
    let text = STR_HELLO_WORLD;
    let result = text.replace("world", "rust");
    result.to_string()
}
#[doc = "Test str.replace() with multiple occurrences"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_replace_multiple() -> String {
    let text = "hello hello hello";
    let result = text.replace("hello", "hi");
    result.to_string()
}
#[doc = "Test str.count() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count() -> i32 {
    let text = "hello hello world";
    let count = text.matches("hello").count() as i32;
    count
}
#[doc = "Test str.count() with single occurrence"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count_single() -> i32 {
    let text = STR_HELLO_WORLD;
    let count = text.matches("world").count() as i32;
    count
}
#[doc = "Test str.count() with no occurrences"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count_none() -> i32 {
    let text = STR_HELLO_WORLD;
    let count = text.matches("xyz").count() as i32;
    count
}
#[doc = "Test str.isdigit() returns True for digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isdigit_true() -> bool {
    let text = "12345";
    let result = text.chars().all(|c| c.is_numeric());
    result
}
#[doc = "Test str.isdigit() returns False for non-digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isdigit_false() -> bool {
    let text = STR_HELLO;
    let result = text.chars().all(|c| c.is_numeric());
    result
}
#[doc = "Test str.isalpha() returns True for letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isalpha_true() -> bool {
    let text = STR_HELLO;
    let result = text.chars().all(|c| c.is_alphabetic());
    result
}
#[doc = "Test str.isalpha() returns False for non-letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isalpha_false() -> bool {
    let text = "hello123";
    let result = text.chars().all(|c| c.is_alphabetic());
    result
}
#[doc = "Test split on empty string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_empty_split() -> i32 {
    let text = "";
    let parts = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    parts.len() as i32 as i32
}
#[doc = "Test string methods on single character"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_single_char() -> String {
    let text = "a";
    let result = text.to_uppercase();
    result.to_string()
}
#[doc = "Test string methods with special characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_special_chars() -> bool {
    let text = "hello-world_123";
    let result = text.starts_with("hello");
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_split_whitespace_examples() {
        let _ = test_str_split_whitespace();
    }
    #[test]
    fn test_test_str_split_separator_examples() {
        let _ = test_str_split_separator();
    }
    #[test]
    fn test_test_str_find_found_examples() {
        let _ = test_str_find_found();
    }
    #[test]
    fn test_test_str_find_not_found_examples() {
        let _ = test_str_find_not_found();
    }
    #[test]
    fn test_test_str_count_examples() {
        let _ = test_str_count();
    }
    #[test]
    fn test_test_str_count_single_examples() {
        let _ = test_str_count_single();
    }
    #[test]
    fn test_test_str_count_none_examples() {
        let _ = test_str_count_none();
    }
    #[test]
    fn test_test_str_empty_split_examples() {
        let _ = test_str_empty_split();
    }
}
