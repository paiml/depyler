#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
use std::f64 as math;
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
#[doc = r" DEPYLER-1040b: Now implements Hash + Eq to support non-string dict keys"]
#[derive(Debug, Clone, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<DepylerValue, DepylerValue>),
}
impl PartialEq for DepylerValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => _dv_a == _dv_b,
            (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => {
                _dv_a.to_bits() == _dv_b.to_bits()
            }
            (DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) => _dv_a == _dv_b,
            (DepylerValue::Bool(_dv_a), DepylerValue::Bool(_dv_b)) => _dv_a == _dv_b,
            (DepylerValue::None, DepylerValue::None) => true,
            (DepylerValue::List(_dv_a), DepylerValue::List(_dv_b)) => _dv_a == _dv_b,
            (DepylerValue::Dict(_dv_a), DepylerValue::Dict(_dv_b)) => _dv_a == _dv_b,
            _ => false,
        }
    }
}
impl Eq for DepylerValue {}
impl std::hash::Hash for DepylerValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            DepylerValue::Int(_dv_int) => _dv_int.hash(state),
            DepylerValue::Float(_dv_float) => _dv_float.to_bits().hash(state),
            DepylerValue::Str(_dv_str) => _dv_str.hash(state),
            DepylerValue::Bool(_dv_bool) => _dv_bool.hash(state),
            DepylerValue::None => {}
            DepylerValue::List(_dv_list) => _dv_list.hash(state),
            DepylerValue::Dict(_) => {
                0u8.hash(state);
            }
        }
    }
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, _dv_fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(_dv_int) => write!(_dv_fmt, "{}", _dv_int),
            DepylerValue::Float(_dv_float) => write!(_dv_fmt, "{}", _dv_float),
            DepylerValue::Str(_dv_str) => write!(_dv_fmt, "{}", _dv_str),
            DepylerValue::Bool(_dv_bool) => write!(_dv_fmt, "{}", _dv_bool),
            DepylerValue::None => write!(_dv_fmt, "None"),
            DepylerValue::List(_dv_list) => write!(_dv_fmt, "{:?}", _dv_list),
            DepylerValue::Dict(_dv_dict) => write!(_dv_fmt, "{:?}", _dv_dict),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    #[doc = r" DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(_dv_str) => _dv_str.len(),
            DepylerValue::List(_dv_list) => _dv_list.len(),
            DepylerValue::Dict(_dv_dict) => _dv_dict.len(),
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
            DepylerValue::Str(_dv_str) => _dv_str.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys for non-string dict keys"]
    pub fn insert(&mut self, key: impl Into<DepylerValue>, value: impl Into<DepylerValue>) {
        if let DepylerValue::Dict(_dv_dict) = self {
            _dv_dict.insert(key.into(), value.into());
        }
    }
    #[doc = r" Get value from dict by key"]
    #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys"]
    pub fn get(&self, key: &DepylerValue) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(_dv_dict) = self {
            _dv_dict.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Get value from dict by string key(convenience method)"]
    pub fn get_str(&self, key: &str) -> Option<&DepylerValue> {
        self.get(&DepylerValue::Str(key.to_string()))
    }
    #[doc = r" Check if dict contains key"]
    #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys"]
    pub fn contains_key(&self, key: &DepylerValue) -> bool {
        if let DepylerValue::Dict(_dv_dict) = self {
            _dv_dict.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Check if dict contains string key(convenience method)"]
    pub fn contains_key_str(&self, key: &str) -> bool {
        self.contains_key(&DepylerValue::Str(key.to_string()))
    }
    #[doc = r" DEPYLER-1051: Get iterator over list values"]
    #[doc = r" Returns an empty iterator for non-list types"]
    pub fn iter(&self) -> std::slice::Iter<'_, DepylerValue> {
        match self {
            DepylerValue::List(_dv_list) => _dv_list.iter(),
            _ => [].iter(),
        }
    }
    #[doc = r" DEPYLER-1051: Get mutable iterator over list values"]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, DepylerValue> {
        match self {
            DepylerValue::List(_dv_list) => _dv_list.iter_mut(),
            _ => [].iter_mut(),
        }
    }
    #[doc = r" DEPYLER-1051: Get iterator over dict key-value pairs"]
    #[doc = r" DEPYLER-1040b: Now uses DepylerValue keys"]
    pub fn items(&self) -> std::collections::hash_map::Iter<'_, DepylerValue, DepylerValue> {
        static EMPTY_MAP: std::sync::LazyLock<
            std::collections::HashMap<DepylerValue, DepylerValue>,
        > = std::sync::LazyLock::new(|| std::collections::HashMap::new());
        match self {
            DepylerValue::Dict(_dv_dict) => _dv_dict.iter(),
            _ => EMPTY_MAP.iter(),
        }
    }
    #[doc = r" DEPYLER-1051: Get iterator over dict keys"]
    #[doc = r" DEPYLER-1040b: Now returns DepylerValue keys"]
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, DepylerValue, DepylerValue> {
        static EMPTY_MAP: std::sync::LazyLock<
            std::collections::HashMap<DepylerValue, DepylerValue>,
        > = std::sync::LazyLock::new(|| std::collections::HashMap::new());
        match self {
            DepylerValue::Dict(_dv_dict) => _dv_dict.keys(),
            _ => EMPTY_MAP.keys(),
        }
    }
    #[doc = r" DEPYLER-1051: Get iterator over dict values"]
    #[doc = r" DEPYLER-1040b: Now uses DepylerValue keys internally"]
    pub fn values(&self) -> std::collections::hash_map::Values<'_, DepylerValue, DepylerValue> {
        static EMPTY_MAP: std::sync::LazyLock<
            std::collections::HashMap<DepylerValue, DepylerValue>,
        > = std::sync::LazyLock::new(|| std::collections::HashMap::new());
        match self {
            DepylerValue::Dict(_dv_dict) => _dv_dict.values(),
            _ => EMPTY_MAP.values(),
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(_dv_str) => _dv_str.clone(),
            DepylerValue::Int(_dv_int) => _dv_int.to_string(),
            DepylerValue::Float(_dv_float) => _dv_float.to_string(),
            DepylerValue::Bool(_dv_bool) => _dv_bool.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(_dv_list) => format!("{:?}", _dv_list),
            DepylerValue::Dict(_dv_dict) => format!("{:?}", _dv_dict),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(_dv_int) => *_dv_int,
            DepylerValue::Float(_dv_float) => *_dv_float as i64,
            DepylerValue::Bool(_dv_bool) => {
                if *_dv_bool {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(_dv_str) => _dv_str.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(_dv_float) => *_dv_float,
            DepylerValue::Int(_dv_int) => *_dv_int as f64,
            DepylerValue::Bool(_dv_bool) => {
                if *_dv_bool {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(_dv_str) => _dv_str.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(_dv_bool) => *_dv_bool,
            DepylerValue::Int(_dv_int) => *_dv_int != 0,
            DepylerValue::Float(_dv_float) => *_dv_float != 0.0,
            DepylerValue::Str(_dv_str) => !_dv_str.is_empty(),
            DepylerValue::List(_dv_list) => !_dv_list.is_empty(),
            DepylerValue::Dict(_dv_dict) => !_dv_dict.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(_dv_list) => &_dv_list[_dv_idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(_dv_dict) => _dv_dict
                .get(&DepylerValue::Str(_dv_key.to_string()))
                .unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
impl std::ops::Index<DepylerValue> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: DepylerValue) -> &Self::Output {
        match self {
            DepylerValue::Dict(_dv_dict) => _dv_dict.get(&_dv_key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue"),
        }
    }
}
impl std::ops::Index<i64> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: i64) -> &Self::Output {
        match self {
            DepylerValue::Dict(_dv_dict) => _dv_dict
                .get(&DepylerValue::Int(_dv_key))
                .unwrap_or(&DepylerValue::None),
            DepylerValue::List(_dv_list) => &_dv_list[_dv_key as usize],
            _ => panic!("Cannot index DepylerValue with integer"),
        }
    }
}
impl std::ops::Index<i32> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: i32) -> &Self::Output {
        &self[_dv_key as i64]
    }
}
impl From<i64> for DepylerValue {
    fn from(v: i64) -> Self {
        DepylerValue::Int(v)
    }
}
impl From<i32> for DepylerValue {
    fn from(v: i32) -> Self {
        DepylerValue::Int(v as i64)
    }
}
impl From<f64> for DepylerValue {
    fn from(v: f64) -> Self {
        DepylerValue::Float(v)
    }
}
impl From<String> for DepylerValue {
    fn from(v: String) -> Self {
        DepylerValue::Str(v)
    }
}
impl From<&str> for DepylerValue {
    fn from(v: &str) -> Self {
        DepylerValue::Str(v.to_string())
    }
}
impl From<bool> for DepylerValue {
    fn from(v: bool) -> Self {
        DepylerValue::Bool(v)
    }
}
impl From<Vec<DepylerValue>> for DepylerValue {
    fn from(v: Vec<DepylerValue>) -> Self {
        DepylerValue::List(v)
    }
}
impl From<std::collections::HashMap<DepylerValue, DepylerValue>> for DepylerValue {
    fn from(v: std::collections::HashMap<DepylerValue, DepylerValue>) -> Self {
        DepylerValue::Dict(v)
    }
}
impl From<std::collections::HashMap<String, DepylerValue>> for DepylerValue {
    fn from(v: std::collections::HashMap<String, DepylerValue>) -> Self {
        let converted: std::collections::HashMap<DepylerValue, DepylerValue> = v
            .into_iter()
            .map(|(k, v)| (DepylerValue::Str(k), v))
            .collect();
        DepylerValue::Dict(converted)
    }
}
impl std::ops::Add for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => {
                DepylerValue::Int(_dv_a + _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => {
                DepylerValue::Float(_dv_a + _dv_b)
            }
            (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => {
                DepylerValue::Float(_dv_a as f64 + _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => {
                DepylerValue::Float(_dv_a + _dv_b as f64)
            }
            (DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) => {
                DepylerValue::Str(_dv_a + &_dv_b)
            }
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Sub for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => {
                DepylerValue::Int(_dv_a - _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => {
                DepylerValue::Float(_dv_a - _dv_b)
            }
            (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => {
                DepylerValue::Float(_dv_a as f64 - _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => {
                DepylerValue::Float(_dv_a - _dv_b as f64)
            }
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Mul for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => {
                DepylerValue::Int(_dv_a * _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => {
                DepylerValue::Float(_dv_a * _dv_b)
            }
            (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => {
                DepylerValue::Float(_dv_a as f64 * _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => {
                DepylerValue::Float(_dv_a * _dv_b as f64)
            }
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Div for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => {
                DepylerValue::Int(_dv_a / _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 => {
                DepylerValue::Float(_dv_a / _dv_b)
            }
            (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 => {
                DepylerValue::Float(_dv_a as f64 / _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => {
                DepylerValue::Float(_dv_a / _dv_b as f64)
            }
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Add<i64> for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: i64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Int(_dv_int + rhs),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float + rhs as f64),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Add<i32> for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: i32) -> Self::Output {
        self + (rhs as i64)
    }
}
impl std::ops::Add<DepylerValue> for i32 {
    type Output = i32;
    fn add(self, rhs: DepylerValue) -> Self::Output {
        self + rhs.to_i64() as i32
    }
}
impl std::ops::Add<DepylerValue> for i64 {
    type Output = i64;
    fn add(self, rhs: DepylerValue) -> Self::Output {
        self + rhs.to_i64()
    }
}
impl std::ops::Add<DepylerValue> for f64 {
    type Output = f64;
    fn add(self, rhs: DepylerValue) -> Self::Output {
        self + rhs.to_f64()
    }
}
impl std::ops::Sub<i64> for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: i64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Int(_dv_int - rhs),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float - rhs as f64),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Sub<i32> for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: i32) -> Self::Output {
        self - (rhs as i64)
    }
}
impl std::ops::Sub<f64> for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: f64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Float(_dv_int as f64 - rhs),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float - rhs),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Sub<DepylerValue> for i32 {
    type Output = i32;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
        self - rhs.to_i64() as i32
    }
}
impl std::ops::Sub<DepylerValue> for i64 {
    type Output = i64;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
        self - rhs.to_i64()
    }
}
impl std::ops::Sub<DepylerValue> for f64 {
    type Output = f64;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
        self - rhs.to_f64()
    }
}
impl std::ops::Mul<i64> for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: i64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Int(_dv_int * rhs),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float * rhs as f64),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Mul<i32> for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: i32) -> Self::Output {
        self * (rhs as i64)
    }
}
impl std::ops::Mul<f64> for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Float(_dv_int as f64 * rhs),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float * rhs),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Mul<DepylerValue> for i32 {
    type Output = i32;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
        self * rhs.to_i64() as i32
    }
}
impl std::ops::Mul<DepylerValue> for i64 {
    type Output = i64;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
        self * rhs.to_i64()
    }
}
impl std::ops::Mul<DepylerValue> for f64 {
    type Output = f64;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
        self * rhs.to_f64()
    }
}
impl std::ops::Div<i64> for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: i64) -> Self::Output {
        if rhs == 0 {
            return DepylerValue::None;
        }
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Int(_dv_int / rhs),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float / rhs as f64),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Div<i32> for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: i32) -> Self::Output {
        self / (rhs as i64)
    }
}
impl std::ops::Div<f64> for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return DepylerValue::None;
        }
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Float(_dv_int as f64 / rhs),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float / rhs),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Div<DepylerValue> for i32 {
    type Output = i32;
    fn div(self, rhs: DepylerValue) -> Self::Output {
        let divisor = rhs.to_i64() as i32;
        if divisor == 0 {
            0
        } else {
            self / divisor
        }
    }
}
impl std::ops::Div<DepylerValue> for i64 {
    type Output = i64;
    fn div(self, rhs: DepylerValue) -> Self::Output {
        let divisor = rhs.to_i64();
        if divisor == 0 {
            0
        } else {
            self / divisor
        }
    }
}
impl std::ops::Div<DepylerValue> for f64 {
    type Output = f64;
    fn div(self, rhs: DepylerValue) -> Self::Output {
        let divisor = rhs.to_f64();
        if divisor == 0.0 {
            0.0
        } else {
            self / divisor
        }
    }
}
impl std::ops::Add<f64> for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: f64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Float(_dv_int as f64 + rhs),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float + rhs),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Neg for DepylerValue {
    type Output = DepylerValue;
    fn neg(self) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Int(-_dv_int),
            DepylerValue::Float(_dv_float) => DepylerValue::Float(-_dv_float),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::Not for DepylerValue {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.to_bool()
    }
}
impl std::ops::BitXor<i64> for DepylerValue {
    type Output = DepylerValue;
    fn bitxor(self, rhs: i64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Int(_dv_int ^ rhs),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::BitAnd<i64> for DepylerValue {
    type Output = DepylerValue;
    fn bitand(self, rhs: i64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Int(_dv_int & rhs),
            _ => DepylerValue::None,
        }
    }
}
impl std::ops::BitOr<i64> for DepylerValue {
    type Output = DepylerValue;
    fn bitor(self, rhs: i64) -> Self::Output {
        match self {
            DepylerValue::Int(_dv_int) => DepylerValue::Int(_dv_int | rhs),
            _ => DepylerValue::None,
        }
    }
}
impl IntoIterator for DepylerValue {
    type Item = DepylerValue;
    type IntoIter = std::vec::IntoIter<DepylerValue>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            DepylerValue::List(_dv_list) => _dv_list.into_iter(),
            DepylerValue::Dict(_dv_dict) => _dv_dict.into_keys().collect::<Vec<_>>().into_iter(),
            DepylerValue::Str(_dv_str) => _dv_str
                .chars()
                .map(|_dv_c| DepylerValue::Str(_dv_c.to_string()))
                .collect::<Vec<_>>()
                .into_iter(),
            _ => Vec::new().into_iter(),
        }
    }
}
impl<'_dv_a> IntoIterator for &'_dv_a DepylerValue {
    type Item = DepylerValue;
    type IntoIter = std::vec::IntoIter<DepylerValue>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            DepylerValue::List(_dv_list) => {
                _dv_list.iter().cloned().collect::<Vec<_>>().into_iter()
            }
            DepylerValue::Dict(_dv_dict) => {
                _dv_dict.keys().cloned().collect::<Vec<_>>().into_iter()
            }
            DepylerValue::Str(_dv_str) => _dv_str
                .chars()
                .map(|_dv_c| DepylerValue::Str(_dv_c.to_string()))
                .collect::<Vec<_>>()
                .into_iter(),
            _ => Vec::new().into_iter(),
        }
    }
}
#[doc = "Generate sample data using normal distribution(simplified)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn generate_sample_data(size: i32, mean: f64, stddev: f64) -> Vec<f64> {
    let mut data: Vec<f64> = vec![];
    for _i in 0..(size) {
        let value: f64 = 0.5_f64 * stddev + mean;
        data.push(value);
    }
    data
}
#[doc = "Calculate comprehensive statistics on dataset"]
pub fn calculate_statistics(
    data: &Vec<f64>,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut min_val: f64 = Default::default();
    let mut variance_sum: f64 = Default::default();
    let mut max_val: f64 = Default::default();
    let mut total: f64 = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok({
            let map: HashMap<String, f64> = HashMap::new();
            map
        });
    }
    let mut stats: std::collections::HashMap<String, f64> = {
        let map: HashMap<String, f64> = HashMap::new();
        map
    };
    total = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_2 = (_cse_temp_0) as f64;
    let _cse_temp_3 = ((total) as f64) / ((_cse_temp_2) as f64);
    let mean: f64 = _cse_temp_3;
    stats.insert("mean".to_string(), mean);
    variance_sum = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_4 = ((variance_sum) as f64) / ((_cse_temp_2) as f64);
    let variance: f64 = _cse_temp_4;
    stats.insert("variance".to_string(), variance);
    stats.insert("std_dev".to_string(), (variance as f64).sqrt());
    min_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    max_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for value in data.iter().cloned() {
        if value < min_val {
            min_val = value;
        }
        if value > max_val {
            max_val = value;
        }
    }
    stats.insert("min".to_string(), min_val);
    stats.insert("max".to_string(), max_val);
    stats.insert("range".to_string(), max_val - min_val);
    let mut sorted_data: Vec<f64> = data.clone();
    for i in 0..(sorted_data.len() as i32) {
        for j in (i + 1)..(sorted_data.len() as i32) {
            if sorted_data
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                let temp: f64 = sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                sorted_data.insert(
                    (i) as usize,
                    sorted_data
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    let _cse_temp_5 = sorted_data.len() as i32;
    let _cse_temp_6 = {
        let a = _cse_temp_5;
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
    let mid: i32 = _cse_temp_6;
    let _cse_temp_7 = _cse_temp_5 % 2;
    let _cse_temp_8 = _cse_temp_7 == 1;
    if _cse_temp_8 {
        stats.insert(
            "median".to_string(),
            sorted_data
                .get(mid as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    } else {
        let _cse_temp_9 = {
            let base = &sorted_data;
            let idx: i32 = mid - 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        } + sorted_data
            .get(mid as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let _cse_temp_10 = ((_cse_temp_9) as f64) / ((2.0) as f64);
        stats.insert("median".to_string(), _cse_temp_10);
    }
    Ok(stats)
}
#[doc = "Calculate quartiles using math and sorting"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_percentiles(
    data: &Vec<f64>,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok({
            let map: HashMap<String, f64> = HashMap::new();
            map
        });
    }
    let mut sorted_data: Vec<f64> = data.clone();
    for i in 0..(sorted_data.len() as i32) {
        for j in (i + 1)..(sorted_data.len() as i32) {
            if sorted_data
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                let temp: f64 = sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                sorted_data.insert(
                    (i) as usize,
                    sorted_data
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    let mut percentiles: std::collections::HashMap<String, f64> = {
        let map: HashMap<String, f64> = HashMap::new();
        map
    };
    let _cse_temp_2 = sorted_data.len() as i32;
    let _cse_temp_3 = {
        let a = _cse_temp_2;
        let b = 4;
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
    let q1_idx: i32 = _cse_temp_3;
    percentiles.insert(
        "q1".to_string(),
        sorted_data
            .get(q1_idx as usize)
            .cloned()
            .expect("IndexError: list index out of range"),
    );
    let _cse_temp_4 = {
        let a = _cse_temp_2;
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
    let q2_idx: i32 = _cse_temp_4;
    percentiles.insert(
        "q2".to_string(),
        sorted_data
            .get(q2_idx as usize)
            .cloned()
            .expect("IndexError: list index out of range"),
    );
    let _cse_temp_5 = 3 * _cse_temp_2;
    let _cse_temp_6 = {
        let a = _cse_temp_5;
        let b = 4;
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
    let q3_idx: i32 = _cse_temp_6;
    percentiles.insert(
        "q3".to_string(),
        sorted_data
            .get(q3_idx as usize)
            .cloned()
            .expect("IndexError: list index out of range"),
    );
    let _cse_temp_7 = percentiles.get("q3").cloned().unwrap_or_default()
        - percentiles.get("q1").cloned().unwrap_or_default();
    percentiles.insert("iqr".to_string(), _cse_temp_7);
    Ok(percentiles)
}
#[doc = "Detect outliers using IQR method(combines statistics + collections)"]
pub fn detect_outliers(data: &Vec<f64>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let percentiles: std::collections::HashMap<String, f64> = calculate_percentiles(&data)?;
    let _cse_temp_0 = percentiles.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    let q1: f64 = percentiles.get("q1").cloned().unwrap_or_default();
    let q3: f64 = percentiles.get("q3").cloned().unwrap_or_default();
    let iqr: f64 = percentiles.get("iqr").cloned().unwrap_or_default();
    let _cse_temp_2 = 1.5 * iqr;
    let lower_bound: f64 = q1 - _cse_temp_2;
    let upper_bound: f64 = q3 + _cse_temp_2;
    let mut outliers: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        if (value < lower_bound) || (value > upper_bound) {
            outliers.push(value);
        }
    }
    Ok(outliers)
}
#[doc = "Create histogram bins(uses collections + math)"]
pub fn bin_data(
    data: &Vec<f64>,
    num_bins: i32,
) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let mut max_val: f64 = Default::default();
    let mut min_val: f64 = Default::default();
    let mut bin_index: i32 = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let _cse_temp_2 = num_bins <= 0;
    let _cse_temp_3 = (_cse_temp_1) || (_cse_temp_2);
    if _cse_temp_3 {
        return Ok({
            let map: HashMap<i32, i32> = HashMap::new();
            map
        });
    }
    min_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    max_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for value in data.iter().cloned() {
        if value < min_val {
            min_val = value;
        }
        if value > max_val {
            max_val = value;
        }
    }
    let _cse_temp_4 = (num_bins) as f64;
    let _cse_temp_5 = ((max_val - min_val) as f64) / ((_cse_temp_4) as f64);
    let bin_width: f64 = _cse_temp_5;
    let mut bins: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    for i in 0..(num_bins) {
        bins.insert(i.clone(), 0);
    }
    for value in data.iter().cloned() {
        bin_index = (((value - min_val) as f64) / ((bin_width) as f64)) as i32;
        if bin_index >= num_bins {
            bin_index = num_bins - 1;
        }
        {
            let _key = bin_index;
            let _old_val = bins.get(&_key).cloned().unwrap_or_default();
            bins.insert(_key, _old_val + 1);
        }
    }
    Ok(bins)
}
#[doc = "Calculate Pearson correlation coefficient"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_correlation<'a, 'b>(
    x: &'a Vec<f64>,
    y: &'b Vec<f64>,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut y_sum: f64 = Default::default();
    let mut x_variance_sum: f64 = Default::default();
    let mut numerator: f64 = Default::default();
    let mut x_sum: f64 = Default::default();
    let mut y_variance_sum: f64 = Default::default();
    let _cse_temp_0 = x.len() as i32;
    let _cse_temp_1 = y.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let _cse_temp_3 = _cse_temp_0 == 0;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    x_sum = 0.0;
    y_sum = 0.0;
    for i in 0..(x.len() as i32) {
        x_sum = x_sum
            + x.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
        y_sum = y_sum
            + y.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
    }
    let _cse_temp_5 = (_cse_temp_0) as f64;
    let _cse_temp_6 = ((x_sum) as f64) / ((_cse_temp_5) as f64);
    let x_mean: f64 = _cse_temp_6;
    let _cse_temp_7 = (_cse_temp_1) as f64;
    let _cse_temp_8 = ((y_sum) as f64) / ((_cse_temp_7) as f64);
    let y_mean: f64 = _cse_temp_8;
    numerator = 0.0;
    x_variance_sum = 0.0;
    y_variance_sum = 0.0;
    for i in 0..(x.len() as i32) {
        let x_diff: f64 = x
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            - x_mean;
        let y_diff: f64 = y
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            - y_mean;
        numerator = numerator + x_diff * y_diff;
        x_variance_sum = x_variance_sum + x_diff * x_diff;
        y_variance_sum = y_variance_sum + y_diff * y_diff;
    }
    let denominator: f64 = (x_variance_sum * y_variance_sum as f64).sqrt();
    let _cse_temp_9 = denominator == 0.0;
    if _cse_temp_9 {
        return Ok(0.0);
    }
    let _cse_temp_10 = ((numerator) as f64) / ((denominator) as f64);
    let correlation: f64 = _cse_temp_10;
    Ok(correlation)
}
#[doc = "Z-score normalization using statistics"]
pub fn normalize_data(data: Vec<f64>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut total: f64 = Default::default();
    let mut variance_sum: f64 = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    total = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_2 = (_cse_temp_0) as f64;
    let _cse_temp_3 = ((total) as f64) / ((_cse_temp_2) as f64);
    let mean: f64 = _cse_temp_3;
    variance_sum = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let stddev: f64 =
        (((variance_sum) as f64) / (((data.len() as i32) as f64) as f64) as f64).sqrt();
    let _cse_temp_4 = stddev == 0.0;
    if _cse_temp_4 {
        return Ok(data);
    }
    let mut normalized: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        let z_score: f64 = ((value - mean) as f64) / ((stddev) as f64);
        normalized.push(z_score);
    }
    Ok(normalized)
}
#[doc = "Group data by ranges using collections"]
pub fn group_by_range<'b, 'a>(
    data: &'a Vec<f64>,
    ranges: &'b Vec<(f64, f64)>,
) -> Result<HashMap<String, Vec<f64>>, Box<dyn std::error::Error>> {
    let mut range_tuple: (f64, f64) = Default::default();
    let mut range_key: String = Default::default();
    let mut groups: std::collections::HashMap<String, Vec<f64>> = {
        let map: HashMap<String, Vec<f64>> = HashMap::new();
        map
    };
    for i in 0..(ranges.len() as i32) {
        range_tuple = ranges
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        range_key = format!("{}-{}", range_tuple.0, range_tuple.1);
        groups.insert(range_key.to_string().clone(), vec![]);
    }
    for value in data.iter().cloned() {
        for i in 0..(ranges.len() as i32) {
            range_tuple = ranges
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if (value >= (range_tuple.0 as f64)) && (value < (range_tuple.1 as f64)) {
                range_key = format!("{}-{}", range_tuple.0, range_tuple.1);
                groups
                    .get(&range_key)
                    .cloned()
                    .unwrap_or_default()
                    .push(value);
                break;
            }
        }
    }
    Ok(groups)
}
#[doc = "Monte Carlo simulation combining random + math + statistics"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn monte_carlo_simulation(
    num_trials: i32,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut results: Vec<f64> = vec![];
    for _trial in 0..(num_trials) {
        let x: f64 = 0.5_f64 * 10.0;
        let y: f64 = 0.5_f64 * 10.0;
        let distance: f64 = (x * x + y * y as f64).sqrt();
        results.push(distance);
    }
    let stats: std::collections::HashMap<String, f64> = calculate_statistics(&results)?;
    Ok(stats)
}
#[doc = "Main analysis pipeline combining all modules"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn analyze_dataset() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=== Comprehensive Data Analysis Demo ===");
    ();
    let sample_size: i32 = 100;
    let dataset: Vec<f64> = generate_sample_data(sample_size, 50.0, 10.0);
    let stats: std::collections::HashMap<String, f64> = calculate_statistics(&dataset)?;
    println!(
        "{}",
        format!(
            "Mean: {}, StdDev: {}",
            stats.get("mean").cloned().unwrap_or_default(),
            stats.get("std_dev").cloned().unwrap_or_default()
        )
    );
    let percentiles: std::collections::HashMap<String, f64> = calculate_percentiles(&dataset)?;
    println!(
        "{}",
        format!(
            "Q1: {}, Median: {}, Q3: {}",
            percentiles.get("q1").cloned().unwrap_or_default(),
            percentiles.get("q2").cloned().unwrap_or_default(),
            percentiles.get("q3").cloned().unwrap_or_default()
        )
    );
    let outliers: Vec<f64> = detect_outliers(&dataset)?;
    println!("{}", format!("Outliers found: {}", outliers.len() as i32));
    let histogram: std::collections::HashMap<i32, i32> = bin_data(&dataset, 10)?;
    println!(
        "{}",
        format!("Histogram bins created: {}", histogram.len() as i32)
    );
    let normalized: Vec<f64> = normalize_data(dataset)?;
    let normalized_stats: std::collections::HashMap<String, f64> =
        calculate_statistics(&normalized)?;
    println!(
        "{}",
        format!(
            "Normalized mean: {}",
            normalized_stats.get("mean").cloned().unwrap_or_default()
        )
    );
    let dataset2: Vec<f64> = generate_sample_data(sample_size, 60.0, 12.0);
    let corr: f64 = calculate_correlation(&dataset, &dataset2)?;
    println!("{}", format!("Correlation: {}", corr));
    let ranges: Vec<(f64, f64)> = vec![(0.0, 25.0), (25.0, 50.0), (50.0, 75.0), (75.0, 100.0)];
    let groups: std::collections::HashMap<String, Vec<f64>> = group_by_range(&dataset, &ranges)?;
    println!(
        "{}",
        format!("Range groups created: {}", groups.len() as i32)
    );
    let mc_stats: std::collections::HashMap<String, f64> = monte_carlo_simulation(1000)?;
    println!(
        "{}",
        format!(
            "Monte Carlo mean: {}",
            mc_stats.get("mean").cloned().unwrap_or_default()
        )
    );
    println!("{}", "=== Analysis Complete ===");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_detect_outliers_examples() {
        assert_eq!(detect_outliers(vec![]), vec![]);
        assert_eq!(detect_outliers(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_normalize_data() {
        fn prop(data: Vec<f64>) -> TestResult {
            let once = normalize_data(&data);
            let twice = normalize_data(once.clone());
            if once != twice {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<f64>) -> TestResult);
    }
    #[test]
    fn test_normalize_data_examples() {
        assert_eq!(normalize_data(vec![]), vec![]);
        assert_eq!(normalize_data(vec![1]), vec![1]);
    }
}
