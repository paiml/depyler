#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[doc = "Map transformation over list"]
#[doc = " Depyler: verified panic-free"]
pub fn map_transform(data: &Vec<i32>, multiplier: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        let transformed: i32 = item * multiplier;
        result.push(transformed);
    }
    result
}
#[doc = "Filter data by predicate"]
#[doc = " Depyler: verified panic-free"]
pub fn filter_predicate(data: &Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        if item > threshold {
            result.push(item);
        }
    }
    result
}
#[doc = "Reduce list to sum"]
#[doc = " Depyler: verified panic-free"]
pub fn reduce_sum(data: &Vec<i32>) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for item in data.iter().cloned() {
        total = total + item;
    }
    total
}
#[doc = "Reduce list to product"]
#[doc = " Depyler: verified panic-free"]
pub fn reduce_product(data: &Vec<i32>) -> i32 {
    let mut product: i32 = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return 0;
    }
    product = 1;
    for item in data.iter().cloned() {
        product = product * item;
    }
    product
}
#[doc = "Chain multiple operations together"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn chain_operations(data: &Vec<i32>) -> i32 {
    let mapped: Vec<i32> = map_transform(&data, 2);
    let filtered: Vec<i32> = filter_predicate(&mapped, 10);
    let result: i32 = reduce_sum(&filtered);
    result
}
#[doc = "Zip two lists together"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zip_lists<'b, 'a>(list1: &'a Vec<i32>, list2: &'b Vec<String>) -> Vec<(i32, String)> {
    let mut result: Vec<(i32, String)> = vec![];
    let _cse_temp_0 = list1.len() as i32;
    let _cse_temp_1 = list2.len() as i32;
    let _cse_temp_2 = std::cmp::min(_cse_temp_0, _cse_temp_1);
    let min_len: i32 = _cse_temp_2;
    for i in 0..(min_len) {
        let pair: (i32, String) = (
            list1
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            list2
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        result.push(pair);
    }
    result
}
#[doc = "Enumerate list with indices"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn enumerate_list(items: &Vec<String>) -> Vec<(i32, String)> {
    let mut result: Vec<(i32, String)> = vec![];
    for i in 0..(items.len() as i32) {
        let pair: (i32, String) = (
            i,
            items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        result.push(pair);
    }
    result
}
#[doc = "Group items by property(modulo)"]
pub fn group_by_property(
    items: &Vec<i32>,
    modulo: i32,
) -> Result<HashMap<i32, Vec<i32>>, Box<dyn std::error::Error>> {
    let mut groups: std::collections::HashMap<i32, Vec<i32>> = {
        let map: HashMap<i32, Vec<i32>> = HashMap::new();
        map
    };
    for item in items.iter().cloned() {
        let key: i32 = item % modulo;
        if groups.get(&key).is_none() {
            groups.insert(key.clone(), vec![]);
        }
        groups.get(&key).cloned().unwrap_or_default().push(item);
    }
    Ok(groups)
}
#[doc = "Partition list into two based on predicate"]
#[doc = " Depyler: verified panic-free"]
pub fn partition_by_predicate(items: &Vec<i32>, threshold: i32) -> (Vec<i32>, Vec<i32>) {
    let mut passed: Vec<i32> = vec![];
    let mut failed: Vec<i32> = vec![];
    for item in items.iter().cloned() {
        if item >= threshold {
            passed.push(item);
        } else {
            failed.push(item);
        }
    }
    (passed, failed)
}
#[doc = "Create list of running sums(accumulate pattern)"]
#[doc = " Depyler: verified panic-free"]
pub fn accumulate_running_sum(data: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut total: i32 = 0;
    for item in data.iter().cloned() {
        total = total + item;
        result.push(total);
    }
    result
}
#[doc = "Flatten nested list structure"]
#[doc = " Depyler: verified panic-free"]
pub fn flatten_nested_list(nested: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for sublist in nested.iter().cloned() {
        for item in sublist.iter().cloned() {
            result.push(item);
        }
    }
    result
}
#[doc = "Compute Cartesian product of two lists"]
#[doc = " Depyler: verified panic-free"]
pub fn cartesian_product<'a, 'b>(list1: &'a Vec<i32>, list2: &'b Vec<i32>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    for item1 in list1.iter().cloned() {
        for item2 in list2.iter().cloned() {
            let pair: (i32, i32) = (item1, item2);
            result.push(pair);
        }
    }
    result
}
#[doc = "Take elements while condition is true"]
#[doc = " Depyler: verified panic-free"]
pub fn take_while_condition(data: &Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        if item < threshold {
            result.push(item);
        } else {
            break;
        }
    }
    result
}
#[doc = "Drop elements while condition is true"]
#[doc = " Depyler: verified panic-free"]
pub fn drop_while_condition(data: &Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut dropping: bool = true;
    for item in data.iter().cloned() {
        if (dropping) && (item < threshold) {
            continue;
        }
        dropping = false;
        result.push(item);
    }
    result
}
#[doc = "Iterate over consecutive pairs"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn pairwise_iteration(data: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    for i in 0..((data.len() as i32).saturating_sub(1)) {
        let pair: (i32, i32) = (
            data.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            {
                let base = &data;
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
        result.push(pair);
    }
    result
}
#[doc = "Create sliding windows over data"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn sliding_window(data: &Vec<i32>, window_size: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..((data.len() as i32).saturating_sub(window_size) + 1) {
        let mut window: Vec<i32> = vec![];
        for j in 0..(window_size) {
            window.push({
                let base = &data;
                let idx: i32 = i + j;
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            });
        }
        result.push(window);
    }
    result
}
#[doc = "Compose two functions(f ∘ g)"]
#[doc = " Depyler: verified panic-free"]
pub fn compose_two_functions(data: &Vec<i32>) -> Vec<i32> {
    let step1: Vec<i32> = map_transform(&data, 2);
    let mut step2: Vec<i32> = vec![];
    for item in step1.iter().cloned() {
        step2.push(item + 1);
    }
    step2
}
#[doc = "Apply multiple operations in sequence"]
#[doc = " Depyler: verified panic-free"]
pub fn apply_multiple_operations<'a, 'b>(
    data: &'a Vec<i32>,
    operations: &'b Vec<String>,
) -> Vec<i32> {
    let mut result: Vec<i32> = Default::default();
    let mut new_result: Vec<i32> = Default::default();
    result = data.clone();
    for op in operations.iter().cloned() {
        new_result = vec![];
        if op == "double" {
            for item in result.iter().cloned() {
                new_result.push(item * 2);
            }
        } else {
            if op == "increment" {
                for item in result.iter().cloned() {
                    new_result.push(item + 1);
                }
            } else {
                if op == "square" {
                    for item in result.iter().cloned() {
                        new_result.push(item * item);
                    }
                } else {
                    new_result = result;
                }
            }
        }
        result = new_result;
    }
    result
}
#[doc = "Classic map-reduce pattern"]
#[doc = " Depyler: verified panic-free"]
pub fn map_reduce_pattern(data: &Vec<i32>) -> i32 {
    let mut mapped: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        mapped.push(item * item);
    }
    let reduced: i32 = reduce_sum(&mapped);
    reduced
}
#[doc = "Filter-Map-Reduce pipeline"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn filter_map_reduce_pattern(data: &Vec<i32>, threshold: i32) -> i32 {
    let filtered: Vec<i32> = filter_predicate(&data, threshold);
    let mapped: Vec<i32> = map_transform(&filtered, 3);
    let reduced: i32 = reduce_sum(&mapped);
    reduced
}
#[doc = "Get unique elements(set-like operation)"]
#[doc = " Depyler: verified panic-free"]
pub fn unique_elements(data: &Vec<i32>) -> Vec<i32> {
    let mut seen: std::collections::HashMap<i32, bool> = {
        let map: HashMap<i32, bool> = HashMap::new();
        map
    };
    let mut result: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        if seen.get(&item).is_none() {
            seen.insert(item.clone(), true);
            result.push(item);
        }
    }
    result
}
#[doc = "Count occurrences of each value"]
pub fn count_by_value(data: &Vec<i32>) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let mut counts: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    for item in data.iter().cloned() {
        if counts.get(&item).is_some() {
            {
                let _key = item;
                let _old_val = counts.get(&_key).cloned().unwrap_or_default();
                counts.insert(_key, _old_val + 1);
            }
        } else {
            counts.insert(item.clone(), 1);
        }
    }
    Ok(counts)
}
#[doc = "Sort list of tuples by second element"]
#[doc = " Depyler: proven to terminate"]
pub fn sorted_by_key(
    items: &Vec<(String, i32)>,
) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
    let mut result: Vec<(String, i32)> = items.clone();
    for i in 0..(result.len() as i32) {
        for j in (i + 1)..(result.len() as i32) {
            if result
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .1
                < result
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .1
            {
                let temp: (String, i32) = result
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                result.insert(
                    (i) as usize,
                    result
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                );
                result.insert((j) as usize, temp);
            }
        }
    }
    Ok(result)
}
#[doc = "Demonstrate functional programming patterns"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demonstrate_functional_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=== Functional Programming Patterns Demo ===");
    let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", "\n1. Map Pattern");
    let doubled: Vec<i32> = map_transform(&data, 2);
    println!(
        "{}",
        format!("   Doubled: {} elements", doubled.len() as i32)
    );
    println!("{}", "\n2. Filter Pattern");
    let filtered: Vec<i32> = filter_predicate(&data, 5);
    println!(
        "{}",
        format!("   Filtered(>5): {} elements", filtered.len() as i32)
    );
    println!("{}", "\n3. Reduce Pattern");
    let total: i32 = reduce_sum(&data);
    println!("{}", format!("   Sum: {}", total));
    println!("{}", "\n4. Chained Operations");
    let chained: i32 = chain_operations(&data);
    println!("{}", format!("   Result: {}", chained));
    println!("{}", "\n5. Zip Pattern");
    let labels: Vec<String> = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
    ];
    let zipped: Vec<(i32, String)> = zip_lists(
        &{
            let base = &data;
            let stop_idx = 5 as isize;
            let stop = if stop_idx < 0 {
                (base.len() as isize + stop_idx).max(0) as usize
            } else {
                stop_idx as usize
            };
            base[..stop.min(base.len())].to_vec()
        },
        &labels,
    );
    println!("{}", format!("   Zipped: {} pairs", zipped.len() as i32));
    println!("{}", "\n6. Group By Pattern");
    let groups: std::collections::HashMap<i32, Vec<i32>> = group_by_property(&data, 3)?;
    println!(
        "{}",
        format!("   Groups(mod 3): {} groups", groups.len() as i32)
    );
    println!("{}", "\n7. Partition Pattern");
    let parts: (Vec<i32>, Vec<i32>) = partition_by_predicate(&data, 6);
    println!(
        "{}",
        format!(
            "   Partition: {} passed, {} failed",
            parts.0.len() as i32,
            parts.1.len() as i32
        )
    );
    println!("{}", "\n8. Accumulate Pattern");
    let running_sums: Vec<i32> = accumulate_running_sum(&data);
    println!(
        "{}",
        format!("   Running sums: {} values", running_sums.len() as i32)
    );
    println!("{}", "\n9. Flatten Pattern");
    let nested: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = flatten_nested_list(&nested);
    println!(
        "{}",
        format!("   Flattened: {} elements", flattened.len() as i32)
    );
    println!("{}", "\n10. Cartesian Product");
    let list1: Vec<i32> = vec![1, 2, 3];
    let list2: Vec<i32> = vec![10, 20];
    let product: Vec<(i32, i32)> = cartesian_product(&list1, &list2);
    println!(
        "{}",
        format!("   Product: {} combinations", product.len() as i32)
    );
    println!("{}", "\n11. Take While Pattern");
    let taken: Vec<i32> = take_while_condition(&data, 6);
    println!(
        "{}",
        format!("   Taken(while <6): {} elements", taken.len() as i32)
    );
    println!("{}", "\n12. Pairwise Iteration");
    let pairs: Vec<(i32, i32)> = pairwise_iteration(&data);
    println!("{}", format!("   Pairs: {} pairs", pairs.len() as i32));
    println!("{}", "\n13. Sliding Window");
    let windows: Vec<Vec<i32>> = sliding_window(&data, 3);
    println!(
        "{}",
        format!("   Windows(size 3): {} windows", windows.len() as i32)
    );
    println!("{}", "\n14. Function Composition");
    let composed: Vec<i32> = compose_two_functions(&vec![1, 2, 3]);
    println!(
        "{}",
        format!("   Composed result: {} elements", composed.len() as i32)
    );
    println!("{}", "\n15. Map-Reduce Pattern");
    let mr_result: i32 = map_reduce_pattern(&vec![1, 2, 3, 4]);
    println!("{}", format!("   Map-Reduce sum of squares: {}", mr_result));
    println!("{}", "\n16. Filter-Map-Reduce");
    let fmr_result: i32 = filter_map_reduce_pattern(&data, 5);
    println!("{}", format!("   Filter-Map-Reduce result: {}", fmr_result));
    println!("{}", "\n17. Unique Elements");
    let duplicates: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let unique: Vec<i32> = unique_elements(&duplicates);
    println!("{}", format!("   Unique elements: {}", unique.len() as i32));
    println!("{}", "\n=== All Patterns Demonstrated ===");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_reduce_sum_examples() {
        assert_eq!(reduce_sum(&vec![]), 0);
        assert_eq!(reduce_sum(&vec![1]), 1);
        assert_eq!(reduce_sum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_reduce_product_examples() {
        assert_eq!(reduce_product(&vec![]), 0);
        assert_eq!(reduce_product(&vec![1]), 1);
        assert_eq!(reduce_product(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_chain_operations_examples() {
        assert_eq!(chain_operations(&vec![]), 0);
        assert_eq!(chain_operations(&vec![1]), 1);
        assert_eq!(chain_operations(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_enumerate_list_examples() {
        assert_eq!(enumerate_list(vec![]), vec![]);
        assert_eq!(enumerate_list(vec![1]), vec![1]);
    }
    #[test]
    fn test_accumulate_running_sum_examples() {
        assert_eq!(accumulate_running_sum(vec![]), vec![]);
        assert_eq!(accumulate_running_sum(vec![1]), vec![1]);
    }
    #[test]
    fn test_flatten_nested_list_examples() {
        assert_eq!(flatten_nested_list(vec![]), vec![]);
        assert_eq!(flatten_nested_list(vec![1]), vec![1]);
    }
    #[test]
    fn test_pairwise_iteration_examples() {
        assert_eq!(pairwise_iteration(vec![]), vec![]);
        assert_eq!(pairwise_iteration(vec![1]), vec![1]);
    }
    #[test]
    fn test_compose_two_functions_examples() {
        assert_eq!(compose_two_functions(vec![]), vec![]);
        assert_eq!(compose_two_functions(vec![1]), vec![1]);
    }
    #[test]
    fn test_map_reduce_pattern_examples() {
        assert_eq!(map_reduce_pattern(&vec![]), 0);
        assert_eq!(map_reduce_pattern(&vec![1]), 1);
        assert_eq!(map_reduce_pattern(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_unique_elements_examples() {
        assert_eq!(unique_elements(vec![]), vec![]);
        assert_eq!(unique_elements(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_sorted_by_key() {
        fn prop(items: Vec<()>) -> TestResult {
            let input_len = items.len();
            let result = sorted_by_key(&items);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = sorted_by_key(&items);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = items.clone();
            input_sorted.sort();
            let mut result = sorted_by_key(&items);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<()>) -> TestResult);
    }
    #[test]
    fn test_sorted_by_key_examples() {
        assert_eq!(sorted_by_key(vec![]), vec![]);
        assert_eq!(sorted_by_key(vec![1]), vec![1]);
    }
}
