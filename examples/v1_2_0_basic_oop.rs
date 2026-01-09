#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x = self.x.clone() + dx;
        self.y = self.y.clone() + dy;
    }
    pub fn distance_squared(&self) -> i32 {
        return self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone();
    }
    pub fn origin() -> i32 {
        return 0;
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
#[doc = "Test Point class"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_point() -> i32 {
    let mut p = Point::new(3, 4);
    p.translate(1, 2);
    let dist_sq = p.distance_squared();
    dist_sq
}
#[doc = "Test Rectangle class"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_rectangle() -> i32 {
    let rect = Rectangle::new(10, 20);
    let area = rect.area();
    let perim = rect.perimeter();
    let square = rect.is_square();
    let sq = Rectangle::new(5, 5);
    let is_sq = sq.is_square();
    area + perim
}
#[doc = "Test static methods"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_static() {
    let zero = Point::origin();
    let _ = zero;
}
#[doc = "Run all tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> i32 {
    let point_result = test_point();
    let rect_result = test_rectangle();
    let static_result = test_static();
    let _ = point_result + rect_result + static_result;
}
