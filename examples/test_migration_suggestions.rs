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
#[doc = "Pattern: accumulator - should suggest iterator methods."]
#[doc = " Depyler: verified panic-free"]
pub fn accumulator_pattern(items: &Vec<i32>) -> Vec<DepylerValue> {
    let mut result = vec![];
    for item in items.iter().cloned() {
        if item > 0 {
            result.push(item * 2);
        }
    }
    result
}
#[doc = "Pattern: returning None for errors - should suggest Result."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn error_with_none(value: &str) -> Option<DepylerValue> {
    if !validate(value.to_string()) {
        return None;
    }
    let processed = process_data(value.to_string());
    if processed.is_none() {
        return None;
    }
    Some(processed)
}
#[doc = "Pattern: mutating parameters - should suggest ownership patterns."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn mutating_parameter(data: &mut Vec<DepylerValue>) -> Vec<String> {
    data.push(42);
    data.sort();
    data
}
#[doc = "Pattern: runtime type checking - should suggest enums."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn type_checking_pattern(value: &str) -> String {
    if true {
        return value.to_uppercase();
    } else {
        if true {
            return value * 2;
        } else {
            return (value).to_string();
        }
    }
}
#[doc = "Pattern: string concatenation - should suggest efficient methods."]
#[doc = " Depyler: verified panic-free"]
pub fn inefficient_string_building(items: &DepylerValue) -> String {
    let mut result: String = Default::default();
    result = "".to_string();
    for item in items.iter().cloned() {
        result = format!("{}{}", format!("{}{}", result, (item).to_string()), ", ");
    }
    result.to_string()
}
#[doc = "Pattern: range(len()) - should suggest enumerate."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn enumerate_pattern(items: &Vec<DepylerValue>) {
    for i in 0..(items.len() as i32) {
        println!(
            "{}",
            format!(
                "{}: {}",
                i,
                items
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            )
        );
    }
}
#[doc = "Pattern: filter + map in loop - should suggest filter_map."]
#[doc = " Depyler: verified panic-free"]
pub fn filter_map_pattern(data: &Vec<i32>) -> Vec<DepylerValue> {
    let mut output = vec![];
    for x in data.iter().cloned() {
        if x > 0 {
            output.push(x * x);
        }
    }
    output
}
#[doc = "Pattern: while True - should suggest loop."]
#[doc = " Depyler: verified panic-free"]
pub fn while_true_pattern() -> i32 {
    let mut counter: i32 = Default::default();
    counter = 0;
    loop {
        counter = counter + 1;
        if counter > 10 {
            break;
        }
    }
    counter
}
#[doc = "Pattern: None checking - should suggest pattern matching."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn none_checking_pattern(optional_value: &DepylerValue) {
    if optional_value.is_some() {
        return process(optional_value);
    } else {
        return default_value();
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate(x: &str) -> bool {
    (x).as_str() > 0
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_data(x: i32) -> i32 {
    x * 2
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process(x: DepylerValue) {
    let _ = x;
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn default_value() -> i32 {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_accumulator_pattern_examples() {
        assert_eq!(accumulator_pattern(vec![]), vec![]);
        assert_eq!(accumulator_pattern(vec![1]), vec![1]);
    }
    #[test]
    fn test_mutating_parameter_examples() {
        assert_eq!(mutating_parameter(vec![]), vec![]);
        assert_eq!(mutating_parameter(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_filter_map_pattern() {
        fn prop(data: Vec<i32>) -> TestResult {
            let input_len = data.len();
            let result = filter_map_pattern(&data);
            if result.len() != input_len {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_filter_map_pattern_examples() {
        assert_eq!(filter_map_pattern(vec![]), vec![]);
        assert_eq!(filter_map_pattern(vec![1]), vec![1]);
    }
    #[test]
    fn test_while_true_pattern_examples() {
        let _ = while_true_pattern();
    }
    #[test]
    fn quickcheck_process() {
        fn prop(x: ()) -> TestResult {
            let result = process(x.clone());
            if result != x {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(()) -> TestResult);
    }
    #[test]
    fn test_default_value_examples() {
        let _ = default_value();
    }
}
