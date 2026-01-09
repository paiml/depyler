#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR_EMPTY: &'static str = "";
use std::collections::HashMap;
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
#[doc = "Count word frequencies in text"]
pub fn word_frequency(text: &str) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let words = text
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut frequency: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    for word in words.iter().cloned() {
        let mut clean_word = STR_EMPTY.to_string();
        for char in word.chars() {
            if char.is_alphabetic() {
                clean_word = format!("{}{}", clean_word, char);
            }
        }
        if !clean_word.is_empty() {
            if frequency.get(&clean_word).is_some() {
                {
                    let _key = clean_word;
                    let _old_val = frequency.get(&_key).cloned().unwrap_or_default();
                    frequency.insert(_key, _old_val + 1);
                }
            } else {
                frequency.insert(clean_word.to_string().clone(), 1);
            }
        }
    }
    Ok(frequency)
}
#[doc = "Group words that are anagrams of each other"]
#[doc = " Depyler: verified panic-free"]
pub fn find_anagrams(words: &Vec<String>) -> Vec<Vec<String>> {
    let mut groups: std::collections::HashMap<String, Vec<String>> = {
        let map: HashMap<String, Vec<String>> = HashMap::new();
        map
    };
    for word in words.iter().cloned() {
        let sorted_chars = {
            let mut sorted_vec = word.to_lowercase().iter().cloned().collect::<Vec<_>>();
            sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            sorted_vec
        }
        .join("");
        if groups.get(&sorted_chars).is_some() {
            groups
                .get(&sorted_chars)
                .cloned()
                .unwrap_or_default()
                .push(word);
        } else {
            groups.insert(sorted_chars.to_string().clone(), vec![word]);
        }
    }
    let mut result: Vec<Vec<String>> = vec![];
    for group in groups.values().cloned().collect::<Vec<_>>() {
        if group.len() as i32 > 1 {
            result.push(group);
        }
    }
    result
}
#[doc = "Find the longest common prefix among strings"]
pub fn longest_common_prefix(strings: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut prefix: String = Default::default();
    let mut min_length: i32 = Default::default();
    if strings.is_empty() {
        return Ok(STR_EMPTY);
    }
    let _cse_temp_0 = strings.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 1;
    if _cse_temp_1 {
        return Ok(strings
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range"));
    }
    let _cse_temp_2 = strings
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        .len() as i32;
    min_length = _cse_temp_2;
    for s in {
        let base = &strings;
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
        if (s.len() as i32) < min_length {
            min_length = s.len() as i32;
        }
    }
    prefix = STR_EMPTY;
    for i in 0..(min_length) {
        let char = strings
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let mut all_match = true;
        for s in {
            let base = &strings;
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
            if {
                let base = &s;
                let idx: i32 = i;
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            } != *char
            {
                all_match = false;
                break;
            }
        }
        if all_match {
            prefix = format!("{}{}", prefix, char);
        } else {
            break;
        }
    }
    Ok(prefix.to_string())
}
#[doc = "Check if string is a palindrome(ignoring case and non-alphanumeric)"]
pub fn is_palindrome(s: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut cleaned: String = Default::default();
    cleaned = STR_EMPTY;
    for char in s.to_lowercase() {
        if char.is_alphanumeric() {
            cleaned = format!("{}{}", cleaned, char);
        }
    }
    let _cse_temp_0 = cleaned.len() as i32;
    let length = _cse_temp_0;
    for i in 0..({
        let a = length;
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
    }) {
        if {
            let base = &cleaned;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } != {
            let base = &cleaned;
            let idx: i32 = length - 1 - i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } {
            return Ok(false);
        }
    }
    Ok(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_find_anagrams_examples() {
        assert_eq!(find_anagrams(vec![]), vec![]);
        assert_eq!(find_anagrams(vec![1]), vec![1]);
    }
    #[test]
    fn test_is_palindrome_examples() {
        let _ = is_palindrome(Default::default());
    }
}
