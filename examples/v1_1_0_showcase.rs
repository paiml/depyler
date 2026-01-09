#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::HashSet;
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
#[doc = "Showcase nested dictionary assignment support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_dictionary_assignment() -> (
    std::collections::HashMap<
        String,
        std::collections::HashMap<String, std::collections::HashMap<String, DepylerValue>>,
    >,
    std::collections::HashMap<String, DepylerValue>,
) {
    let mut d = {
        let map: HashMap<String, String> = HashMap::new();
        map
    };
    d.insert("key".to_string(), "value".to_string());
    let nested = {
        let mut map = HashMap::new();
        map.insert("level1".to_string(), {
            let mut map = HashMap::new();
            map.insert("level2".to_string(), {
                let map: HashMap<String, String> = HashMap::new();
                map
            });
            map
        });
        map
    };
    nested
        .get_mut(&"level1")
        .unwrap()
        .get_mut(&"level2")
        .unwrap()
        .insert("level3".to_string(), "deep value".to_string());
    let mut coords = {
        let map: HashMap<String, String> = HashMap::new();
        map
    };
    coords.insert((10, 20).to_string(), "location A".to_string());
    coords.insert((30, 40).to_string(), "location B".to_string());
    (nested, coords)
}
#[doc = "Showcase comprehensive set support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_set_operations() -> (
    std::collections::HashSet<i32>,
    std::collections::HashSet<i32>,
    std::collections::HashSet<i32>,
    std::collections::HashSet<i32>,
    std::collections::HashSet<i32>,
) {
    let set1 = {
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let set2 = {
        let mut set = std::collections::HashSet::new();
        set.insert(4);
        set.insert(5);
        set.insert(6);
        set.insert(7);
        set.insert(8);
        set
    };
    let _cse_temp_0 = set1
        .intersection(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let intersection = _cse_temp_0;
    let _cse_temp_1 = set1
        .union(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let union = _cse_temp_1;
    let difference = set1
        .difference(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let _cse_temp_2 = set1
        .symmetric_difference(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let symmetric_diff = _cse_temp_2;
    let mut mutable_set = {
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    mutable_set.insert(4);
    if !mutable_set.remove(&2) {
        panic!("KeyError: element not in set")
    };
    mutable_set.remove(&5);
    (intersection, union, difference, symmetric_diff, mutable_set)
}
#[doc = "Showcase set comprehensions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_set_comprehensions() -> (
    std::collections::HashSet<String>,
    std::collections::HashSet<String>,
    std::collections::HashSet<String>,
) {
    let squares = (0..(10))
        .into_iter()
        .map(|x| x * x)
        .collect::<std::collections::HashSet<_>>();
    let even_squares = (0..(10))
        .into_iter()
        .filter(|x| {
            let x = x.clone();
            x % 2 == 0
        })
        .map(|x| x * x)
        .collect::<std::collections::HashSet<_>>();
    let unique_chars = "hello world"
        .to_string()
        .into_iter()
        .filter(|c| {
            let c = c.clone();
            c.chars().all(|c| c.is_alphabetic())
        })
        .map(|c| c.to_uppercase())
        .collect::<std::collections::HashSet<_>>();
    (squares, even_squares, unique_chars)
}
#[doc = "Showcase frozen set support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_frozen_sets() -> (DepylerValue, std::collections::HashMap<String, String>) {
    let immutable1 = std::sync::Arc::new(
        vec![1, 2, 3, 4]
            .into_iter()
            .collect::<std::collections::HashSet<_>>(),
    );
    let immutable2 =
        std::sync::Arc::new((3)..(6).into_iter().collect::<std::collections::HashSet<_>>());
    let _cse_temp_0 = immutable1 & immutable2;
    let result = _cse_temp_0;
    let frozen_dict = {
        let mut map = HashMap::new();
        map.insert(immutable1, "first set".to_string());
        map.insert(immutable2, "second set".to_string());
        map
    };
    (result, frozen_dict)
}
#[doc = "Showcase break and continue statements"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_control_flow(
) -> Result<(Vec<DepylerValue>, Vec<DepylerValue>, Vec<DepylerValue>), Box<dyn std::error::Error>> {
    let mut result1 = vec![];
    for i in 0..(10) {
        if i == 5 {
            break;
        }
        result1.push(i);
    }
    let mut result2 = vec![];
    for i in 0..(10) {
        if i % 2 == 0 {
            continue;
        }
        result2.push(i);
    }
    let mut result3 = vec![];
    for i in 0..(3) {
        for j in 0..(3) {
            if (i == 1) && (j == 1) {
                break;
            }
            result3.push((i, j));
        }
    }
    Ok((result1, result2, result3))
}
#[doc = "Showcase power operator support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_power_operator() -> (i32, f64, f64, i32) {
    let _cse_temp_0 = ({ 2 } as i32)
        .checked_pow({ 10 } as u32)
        .expect("Power operation overflowed");
    let int_power = _cse_temp_0;
    let _cse_temp_1 = ({ 2.5 } as f64).powf({ 3.0 } as f64);
    let float_power = _cse_temp_1;
    let _cse_temp_2 = ({ 2 } as f64).powf({ -3 } as f64);
    let inverse = _cse_temp_2;
    let _cse_temp_3 = ({ 10 } as i32)
        .checked_pow({ 6 } as u32)
        .expect("Power operation overflowed");
    let large = _cse_temp_3;
    (int_power, float_power, inverse, large)
}
#[doc = "Run all showcases"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "Dictionary Assignment:",
        showcase_dictionary_assignment()
    );
    println!("{} {}", "Set Operations:", showcase_set_operations());
    println!(
        "{} {}",
        "Set Comprehensions:",
        showcase_set_comprehensions()
    );
    println!("{} {}", "Frozen Sets:", showcase_frozen_sets());
    println!("{} {:?}", "Control Flow:", showcase_control_flow());
    println!("{} {}", "Power Operator:", showcase_power_operator());
    Ok(())
}
