#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::borrow::Cow;
#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
#[doc = "// NOTE: Map Python module 'enum'(tracked in DEPYLER-0424)"]
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::io::Write;
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
#[derive(Debug, Clone)]
pub struct ValueError {
    message: String,
}
impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value error: {}", self.message)
    }
}
impl std::error::Error for ValueError {}
impl ValueError {
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
    #[doc = r" DEPYLER-1050: Tuple variant for Python tuple support"]
    Tuple(Vec<DepylerValue>),
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
            (DepylerValue::Tuple(_dv_a), DepylerValue::Tuple(_dv_b)) => _dv_a == _dv_b,
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
            DepylerValue::Tuple(_dv_tuple) => _dv_tuple.hash(state),
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
            DepylerValue::Tuple(_dv_tuple) => write!(_dv_fmt, "{:?}", _dv_tuple),
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
            DepylerValue::Tuple(_dv_tuple) => _dv_tuple.len(),
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
            DepylerValue::Tuple(_dv_tuple) => format!("{:?}", _dv_tuple),
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
            DepylerValue::Tuple(_dv_tuple) => !_dv_tuple.is_empty(),
            DepylerValue::None => false,
        }
    }
    #[doc = r" DEPYLER-1064: Get tuple element by index for tuple unpacking"]
    #[doc = r" Returns the element at the given index, or panics with a readable error"]
    #[doc = r" Works on both Tuple and List variants(Python treats them similarly for unpacking)"]
    pub fn get_tuple_elem(&self, _dv_idx: usize) -> DepylerValue {
        match self {
            DepylerValue::Tuple(_dv_tuple) => {
                if _dv_idx < _dv_tuple.len() {
                    _dv_tuple[_dv_idx].clone()
                } else {
                    panic!(
                        "Tuple index {} out of bounds(length {})",
                        _dv_idx,
                        _dv_tuple.len()
                    )
                }
            }
            DepylerValue::List(_dv_list) => {
                if _dv_idx < _dv_list.len() {
                    _dv_list[_dv_idx].clone()
                } else {
                    panic!(
                        "List index {} out of bounds(length {})",
                        _dv_idx,
                        _dv_list.len()
                    )
                }
            }
            _dv_other => panic!(
                "Expected tuple or list for unpacking, found {:?}",
                _dv_other
            ),
        }
    }
    #[doc = r" DEPYLER-1064: Extract tuple as Vec for multiple assignment"]
    #[doc = r" Validates that the value is a tuple/list with the expected number of elements"]
    pub fn extract_tuple(&self, _dv_expected_len: usize) -> Vec<DepylerValue> {
        match self {
            DepylerValue::Tuple(_dv_tuple) => {
                if _dv_tuple.len() != _dv_expected_len {
                    panic!(
                        "Expected tuple of length {}, got length {}",
                        _dv_expected_len,
                        _dv_tuple.len()
                    )
                }
                _dv_tuple.clone()
            }
            DepylerValue::List(_dv_list) => {
                if _dv_list.len() != _dv_expected_len {
                    panic!(
                        "Expected list of length {}, got length {}",
                        _dv_expected_len,
                        _dv_list.len()
                    )
                }
                _dv_list.clone()
            }
            _dv_other => panic!(
                "Expected tuple or list for unpacking, found {:?}",
                _dv_other
            ),
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(_dv_list) => &_dv_list[_dv_idx],
            DepylerValue::Tuple(_dv_tuple) => &_dv_tuple[_dv_idx],
            _ => panic!("Cannot index non-list/tuple DepylerValue"),
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
            DepylerValue::Tuple(_dv_tuple) => &_dv_tuple[_dv_key as usize],
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
            DepylerValue::Tuple(_dv_tuple) => _dv_tuple.into_iter(),
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
            DepylerValue::Tuple(_dv_tuple) => {
                _dv_tuple.iter().cloned().collect::<Vec<_>>().into_iter()
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
impl std::cmp::PartialOrd for DepylerValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => Some(_dv_a.cmp(_dv_b)),
            (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => {
                Some(_dv_a.total_cmp(_dv_b))
            }
            (DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) => Some(_dv_a.cmp(_dv_b)),
            (DepylerValue::Bool(_dv_a), DepylerValue::Bool(_dv_b)) => Some(_dv_a.cmp(_dv_b)),
            (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => {
                Some((*_dv_a as f64).total_cmp(_dv_b))
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => {
                Some(_dv_a.total_cmp(&(*_dv_b as f64)))
            }
            (DepylerValue::None, DepylerValue::None) => Some(std::cmp::Ordering::Equal),
            (DepylerValue::None, _) => Some(std::cmp::Ordering::Less),
            (_, DepylerValue::None) => Some(std::cmp::Ordering::Greater),
            (DepylerValue::List(_dv_a), DepylerValue::List(_dv_b)) => _dv_a.partial_cmp(_dv_b),
            (DepylerValue::Tuple(_dv_a), DepylerValue::Tuple(_dv_b)) => _dv_a.partial_cmp(_dv_b),
            _ => Option::None,
        }
    }
}
impl std::cmp::Ord for DepylerValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}
pub fn depyler_min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a.partial_cmp(&b).map_or(true, |c| {
        c == std::cmp::Ordering::Less || c == std::cmp::Ordering::Equal
    }) {
        a
    } else {
        b
    }
}
pub fn depyler_max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a.partial_cmp(&b).map_or(true, |c| {
        c == std::cmp::Ordering::Greater || c == std::cmp::Ordering::Equal
    }) {
        a
    } else {
        b
    }
}
#[doc = r" DEPYLER-1066: Wrapper for Python datetime.date"]
#[doc = r" Provides .day(), .month(), .year() methods matching Python's API"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct DepylerDate(pub u32, pub u32, pub u32);
impl DepylerDate {
    #[doc = r" Create a new date from year, month, day"]
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        DepylerDate(year, month, day)
    }
    #[doc = r" Get today's date(NASA mode: computed from SystemTime)"]
    pub fn today() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let days = (secs / 86400) as i64;
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = (z - era * 146097) as u32;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        DepylerDate(y as u32, m, d)
    }
    #[doc = r" Get the year component"]
    pub fn year(&self) -> u32 {
        self.0
    }
    #[doc = r" Get the month component(1-12)"]
    pub fn month(&self) -> u32 {
        self.1
    }
    #[doc = r" Get the day component(1-31)"]
    pub fn day(&self) -> u32 {
        self.2
    }
    #[doc = r" Convert to tuple(year, month, day) for interop"]
    pub fn to_tuple(&self) -> (u32, u32, u32) {
        (self.0, self.1, self.2)
    }
    #[doc = r" Get weekday(0 = Monday, 6 = Sunday) - Python datetime.date.weekday()"]
    pub fn weekday(&self) -> u32 {
        let (mut y, mut m, d) = (self.0 as i32, self.1 as i32, self.2 as i32);
        if m < 3 {
            m += 12;
            y -= 1;
        }
        let q = d;
        let k = y % 100;
        let j = y / 100;
        let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        ((h + 5) % 7) as u32
    }
    #[doc = r" Get ISO weekday(1 = Monday, 7 = Sunday) - Python datetime.date.isoweekday()"]
    pub fn isoweekday(&self) -> u32 {
        self.weekday() + 1
    }
    #[doc = r" Create date from ordinal(days since year 1, January 1 = ordinal 1)"]
    #[doc = r" Python: date.fromordinal(730120) -> date(2000, 1, 1)"]
    pub fn from_ordinal(ordinal: i64) -> Self {
        let days = ordinal - 719163 - 1;
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = (z - era * 146097) as u32;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        DepylerDate(y as u32, m, d)
    }
}
impl std::fmt::Display for DepylerDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.0, self.1, self.2)
    }
}
#[doc = r" DEPYLER-1067: Wrapper for Python datetime.datetime"]
#[doc = r" Provides .year(), .month(), .day(), .hour(), .minute(), .second(), .microsecond() methods"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct DepylerDateTime {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub microsecond: u32,
}
impl DepylerDateTime {
    #[doc = r" Create a new datetime from components"]
    pub fn new(
        year: u32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
    ) -> Self {
        DepylerDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            microsecond,
        }
    }
    #[doc = r" Get current datetime(NASA mode: computed from SystemTime)"]
    pub fn now() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        let days = (secs / 86400) as i64;
        let day_secs = (secs % 86400) as u32;
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = (z - era * 146097) as u32;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        let hour = day_secs / 3600;
        let minute = (day_secs % 3600) / 60;
        let second = day_secs % 60;
        let microsecond = nanos / 1000;
        DepylerDateTime {
            year: y as u32,
            month: m,
            day: d,
            hour,
            minute,
            second,
            microsecond,
        }
    }
    #[doc = r" Alias for now() - Python datetime.datetime.today()"]
    pub fn today() -> Self {
        Self::now()
    }
    pub fn year(&self) -> u32 {
        self.year
    }
    pub fn month(&self) -> u32 {
        self.month
    }
    pub fn day(&self) -> u32 {
        self.day
    }
    pub fn hour(&self) -> u32 {
        self.hour
    }
    pub fn minute(&self) -> u32 {
        self.minute
    }
    pub fn second(&self) -> u32 {
        self.second
    }
    pub fn microsecond(&self) -> u32 {
        self.microsecond
    }
    #[doc = r" Get weekday(0 = Monday, 6 = Sunday)"]
    pub fn weekday(&self) -> u32 {
        DepylerDate::new(self.year, self.month, self.day).weekday()
    }
    #[doc = r" Get ISO weekday(1 = Monday, 7 = Sunday)"]
    pub fn isoweekday(&self) -> u32 {
        self.weekday() + 1
    }
    #[doc = r" Extract date component"]
    pub fn date(&self) -> DepylerDate {
        DepylerDate::new(self.year, self.month, self.day)
    }
    #[doc = r" Get Unix timestamp"]
    pub fn timestamp(&self) -> f64 {
        let days = self.days_since_epoch();
        let secs = days as f64 * 86400.0
            + self.hour as f64 * 3600.0
            + self.minute as f64 * 60.0
            + self.second as f64
            + self.microsecond as f64 / 1_000_000.0;
        secs
    }
    fn days_since_epoch(&self) -> i64 {
        let (mut y, mut m) = (self.year as i64, self.month as i64);
        if m <= 2 {
            y -= 1;
            m += 12;
        }
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = (y - era * 400) as u32;
        let doy = (153 * (m as u32 - 3) + 2) / 5 + self.day - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146097 + doe as i64 - 719468
    }
    #[doc = r" Create from Unix timestamp"]
    pub fn fromtimestamp(ts: f64) -> Self {
        let secs = ts as u64;
        let microsecond = ((ts - secs as f64) * 1_000_000.0) as u32;
        let days = (secs / 86400) as i64;
        let day_secs = (secs % 86400) as u32;
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = (z - era * 146097) as u32;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        let hour = day_secs / 3600;
        let minute = (day_secs % 3600) / 60;
        let second = day_secs % 60;
        DepylerDateTime {
            year: y as u32,
            month: m,
            day: d,
            hour,
            minute,
            second,
            microsecond,
        }
    }
    #[doc = r" ISO format string"]
    pub fn isoformat(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
impl std::fmt::Display for DepylerDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
#[doc = r" DEPYLER-1068: Wrapper for Python datetime.timedelta"]
#[doc = r" Provides .days, .seconds, .microseconds, .total_seconds() methods"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct DepylerTimeDelta {
    pub days: i64,
    pub seconds: i64,
    pub microseconds: i64,
}
impl DepylerTimeDelta {
    #[doc = r" Create a new timedelta from components"]
    pub fn new(days: i64, seconds: i64, microseconds: i64) -> Self {
        let total_us = days * 86400 * 1_000_000 + seconds * 1_000_000 + microseconds;
        let total_secs = total_us / 1_000_000;
        let us = total_us % 1_000_000;
        let d = total_secs / 86400;
        let s = total_secs % 86400;
        DepylerTimeDelta {
            days: d,
            seconds: s,
            microseconds: us,
        }
    }
    #[doc = r" Create from keyword-style arguments(hours, minutes, etc.)"]
    pub fn from_components(
        days: i64,
        seconds: i64,
        microseconds: i64,
        milliseconds: i64,
        minutes: i64,
        hours: i64,
        weeks: i64,
    ) -> Self {
        let total_days = days + weeks * 7;
        let total_secs = seconds + minutes * 60 + hours * 3600;
        let total_us = microseconds + milliseconds * 1000;
        Self::new(total_days, total_secs, total_us)
    }
    #[doc = r" Get total seconds as f64"]
    pub fn total_seconds(&self) -> f64 {
        self.days as f64 * 86400.0 + self.seconds as f64 + self.microseconds as f64 / 1_000_000.0
    }
    #[doc = r" Get days component"]
    pub fn days(&self) -> i64 {
        self.days
    }
    #[doc = r" Get seconds component(0-86399)"]
    pub fn seconds(&self) -> i64 {
        self.seconds
    }
    #[doc = r" Get microseconds component(0-999999)"]
    pub fn microseconds(&self) -> i64 {
        self.microseconds
    }
}
impl std::ops::Add for DepylerTimeDelta {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.days + other.days,
            self.seconds + other.seconds,
            self.microseconds + other.microseconds,
        )
    }
}
impl std::ops::Sub for DepylerTimeDelta {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.days - other.days,
            self.seconds - other.seconds,
            self.microseconds - other.microseconds,
        )
    }
}
impl std::fmt::Display for DepylerTimeDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.seconds / 3600;
        let mins = (self.seconds % 3600) / 60;
        let secs = self.seconds % 60;
        if self.days != 0 {
            write!(
                f,
                "{} day{}, {:02}:{:02}:{:02}",
                self.days,
                if self.days == 1 { "" } else { "s" },
                hours,
                mins,
                secs
            )
        } else {
            write!(f, "{:02}:{:02}:{:02}", hours, mins, secs)
        }
    }
}
#[doc = r" DEPYLER-1070: Wrapper for Python re.Match object"]
#[doc = r" Provides .group(), .groups(), .start(), .end(), .span() methods"]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DepylerRegexMatch {
    pub matched: String,
    pub start: usize,
    pub end: usize,
    pub groups: Vec<String>,
}
impl DepylerRegexMatch {
    #[doc = r" Create a new match from a string slice match"]
    pub fn new(text: &str, start: usize, end: usize) -> Self {
        DepylerRegexMatch {
            matched: text[start..end].to_string(),
            start,
            end,
            groups: vec![text[start..end].to_string()],
        }
    }
    #[doc = r" Create a match with capture groups"]
    pub fn with_groups(text: &str, start: usize, end: usize, groups: Vec<String>) -> Self {
        DepylerRegexMatch {
            matched: text[start..end].to_string(),
            start,
            end,
            groups,
        }
    }
    #[doc = r" Get the matched string(group 0)"]
    pub fn group(&self, n: usize) -> String {
        self.groups.get(n).cloned().unwrap_or_default()
    }
    #[doc = r" Get all capture groups as a tuple-like Vec"]
    pub fn groups(&self) -> Vec<String> {
        if self.groups.len() > 1 {
            self.groups[1..].to_vec()
        } else {
            vec![]
        }
    }
    #[doc = r" Get the start position"]
    pub fn start(&self) -> usize {
        self.start
    }
    #[doc = r" Get the end position"]
    pub fn end(&self) -> usize {
        self.end
    }
    #[doc = r" Get(start, end) tuple"]
    pub fn span(&self) -> (usize, usize) {
        (self.start, self.end)
    }
    #[doc = r" Get the matched string(equivalent to group(0))"]
    pub fn as_str(&self) -> &str {
        &self.matched
    }
    #[doc = r" Simple pattern search(NASA mode alternative to regex)"]
    #[doc = r" Searches for literal string pattern in text"]
    pub fn search(pattern: &str, text: &str) -> Option<Self> {
        text.find(pattern).map(|start| {
            let end = start + pattern.len();
            DepylerRegexMatch::new(text, start, end)
        })
    }
    #[doc = r" Simple pattern match at start(NASA mode alternative to regex)"]
    pub fn match_start(pattern: &str, text: &str) -> Option<Self> {
        if text.starts_with(pattern) {
            Some(DepylerRegexMatch::new(text, 0, pattern.len()))
        } else {
            None
        }
    }
    #[doc = r" Find all occurrences(NASA mode alternative to regex findall)"]
    pub fn findall(pattern: &str, text: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut start = 0;
        while let Some(pos) = text[start..].find(pattern) {
            results.push(pattern.to_string());
            start += pos + pattern.len();
        }
        results
    }
    #[doc = r" Simple string replacement(NASA mode alternative to regex sub)"]
    pub fn sub(pattern: &str, repl: &str, text: &str) -> String {
        text.replace(pattern, repl)
    }
    #[doc = r" Simple string split(NASA mode alternative to regex split)"]
    pub fn split(pattern: &str, text: &str) -> Vec<String> {
        text.split(pattern).map(|s| s.to_string()).collect()
    }
}
#[derive(Debug, Clone)]
pub struct QuoteStyle {}
impl QuoteStyle {
    pub const NONE: i32 = auto();
    pub const SINGLE: i32 = auto();
    pub const DOUBLE: i32 = auto();
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct EnvEntry {
    pub key: String,
    pub value: String,
    pub quote_style: QuoteStyle,
    pub comment: String,
    pub exported: bool,
}
impl EnvEntry {
    pub fn new(
        key: String,
        value: String,
        quote_style: QuoteStyle,
        comment: String,
        exported: bool,
    ) -> Self {
        Self {
            key,
            value,
            quote_style,
            comment,
            exported,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ParseResult {
    pub entries: Vec<EnvEntry>,
    pub errors: Vec<(i32, String)>,
}
impl ParseResult {
    pub fn new(entries: Vec<EnvEntry>, errors: Vec<(i32, String)>) -> Self {
        Self { entries, errors }
    }
}
#[derive(Default)]
enum Commands {
    #[default]
    __DepylerNone,
    
    Validate {
        #[doc = ".env file"]
        file: String,
    },
    
    Diff {
        #[doc = "First .env file"]
        file1: String,
        #[doc = "Second .env file"]
        file2: String,
    },
    
    Get {
        #[doc = ".env file"]
        file: String,
        #[doc = "Variable name"]
        key: String,
    },
    
    Set {
        #[doc = ".env file"]
        file: String,
        #[doc = "Variable name"]
        key: String,
        #[doc = "Value"]
        value: String,
    },
    
    Parse {
        #[doc = ".env file"]
        file: String,
        
        format: Option<String>,
        
        #[doc = "Interpolate variables"]
        interpolate: bool,
    },
    
    Merge {
        #[doc = ".env files to merge"]
        files: Vec<String>,
    },
}
#[derive(Default)]

struct Args {
    
    command: Option<Commands>,
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn dataclass<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn Enum<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn auto<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = "Parse value and determine quote style."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_value(mut value: String) -> Result<(String, QuoteStyle), Box<dyn std::error::Error>> {
    let mut end: DepylerValue = Default::default();
    value = value.trim().to_string();
    if value.is_empty() {
        return Ok(("".to_string().to_string(), QuoteStyle::NONE));
    }
    if value.starts_with("\"") {
        end = find_closing_quote(&value, "\"".to_string())?;
        let _cse_temp_0 = end > 0;
        if _cse_temp_0 {
            let mut inner = {
                let base = value;
                let start_idx: i32 = 1;
                let stop_idx: i32 = end;
                let len = base.chars().count() as i32;
                let actual_start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx.min(len) as usize
                };
                let actual_stop = if stop_idx < 0 {
                    (len + stop_idx).max(0) as usize
                } else {
                    stop_idx.min(len) as usize
                };
                if actual_start < actual_stop {
                    base.chars()
                        .skip(actual_start)
                        .take(actual_stop - actual_start)
                        .collect::<String>()
                } else {
                    String::new()
                }
            };
            inner = process_escape_sequences(&inner)?;
            return Ok((inner, QuoteStyle::DOUBLE));
        }
        return Ok((
            {
                let base = value;
                let start_idx: i32 = 1;
                let len = base.chars().count() as i32;
                let actual_start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx.min(len) as usize
                };
                base.chars().skip(actual_start).collect::<String>()
            },
            QuoteStyle::DOUBLE,
        ));
    }
    if value.starts_with("'") {
        end = find_closing_quote(&value, "'".to_string())?;
        let _cse_temp_1 = end > 0;
        if _cse_temp_1 {
            return Ok((
                {
                    let base = value;
                    let start_idx: i32 = 1;
                    let stop_idx: i32 = end;
                    let len = base.chars().count() as i32;
                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };
                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };
                    if actual_start < actual_stop {
                        base.chars()
                            .skip(actual_start)
                            .take(actual_stop - actual_start)
                            .collect::<String>()
                    } else {
                        String::new()
                    }
                },
                QuoteStyle::SINGLE,
            ));
        }
        return Ok((
            {
                let base = value;
                let start_idx: i32 = 1;
                let len = base.chars().count() as i32;
                let actual_start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx.min(len) as usize
                };
                base.chars().skip(actual_start).collect::<String>()
            },
            QuoteStyle::SINGLE,
        ));
    }
    let comment_idx = value.find(" #").map(|i| i as i32).unwrap_or(-1);
    let _cse_temp_2 = comment_idx.unwrap_or_default() > 0;
    if _cse_temp_2 {
        value = {
            let base = value;
            let stop_idx: i32 = comment_idx;
            let len = base.chars().count() as i32;
            let actual_stop = if stop_idx < 0 {
                (len + stop_idx).max(0) as usize
            } else {
                stop_idx.min(len) as usize
            };
            base.chars().take(actual_stop).collect::<String>()
        }
        .trim()
        .to_string();
    }
    Ok((value, QuoteStyle::NONE))
}
#[doc = "Find closing quote, respecting escapes for double quotes."]
pub fn find_closing_quote<'a, 'b>(
    s: &'a str,
    quote: &'b str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut i: i32 = Default::default();
    i = 1;
    while i < s.len() as i32 {
        if {
            let base = &s;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == *(*quote)
        {
            if (((*quote) == "\"") && (i > 0))
                && ({
                    let base = &s;
                    let idx: i32 = i - 1;
                    let actual_idx = if idx < 0 {
                        base.chars().count().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.chars()
                        .nth(actual_idx)
                        .map(|c| c.to_string())
                        .unwrap_or_default()
                } == "\\")
            {
                i = i + 1;
                continue;
            }
            return Ok(i);
        }
        i = i + 1;
    }
    Ok(-1)
}
#[doc = "Process escape sequences in double-quoted strings."]
pub fn process_escape_sequences(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = vec![];
    let mut i = 0;
    while i < s.len() as i32 {
        if ({
            let base = &s;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == "\\")
            && (i + 1 < s.len() as i32)
        {
            let next_char = {
                let base = &s;
                let idx: i32 = i + 1;
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            };
            if next_char == "n" {
                result.push(DepylerValue::Str("\n".to_string().to_string()));
            } else {
                if next_char == "t" {
                    result.push(DepylerValue::Str("\t".to_string().to_string()));
                } else {
                    if next_char == "r" {
                        result.push(DepylerValue::Str("\r".to_string().to_string()));
                    } else {
                        if next_char == "\"" {
                            result.push(DepylerValue::Str("\"".to_string()));
                        } else {
                            if next_char == "\\" {
                                result.push(DepylerValue::Str("\\".to_string()));
                            } else {
                                if next_char == "$" {
                                    result.push(DepylerValue::Str("$".to_string()));
                                } else {
                                    result.push(DepylerValue::Str(format!("{:?}", {
                                        let base = s;
                                        let start_idx: i32 = i;
                                        let stop_idx: i32 = i + 2;
                                        let len = base.chars().count() as i32;
                                        let actual_start = if start_idx < 0 {
                                            (len + start_idx).max(0) as usize
                                        } else {
                                            start_idx.min(len) as usize
                                        };
                                        let actual_stop = if stop_idx < 0 {
                                            (len + stop_idx).max(0) as usize
                                        } else {
                                            stop_idx.min(len) as usize
                                        };
                                        if actual_start < actual_stop {
                                            base.chars()
                                                .skip(actual_start)
                                                .take(actual_stop - actual_start)
                                                .collect::<String>()
                                        } else {
                                            String::new()
                                        }
                                    })));
                                }
                            }
                        }
                    }
                }
            }
            i = i + 2;
        } else {
            result.push(DepylerValue::Str(format!("{:?}", {
                let base = &s;
                let idx: i32 = i;
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            })));
            i = i + 1;
        }
    }
    Ok(result.join(""))
}
#[doc = "Parse .env content."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse(content: &str) -> Result<ParseResult, Box<dyn std::error::Error>> {
    let mut stripped: String = Default::default();
    let mut entries = vec![];
    let mut errors = vec![];
    for (line_num, line) in content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| ((i + 1 as usize) as i32, x))
    {
        let line_num = line_num as i32;
        stripped = line.trim().to_string();
        if stripped.is_empty() {
            continue;
        }
        if stripped.starts_with("#") {
            continue;
        }
        let mut exported = false;
        if stripped.starts_with("export ") {
            exported = true;
            stripped = {
                let base = stripped;
                let start_idx: i32 = 7;
                let len = base.chars().count() as i32;
                let actual_start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx.min(len) as usize
                };
                base.chars().skip(actual_start).collect::<String>()
            }
            .trim()
            .to_string();
        }
        let eq_idx = stripped.find("=").map(|i| i as i32).unwrap_or(-1);
        if eq_idx.unwrap_or_default() <= 0 {
            errors.push(DepylerValue::Str(format!(
                "{:?}",
                (line_num, format!("Invalid line: {:?}", line))
            )));
            continue;
        }
        let key = {
            let base = stripped;
            let stop_idx: i32 = eq_idx;
            let len = base.chars().count() as i32;
            let actual_stop = if stop_idx < 0 {
                (len + stop_idx).max(0) as usize
            } else {
                stop_idx.min(len) as usize
            };
            base.chars().take(actual_stop).collect::<String>()
        }
        .trim()
        .to_string();
        let value_part = {
            let base = stripped;
            let start_idx: i32 = eq_idx + 1;
            let len = base.chars().count() as i32;
            let actual_start = if start_idx < 0 {
                (len + start_idx).max(0) as usize
            } else {
                start_idx.min(len) as usize
            };
            base.chars().skip(actual_start).collect::<String>()
        };
        if !is_valid_key(&key) {
            errors.push(DepylerValue::Str(format!(
                "{:?}",
                (line_num, format!("Invalid key: {}", key))
            )));
            continue;
        }
        let (value, quote_style) = parse_value(value_part)?;
        entries.push(DepylerValue::Str(format!(
            "{:?}",
            EnvEntry::new(key, value, quote_style, exported, false)
        )));
    }
    Ok(ParseResult::new(entries, errors))
}
#[doc = "Check if key is valid."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_valid_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }
    if !({
        let base = &key;
        let idx: i32 = 0;
        let actual_idx = if idx < 0 {
            base.chars().count().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.chars()
            .nth(actual_idx)
            .map(|c| c.to_string())
            .unwrap_or_default()
    }
    .chars()
    .all(|c| c.is_alphabetic()))
        || ({
            let base = &key;
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == "_")
    {
        return false;
    }
    key.chars()
        .map(|c| (c.is_alphanumeric()) || (c.to_string() == "_"))
        .all(|x| x)
}
#[doc = "Interpolate variables in value."]
pub fn interpolate<'a, 'b>(
    value: &'a str,
    env: &'b std::collections::HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut i: i32 = Default::default();
    let mut result = vec![];
    i = 0;
    while i < value.len() as i32 {
        if {
            let base = &value;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == "$"
        {
            let mut var_value;
            if (i + 1 < value.len() as i32)
                && ({
                    let base = &value;
                    let idx: i32 = i + 1;
                    let actual_idx = if idx < 0 {
                        base.chars().count().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.chars()
                        .nth(actual_idx)
                        .map(|c| c.to_string())
                        .unwrap_or_default()
                } == "{")
            {
                let end = value[i + 2 as usize..]
                    .find("}")
                    .map(|i| (i + i + 2 as usize) as i32)
                    .unwrap_or(-1);
                if end.unwrap_or_default() != -1 {
                    let var_expr = {
                        let base = value;
                        let start_idx: i32 = i + 2;
                        let stop_idx: i32 = end;
                        let len = base.chars().count() as i32;
                        let actual_start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx.min(len) as usize
                        };
                        let actual_stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx.min(len) as usize
                        };
                        if actual_start < actual_stop {
                            base.chars()
                                .skip(actual_start)
                                .take(actual_stop - actual_start)
                                .collect::<String>()
                        } else {
                            String::new()
                        }
                    };
                    var_value = Some(resolve_var_expr(&var_expr, &env)?);
                    result.push(DepylerValue::Str(format!("{:?}", var_value)));
                    i = end + 1;
                    continue;
                }
            } else {
                if (i + 1 < value.len() as i32)
                    && (({
                        let base = &value;
                        let idx: i32 = i + 1;
                        let actual_idx = if idx < 0 {
                            base.chars().count().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.chars()
                            .nth(actual_idx)
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    }
                    .chars()
                    .all(|c| c.is_alphabetic()))
                        || ({
                            let base = &value;
                            let idx: i32 = i + 1;
                            let actual_idx = if idx < 0 {
                                base.chars().count().saturating_sub(idx.abs() as usize)
                            } else {
                                idx as usize
                            };
                            base.chars()
                                .nth(actual_idx)
                                .map(|c| c.to_string())
                                .unwrap_or_default()
                        } == "_"))
                {
                    let mut j = i + 1;
                    while (j < value.len() as i32)
                        && (({
                            let base = &value;
                            let idx: i32 = j;
                            let actual_idx = if idx < 0 {
                                base.chars().count().saturating_sub(idx.abs() as usize)
                            } else {
                                idx as usize
                            };
                            base.chars()
                                .nth(actual_idx)
                                .map(|c| c.to_string())
                                .unwrap_or_default()
                        }
                        .chars()
                        .all(|c| c.is_alphanumeric()))
                            || ({
                                let base = &value;
                                let idx: i32 = j;
                                let actual_idx = if idx < 0 {
                                    base.chars().count().saturating_sub(idx.abs() as usize)
                                } else {
                                    idx as usize
                                };
                                base.chars()
                                    .nth(actual_idx)
                                    .map(|c| c.to_string())
                                    .unwrap_or_default()
                            } == "_"))
                    {
                        j = j + 1;
                    }
                    let var_name = {
                        let base = value;
                        let start_idx: i32 = i + 1;
                        let stop_idx: i32 = j;
                        let len = base.chars().count() as i32;
                        let actual_start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx.min(len) as usize
                        };
                        let actual_stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx.min(len) as usize
                        };
                        if actual_start < actual_stop {
                            base.chars()
                                .skip(actual_start)
                                .take(actual_stop - actual_start)
                                .collect::<String>()
                        } else {
                            String::new()
                        }
                    };
                    var_value = env.get(&var_name).cloned().unwrap_or("".to_string());
                    result.push(DepylerValue::Str(format!("{:?}", var_value)));
                    i = j;
                    continue;
                }
            }
        }
        result.push(DepylerValue::Str(format!("{:?}", {
            let base = &value;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        })));
        i = i + 1;
    }
    Ok(result.join(""))
}
#[doc = "Resolve variable expression with default value support."]
#[doc = " Depyler: proven to terminate"]
pub fn resolve_var_expr<'a, 'b>(
    expr: &'a str,
    env: &'b mut std::collections::HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut default: DepylerValue = Default::default();
    let mut var_name: DepylerValue = Default::default();
    let _cse_temp_0 = expr.contains(":-");
    if _cse_temp_0 {
        let _split_parts = expr
            .splitn((1 + 1) as usize, ":-")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut var_name = _split_parts.get(0).cloned().unwrap_or_default();
        let mut default = _split_parts.get(1).cloned().unwrap_or_default();
        return Ok(env.get(&var_name).cloned().unwrap_or(default));
    }
    let _cse_temp_1 = expr.contains(":=");
    if _cse_temp_1 {
        let _split_parts = expr
            .splitn((1 + 1) as usize, ":=")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut var_name = _split_parts.get(0).cloned().unwrap_or_default();
        let mut default = _split_parts.get(1).cloned().unwrap_or_default();
        let _cse_temp_2 = env.get(&var_name).is_none();
        if _cse_temp_2 {
            env.insert(var_name.to_string().clone(), default);
        }
        return Ok(env.get(&var_name).cloned().unwrap_or(default));
    }
    let _cse_temp_3 = expr.contains(":?");
    if _cse_temp_3 {
        let _split_parts = expr
            .splitn((1 + 1) as usize, ":?")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut var_name = _split_parts.get(0).cloned().unwrap_or_default();
        let error = _split_parts.get(1).cloned().unwrap_or_default();
        let _cse_temp_4 = env.get(&var_name).is_none();
        if _cse_temp_4 {
            return Err(Box::new(ValueError::new(
                (error) || (format!("Variable {:?} is not set", var_name)),
            )));
        }
        return Ok(env
            .get(&var_name)
            .cloned()
            .unwrap_or_default()
            .as_str()
            .unwrap_or("")
            .to_string());
    }
    let _cse_temp_5 = expr.contains(":+");
    if _cse_temp_5 {
        let _split_parts = expr
            .splitn((1 + 1) as usize, ":+")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut var_name = _split_parts.get(0).cloned().unwrap_or_default();
        let alt = _split_parts.get(1).cloned().unwrap_or_default();
        let _cse_temp_6 = env.get(&var_name).is_some();
        let _cse_temp_7 = (_cse_temp_6) && (env.get(&var_name).cloned().unwrap_or_default());
        if _cse_temp_7 {
            return Ok(alt.to_string());
        }
        return Ok("".to_string());
    }
    Ok(env.get(expr).cloned().unwrap_or("".to_string()))
}
#[doc = "Convert entries to dictionary."]
#[doc = " Depyler: verified panic-free"]
pub fn to_dict(
    entries: &Vec<EnvEntry>,
    interpolate_vars: bool,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut value: DepylerValue = Default::default();
    let mut result = {
        let map: HashMap<String, String> = HashMap::new();
        map
    };
    for entry in entries.iter().cloned() {
        value = entry.value;
        if interpolate_vars {
            value = interpolate(&value, &result)?;
        }
        result.insert(entry.key.to_string(), value);
    }
    Ok(result)
}
#[doc = "Format value with appropriate quoting."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn format_value(value: String, quote_style: &QuoteStyle) -> String {
    let mut escaped: String = Default::default();
    let _cse_temp_0 = (*quote_style) == QuoteStyle::DOUBLE;
    if _cse_temp_0 {
        escaped = value
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\t", "\\t");
        return format!("\"{}\"", escaped);
    }
    if _cse_temp_0 {
        return format!("'{}'", value);
    }
    if needs_quoting(&value) {
        escaped = value.replace("\\", "\\\\").replace("\"", "\\\"");
        return format!("\"{}\"", escaped);
    }
    value.to_string()
}
#[doc = "Check if value needs quoting."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn needs_quoting(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    let _cse_temp_0 = ({
        let base = &value;
        let idx: i32 = 0;
        let actual_idx = if idx < 0 {
            base.chars().count().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.chars()
            .nth(actual_idx)
            .map(|c| c.to_string())
            .unwrap_or_default()
    }
    .chars()
    .all(|c| c.is_whitespace()))
        || (value
            .get(&-1)
            .cloned()
            .unwrap_or_default()
            .chars()
            .all(|c| c.is_whitespace()));
    if _cse_temp_0 {
        return true;
    }
    if " \t\n\r#'\"$`"
        .to_string()
        .into_iter()
        .map(|c| value.contains(c))
        .any(|x| x)
    {
        return true;
    }
    false
}
#[doc = "Serialize entries to .env format."]
#[doc = " Depyler: verified panic-free"]
pub fn serialize(entries: &Vec<EnvEntry>) -> String {
    let mut line: String = Default::default();
    let mut lines = vec![];
    for entry in entries.iter().cloned() {
        let prefix = if entry.exported {
            "export ".to_string()
        } else {
            "".to_string()
        };
        let value = format_value(entry.value, entry.quote_style);
        line = format!("{}{}={:?}", prefix, entry.key, value);
        if entry.comment {
            line = format!("{}{}", line, format!("  # {}", entry.comment));
        }
        lines.push(DepylerValue::Str(line.to_string()));
    }
    lines.join("\n")
}
#[doc = "Merge two entry lists, override taking precedence."]
#[doc = " Depyler: verified panic-free"]
pub fn merge<'a, 'b>(base: &'a Vec<EnvEntry>, r#override: &'b Vec<EnvEntry>) -> Vec<EnvEntry> {
    let mut result = base
        .iter()
        .cloned()
        .map(|e| {
            let _v = e;
            (e.key, _v)
        })
        .collect::<std::collections::HashMap<_, _>>();
    for entry in r#override.iter().cloned() {
        result.insert(
            entry.key.to_string(),
            DepylerValue::Str(format!("{:?}", entry)),
        );
    }
    result.values().cloned().collect::<Vec<_>>()
}
#[doc = "Find differences between two entry lists."]
pub fn diff<'b, 'a>(
    entries1: &'a Vec<EnvEntry>,
    entries2: &'b Vec<EnvEntry>,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>> {
    let dict1 = entries1
        .iter()
        .cloned()
        .map(|e| {
            let _v = e.value;
            (e.key, _v)
        })
        .collect::<std::collections::HashMap<_, _>>();
    let dict2 = entries2
        .iter()
        .cloned()
        .map(|e| {
            let _v = e.value;
            (e.key, _v)
        })
        .collect::<std::collections::HashMap<_, _>>();
    let _cse_temp_0 = dict1
        .keys()
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .union(
            &dict2
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<std::collections::HashSet<_>>(),
        )
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let all_keys = _cse_temp_0;
    let result = {
        let mut map = HashMap::new();
        map.insert(
            "added".to_string(),
            DepylerValue::Str(format!("{:?}", {
                let mut map = HashMap::new();
                map
            })),
        );
        map.insert(
            "removed".to_string(),
            DepylerValue::Str(format!("{:?}", {
                let mut map = HashMap::new();
                map
            })),
        );
        map.insert(
            "changed".to_string(),
            DepylerValue::Str(format!("{:?}", {
                let mut map = HashMap::new();
                map
            })),
        );
        map
    };
    for key in all_keys.iter().cloned() {
        if dict1.get(&key).is_none() {
            result.get_mut(&"added").unwrap().insert(
                key.to_string().clone(),
                DepylerValue::Str(format!(
                    "{:?}",
                    dict2.get(&key).cloned().unwrap_or_default()
                )),
            );
        } else {
            if dict2.get(&key).is_none() {
                result.get_mut(&"removed").unwrap().insert(
                    key.to_string().clone(),
                    DepylerValue::Str(format!(
                        "{:?}",
                        dict1.get(&key).cloned().unwrap_or_default()
                    )),
                );
            } else {
                if dict1.get(&key).cloned().unwrap_or_default()
                    != dict2.get(&key).cloned().unwrap_or_default()
                {
                    result.get_mut(&"changed").unwrap().insert(
                        key.to_string().clone(),
                        DepylerValue::Str(format!(
                            "{:?}",
                            format!("{:?}", {
                                let mut map = HashMap::new();
                                map.insert(
                                    "from".to_string(),
                                    DepylerValue::Str(format!(
                                        "{:?}",
                                        dict1.get(&key).cloned().unwrap_or_default()
                                    )),
                                );
                                map.insert(
                                    "to".to_string(),
                                    DepylerValue::Str(format!(
                                        "{:?}",
                                        dict2.get(&key).cloned().unwrap_or_default()
                                    )),
                                );
                                map
                            })
                        )),
                    );
                }
            }
        }
    }
    Ok(result)
}
#[doc = "Validate entries."]
#[doc = " Depyler: verified panic-free"]
pub fn validate_entries(entries: &Vec<EnvEntry>) -> Vec<String> {
    let mut errors = vec![];
    for entry in entries.iter().cloned() {
        if !is_valid_key(entry.key) {
            errors.push(DepylerValue::Str(format!(
                "{:?}",
                format!("Invalid key: {}", entry.key)
            )));
        }
        if entry.key.starts_with("_") {}
    }
    errors
}
#[doc = "Load entries into os.environ."]
#[doc = " Depyler: verified panic-free"]
pub fn load_into_env(entries: &Vec<EnvEntry>, r#override: bool) {
    for entry in entries.iter().cloned() {
        if (r#override) || (!std::env::var(entry.key).is_ok()) {
            std::env::vars()
                .collect::<std::collections::HashMap<String, String>>()
                .insert(entry.key.to_string(), entry.value);
        }
    }
}
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result: DepylerValue = Default::default();
    let mut env_dict: DepylerValue = Default::default();
    ();
    ();
    ();
    ();
    ();
    ();
    ();
    ();
    ();
    ();
    ();
    ();
    let args = Args::default();
    let _cse_temp_0 = matches!(args.command, Some(Commands::Parse { .. }));
    match &args.command {
        Some(Commands::Parse {
            ref file,
            ref format,
            ref interpolate,
            ..
        }) => {
            let interpolate = *interpolate;
            let mut f = std::fs::File::open(&file)?;
            let mut content = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            result = parse(&content)?;
            if result.errors {
                for (line_num, msg) in result.errors {
                    eprintln!("{}", format!("Line {:?}: {:?}", line_num, msg));
                }
            }
            env_dict = to_dict(result.entries, interpolate)?;
            let _cse_temp_1 = format == "json";
            if _cse_temp_1 {
                println!("{}", format!("{:?}", env_dict));
            } else {
                let _cse_temp_2 = format == "shell";
                if _cse_temp_2 {
                    for (key, value) in env_dict
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect::<Vec<_>>()
                    {
                        let escaped = value.replace("'", "'\"'\"'");
                        println!("{}", format!("export {:?}='{}'", key, escaped));
                    }
                } else {
                    println!("{}", serialize(result.entries));
                }
            }
            return Ok(());
        }
        _ => {}
    }
    let _cse_temp_3 = matches!(args.command, Some(Commands::Get { .. }));
    match &args.command {
        Some(Commands::Get {
            ref file, ref key, ..
        }) => {
            let key = key.to_string();
            let mut f = std::fs::File::open(&file)?;
            let mut content = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            result = parse(&content)?;
            env_dict = to_dict(result.entries, false)?;
            let _cse_temp_4 = env_dict.get(&key).is_some();
            if _cse_temp_4 {
                println!(
                    "{}",
                    env_dict
                        .get(&DepylerValue::Int(key as i64))
                        .cloned()
                        .unwrap_or_default()
                );
                return Ok(());
            } else {
                eprintln!("{}", format!("Key '{}' not found", key));
                std::process::exit(1i32)
            }
        }
        _ => unreachable!("Other command variants handled elsewhere"),
    }
    let _cse_temp_5 = matches!(args.command, Some(Commands::Set { .. }));
    match &args.command {
        Some(Commands::Set {
            ref file,
            ref key,
            ref value,
            ..
        }) => {
            let key = key.to_string();
            let mut f = std::fs::File::open(&file)?;
            let mut content = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            result = parse(&content)?;
            let mut found = false;
            for entry in result.entries {
                if entry.key == key {
                    entry.value = value;
                    found = true;
                    break;
                }
            }
            if !found {
                result
                    .entries
                    .push(EnvEntry::new(key, value, "".to_string(), false));
            }
            println!("{}", serialize(result.entries));
            return Ok(());
        }
        _ => {}
    }
    let _cse_temp_6 = matches!(args.command, Some(Commands::Merge { .. }));
    match &args.command {
        Some(Commands::Merge { ref files, .. }) => {
            let mut merged = vec![];
            for _file_path in files {
                let mut f = std::fs::File::open(&file_path)?;
                let mut content = {
                    let mut content = String::new();
                    f.read_to_string(&mut content)?;
                    content
                };
                result = parse(&content)?;
                merged = merge(&merged, result.entries);
            }
            println!("{}", serialize(&merged));
            return Ok(());
        }
        _ => {}
    }
    let _cse_temp_7 = matches!(args.command, Some(Commands::Diff { .. }));
    match &args.command {
        Some(Commands::Diff {
            ref file1,
            ref file2,
            ..
        }) => {
            let mut f = std::fs::File::open(&file1)?;
            let content1 = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            let mut f = std::fs::File::open(&file2)?;
            let content2 = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            let result1 = parse(&content1)?;
            let result2 = parse(&content2)?;
            let differences = diff(result1.entries, result2.entries)?;
            println!("{}", format!("{:?}", differences));
            return Ok(());
        }
        _ => {}
    }
    let _cse_temp_8 = matches!(args.command, Some(Commands::Validate { .. }));
    match &args.command {
        Some(Commands::Validate { ref file, .. }) => {
            let mut f = std::fs::File::open(&file)?;
            let mut content = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            result = parse(&content)?;
            if result.errors {
                for (line_num, msg) in result.errors {
                    eprintln!("{}", format!("Line {:?}: {:?}", line_num, msg));
                }
                std::process::exit(1i32)
            }
            let errors = validate_entries(result.entries);
            if !errors.is_empty() {
                for error in errors.iter().cloned() {
                    eprintln!("{}", format!("Error: {}", error));
                }
                std::process::exit(1i32)
            }
            println!(
                "{}",
                format!("Valid: {} entries", result.entries.len() as i32)
            );
            return Ok(());
        }
        _ => {}
    }
    {
        ()
    };
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_is_valid_key_examples() {
        let _ = is_valid_key(Default::default());
    }
    #[test]
    fn test_needs_quoting_examples() {
        let _ = needs_quoting(Default::default());
    }
    #[test]
    fn test_validate_entries_examples() {
        assert_eq!(validate_entries(vec![]), vec![]);
        assert_eq!(validate_entries(vec![1]), vec![1]);
    }
}
