#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
use std::collections::HashMap;
use std::collections::HashSet;
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
#[derive(Debug, Clone, PartialEq)]
pub struct SchemaType {
    pub type_name: String,
    pub required: bool,
    pub default: DepylerValue,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub pattern: Option<String>,
    pub enum_values: Option<Vec<DepylerValue>>,
}
impl SchemaType {
    pub fn new(
        type_name: String,
        required: bool,
        default: DepylerValue,
        min_value: Option<f64>,
        max_value: Option<f64>,
        min_length: Option<i32>,
        max_length: Option<i32>,
        pattern: Option<String>,
        enum_values: Option<Vec<DepylerValue>>,
    ) -> Self {
        Self {
            type_name,
            required,
            default,
            min_value,
            max_value,
            min_length,
            max_length,
            pattern,
            enum_values,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub fields: std::collections::HashMap<String, SchemaType>,
    pub allow_extra: bool,
}
impl Schema {
    pub fn new(fields: std::collections::HashMap<String, SchemaType>, allow_extra: bool) -> Self {
        Self {
            fields,
            allow_extra,
        }
    }
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn dataclass<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = "Validate a value against a schema type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_type<'b, 'a>(
    value: &'a DepylerValue,
    schema_type: &'b SchemaType,
) -> (bool, String) {
    let type_map = {
        let mut map = HashMap::new();
        map.insert("string".to_string(), |x: &_| x.to_string());
        map.insert("number".to_string(), (|x| x as i32, |x| x as f64));
        map.insert("integer".to_string(), |x| x as i32);
        map.insert("boolean".to_string(), |x| x != 0);
        map.insert("array".to_string(), list);
        map.insert("object".to_string(), dict);
        map.insert("null".to_string(), std::any::type_name_of_val(&None));
        map
    };
    let expected = type_map.get(&schema_type.type_name).cloned();
    if expected.is_none() {
        return (false, format!("Unknown type: {}", schema_type.type_name));
    }
    if !true {
        return (
            false,
            format!(
                "Expected {}, got {}",
                schema_type.type_name,
                std::any::type_name_of_val(&value).__name__
            ),
        );
    }
    let _cse_temp_0 = schema_type.type_name == "string";
    if _cse_temp_0 {
        let _cse_temp_1 = value.len() as i32;
        let _cse_temp_2 = _cse_temp_1 < schema_type.min_length;
        let _cse_temp_3 = (schema_type.min_length.is_some()) && (_cse_temp_2);
        if _cse_temp_3 {
            return (
                false,
                format!(
                    "String too short: {}<{}",
                    value.len() as i32,
                    schema_type.min_length
                ),
            );
        }
        let _cse_temp_4 = _cse_temp_1 > schema_type.max_length;
        let _cse_temp_5 = (schema_type.max_length.is_some()) && (_cse_temp_4);
        if _cse_temp_5 {
            return (
                false,
                format!(
                    "String too long: {}>{}",
                    value.len() as i32,
                    schema_type.max_length
                ),
            );
        }
    }
    let _cse_temp_6 =
        ["number".to_string(), "integer".to_string()].contains(&schema_type.type_name);
    if _cse_temp_6 {
        let _cse_temp_7 = (*value) < schema_type.min_value;
        let _cse_temp_8 = (schema_type.min_value.is_some()) && (_cse_temp_7);
        if _cse_temp_8 {
            return (
                false,
                format!("Value too small: {}<{}", value, schema_type.min_value),
            );
        }
        let _cse_temp_9 = (*value) > schema_type.max_value;
        let _cse_temp_10 = (schema_type.max_value.is_some()) && (_cse_temp_9);
        if _cse_temp_10 {
            return (
                false,
                format!("Value too large: {}>{}", value, schema_type.max_value),
            );
        }
    }
    let _cse_temp_11 = !schema_type.enum_values.get(&value).is_some();
    let _cse_temp_12 = (schema_type.enum_values.is_some()) && (_cse_temp_11);
    if _cse_temp_12 {
        return (false, format!("Value not in enum: {}", value));
    }
    (true, "".to_string().to_string())
}
#[doc = "Validate a JSON object against a schema."]
pub fn validate_object<'a, 'b>(
    data: &'a std::collections::HashMap<String, DepylerValue>,
    schema: &'b Schema,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut errors = vec![];
    for (field_name, field_type) in schema
        .fields
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        if data.get(&field_name).is_none() {
            if (field_type.required) && (field_type.default.is_none()) {
                errors.push(DepylerValue::Str(format!(
                    "{:?}",
                    format!("Missing required field: {:?}", field_name)
                )));
            }
            continue;
        }
        let (valid, msg) = validate_type(
            data.get(&field_name).cloned().unwrap_or_default(),
            &field_type,
        );
        if !valid {
            errors.push(DepylerValue::Str(format!(
                "{:?}",
                format!("Field '{:?}': {:?}", field_name, msg)
            )));
        }
    }
    if !schema.allow_extra {
        for key in data.keys().cloned() {
            if schema.fields.get(&key).is_none() {
                errors.push(DepylerValue::Str(format!(
                    "{:?}",
                    format!("Unexpected field: {}", key)
                )));
            }
        }
    }
    Ok(errors)
}
#[doc = "Parse JSON string safely."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn json_parse(text: &str) -> (DepylerValue, Option<String>) {
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        return Ok((
            DepylerValue::from(std::collections::HashMap::<String, DepylerValue>::new()),
            DepylerValue::from(None),
        ));
    })() {
        Ok(_result) => {
            return _result;
        }
        Err(e) => {
            return (
                DepylerValue::from(None),
                DepylerValue::from((e).to_string()),
            );
        }
    }
}
#[doc = "Convert data to JSON string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn json_stringify(data: &DepylerValue, pretty: bool) -> String {
    if pretty {
        return format!("{:?}", data);
    }
    format!("{:?}", data)
}
#[doc = "Get value at JSON path(dot notation)."]
pub fn json_get(
    data: DepylerValue,
    path: &str,
) -> Result<DepylerValue, Box<dyn std::error::Error>> {
    let mut current: DepylerValue = Default::default();
    if path.is_empty() {
        return Ok(data);
    }
    let parts = path
        .split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    current = data;
    for part in parts.iter().cloned() {
        if (true) && (current.contains(&*part)) {
            current = current.get(&part).cloned().unwrap_or_default();
        } else {
            if true {
                match part.parse::<i32>() {
                    Ok(idx) => {
                        if (0 <= idx) && (idx < current.len() as i32) {
                            current = current
                                .get(idx as usize)
                                .cloned()
                                .expect("IndexError: list index out of range");
                        } else {
                            return Ok(None);
                        }
                    }
                    Err(_) => {
                        return Ok(None);
                    }
                }
            } else {
                return Ok(None);
            }
        }
    }
    Ok(current)
}
#[doc = "Set value at JSON path(returns new dict)."]
pub fn json_set<'b, 'a, 'c>(
    data: &'a std::collections::HashMap<String, DepylerValue>,
    path: &'b str,
    value: &'c DepylerValue,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>> {
    let mut current: i32 = Default::default();
    let result = std::collections::HashMap::<String, DepylerValue>::new();
    let parts = path
        .split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    current = result;
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
                (part) as usize,
                format!("{:?}", {
                    let map: HashMap<String, String> = HashMap::new();
                    map
                }),
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
        value,
    );
    Ok(result)
}
#[doc = "Delete value at JSON path(returns new dict)."]
pub fn json_delete<'b, 'a>(
    data: &'a std::collections::HashMap<String, DepylerValue>,
    path: &'b str,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>> {
    let mut current: i32 = Default::default();
    let result = std::collections::HashMap::<String, DepylerValue>::new();
    let parts = path
        .split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    current = result;
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
            return Ok(result);
        }
        current = current
            .get(&DepylerValue::Int(part as i64))
            .cloned()
            .unwrap_or_default();
    }
    let _cse_temp_0 = current
        .get(&{
            let base = &parts;
            base.get(base.len().saturating_sub(1usize))
                .cloned()
                .unwrap_or_default()
        })
        .is_some();
    if _cse_temp_0 {}
    Ok(result)
}
#[doc = "Deep merge two JSON objects."]
pub fn json_merge<'b, 'a>(
    base: &'a std::collections::HashMap<String, DepylerValue>,
    overlay: &'b std::collections::HashMap<String, DepylerValue>,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>> {
    let mut result = std::collections::HashMap::<String, DepylerValue>::new();
    for (key, value) in overlay
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        if ((result.get(&key).is_some()) && (true)) && (true) {
            result.insert(
                key.to_string().clone(),
                json_merge(result.get(&key).cloned().unwrap_or_default(), &value)?,
            );
        } else {
            result.insert(
                key.to_string().clone(),
                std::collections::HashMap::<String, DepylerValue>::new(),
            );
        }
    }
    Ok(result)
}
#[doc = "Find differences between two JSON values."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn json_diff<'c, 'a, 'b>(
    obj1: &'a DepylerValue,
    obj2: &'b DepylerValue,
    path: &'c str,
) -> Vec<String> {
    let mut diffs = vec![];
    let _cse_temp_0 = std::any::type_name_of_val(&obj1) != std::any::type_name_of_val(&obj2);
    if _cse_temp_0 {
        diffs.push(DepylerValue::Str(format!(
            "{:?}",
            format!(
                "{}: type {}!= {}",
                path,
                std::any::type_name_of_val(&obj1).__name__,
                std::any::type_name_of_val(&obj2).__name__
            )
        )));
        return diffs;
    }
    if true {
        let _cse_temp_1 = obj1
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .union(
                &obj2
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .collect::<std::collections::HashSet<_>>(),
            )
            .cloned()
            .collect::<std::collections::HashSet<_>>();
        let all_keys = _cse_temp_1;
        for key in {
            let mut sorted_vec = all_keys.iter().cloned().collect::<Vec<_>>();
            sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            sorted_vec
        } {
            let new_path = if !path.is_empty() {
                format!("{}.{:?}", path, key)
            } else {
                key
            };
            if !obj1.get(&key).is_some() {
                diffs.push(DepylerValue::Str(format!(
                    "{:?}",
                    format!("{}: added", new_path)
                )));
            } else {
                if !obj2.get(&key).is_some() {
                    diffs.push(DepylerValue::Str(format!(
                        "{:?}",
                        format!("{}: removed", new_path)
                    )));
                } else {
                    diffs.extend(
                        json_diff(
                            obj1.get(&key).cloned().unwrap_or_default(),
                            obj2.get(&key).cloned().unwrap_or_default(),
                            &new_path,
                        )
                        .iter()
                        .cloned(),
                    );
                }
            }
        }
    } else {
        if true {
            let _cse_temp_2 = obj1.len() as i32;
            let _cse_temp_3 = obj2.len() as i32;
            let _cse_temp_4 = _cse_temp_2 != _cse_temp_3;
            if _cse_temp_4 {
                diffs.push(DepylerValue::Str(format!(
                    "{:?}",
                    format!(
                        "{}: length {}!= {}",
                        path,
                        obj1.len() as i32,
                        obj2.len() as i32
                    )
                )));
            } else {
                for (i, _nested) in obj1
                    .iter()
                    .zip(obj2.iter())
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(i, x)| (i as i32, x))
                {
                    let i = i as i32;
                    diffs.extend(
                        json_diff(&a, &b, format!("{}[{:?}]", path, i))
                            .iter()
                            .cloned(),
                    );
                }
            }
        } else {
            let _cse_temp_5 = obj1 != obj2;
            if _cse_temp_5 {
                diffs.push(DepylerValue::Str(format!(
                    "{:?}",
                    format!("{}: {}!= {}", path, obj1, obj2)
                )));
            }
        }
    }
    diffs
}
#[doc = "Flatten nested JSON to dot-notation keys."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn json_flatten<'a, 'b>(
    data: &'a DepylerValue,
    prefix: &'b str,
) -> HashMap<String, DepylerValue> {
    let mut result = {
        let map: HashMap<String, ()> = HashMap::new();
        map
    };
    if true {
        for (key, value) in data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>()
        {
            let mut new_key = if !prefix.is_empty() {
                format!("{}.{:?}", prefix, key)
            } else {
                key
            };
            for (k, v) in (json_flatten(&value, &new_key)).iter() {
                result.insert(k.clone(), v.clone());
            }
        }
    } else {
        if true {
            for (i, value) in data.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) {
                let i = i as i32;
                let mut new_key = format!("{}[{:?}]", prefix, i);
                for (k, v) in (json_flatten(&value, &new_key)).iter() {
                    result.insert(k.clone(), v.clone());
                }
            }
        } else {
            result.insert(prefix.to_string().clone(), data);
        }
    }
    result
}
#[doc = "Unflatten dot-notation keys to nested JSON."]
pub fn json_unflatten(
    data: &std::collections::HashMap<String, DepylerValue>,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>> {
    let result: std::collections::HashMap<String, DepylerValue> = {
        let map: HashMap<String, ()> = HashMap::new();
        map
    };
    for (key, value) in data
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        let parts = key
            .replace("[", ".")
            .replace("]", "")
            .split(".")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut current = result.clone();
        for (i, part) in {
            let base = &parts;
            let stop_idx = (-1) as isize;
            let stop = if stop_idx < 0 {
                (base.len() as isize + stop_idx).max(0) as usize
            } else {
                stop_idx as usize
            };
            base[..stop.min(base.len())].to_vec()
        }
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| (i as i32, x))
        {
            let i = i as i32;
            let next_part = {
                let base = &parts;
                let idx: i32 = i + 1;
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            };
            let is_array = next_part.chars().all(|c| c.is_numeric());
            if current.get(&part).is_none() {
                current.insert(
                    part.to_string().clone(),
                    if is_array {
                        vec![]
                    } else {
                        {
                            let map: HashMap<String, String> = HashMap::new();
                            map
                        }
                    },
                );
            }
            current = current
                .get(&DepylerValue::Int(part as i64))
                .cloned()
                .unwrap_or_default();
            if (is_array) && (true) {
                let mut idx = next_part.parse::<i32>().unwrap_or_default();
                while current.len() as i32 <= idx {
                    current.push({
                        let map: HashMap<String, String> = HashMap::new();
                        map
                    });
                }
            }
        }
        let final_part = {
            let base = &parts;
            base.get(base.len().saturating_sub(1usize))
                .cloned()
                .unwrap_or_default()
        };
        if (final_part.chars().all(|c| c.is_numeric())) && (true) {
            let mut idx = final_part.parse::<i32>().unwrap_or_default();
            while current.len() as i32 <= idx {
                current.push(None);
            }
            current.insert(idx.to_string().clone(), value);
        } else {
            current.insert(final_part.to_string().clone(), value);
        }
    }
    Ok(result)
}
#[doc = "Query array of objects with conditions."]
#[doc = " Depyler: verified panic-free"]
pub fn json_query<'b, 'a>(
    data: &'a Vec<std::collections::HashMap<String, DepylerValue>>,
    conditions: &'b std::collections::HashMap<String, DepylerValue>,
) -> Result<Vec<HashMap<String, DepylerValue>>, Box<dyn std::error::Error>> {
    let mut results = vec![];
    for item in data.iter().cloned() {
        let mut matches = true;
        for (key, expected) in conditions
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>()
        {
            let actual = json_get(&item, &key)?;
            if actual != expected {
                matches = false;
                break;
            }
        }
        if matches {
            results.push(DepylerValue::Str(format!("{:?}", item)));
        }
    }
    Ok(results)
}
#[doc = "Simulate JSON operations from command strings."]
pub fn simulate_json(operations: &Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut results = vec![];
    let mut context: std::collections::HashMap<String, DepylerValue> = {
        let map: HashMap<String, ()> = HashMap::new();
        map
    };
    for op in operations.iter().cloned() {
        let parts = op
            .splitn((1 + 1) as usize, ":")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let cmd = parts
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if cmd == "parse" {
            let _tuple_tmp = json_parse(
                parts
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
            let data = _tuple_tmp.0.clone();
            let err = _tuple_tmp.1.clone();
            if err {
                results.push(DepylerValue::Str(format!("{:?}", format!("error:{}", err))));
            } else {
                context.insert("data".to_string(), data);
                results.push(DepylerValue::Str("ok".to_string().to_string()));
            }
        } else {
            if cmd == "stringify" {
                results.push(DepylerValue::Str(format!(
                    "{:?}",
                    json_stringify(
                        context.get("data").cloned().unwrap_or({
                            let map: HashMap<String, String> = HashMap::new();
                            map
                        }),
                        false
                    )
                )));
            } else {
                let mut value;
                if cmd == "get" {
                    value = json_get(
                        context.get("data").cloned(),
                        parts
                            .get(1usize)
                            .cloned()
                            .expect("IndexError: list index out of range"),
                    )?;
                    results.push(DepylerValue::Str(format!("{:?}", format!("{:?}", value))));
                } else {
                    if cmd == "set" {
                        let path_value = parts
                            .get(1usize)
                            .cloned()
                            .expect("IndexError: list index out of range")
                            .splitn((1 + 1) as usize, "=")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();
                        let _tuple_tmp = json_parse(
                            path_value
                                .get(1usize)
                                .cloned()
                                .expect("IndexError: list index out of range"),
                        );
                        let mut value = _tuple_tmp.0.clone();
                        let __sanitized = _tuple_tmp.1.clone();
                        context.insert(
                            "data".to_string(),
                            json_set(
                                context.get("data").cloned().unwrap_or({
                                    let map: HashMap<String, String> = HashMap::new();
                                    map
                                }),
                                path_value
                                    .get(0usize)
                                    .cloned()
                                    .expect("IndexError: list index out of range"),
                                &value,
                            )?,
                        );
                        results.push(DepylerValue::Str("ok".to_string().to_string()));
                    } else {
                        if cmd == "delete" {
                            context.insert(
                                "data".to_string(),
                                json_delete(
                                    context.get("data").cloned().unwrap_or({
                                        let map: HashMap<String, String> = HashMap::new();
                                        map
                                    }),
                                    parts
                                        .get(1usize)
                                        .cloned()
                                        .expect("IndexError: list index out of range"),
                                )?,
                            );
                            results.push(DepylerValue::Str("ok".to_string().to_string()));
                        } else {
                            if cmd == "flatten" {
                                let flat = json_flatten(
                                    context.get("data").cloned().unwrap_or({
                                        let map: HashMap<String, ()> = HashMap::new();
                                        map
                                    }),
                                    "",
                                );
                                results.push(DepylerValue::Str(format!(
                                    "{:?}",
                                    json_stringify(&flat, false)
                                )));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(results)
}
#[doc = "CLI entry point."]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _cse_temp_0 = std::env::args().collect::<Vec<String>>().len() as i32;
    let _cse_temp_1 = _cse_temp_0 < 2;
    if _cse_temp_1 {
        println!("{}", "Usage: serial_json_cli.py <command>[args...]");
        println!(
            "{}",
            "Commands: parse, validate, get, set, merge, diff, flatten"
        );
        std::process::exit(1i32)
    }
    let cmd = std::env::args()
        .collect::<Vec<String>>()
        .get(1usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let _cse_temp_2 = cmd == "parse";
    let mut data;
    if _cse_temp_2 {
        let text = {
            use std::io::Read;
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        };
        let _tuple_tmp = json_parse(&text);
        let mut data = _tuple_tmp.0.clone();
        let err = _tuple_tmp.1.clone();
        if err {
            eprintln!("{}", format!("Error: {}", err));
            std::process::exit(1i32)
        }
        println!("{}", json_stringify(&data, true));
    } else {
        let _cse_temp_3 = cmd == "validate";
        let mut __sanitized;
        if _cse_temp_3 {
            let _cse_temp_4 = _cse_temp_0 < 3;
            if _cse_temp_4 {
                eprintln!("{}", "Usage: validate <schema_json>");
                std::process::exit(1i32)
            }
            let _tuple_tmp = json_parse(
                std::env::args()
                    .collect::<Vec<String>>()
                    .get(2usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
            let schema_data = _tuple_tmp.0.clone();
            let mut __sanitized = _tuple_tmp.1.clone();
            let _tuple_tmp = json_parse({
                use std::io::Read;
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer).unwrap();
                buffer
            });
            let mut data = _tuple_tmp.0.clone();
            let mut __sanitized = _tuple_tmp.1.clone();
            let mut fields = {
                let mut map = HashMap::new();
                map
            };
            for (name, _spec) in schema_data
                .get("fields")
                .cloned()
                .unwrap_or({
                    let map: HashMap<String, String> = HashMap::new();
                    map
                })
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<_>>()
            {
                fields.insert(
                    name.to_string().clone(),
                    DepylerValue::Str(format!(
                        "{:?}",
                        SchemaType::new(
                            spec.get("type").cloned().unwrap_or("string"),
                            spec.get("required").cloned().unwrap_or(true),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None
                        )
                    )),
                );
            }
            let schema = Schema::new(fields, false);
            let errors = validate_object(&data, &schema)?;
            if !errors.is_empty() {
                for err in errors.iter().cloned() {
                    println!("{}", err);
                }
                std::process::exit(1i32)
            }
            println!("{}", "Valid");
        } else {
            let _cse_temp_5 = cmd == "get";
            if _cse_temp_5 {
                let _cse_temp_6 = _cse_temp_0 < 3;
                if _cse_temp_6 {
                    eprintln!("{}", "Usage: get <path>");
                    std::process::exit(1i32)
                }
                let _tuple_tmp = json_parse({
                    use std::io::Read;
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();
                    buffer
                });
                let mut data = _tuple_tmp.0.clone();
                let mut __sanitized = _tuple_tmp.1.clone();
                let value = json_get(
                    &data,
                    std::env::args()
                        .collect::<Vec<String>>()
                        .get(2usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                )?;
                println!("{}", json_stringify(&value, true));
            } else {
                let _cse_temp_7 = cmd == "flatten";
                if _cse_temp_7 {
                    let _tuple_tmp = json_parse({
                        use std::io::Read;
                        let mut buffer = String::new();
                        std::io::stdin().read_to_string(&mut buffer).unwrap();
                        buffer
                    });
                    let mut data = _tuple_tmp.0.clone();
                    let mut __sanitized = _tuple_tmp.1.clone();
                    let flat = json_flatten(&data, "");
                    println!("{}", json_stringify(&flat, true));
                }
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_simulate_json_examples() {
        assert_eq!(simulate_json(vec![]), vec![]);
        assert_eq!(simulate_json(vec![1]), vec![1]);
    }
}
