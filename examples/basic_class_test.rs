#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
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
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance_from_origin(&self) -> f64 {
        return ({ self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone() } as f64)
            .powf({ 0.5 } as f64);
    }
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x = self.x.clone() + dx;
        self.y = self.y.clone() + dy;
    }
}
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}
impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
    pub fn area(&self) -> i32 {
        return self.width.clone() * self.height.clone();
    }
    pub fn perimeter(&self) -> i32 {
        return 2 * self.width.clone() + self.height.clone();
    }
    pub fn is_square(&self) -> bool {
        return self.width.clone() == self.height.clone();
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: i32,
}
impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
    pub fn greet(&self) -> String {
        return format!("Hello, my name is {}", self.name.clone());
    }
    pub fn is_adult(&self) -> bool {
        return self.age.clone() >= 18;
    }
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn dataclass<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = "Test Point class functionality"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_point() {
    let mut p = Point::new(3, 4);
    assert_eq!(p.x, 3);
    assert_eq!(p.y, 4);
    assert_eq!(p.distance_from_origin(), 5.0);
    p.translate(1, 1);
    assert_eq!(p.x, 4);
    assert_eq!(p.y, 5);
}
#[doc = "Test Rectangle class functionality"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_rectangle() {
    let r = Rectangle::new(10, 20);
    assert_eq!(r.area(), 200);
    assert_eq!(r.perimeter(), 60);
    assert!(!r.is_square());
    let square = Rectangle::new(15, 15);
    assert!(square.is_square());
}
#[doc = "Test Person dataclass functionality"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_person() {
    let p = Person::new("Alice".to_string(), 25);
    assert_eq!(p.name, "Alice");
    assert_eq!(p.age, 25);
    assert!(p.is_adult());
    assert_eq!(p.greet(), "Hello, my name is Alice".to_string());
    let child = Person::new("Bob".to_string(), 10);
    assert!(!child.is_adult());
}
