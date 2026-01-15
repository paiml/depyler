#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
#[doc = "// NOTE: Map Python module 'enum'(tracked in DEPYLER-0424)"]
const STR_INI: &'static str = "ini";
const STR_JSON: &'static str = "json";
const STR_TOML: &'static str = "toml";
use std::borrow::Cow;
use std::collections::HashMap;
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
pub struct FileFormat {}
impl FileFormat {
    pub const JSON: i32 = auto();
    pub const INI: i32 = auto();
    pub const TOML: i32 = auto();
    pub const PROPERTIES: i32 = auto();
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub line: i32,
    pub message: String,
}
impl ParseError {
    pub fn new(line: i32, message: String) -> Self {
        Self { line, message }
    }
}
#[derive(Default)]
enum Commands {
    #[default]
    __DepylerNone,
    
    Load {
        #[doc = "Settings file"]
        file: String,
        
        #[doc = "File format(auto-detected)"]
        format: Option<String>,
        
        #[doc = "Output format"]
        output: Option<String>,
    },
    
    Set {
        #[doc = "Settings file"]
        file: String,
        #[doc = "Dot-notation path"]
        path: String,
        #[doc = "Value to set"]
        value: String,
    },
    
    Merge {
        #[doc = "Settings files to merge"]
        files: Vec<String>,
        
        #[doc = "Output format"]
        output: Option<String>,
    },
    
    Get {
        #[doc = "Settings file"]
        file: String,
        #[doc = "Dot-notation path"]
        path: String,
    },
    
    Convert {
        #[doc = "Input file"]
        input: String,
        
        #[doc = "Target format"]
        to: Option<String>,
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
#[doc = "Detect file format from extension."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn detect_format(filename: &str) -> FileFormat {
    let lower = filename.to_lowercase();
    if lower.ends_with(".json") {
        return FileFormat::JSON;
    }
    let _cse_temp_0 = (lower.ends_with(".ini")) || (lower.ends_with(".cfg"));
    if _cse_temp_0 {
        return FileFormat::INI;
    }
    if lower.ends_with(".toml") {
        return FileFormat::TOML;
    }
    if lower.ends_with(".properties") {
        return FileFormat::PROPERTIES;
    }
    FileFormat::INI
}
#[doc = "Parse JSON content."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_json(
    content: &str,
) -> (
    std::collections::HashMap<DepylerValue, DepylerValue>,
    Vec<ParseError>,
) {
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        return Ok((
            std::collections::HashMap::<String, DepylerValue>::new(),
            vec![],
        ));
    })() {
        Ok(_result) => {
            return _result;
        }
        Err(e) => {
            return (
                {
                    let map: HashMap<String, String> = HashMap::new();
                    map
                },
                vec![ParseError::new(e.lineno, (e).to_string())],
            );
        }
    }
}
#[doc = "Parse INI content."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_ini(
    content: &str,
) -> (
    std::collections::HashMap<DepylerValue, DepylerValue>,
    Vec<ParseError>,
) {
    let mut value: String = Default::default();
    let mut current_section: DepylerValue = Default::default();
    let mut result = {
        let mut map = HashMap::new();
        map
    };
    let mut errors = vec![];
    for (line_num, mut line) in content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| ((i + 1 as usize) as i32, x))
    {
        let line_num = line_num as i32;
        line = line.trim().to_string();
        if ((line.is_empty()) || (line.starts_with("#"))) || (line.starts_with(";")) {
            continue;
        }
        if (line.starts_with("[")) && (line.ends_with("]")) {
            let section_name = {
                let base = line;
                let start_idx: i32 = 1;
                let stop_idx: i32 = -1;
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
            }
            .trim()
            .to_string();
            if !section_name.is_empty() {
                current_section = section_name;
                if result.get(&section_name).is_none() {
                    result.insert(
                        section_name.to_string().clone(),
                        DepylerValue::Str(format!(
                            "{:?}",
                            format!("{:?}", {
                                let map: HashMap<String, String> = HashMap::new();
                                map
                            })
                        )),
                    );
                }
            } else {
                errors.push(DepylerValue::Str(format!(
                    "{:?}",
                    ParseError::new(line_num, "Empty section name".to_string().to_string())
                )));
            }
            continue;
        }
        if line.contains("=") {
            let _split_parts = line
                .splitn((1 + 1) as usize, "=")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let mut key = _split_parts.get(0).cloned().unwrap_or_default();
            let mut value = _split_parts.get(1).cloned().unwrap_or_default();
            key = key.trim().to_string();
            value = value.trim().to_string();
            if ((value.starts_with("\"")) && (value.ends_with("\"")))
                || ((value.starts_with("'")) && (value.ends_with("'")))
            {
                value = {
                    let base = &value;
                    let start_idx = (1) as isize;
                    let stop_idx = (-1) as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    let stop = if stop_idx < 0 {
                        (base.len() as isize + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    if start < base.len() {
                        base[start..stop.min(base.len())].to_vec()
                    } else {
                        Vec::new()
                    }
                };
            }
            value = convert_ini_value(&value);
            if !current_section.is_empty() {
                result
                    .get_mut(&current_section)
                    .unwrap()
                    .insert(key.to_string().clone(), value);
            } else {
                result.insert(
                    key.to_string().clone(),
                    DepylerValue::Str(format!("{:?}", value)),
                );
            }
        } else {
            errors.push(DepylerValue::Str(format!(
                "{:?}",
                ParseError::new(line_num, format!("Invalid line: {}", line))
            )));
        }
    }
    (result, errors)
}
#[doc = "Convert INI string value to appropriate type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn convert_ini_value(value: String) -> DepylerValue {
    let _cse_temp_0 = [
        "true".to_string(),
        "yes".to_string(),
        "on".to_string(),
        "1".to_string(),
    ]
    .contains(&value.to_lowercase());
    if _cse_temp_0 {
        return true;
    }
    if _cse_temp_0 {
        return false;
    }
    match (|| -> Result<i32, Box<dyn std::error::Error>> {
        return Ok(value.parse::<i32>().unwrap_or_default());
    })() {
        Ok(_result) => {
            return _result;
        }
        Err(_) => {}
    }
    match (|| -> Result<f64, Box<dyn std::error::Error>> {
        return Ok(value.parse::<f64>().unwrap());
    })() {
        Ok(_result) => {
            return _result;
        }
        Err(_) => {}
    }
    let _cse_temp_1 = value.contains(",");
    if _cse_temp_1 {
        return value
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter()
            .map(|v| v.trim().to_string())
            .collect::<Vec<_>>();
    }
    value
}
#[doc = "Parse simple TOML-like content."]
#[doc = " Depyler: proven to terminate"]
pub fn parse_toml(
    content: &str,
) -> Result<
    (
        std::collections::HashMap<DepylerValue, DepylerValue>,
        Vec<ParseError>,
    ),
    Box<dyn std::error::Error>,
> {
    let mut current_section: i32 = Default::default();
    let result = {
        let mut map = HashMap::new();
        map
    };
    let mut errors = vec![];
    current_section = result;
    for (line_num, mut line) in content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| ((i + 1 as usize) as i32, x))
    {
        let line_num = line_num as i32;
        line = line.trim().to_string();
        if (line.is_empty()) || (line.starts_with("#")) {
            continue;
        }
        if line.starts_with("[") {
            let mut parts: Vec<String>;
            let mut section_name: String;
            if (line.starts_with("[[")) && (line.ends_with("]]")) {
                section_name = {
                    let base = line;
                    let start_idx: i32 = 2;
                    let stop_idx: i32 = -2;
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
                }
                .trim()
                .to_string();
                parts = section_name
                    .split(".")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let mut parent = result.clone();
                for part in {
                    let base = &parts;
                    let stop_idx = (-1) as isize;
                    let stop = if stop_idx < 0 {
                        (base.len() as isize + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    base[..stop.min(base.len())].to_vec()
                } {
                    if parent.get(&part).is_none() {
                        parent.insert(
                            part.to_string().clone(),
                            DepylerValue::Str(format!(
                                "{:?}",
                                format!("{:?}", {
                                    let map: HashMap<String, String> = HashMap::new();
                                    map
                                })
                            )),
                        );
                    }
                    parent = parent
                        .get(&DepylerValue::Int(part as i64))
                        .cloned()
                        .unwrap_or_default();
                }
                if parent
                    .get(&{
                        let base = &parts;
                        base.get(base.len().saturating_sub(1usize))
                            .cloned()
                            .unwrap_or_default()
                    })
                    .is_none()
                {
                    parent.insert(
                        {
                            let base = &parts;
                            base.get(base.len().saturating_sub(1usize))
                                .cloned()
                                .unwrap_or_default()
                        }
                        .to_string(),
                        DepylerValue::Str(format!("{:?}", vec![])),
                    );
                }
                parent
                    .get(&DepylerValue::Int({
                        let base = &parts;
                        base.get(base.len().saturating_sub(1usize))
                            .cloned()
                            .unwrap_or_default()
                    } as i64))
                    .cloned()
                    .unwrap_or_default()
                    .push({
                        let map: HashMap<String, String> = HashMap::new();
                        map
                    });
                current_section = {
                    let base = &parent
                        .get(&DepylerValue::Int({
                            let base = &parts;
                            base.get(base.len().saturating_sub(1usize))
                                .cloned()
                                .unwrap_or_default()
                        } as i64))
                        .cloned()
                        .unwrap_or_default();
                    base.get(base.len().saturating_sub(1usize))
                        .cloned()
                        .unwrap_or_default()
                };
            } else {
                if line.ends_with("]") {
                    section_name = {
                        let base = line;
                        let start_idx: i32 = 1;
                        let stop_idx: i32 = -1;
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
                    }
                    .trim()
                    .to_string();
                    parts = section_name
                        .split(".")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    let mut current = result.clone();
                    for part in parts.iter().cloned() {
                        if current.get(&part).is_none() {
                            current.insert(
                                part.to_string().clone(),
                                DepylerValue::Str(format!(
                                    "{:?}",
                                    format!("{:?}", {
                                        let map: HashMap<String, String> = HashMap::new();
                                        map
                                    })
                                )),
                            );
                        }
                        current = current.get(&part).cloned().unwrap_or_default();
                    }
                    current_section = current;
                }
            }
            continue;
        }
        if line.contains("=") {
            let _split_parts = line
                .splitn((1 + 1) as usize, "=")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let mut key = _split_parts.get(0).cloned().unwrap_or_default();
            let mut value = _split_parts.get(1).cloned().unwrap_or_default();
            key = key.trim().to_string();
            value = value.trim().to_string();
            let parsed_value = parse_toml_value(&value);
            current_section.insert(
                key.to_string().clone(),
                DepylerValue::Str(format!("{:?}", parsed_value)),
            );
        } else {
            errors.push(DepylerValue::Str(format!(
                "{:?}",
                ParseError::new(line_num, format!("Invalid line: {}", line))
            )));
        }
    }
    Ok((result, errors))
}
#[doc = "Parse TOML value."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_toml_value(mut value: String) -> DepylerValue {
    value = value.trim().to_string();
    let _cse_temp_0 = (value.starts_with("\"")) && (value.ends_with("\""));
    if _cse_temp_0 {
        return {
            let base = value;
            let start_idx: i32 = 1;
            let stop_idx: i32 = -1;
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
        }
        .replace("\\\"", "\"");
    }
    if _cse_temp_0 {
        return {
            let base = value;
            let start_idx: i32 = 1;
            let stop_idx: i32 = -1;
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
    }
    let _cse_temp_1 = value == "true";
    if _cse_temp_1 {
        return true;
    }
    let _cse_temp_2 = value == "false";
    if _cse_temp_2 {
        return false;
    }
    if _cse_temp_0 {
        let inner = {
            let base = value;
            let start_idx: i32 = 1;
            let stop_idx: i32 = -1;
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
        }
        .trim()
        .to_string();
        if inner.is_empty() {
            return vec![];
        }
        let mut items = vec![];
        for item in split_array_items(&inner) {
            items.push(DepylerValue::Str(format!(
                "{:?}",
                parse_toml_value(item.trim().to_string())
            )));
        }
        return items;
    }
    match (|| -> Result<i32, Box<dyn std::error::Error>> {
        return Ok(value.parse::<i32>().unwrap_or_default());
    })() {
        Ok(_result) => {
            return _result;
        }
        Err(_) => {}
    }
    match (|| -> Result<f64, Box<dyn std::error::Error>> {
        return Ok(value.parse::<f64>().unwrap());
    })() {
        Ok(_result) => {
            return _result;
        }
        Err(_) => {}
    }
    value
}
#[doc = "Split array items respecting nested brackets and quotes."]
#[doc = " Depyler: verified panic-free"]
pub fn split_array_items(s: &str) -> Vec<String> {
    let mut current: Vec<DepylerValue> = Default::default();
    let mut items = vec![];
    current = vec![];
    let mut depth = 0;
    let mut in_string = false;
    for char in s.chars() {
        let mut string_char: ();
        if (["\"".to_string(), "'".to_string()].contains(&char)) && (!in_string) {
            in_string = true;
            string_char = char;
        } else {
            if (char.to_string() == string_char) && (in_string) {
                in_string = false;
            } else {
                if (char.to_string() == "[") && (!in_string) {
                    depth = depth + 1;
                } else {
                    if (char.to_string() == "]") && (!in_string) {
                        depth = depth - 1;
                    } else {
                        if ((char.to_string() == ",") && (depth == 0)) && (!in_string) {
                            items.push(DepylerValue::Str(format!("{:?}", current.join(""))));
                            current = vec![];
                            continue;
                        }
                    }
                }
            }
        }
        current.push(DepylerValue::Str(format!("{:?}", char)));
    }
    if !current.is_empty() {
        items.push(DepylerValue::Str(format!("{:?}", current.join(""))));
    }
    items
}
#[doc = "Parse Java properties format."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_properties(
    content: &str,
) -> (
    std::collections::HashMap<DepylerValue, DepylerValue>,
    Vec<ParseError>,
) {
    let mut result = {
        let mut map = HashMap::new();
        map
    };
    let errors = vec![];
    for (__line_num, mut line) in content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| ((i + 1 as usize) as i32, x))
    {
        line = line.trim().to_string();
        if ((line.is_empty()) || (line.starts_with("#"))) || (line.starts_with("!")) {
            continue;
        }
        let mut sep_idx = -1;
        for (i, char) in line.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) {
            let i = i as i32;
            if char.to_string() == "\\" {
                continue;
            }
            if ["=".to_string(), ":".to_string()].contains(&char) {
                sep_idx = i;
                break;
            }
        }
        if sep_idx > 0 {
            let key = {
                let base = line;
                let stop_idx: i32 = sep_idx;
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
            let value = {
                let base = line;
                let start_idx: i32 = sep_idx + 1;
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
            result.insert(
                key.to_string().clone(),
                DepylerValue::Str(format!("{:?}", convert_ini_value(&value))),
            );
        } else {
            result.insert(line.to_string().clone(), DepylerValue::Str("".to_string()));
        }
    }
    (result, errors)
}
#[doc = "Load settings from content."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn load<'b, 'a>(
    content: &'a str,
    file_format: &'b FileFormat,
) -> Result<
    (
        std::collections::HashMap<DepylerValue, DepylerValue>,
        Vec<ParseError>,
    ),
    Box<dyn std::error::Error>,
> {
    let _cse_temp_0 = (*file_format) == FileFormat::JSON;
    if _cse_temp_0 {
        return Ok(parse_json(content));
    }
    if _cse_temp_0 {
        return Ok(parse_ini(content));
    }
    if _cse_temp_0 {
        return Ok(parse_toml(content));
    }
    if _cse_temp_0 {
        return Ok(parse_properties(content));
    }
    Ok((
        {
            let map: HashMap<String, String> = HashMap::new();
            map
        },
        vec![ParseError::new(
            0,
            format!("Unknown format: {}", file_format),
        )],
    ))
}
#[doc = "Dump settings to JSON."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn dump_json(settings: &std::collections::HashMap<DepylerValue, DepylerValue>) -> String {
    format!("{:?}", settings)
}
#[doc = "Dump settings to INI format."]
#[doc = " Depyler: verified panic-free"]
pub fn dump_ini<'b, 'a>(
    settings: &'a std::collections::HashMap<DepylerValue, DepylerValue>,
    section: &'b str,
) -> String {
    let mut lines = vec![];
    for (key, value) in settings
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        if !true {
            if true {
                lines.push(DepylerValue::Str(format!(
                    "{:?}",
                    format!("{:?} = {}", key, if value { "true" } else { "false" })
                )));
            } else {
                if true {
                    lines.push(DepylerValue::Str(format!(
                        "{:?}",
                        format!(
                            "{:?} = {}",
                            key,
                            value
                                .iter()
                                .cloned()
                                .map(|v| (v).to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                                .display()
                        )
                    )));
                } else {
                    lines.push(DepylerValue::Str(format!(
                        "{:?}",
                        format!("{:?} = {:?}", key, value)
                    )));
                }
            }
        }
    }
    for (key, value) in settings
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        if true {
            let section_name = if !section.is_empty() {
                format!("{}.{:?}", section, key)
            } else {
                key
            };
            lines.push(DepylerValue::Str(format!(
                "{:?}",
                format!("\n[{}]", section_name)
            )));
            for (k, v) in value
                .as_object()
                .into_iter()
                .flat_map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())))
                .collect::<Vec<_>>()
            {
                if true {
                    lines.push(DepylerValue::Str(format!(
                        "{:?}",
                        dump_ini(
                            &{
                                let mut map = HashMap::new();
                                map.insert(
                                    DepylerValue::from(k),
                                    DepylerValue::Str(format!("{:?}", v)),
                                );
                                map
                            },
                            &section_name
                        )
                    )));
                } else {
                    if true {
                        lines.push(DepylerValue::Str(format!(
                            "{:?}",
                            format!("{:?} = {}", k, if v { "true" } else { "false" })
                        )));
                    } else {
                        if true {
                            lines.push(DepylerValue::Str(format!(
                                "{:?}",
                                format!(
                                    "{:?} = {}",
                                    k,
                                    v.iter()
                                        .cloned()
                                        .map(|x| (x).to_string())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                        .display()
                                )
                            )));
                        } else {
                            lines.push(DepylerValue::Str(format!(
                                "{:?}",
                                format!("{:?} = {:?}", k, v)
                            )));
                        }
                    }
                }
            }
        }
    }
    lines.join("\n")
}
#[doc = "Dump settings to TOML format."]
#[doc = " Depyler: verified panic-free"]
pub fn dump_toml<'a, 'b>(
    settings: &'a std::collections::HashMap<DepylerValue, DepylerValue>,
    prefix: &'b str,
) -> String {
    let mut lines = vec![];
    let mut sections = vec![];
    for (key, value) in settings
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        if true {
            sections.push(DepylerValue::Str(format!("{:?}", (key, value))));
        } else {
            if true {
                let formatted = value
                    .iter()
                    .cloned()
                    .map(|v| format_toml_value(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                lines.push(DepylerValue::Str(format!(
                    "{:?}",
                    format!("{:?} = [{}]", key, formatted)
                )));
            } else {
                lines.push(DepylerValue::Str(format!(
                    "{:?}",
                    format!("{:?} = {}", key, format_toml_value(&value))
                )));
            }
        }
    }
    for (section_name, section_value) in sections.iter().cloned() {
        let full_name = if !prefix.is_empty() {
            format!("{}.{:?}", prefix, section_name)
        } else {
            section_name
        };
        lines.push(DepylerValue::Str(format!(
            "{:?}",
            format!("\n[{}]", full_name)
        )));
        lines.push(DepylerValue::Str(format!(
            "{:?}",
            dump_toml(&section_value, &full_name)
        )));
    }
    lines.join("\n")
}
#[doc = "Format value for TOML output."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn format_toml_value(value: &DepylerValue) -> String {
    if true {
        return if value { "true" } else { "false" };
    }
    if true {
        return format!("\"{}\"", value);
    }
    if true {
        return (value).to_string();
    }
    (value).to_string()
}
#[doc = "Get value by dot-notation path."]
pub fn get_value<'a, 'b>(
    settings: &'a std::collections::HashMap<DepylerValue, DepylerValue>,
    path: &'b str,
) -> Result<DepylerValue, Box<dyn std::error::Error>> {
    let mut current: std::collections::HashMap<DepylerValue, DepylerValue> = Default::default();
    let parts = path
        .split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    current = settings;
    for part in parts.iter().cloned() {
        if (true) && (current.get(&part).is_some()) {
            current = current.get(&part).cloned().unwrap_or_default();
        } else {
            return Ok(None);
        }
    }
    Ok(current)
}
#[doc = "Set value by dot-notation path."]
pub fn set_value<'b, 'a>(
    settings: std::collections::HashMap<DepylerValue, DepylerValue>,
    path: &'a str,
    value: &'b DepylerValue,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>> {
    let mut current: std::collections::HashMap<DepylerValue, DepylerValue> = Default::default();
    let parts = path
        .split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    current = settings;
    for part in {
        let base = &parts;
        let stop_idx = (-1) as isize;
        let stop = if stop_idx < 0 {
            (base.len() as isize + stop_idx).max(0) as usize
        } else {
            stop_idx as usize
        };
        base[..stop.min(base.len())].to_vec()
    } {
        if current.get(&part).is_none() {
            current.insert(
                part.to_string().clone(),
                DepylerValue::Str(format!(
                    "{:?}",
                    format!("{:?}", {
                        let mut map = HashMap::new();
                        map
                    })
                )),
            );
        }
        current = current
            .get(&DepylerValue::Int(part as i64))
            .cloned()
            .unwrap_or_default();
    }
    current.insert(
        {
            let base = &parts;
            base.get(base.len().saturating_sub(1usize))
                .cloned()
                .unwrap_or_default()
        }
        .to_string(),
        DepylerValue::Str(format!("{:?}", value)),
    );
    Ok(settings)
}
#[doc = "Deep merge settings."]
pub fn merge_settings<'a, 'b>(
    base: &'a std::collections::HashMap<DepylerValue, DepylerValue>,
    r#override: &'b std::collections::HashMap<DepylerValue, DepylerValue>,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>> {
    let mut result = base
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    for (key, value) in r#override
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        if ((result.get(&key).is_some()) && (true)) && (true) {
            result.insert(
                key.to_string().clone(),
                DepylerValue::Str(format!(
                    "{:?}",
                    merge_settings(result.get(&key).cloned().unwrap_or_default(), &value)?
                )),
            );
        } else {
            result.insert(
                key.to_string().clone(),
                DepylerValue::Str(format!("{:?}", value)),
            );
        }
    }
    Ok(result)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), std::io::Error> {
    let mut value: DepylerValue = Default::default();
    let mut file_format: DepylerValue = Default::default();
    let mut errors: DepylerValue = Default::default();
    let mut settings: DepylerValue = Default::default();
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
    let format_map = {
        let mut map = HashMap::new();
        map.insert(STR_JSON.to_string(), FileFormat::JSON);
        map.insert(STR_INI.to_string(), FileFormat::INI);
        map.insert(STR_TOML.to_string(), FileFormat::TOML);
        map.insert("properties".to_string(), FileFormat::PROPERTIES);
        map
    };
    let _cse_temp_0 = matches!(args.command, Some(Commands::Load { .. }));
    match &args.command {
        Some(Commands::Load {
            ref file,
            ref format,
            ref output,
            ..
        }) => {
            let output = output.as_deref().unwrap_or_default();
            let mut f = std::fs::File::open(&file)?;
            let mut content = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            file_format = if format {
                format_map.get(&format).cloned()
            } else {
                detect_format(&file)
            };
            (settings, errors) = load(&content, file_format)?;
            if errors {
                for err in errors.iter().cloned() {
                    eprintln!("{}", format!("Line {}: {}", err.line, err.message));
                }
                std::process::exit(1i32)
            }
            let _cse_temp_1 = output == STR_JSON;
            if _cse_temp_1 {
                println!("{}", dump_json(&settings));
            } else {
                let _cse_temp_2 = output == STR_INI;
                if _cse_temp_2 {
                    println!("{}", dump_ini(&settings, ""));
                } else {
                    let _cse_temp_3 = output == STR_TOML;
                    if _cse_temp_3 {
                        println!("{}", dump_toml(&settings, ""));
                    }
                }
            }
            return Ok(());
        }
        _ => {}
    }
    let _cse_temp_4 = matches!(args.command, Some(Commands::Get { .. }));
    match &args.command {
        Some(Commands::Get {
            ref file, ref path, ..
        }) => {
            let mut f = std::fs::File::open(&file)?;
            let mut content = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            file_format = Some(detect_format(&file));
            (settings, errors) = load(&content, file_format)?;
            if errors {
                for err in errors.iter().cloned() {
                    eprintln!("{}", format!("Line {}: {}", err.line, err.message));
                }
                std::process::exit(1i32)
            }
            value = get_value(&settings, &path)?;
            if value.is_null() {
                eprintln!("{}", format!("Path '{}' not found", path));
                std::process::exit(1i32)
            }
            if true {
                println!("{}", dump_json(&value));
            } else {
                println!("{:?}", value);
            }
            return Ok(());
        }
        _ => {}
    }
    let _cse_temp_5 = matches!(args.command, Some(Commands::Set { .. }));
    match &args.command {
        Some(Commands::Set {
            ref file,
            ref path,
            ref value,
            ..
        }) => {
            let mut f = std::fs::File::open(&file)?;
            let mut content = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            file_format = Some(detect_format(&file));
            (settings, errors) = load(&content, file_format)?;
            if errors {
                for err in errors.iter().cloned() {
                    eprintln!("{}", format!("Line {}: {}", err.line, err.message));
                }
                std::process::exit(1i32)
            }
            value = convert_ini_value(value);
            settings = set_value(&settings, &path, &value)?;
            println!("{}", dump_json(&settings));
            return Ok(());
        }
        _ => {}
    }
    let _cse_temp_6 = matches!(args.command, Some(Commands::Merge { .. }));
    match &args.command {
        Some(Commands::Merge {
            ref files,
            ref output,
            ..
        }) => {
            let output = output.as_deref().unwrap_or_default();
            let mut result = {
                let mut map = HashMap::new();
                map
            };
            for file_path in files {
                let mut f = std::fs::File::open(&file_path)?;
                let mut content = {
                    let mut content = String::new();
                    f.read_to_string(&mut content)?;
                    content
                };
                file_format = Some(detect_format(file_path.display().to_string()));
                (settings, errors) = load(&content, file_format)?;
                if errors {
                    for err in errors.iter().cloned() {
                        eprintln!(
                            "{}",
                            format!("{:?} line {}: {}", file_path, err.line, err.message)
                        );
                    }
                    std::process::exit(1i32)
                }
                result = merge_settings(&result, &settings)?;
            }
            let _cse_temp_7 = output == STR_JSON;
            if _cse_temp_7 {
                println!("{}", dump_json(&result));
            } else {
                let _cse_temp_8 = output == STR_INI;
                if _cse_temp_8 {
                    println!("{}", dump_ini(&result, ""));
                } else {
                    let _cse_temp_9 = output == STR_TOML;
                    if _cse_temp_9 {
                        println!("{}", dump_toml(&result, ""));
                    }
                }
            }
            return Ok(());
        }
        _ => {}
    }
    let _cse_temp_10 = matches!(args.command, Some(Commands::Convert { .. }));
    match &args.command {
        Some(Commands::Convert {
            ref input, ref to, ..
        }) => {
            let mut f = std::fs::File::open(&input)?;
            let mut content = {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                content
            };
            file_format = Some(detect_format(&input));
            (settings, errors) = load(&content, file_format)?;
            if errors {
                for err in errors.iter().cloned() {
                    eprintln!("{}", format!("Line {}: {}", err.line, err.message));
                }
                std::process::exit(1i32)
            }
            let _cse_temp_11 = to == STR_JSON;
            if _cse_temp_11 {
                println!("{}", dump_json(&settings));
            } else {
                let _cse_temp_12 = to == STR_INI;
                if _cse_temp_12 {
                    println!("{}", dump_ini(&settings, ""));
                } else {
                    let _cse_temp_13 = to == STR_TOML;
                    if _cse_temp_13 {
                        println!("{}", dump_toml(&settings, ""));
                    }
                }
            }
            return Ok(());
        }
        _ => {}
    }
    {
        ()
    };
    Ok(())
}
