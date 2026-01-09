#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x.clone() - other.x;
        let dy = self.y.clone() - other.y;
        let distance_squared = dx * dx + dy * dy;
        if (distance_squared as f64) == 0.0 {
            return 0.0;
        };
        let mut result = distance_squared / 2.0;
        for _ in 0..10 {
            let result = result + distance_squared / result / 2.0;
        }
        return result;
    }
}
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    pub fn area(&self) -> f64 {
        return self.width.clone() * self.height.clone();
    }
    pub fn perimeter(&self) -> f64 {
        return 2.0 * self.width.clone() + self.height.clone();
    }
    pub fn is_square(&self) -> bool {
        return ((self.width.clone() - self.height.clone()).abs() as f64) < 0.0001;
    }
}
#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f64,
}
impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
    pub fn area(&self) -> f64 {
        let pi = 3.14159;
        return pi * self.radius.clone() * self.radius.clone();
    }
    pub fn circumference(&self) -> f64 {
        let pi = 3.14159;
        return 2.0 * pi * self.radius.clone();
    }
    pub fn contains_point(&self, point: &Point) -> bool {
        let distance_squared = point.x * point.x + point.y * point.y;
        let radius_squared = self.radius.clone() * self.radius.clone();
        return distance_squared <= radius_squared;
    }
}
#[doc = "Calculate triangle area"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn triangle_area(base: f64, height: f64) -> f64 {
    0.5 * base * height
}
#[doc = "Calculate triangle area using Heron's formula"]
#[doc = " Depyler: proven to terminate"]
pub fn triangle_area_heron(a: f64, b: f64, c: f64) -> Result<f64, Box<dyn std::error::Error>> {
    let mut result: f64 = Default::default();
    let _cse_temp_0 = a + b <= c;
    let _cse_temp_1 = a + c <= b;
    let _cse_temp_2 = (_cse_temp_0) || (_cse_temp_1);
    let _cse_temp_3 = b + c <= a;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    let _cse_temp_5 = a + b + c;
    let _cse_temp_6 = ((_cse_temp_5) as f64) / ((2.0) as f64);
    let s = _cse_temp_6;
    let _cse_temp_7 = s * (s - a);
    let _cse_temp_8 = _cse_temp_7 * (s - b);
    let _cse_temp_9 = _cse_temp_8 * (s - c);
    let area_squared = _cse_temp_9;
    let _cse_temp_10 = area_squared <= 0.0;
    if _cse_temp_10 {
        return Ok(0.0);
    }
    let _cse_temp_11 = ((area_squared) as f64) / ((2.0) as f64);
    result = _cse_temp_11;
    for __sanitized in 0..(10) {
        result = ((result + ((area_squared) as f64) / ((result) as f64)) as f64) / ((2.0) as f64);
    }
    Ok(result)
}
#[doc = "Find intersection of two lines defined by point pairs"]
#[doc = " Depyler: proven to terminate"]
pub fn line_intersection<'a, 'b, 'l1, 'c>(
    p1: &'a Point,
    p2: &'b Point,
    p3: &'c Point,
    p4: &'l1 Point,
) -> Result<(bool, Point), Box<dyn std::error::Error>> {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);
    let (x4, y4) = (p4.x, p4.y);
    let _cse_temp_0 = (x1 - x2) * (y3 - y4);
    let _cse_temp_1 = (y1 - y2) * (x3 - x4);
    let denominator = _cse_temp_0 - _cse_temp_1;
    let _cse_temp_2 = (denominator).abs();
    let _cse_temp_3 = (_cse_temp_2 as f64) < 0.0001;
    if _cse_temp_3 {
        return Ok((false, Point::new(0.0, 0.0)));
    }
    let _cse_temp_4 = (x1 - x3) * (y3 - y4);
    let _cse_temp_5 = (y1 - y3) * (x3 - x4);
    let _cse_temp_6 = (_cse_temp_4 - _cse_temp_5) / denominator;
    let t = _cse_temp_6;
    let _cse_temp_7 = t * (x2 - x1);
    let intersection_x = x1 + _cse_temp_7;
    let _cse_temp_8 = t * (y2 - y1);
    let intersection_y = y1 + _cse_temp_8;
    Ok((true, Point::new(intersection_x, intersection_y)))
}
