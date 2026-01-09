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
#[doc = "Chain two lists together"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn chain_lists<'b, 'a>(list1: &'a Vec<i32>, list2: &'b Vec<i32>) -> Vec<i32> {
    {
        let a = list1;
        let b = list2;
        a.into_iter().chain(b.into_iter())
    }
    .into_iter()
    .collect::<Vec<_>>()
}
#[doc = "Get all combinations of r items"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_combinations(items: &Vec<String>, r: i32) -> Vec<(String, DepylerValue)> {
    {
                items
            .into_iter()
            .combinations(r as usize)
            .collect::<Vec<_>>()
    }
    .into_iter()
    .collect::<Vec<_>>()
}
#[doc = "Group consecutive numbers"]
#[doc = " Depyler: verified panic-free"]
pub fn group_consecutive(numbers: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut groups = vec![];
    for (_k, g) in {
                numbers
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (i as i32, x))
            .into_iter()
            .group_by(move |x| {
                x.get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    - x.get(1usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
            })
    } {
        groups.push(
            g.iter()
                .cloned()
                .map(|x| {
                    x.get(1usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                })
                .collect::<Vec<_>>(),
        );
    }
    groups
}
#[doc = "Take numbers while they are positive"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn take_while_positive(numbers: &Vec<i32>) -> Vec<i32> {
    {
        let pred = move |x| x > 0;
        let items = numbers;
        items.into_iter().take_while(pred)
    }
    .into_iter()
    .collect::<Vec<_>>()
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_group_consecutive_examples() {
        assert_eq!(group_consecutive(vec![]), vec![]);
        assert_eq!(group_consecutive(vec![1]), vec![1]);
    }
    #[test]
    fn test_take_while_positive_examples() {
        assert_eq!(take_while_positive(vec![]), vec![]);
        assert_eq!(take_while_positive(vec![1]), vec![1]);
    }
}
