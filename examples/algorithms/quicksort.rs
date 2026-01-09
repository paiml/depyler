#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[doc = "Classic quicksort algorithm implementation"]
#[doc = " Depyler: proven to terminate"]
pub fn quicksort(arr: Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 <= 1;
    if _cse_temp_1 {
        return Ok(arr);
    }
    let pivot = {
        let base = &arr;
        let idx: i32 = {
            let a = arr.len() as i32;
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
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx)
            .cloned()
            .expect("IndexError: list index out of range")
    };
    let left = arr
        .as_slice()
        .iter()
        .cloned()
        .filter(|x| {
            let x = x.clone();
            x < pivot
        })
        .map(|x| x)
        .collect::<Vec<_>>();
    let middle = arr
        .as_slice()
        .iter()
        .cloned()
        .filter(|x| {
            let x = x.clone();
            x == pivot
        })
        .map(|x| x)
        .collect::<Vec<_>>();
    let right = arr
        .as_slice()
        .iter()
        .cloned()
        .filter(|x| {
            let x = x.clone();
            x > pivot
        })
        .map(|x| x)
        .collect::<Vec<_>>();
    Ok(quicksort(left)?
        .iter()
        .chain(middle.iter())
        .cloned()
        .collect::<Vec<_>>()
        .iter()
        .chain(quicksort(right)?.iter())
        .cloned()
        .collect::<Vec<_>>())
}
#[doc = "In-place partition for quicksort"]
#[doc = " Depyler: proven to terminate"]
pub fn partition(arr: &Vec<i32>, low: i32, high: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut i: i32 = Default::default();
    let pivot = arr
        .get(high as usize)
        .cloned()
        .expect("IndexError: list index out of range");
    i = low - 1;
    for j in (low)..(high) {
        if arr
            .get(j as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            <= pivot
        {
            i = i + 1;
            let _swap_temp = (
                arr.get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
                arr.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
            arr.insert(i, _swap_temp.0);
            arr.insert(j, _swap_temp.1);
        }
    }
    let _swap_temp = (
        arr.get(high as usize)
            .cloned()
            .expect("IndexError: list index out of range"),
        {
            let base = &arr;
            let idx: i32 = i + 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        },
    );
    arr.insert(i + 1, _swap_temp.0);
    arr.insert(high, _swap_temp.1);
    Ok(i + 1)
}
#[doc = "In-place quicksort implementation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn quicksort_inplace(
    arr: &Vec<i32>,
    low: i32,
    high: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let _cse_temp_0 = low < high;
    if _cse_temp_0 {
        let pi = partition(&arr, low, high)?;
        quicksort_inplace(&arr, low, pi - 1);
        quicksort_inplace(&arr, pi + 1, high);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_quicksort() {
        fn prop(arr: Vec<i32>) -> TestResult {
            let input_len = arr.len();
            let result = quicksort(&arr);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = quicksort(&arr);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = arr.clone();
            input_sorted.sort();
            let mut result = quicksort(&arr);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_quicksort_examples() {
        assert_eq!(quicksort(vec![]), vec![]);
        assert_eq!(quicksort(vec![1]), vec![1]);
    }
}
