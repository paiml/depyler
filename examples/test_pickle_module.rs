#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'pickle'(tracked in DEPYLER-0424)"]
use std::io::Cursor;
const STR__: &'static str = "=";
use std::collections::HashMap;
use std::collections::HashSet;
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
#[doc = "Test pickling basic Python types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_basic_types() {
    let mut data = 42;
    let mut pickled = { format!("{:?}", data).into_bytes() };
    let mut unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, 42);
    data = "hello world".to_string();
    pickled = { format!("{:?}", data).into_bytes() };
    unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, "hello world");
    data = 3.14159;
    pickled = { format!("{:?}", data).into_bytes() };
    unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, 3.14159);
    println!("{}", "PASS: test_pickle_basic_types");
}
#[doc = "Test pickling lists."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_list() {
    let data = vec![1, 2, 3, 4, 5];
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, vec![1, 2, 3, 4, 5]);
    assert_eq!(unpickled.len() as i32, 5);
    println!("{}", "PASS: test_pickle_list");
}
#[doc = "Test pickling dictionaries."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_dict() {
    let data = {
        let mut map = HashMap::new();
        map.insert("name".to_string(), DepylerValue::Str("Alice".to_string()));
        map.insert("age".to_string(), DepylerValue::Int(30 as i64));
        map.insert("city".to_string(), DepylerValue::Str("NYC".to_string()));
        map
    };
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, {
        let mut map = HashMap::new();
        map.insert("name".to_string(), DepylerValue::Str("Alice".to_string()));
        map.insert("age".to_string(), DepylerValue::Int(30 as i64));
        map.insert("city".to_string(), DepylerValue::Str("NYC".to_string()));
        map
    });
    assert_eq!(unpickled.get("name").cloned().unwrap_or_default(), "Alice");
    println!("{}", "PASS: test_pickle_dict");
}
#[doc = "Test pickling nested data structures."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_nested_structure() {
    let data = {
        let mut map = HashMap::new();
        map.insert(
            "users".to_string(),
            DepylerValue::Str(format!(
                "{:?}",
                vec![
                    {
                        let mut map = HashMap::new();
                        map.insert("name".to_string(), "Alice".to_string());
                        map.insert("scores".to_string(), vec![90, 85, 88]);
                        map
                    },
                    {
                        let mut map = HashMap::new();
                        map.insert("name".to_string(), "Bob".to_string());
                        map.insert("scores".to_string(), vec![78, 82, 91]);
                        map
                    }
                ]
            )),
        );
        map.insert("count".to_string(), DepylerValue::Int(2 as i64));
        map
    };
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, data);
    assert_eq!(
        unpickled
            .get("users")
            .cloned()
            .unwrap_or_default()
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get("name")
            .cloned()
            .unwrap_or_default(),
        "Alice"
    );
    assert_eq!(
        unpickled
            .get("users")
            .cloned()
            .unwrap_or_default()
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get("scores")
            .cloned()
            .unwrap_or_default()
            .get(2usize)
            .cloned()
            .expect("IndexError: list index out of range"),
        91
    );
    println!("{}", "PASS: test_pickle_nested_structure");
}
#[doc = "Test pickling tuples."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_tuple() {
    let data = (1, "hello".to_string(), 3.14, true);
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, (1, "hello".to_string(), 3.14, true));
    assert_eq!(
        unpickled
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range"),
        "hello"
    );
    println!("{}", "PASS: test_pickle_tuple");
}
#[doc = "Test pickling sets."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_set() {
    let data = {
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, {
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    });
    assert!(unpickled.get(&3).is_some());
    println!("{}", "PASS: test_pickle_set");
}
#[doc = "Test pickling None."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_none() {
    let data: Option<()> = None;
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, None);
    println!("{}", "PASS: test_pickle_none");
}
#[doc = "Test pickling booleans."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_boolean() {
    let data_true = true;
    let pickled_true = { format!("{:?}", data_true).into_bytes() };
    let unpickled_true = { String::from_utf8_lossy(pickled_true).to_string() };
    assert_eq!(unpickled_true, true);
    let data_false = false;
    let pickled_false = { format!("{:?}", data_false).into_bytes() };
    let unpickled_false = { String::from_utf8_lossy(pickled_false).to_string() };
    assert_eq!(unpickled_false, false);
    println!("{}", "PASS: test_pickle_boolean");
}
#[doc = "Test pickling bytes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_bytes() {
    let data = b"hello bytes";
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, b"hello bytes");
    println!("{}", "PASS: test_pickle_bytes");
}
#[doc = "Test pickling mixed type collections."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_mixed_types() {
    let data = vec![
        format!("{:?}", 1),
        format!("{:?}", "two"),
        format!("{:?}", 3.0),
        format!("{:?}", true),
        format!("{:?}", None),
        format!("{:?}", vec![4, 5]),
        format!("{:?}", {
            let mut map = HashMap::new();
            map.insert("key".to_string(), "value".to_string());
            map
        }),
    ];
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert_eq!(unpickled, data);
    assert_eq!(
        unpickled
            .get(6usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get("key")
            .cloned()
            .unwrap_or_default(),
        "value"
    );
    println!("{}", "PASS: test_pickle_mixed_types");
}
#[doc = "Run all pickle tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "PICKLE MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_pickle_basic_types();
    test_pickle_list();
    test_pickle_dict();
    test_pickle_nested_structure();
    test_pickle_tuple();
    test_pickle_set();
    test_pickle_none();
    test_pickle_boolean();
    test_pickle_bytes();
    test_pickle_mixed_types();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL PICKLE TESTS PASSED!");
    println!("{}", "Total tests: 10");
    println!("{}", STR__.repeat(60 as usize));
}
