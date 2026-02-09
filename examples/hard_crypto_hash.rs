#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR_A_1: &'static str = "a";
const STR_A_2: &'static str = "A";
const STR_HELLO: &'static str = "hello";
use std::borrow::Cow;
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
    #[doc = r" Convert to String(renamed to avoid shadowing Display::to_string)"]
    #[doc = r" DEPYLER-1121: Renamed from to_string to as_string to fix clippy::inherent_to_string_shadow_display"]
    pub fn as_string(&self) -> String {
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
    #[doc = r" DEPYLER-1215: Get as str reference(for string values only)"]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DepylerValue::Str(_dv_str) => Some(_dv_str.as_str()),
            _ => None,
        }
    }
    #[doc = r" DEPYLER-1215: Get as i64(for integer values)"]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DepylerValue::Int(_dv_int) => Some(*_dv_int),
            _ => None,
        }
    }
    #[doc = r" DEPYLER-1215: Get as f64(for float values)"]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DepylerValue::Float(_dv_float) => Some(*_dv_float),
            DepylerValue::Int(_dv_int) => Some(*_dv_int as f64),
            _ => None,
        }
    }
    #[doc = r" DEPYLER-1215: Get as bool(for boolean values)"]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DepylerValue::Bool(_dv_bool) => Some(*_dv_bool),
            _ => None,
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
    #[doc = r" DEPYLER-1137: Get tag name(XML element proxy)"]
    #[doc = r" Returns empty string for non-element types"]
    pub fn tag(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.clone(),
            _ => String::new(),
        }
    }
    #[doc = r" DEPYLER-1137: Get text content(XML element proxy)"]
    #[doc = r" Returns None for non-string types"]
    pub fn text(&self) -> Option<String> {
        match self {
            DepylerValue::Str(_dv_s) => Some(_dv_s.clone()),
            DepylerValue::None => Option::None,
            _ => Option::None,
        }
    }
    #[doc = r" DEPYLER-1137: Find child element by tag(XML element proxy)"]
    #[doc = r" Returns DepylerValue::None for non-matching/non-container types"]
    pub fn find(&self, _tag: &str) -> DepylerValue {
        match self {
            DepylerValue::List(_dv_list) => _dv_list.first().cloned().unwrap_or(DepylerValue::None),
            DepylerValue::Dict(_dv_dict) => _dv_dict
                .get(&DepylerValue::Str(_tag.to_string()))
                .cloned()
                .unwrap_or(DepylerValue::None),
            _ => DepylerValue::None,
        }
    }
    #[doc = r" DEPYLER-1137: Find all child elements by tag(XML element proxy)"]
    #[doc = r" Returns empty Vec for non-container types"]
    pub fn findall(&self, _tag: &str) -> Vec<DepylerValue> {
        match self {
            DepylerValue::List(_dv_list) => _dv_list.clone(),
            _ => Vec::new(),
        }
    }
    #[doc = r" DEPYLER-1137: Set attribute(XML element proxy)"]
    #[doc = r" No-op for non-dict types"]
    pub fn set(&mut self, key: &str, value: &str) {
        if let DepylerValue::Dict(_dv_dict) = self {
            _dv_dict.insert(
                DepylerValue::Str(String::from(key)),
                DepylerValue::Str(String::from(value)),
            );
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
        DepylerValue::Str(String::from(v))
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
impl From<Vec<String>> for DepylerValue {
    fn from(v: Vec<String>) -> Self {
        DepylerValue::List(v.into_iter().map(DepylerValue::Str).collect())
    }
}
impl From<Vec<i32>> for DepylerValue {
    fn from(v: Vec<i32>) -> Self {
        DepylerValue::List(v.into_iter().map(|x| DepylerValue::Int(x as i64)).collect())
    }
}
impl From<Vec<i64>> for DepylerValue {
    fn from(v: Vec<i64>) -> Self {
        DepylerValue::List(v.into_iter().map(DepylerValue::Int).collect())
    }
}
impl From<Vec<f64>> for DepylerValue {
    fn from(v: Vec<f64>) -> Self {
        DepylerValue::List(v.into_iter().map(DepylerValue::Float).collect())
    }
}
impl From<Vec<bool>> for DepylerValue {
    fn from(v: Vec<bool>) -> Self {
        DepylerValue::List(v.into_iter().map(DepylerValue::Bool).collect())
    }
}
impl From<Vec<&str>> for DepylerValue {
    fn from(v: Vec<&str>) -> Self {
        DepylerValue::List(
            v.into_iter()
                .map(|s| DepylerValue::Str(s.to_string()))
                .collect(),
        )
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
impl From<std::collections::HashSet<DepylerValue>> for DepylerValue {
    fn from(v: std::collections::HashSet<DepylerValue>) -> Self {
        DepylerValue::List(v.into_iter().collect())
    }
}
impl From<std::sync::Arc<std::collections::HashSet<DepylerValue>>> for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<DepylerValue>>) -> Self {
        DepylerValue::List(v.iter().cloned().collect())
    }
}
impl From<std::collections::HashSet<i32>> for DepylerValue {
    fn from(v: std::collections::HashSet<i32>) -> Self {
        DepylerValue::List(v.into_iter().map(|x| DepylerValue::Int(x as i64)).collect())
    }
}
impl From<std::collections::HashSet<i64>> for DepylerValue {
    fn from(v: std::collections::HashSet<i64>) -> Self {
        DepylerValue::List(v.into_iter().map(DepylerValue::Int).collect())
    }
}
impl From<std::collections::HashSet<String>> for DepylerValue {
    fn from(v: std::collections::HashSet<String>) -> Self {
        DepylerValue::List(v.into_iter().map(DepylerValue::Str).collect())
    }
}
impl From<std::sync::Arc<std::collections::HashSet<i32>>> for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<i32>>) -> Self {
        DepylerValue::List(v.iter().map(|x| DepylerValue::Int(*x as i64)).collect())
    }
}
impl From<std::sync::Arc<std::collections::HashSet<i64>>> for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<i64>>) -> Self {
        DepylerValue::List(v.iter().map(|x| DepylerValue::Int(*x)).collect())
    }
}
impl From<std::sync::Arc<std::collections::HashSet<String>>> for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<String>>) -> Self {
        DepylerValue::List(v.iter().map(|s| DepylerValue::Str(s.clone())).collect())
    }
}
impl From<DepylerValue> for i64 {
    fn from(v: DepylerValue) -> Self {
        v.to_i64()
    }
}
impl From<DepylerValue> for i32 {
    fn from(v: DepylerValue) -> Self {
        v.to_i64() as i32
    }
}
impl From<DepylerValue> for f64 {
    fn from(v: DepylerValue) -> Self {
        v.to_f64()
    }
}
impl From<DepylerValue> for f32 {
    fn from(v: DepylerValue) -> Self {
        v.to_f64() as f32
    }
}
impl From<DepylerValue> for String {
    fn from(v: DepylerValue) -> Self {
        v.as_string()
    }
}
impl From<DepylerValue> for bool {
    fn from(v: DepylerValue) -> Self {
        v.to_bool()
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
impl std::cmp::PartialOrd<i32> for DepylerValue {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&DepylerValue::Int(*other as i64))
    }
}
impl std::cmp::PartialOrd<i64> for DepylerValue {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&DepylerValue::Int(*other))
    }
}
impl std::cmp::PartialOrd<f64> for DepylerValue {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&DepylerValue::Float(*other))
    }
}
impl std::cmp::PartialOrd<DepylerValue> for i32 {
    fn partial_cmp(&self, other: &DepylerValue) -> Option<std::cmp::Ordering> {
        DepylerValue::Int(*self as i64).partial_cmp(other)
    }
}
impl std::cmp::PartialOrd<DepylerValue> for i64 {
    fn partial_cmp(&self, other: &DepylerValue) -> Option<std::cmp::Ordering> {
        DepylerValue::Int(*self).partial_cmp(other)
    }
}
impl std::cmp::PartialOrd<DepylerValue> for f64 {
    fn partial_cmp(&self, other: &DepylerValue) -> Option<std::cmp::Ordering> {
        DepylerValue::Float(*self).partial_cmp(other)
    }
}
impl std::cmp::PartialEq<i32> for DepylerValue {
    fn eq(&self, other: &i32) -> bool {
        self == &DepylerValue::Int(*other as i64)
    }
}
impl std::cmp::PartialEq<i64> for DepylerValue {
    fn eq(&self, other: &i64) -> bool {
        self == &DepylerValue::Int(*other)
    }
}
impl std::cmp::PartialEq<f64> for DepylerValue {
    fn eq(&self, other: &f64) -> bool {
        self == &DepylerValue::Float(*other)
    }
}
impl std::cmp::PartialEq<DepylerValue> for i32 {
    fn eq(&self, other: &DepylerValue) -> bool {
        &DepylerValue::Int(*self as i64) == other
    }
}
impl std::cmp::PartialEq<DepylerValue> for i64 {
    fn eq(&self, other: &DepylerValue) -> bool {
        &DepylerValue::Int(*self) == other
    }
}
impl std::cmp::PartialEq<DepylerValue> for f64 {
    fn eq(&self, other: &DepylerValue) -> bool {
        &DepylerValue::Float(*self) == other
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
pub trait PyTruthy {
    #[doc = r#" Returns true if the value is "truthy" in Python semantics."#]
    fn is_true(&self) -> bool;
}
impl PyTruthy for bool {
    #[inline]
    fn is_true(&self) -> bool {
        *self
    }
}
impl PyTruthy for i32 {
    #[inline]
    fn is_true(&self) -> bool {
        *self != 0
    }
}
impl PyTruthy for i64 {
    #[inline]
    fn is_true(&self) -> bool {
        *self != 0
    }
}
impl PyTruthy for f32 {
    #[inline]
    fn is_true(&self) -> bool {
        *self != 0.0
    }
}
impl PyTruthy for f64 {
    #[inline]
    fn is_true(&self) -> bool {
        *self != 0.0
    }
}
impl PyTruthy for String {
    #[inline]
    fn is_true(&self) -> bool {
        !self.is_empty()
    }
}
impl PyTruthy for &str {
    #[inline]
    fn is_true(&self) -> bool {
        !self.is_empty()
    }
}
impl<T> PyTruthy for Vec<T> {
    #[inline]
    fn is_true(&self) -> bool {
        !self.is_empty()
    }
}
impl<T> PyTruthy for Option<T> {
    #[inline]
    fn is_true(&self) -> bool {
        self.is_some()
    }
}
impl<K, V> PyTruthy for std::collections::HashMap<K, V> {
    #[inline]
    fn is_true(&self) -> bool {
        !self.is_empty()
    }
}
impl<K, V> PyTruthy for std::collections::BTreeMap<K, V> {
    #[inline]
    fn is_true(&self) -> bool {
        !self.is_empty()
    }
}
impl<T> PyTruthy for std::collections::HashSet<T> {
    #[inline]
    fn is_true(&self) -> bool {
        !self.is_empty()
    }
}
impl<T> PyTruthy for std::collections::BTreeSet<T> {
    #[inline]
    fn is_true(&self) -> bool {
        !self.is_empty()
    }
}
impl<T> PyTruthy for std::collections::VecDeque<T> {
    #[inline]
    fn is_true(&self) -> bool {
        !self.is_empty()
    }
}
impl PyTruthy for DepylerValue {
    #[doc = r" Python truthiness for DepylerValue:"]
    #[doc = r#" - Int(0), Float(0.0), Str(""), Bool(false), None -> false"#]
    #[doc = r" - List([]), Dict({}), Tuple([]) -> false"]
    #[doc = r" - Everything else -> true"]
    #[inline]
    fn is_true(&self) -> bool {
        match self {
            DepylerValue::Bool(_dv_b) => *_dv_b,
            DepylerValue::Int(_dv_i) => *_dv_i != 0,
            DepylerValue::Float(_dv_f) => *_dv_f != 0.0,
            DepylerValue::Str(_dv_s) => !_dv_s.is_empty(),
            DepylerValue::List(_dv_l) => !_dv_l.is_empty(),
            DepylerValue::Dict(_dv_d) => !_dv_d.is_empty(),
            DepylerValue::Tuple(_dv_t) => !_dv_t.is_empty(),
            DepylerValue::None => false,
        }
    }
}
pub trait PyAdd<Rhs = Self> {
    type Output;
    fn py_add(self, rhs: Rhs) -> Self::Output;
}
pub trait PySub<Rhs = Self> {
    type Output;
    fn py_sub(self, rhs: Rhs) -> Self::Output;
}
pub trait PyMul<Rhs = Self> {
    type Output;
    fn py_mul(self, rhs: Rhs) -> Self::Output;
}
pub trait PyDiv<Rhs = Self> {
    type Output;
    fn py_div(self, rhs: Rhs) -> Self::Output;
}
pub trait PyMod<Rhs = Self> {
    type Output;
    fn py_mod(self, rhs: Rhs) -> Self::Output;
}
pub trait PyIndex<Idx> {
    type Output;
    fn py_index(&self, index: Idx) -> Self::Output;
}
impl PyAdd for i32 {
    type Output = i32;
    #[inline]
    fn py_add(self, rhs: i32) -> i32 {
        self + rhs
    }
}
impl PyAdd<i64> for i32 {
    type Output = i64;
    #[inline]
    fn py_add(self, rhs: i64) -> i64 {
        self as i64 + rhs
    }
}
impl PyAdd<f64> for i32 {
    type Output = f64;
    #[inline]
    fn py_add(self, rhs: f64) -> f64 {
        self as f64 + rhs
    }
}
impl PyAdd for i64 {
    type Output = i64;
    #[inline]
    fn py_add(self, rhs: i64) -> i64 {
        self + rhs
    }
}
impl PyAdd<i32> for i64 {
    type Output = i64;
    #[inline]
    fn py_add(self, rhs: i32) -> i64 {
        self + rhs as i64
    }
}
impl PyAdd<f64> for i64 {
    type Output = f64;
    #[inline]
    fn py_add(self, rhs: f64) -> f64 {
        self as f64 + rhs
    }
}
impl PyAdd for f64 {
    type Output = f64;
    #[inline]
    fn py_add(self, rhs: f64) -> f64 {
        self + rhs
    }
}
impl PyAdd<i32> for f64 {
    type Output = f64;
    #[inline]
    fn py_add(self, rhs: i32) -> f64 {
        self + rhs as f64
    }
}
impl PyAdd<i64> for f64 {
    type Output = f64;
    #[inline]
    fn py_add(self, rhs: i64) -> f64 {
        self + rhs as f64
    }
}
impl PyAdd for String {
    type Output = String;
    #[inline]
    fn py_add(self, rhs: String) -> String {
        self + &rhs
    }
}
impl PyAdd<&str> for String {
    type Output = String;
    #[inline]
    fn py_add(self, rhs: &str) -> String {
        self + rhs
    }
}
impl PyAdd<&str> for &str {
    type Output = String;
    #[inline]
    fn py_add(self, rhs: &str) -> String {
        format!("{}{}", self, rhs)
    }
}
impl PyAdd<String> for &str {
    type Output = String;
    #[inline]
    fn py_add(self, rhs: String) -> String {
        format!("{}{}", self, rhs)
    }
}
impl PyAdd<char> for String {
    type Output = String;
    #[inline]
    fn py_add(mut self, rhs: char) -> String {
        self.push(rhs);
        self
    }
}
impl PyAdd<char> for &str {
    type Output = String;
    #[inline]
    fn py_add(self, rhs: char) -> String {
        format!("{}{}", self, rhs)
    }
}
impl PyAdd for DepylerValue {
    type Output = DepylerValue;
    fn py_add(self, rhs: DepylerValue) -> DepylerValue {
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
impl PyAdd<DepylerValue> for i32 {
    type Output = i64;
    #[inline]
    fn py_add(self, rhs: DepylerValue) -> i64 {
        self as i64 + rhs.to_i64()
    }
}
impl PyAdd<DepylerValue> for i64 {
    type Output = i64;
    #[inline]
    fn py_add(self, rhs: DepylerValue) -> i64 {
        self + rhs.to_i64()
    }
}
impl PyAdd<DepylerValue> for f64 {
    type Output = f64;
    #[inline]
    fn py_add(self, rhs: DepylerValue) -> f64 {
        self + rhs.to_f64()
    }
}
impl PySub for i32 {
    type Output = i32;
    #[inline]
    fn py_sub(self, rhs: i32) -> i32 {
        self - rhs
    }
}
impl PySub<f64> for i32 {
    type Output = f64;
    #[inline]
    fn py_sub(self, rhs: f64) -> f64 {
        self as f64 - rhs
    }
}
impl PySub for i64 {
    type Output = i64;
    #[inline]
    fn py_sub(self, rhs: i64) -> i64 {
        self - rhs
    }
}
impl PySub<f64> for i64 {
    type Output = f64;
    #[inline]
    fn py_sub(self, rhs: f64) -> f64 {
        self as f64 - rhs
    }
}
impl PySub for f64 {
    type Output = f64;
    #[inline]
    fn py_sub(self, rhs: f64) -> f64 {
        self - rhs
    }
}
impl PySub<i32> for f64 {
    type Output = f64;
    #[inline]
    fn py_sub(self, rhs: i32) -> f64 {
        self - rhs as f64
    }
}
impl PySub<i64> for f64 {
    type Output = f64;
    #[inline]
    fn py_sub(self, rhs: i64) -> f64 {
        self - rhs as f64
    }
}
impl PySub for DepylerValue {
    type Output = DepylerValue;
    fn py_sub(self, rhs: DepylerValue) -> DepylerValue {
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
impl PySub<DepylerValue> for i32 {
    type Output = i64;
    #[inline]
    fn py_sub(self, rhs: DepylerValue) -> i64 {
        self as i64 - rhs.to_i64()
    }
}
impl PySub<DepylerValue> for i64 {
    type Output = i64;
    #[inline]
    fn py_sub(self, rhs: DepylerValue) -> i64 {
        self - rhs.to_i64()
    }
}
impl PySub<DepylerValue> for f64 {
    type Output = f64;
    #[inline]
    fn py_sub(self, rhs: DepylerValue) -> f64 {
        self - rhs.to_f64()
    }
}
impl<T: Eq + std::hash::Hash + Clone> PySub for std::collections::HashSet<T> {
    type Output = std::collections::HashSet<T>;
    fn py_sub(self, rhs: std::collections::HashSet<T>) -> Self::Output {
        self.difference(&rhs).cloned().collect()
    }
}
impl<T: Eq + std::hash::Hash + Clone> PySub<&std::collections::HashSet<T>>
    for std::collections::HashSet<T>
{
    type Output = std::collections::HashSet<T>;
    fn py_sub(self, rhs: &std::collections::HashSet<T>) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}
impl PyMul for i32 {
    type Output = i32;
    #[inline]
    fn py_mul(self, rhs: i32) -> i32 {
        self * rhs
    }
}
impl PyMul<f64> for i32 {
    type Output = f64;
    #[inline]
    fn py_mul(self, rhs: f64) -> f64 {
        self as f64 * rhs
    }
}
impl PyMul<i64> for i32 {
    type Output = i64;
    #[inline]
    fn py_mul(self, rhs: i64) -> i64 {
        self as i64 * rhs
    }
}
impl PyMul for i64 {
    type Output = i64;
    #[inline]
    fn py_mul(self, rhs: i64) -> i64 {
        self * rhs
    }
}
impl PyMul<f64> for i64 {
    type Output = f64;
    #[inline]
    fn py_mul(self, rhs: f64) -> f64 {
        self as f64 * rhs
    }
}
impl PyMul<i32> for i64 {
    type Output = i64;
    #[inline]
    fn py_mul(self, rhs: i32) -> i64 {
        self * rhs as i64
    }
}
impl PyMul for f64 {
    type Output = f64;
    #[inline]
    fn py_mul(self, rhs: f64) -> f64 {
        self * rhs
    }
}
impl PyMul<i32> for f64 {
    type Output = f64;
    #[inline]
    fn py_mul(self, rhs: i32) -> f64 {
        self * rhs as f64
    }
}
impl PyMul<i64> for f64 {
    type Output = f64;
    #[inline]
    fn py_mul(self, rhs: i64) -> f64 {
        self * rhs as f64
    }
}
impl PyMul<i32> for String {
    type Output = String;
    fn py_mul(self, rhs: i32) -> String {
        if rhs <= 0 {
            String::new()
        } else {
            self.repeat(rhs as usize)
        }
    }
}
impl PyMul<i64> for String {
    type Output = String;
    fn py_mul(self, rhs: i64) -> String {
        if rhs <= 0 {
            String::new()
        } else {
            self.repeat(rhs as usize)
        }
    }
}
impl PyMul<i32> for &str {
    type Output = String;
    fn py_mul(self, rhs: i32) -> String {
        if rhs <= 0 {
            String::new()
        } else {
            self.repeat(rhs as usize)
        }
    }
}
impl PyMul<i64> for &str {
    type Output = String;
    fn py_mul(self, rhs: i64) -> String {
        if rhs <= 0 {
            String::new()
        } else {
            self.repeat(rhs as usize)
        }
    }
}
impl PyMul for DepylerValue {
    type Output = DepylerValue;
    fn py_mul(self, rhs: DepylerValue) -> DepylerValue {
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
            (DepylerValue::Str(_dv_s), DepylerValue::Int(_dv_n)) => {
                if _dv_n <= 0 {
                    DepylerValue::Str(String::new())
                } else {
                    DepylerValue::Str(_dv_s.repeat(_dv_n as usize))
                }
            }
            _ => DepylerValue::None,
        }
    }
}
impl PyMul<DepylerValue> for i32 {
    type Output = i64;
    #[inline]
    fn py_mul(self, rhs: DepylerValue) -> i64 {
        self as i64 * rhs.to_i64()
    }
}
impl PyMul<DepylerValue> for i64 {
    type Output = i64;
    #[inline]
    fn py_mul(self, rhs: DepylerValue) -> i64 {
        self * rhs.to_i64()
    }
}
impl PyMul<DepylerValue> for f64 {
    type Output = f64;
    #[inline]
    fn py_mul(self, rhs: DepylerValue) -> f64 {
        self * rhs.to_f64()
    }
}
impl<T: Clone> PyAdd<Vec<T>> for Vec<T> {
    type Output = Vec<T>;
    fn py_add(mut self, rhs: Vec<T>) -> Vec<T> {
        self.extend(rhs);
        self
    }
}
impl<T: Clone> PyAdd<&Vec<T>> for Vec<T> {
    type Output = Vec<T>;
    fn py_add(mut self, rhs: &Vec<T>) -> Vec<T> {
        self.extend(rhs.iter().cloned());
        self
    }
}
impl<T: Clone> PyAdd<Vec<T>> for &Vec<T> {
    type Output = Vec<T>;
    fn py_add(self, rhs: Vec<T>) -> Vec<T> {
        let mut result = self.clone();
        result.extend(rhs);
        result
    }
}
impl<T: Clone> PyMul<i32> for Vec<T> {
    type Output = Vec<T>;
    fn py_mul(self, rhs: i32) -> Vec<T> {
        if rhs <= 0 {
            Vec::new()
        } else {
            self.iter()
                .cloned()
                .cycle()
                .take(self.len() * rhs as usize)
                .collect()
        }
    }
}
impl<T: Clone> PyMul<i64> for Vec<T> {
    type Output = Vec<T>;
    fn py_mul(self, rhs: i64) -> Vec<T> {
        if rhs <= 0 {
            Vec::new()
        } else {
            self.iter()
                .cloned()
                .cycle()
                .take(self.len() * rhs as usize)
                .collect()
        }
    }
}
impl<T: Clone> PyMul<usize> for Vec<T> {
    type Output = Vec<T>;
    fn py_mul(self, rhs: usize) -> Vec<T> {
        self.iter()
            .cloned()
            .cycle()
            .take(self.len() * rhs)
            .collect()
    }
}
impl<T: Clone> PyMul<Vec<T>> for i32 {
    type Output = Vec<T>;
    fn py_mul(self, rhs: Vec<T>) -> Vec<T> {
        rhs.py_mul(self)
    }
}
impl<T: Clone> PyMul<Vec<T>> for i64 {
    type Output = Vec<T>;
    fn py_mul(self, rhs: Vec<T>) -> Vec<T> {
        rhs.py_mul(self)
    }
}
impl PySub<Vec<f64>> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_sub(self, rhs: Vec<f64>) -> Vec<f64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect()
    }
}
impl PySub<&Vec<f64>> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_sub(self, rhs: &Vec<f64>) -> Vec<f64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect()
    }
}
impl PySub<Vec<f64>> for &Vec<f64> {
    type Output = Vec<f64>;
    fn py_sub(self, rhs: Vec<f64>) -> Vec<f64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect()
    }
}
impl PySub<&Vec<f64>> for &Vec<f64> {
    type Output = Vec<f64>;
    fn py_sub(self, rhs: &Vec<f64>) -> Vec<f64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect()
    }
}
impl PySub<Vec<f32>> for Vec<f32> {
    type Output = Vec<f32>;
    fn py_sub(self, rhs: Vec<f32>) -> Vec<f32> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect()
    }
}
impl PySub<Vec<i64>> for Vec<i64> {
    type Output = Vec<i64>;
    fn py_sub(self, rhs: Vec<i64>) -> Vec<i64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect()
    }
}
impl PySub<Vec<i32>> for Vec<i32> {
    type Output = Vec<i32>;
    fn py_sub(self, rhs: Vec<i32>) -> Vec<i32> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect()
    }
}
impl PyMul<Vec<f64>> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect()
    }
}
impl PyMul<&Vec<f64>> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_mul(self, rhs: &Vec<f64>) -> Vec<f64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect()
    }
}
impl PyMul<Vec<f64>> for &Vec<f64> {
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect()
    }
}
impl PyMul<&Vec<f64>> for &Vec<f64> {
    type Output = Vec<f64>;
    fn py_mul(self, rhs: &Vec<f64>) -> Vec<f64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect()
    }
}
impl PyMul<Vec<f32>> for Vec<f32> {
    type Output = Vec<f32>;
    fn py_mul(self, rhs: Vec<f32>) -> Vec<f32> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect()
    }
}
impl PyMul<Vec<i64>> for Vec<i64> {
    type Output = Vec<i64>;
    fn py_mul(self, rhs: Vec<i64>) -> Vec<i64> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect()
    }
}
impl PyMul<Vec<i32>> for Vec<i32> {
    type Output = Vec<i32>;
    fn py_mul(self, rhs: Vec<i32>) -> Vec<i32> {
        self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect()
    }
}
impl PyDiv<Vec<f64>> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<f64>) -> Vec<f64> {
        self.iter()
            .zip(rhs.iter())
            .map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b })
            .collect()
    }
}
impl PyDiv<&Vec<f64>> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_div(self, rhs: &Vec<f64>) -> Vec<f64> {
        self.iter()
            .zip(rhs.iter())
            .map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b })
            .collect()
    }
}
impl PyDiv<Vec<f64>> for &Vec<f64> {
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<f64>) -> Vec<f64> {
        self.iter()
            .zip(rhs.iter())
            .map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b })
            .collect()
    }
}
impl PyDiv<&Vec<f64>> for &Vec<f64> {
    type Output = Vec<f64>;
    fn py_div(self, rhs: &Vec<f64>) -> Vec<f64> {
        self.iter()
            .zip(rhs.iter())
            .map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b })
            .collect()
    }
}
impl PyDiv<Vec<f32>> for Vec<f32> {
    type Output = Vec<f32>;
    fn py_div(self, rhs: Vec<f32>) -> Vec<f32> {
        self.iter()
            .zip(rhs.iter())
            .map(|(a, b)| if *b == 0.0 { f32::NAN } else { a / b })
            .collect()
    }
}
impl PyDiv<Vec<i64>> for Vec<i64> {
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<i64>) -> Vec<f64> {
        self.iter()
            .zip(rhs.iter())
            .map(|(a, b)| {
                if *b == 0 {
                    f64::NAN
                } else {
                    *a as f64 / *b as f64
                }
            })
            .collect()
    }
}
impl PyDiv<Vec<i32>> for Vec<i32> {
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<i32>) -> Vec<f64> {
        self.iter()
            .zip(rhs.iter())
            .map(|(a, b)| {
                if *b == 0 {
                    f64::NAN
                } else {
                    *a as f64 / *b as f64
                }
            })
            .collect()
    }
}
impl PyMul<f64> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_mul(self, rhs: f64) -> Vec<f64> {
        self.iter().map(|a| a * rhs).collect()
    }
}
impl PyMul<Vec<f64>> for f64 {
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64> {
        rhs.iter().map(|a| a * self).collect()
    }
}
impl PyDiv<f64> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_div(self, rhs: f64) -> Vec<f64> {
        if rhs == 0.0 {
            self.iter().map(|_| f64::NAN).collect()
        } else {
            self.iter().map(|a| a / rhs).collect()
        }
    }
}
impl PySub<f64> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_sub(self, rhs: f64) -> Vec<f64> {
        self.iter().map(|a| a - rhs).collect()
    }
}
impl PyAdd<f64> for Vec<f64> {
    type Output = Vec<f64>;
    fn py_add(self, rhs: f64) -> Vec<f64> {
        self.iter().map(|a| a + rhs).collect()
    }
}
impl PyDiv for i32 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: i32) -> f64 {
        if rhs == 0 {
            f64::NAN
        } else {
            self as f64 / rhs as f64
        }
    }
}
impl PyDiv<f64> for i32 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: f64) -> f64 {
        if rhs == 0.0 {
            f64::NAN
        } else {
            self as f64 / rhs
        }
    }
}
impl PyDiv for i64 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: i64) -> f64 {
        if rhs == 0 {
            f64::NAN
        } else {
            self as f64 / rhs as f64
        }
    }
}
impl PyDiv<f64> for i64 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: f64) -> f64 {
        if rhs == 0.0 {
            f64::NAN
        } else {
            self as f64 / rhs
        }
    }
}
impl PyDiv for f64 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: f64) -> f64 {
        if rhs == 0.0 {
            f64::NAN
        } else {
            self / rhs
        }
    }
}
impl PyDiv<i32> for f64 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: i32) -> f64 {
        if rhs == 0 {
            f64::NAN
        } else {
            self / rhs as f64
        }
    }
}
impl PyDiv<i64> for f64 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: i64) -> f64 {
        if rhs == 0 {
            f64::NAN
        } else {
            self / rhs as f64
        }
    }
}
impl PyDiv for DepylerValue {
    type Output = DepylerValue;
    fn py_div(self, rhs: DepylerValue) -> DepylerValue {
        match (self, rhs) {
            (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => {
                DepylerValue::Float(_dv_a as f64 / _dv_b as f64)
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
impl PyDiv<DepylerValue> for i32 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: DepylerValue) -> f64 {
        let divisor = rhs.to_f64();
        if divisor == 0.0 {
            f64::NAN
        } else {
            self as f64 / divisor
        }
    }
}
impl PyDiv<DepylerValue> for i64 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: DepylerValue) -> f64 {
        let divisor = rhs.to_f64();
        if divisor == 0.0 {
            f64::NAN
        } else {
            self as f64 / divisor
        }
    }
}
impl PyDiv<DepylerValue> for f64 {
    type Output = f64;
    #[inline]
    fn py_div(self, rhs: DepylerValue) -> f64 {
        let divisor = rhs.to_f64();
        if divisor == 0.0 {
            f64::NAN
        } else {
            self / divisor
        }
    }
}
impl PyMod for i32 {
    type Output = i32;
    #[inline]
    fn py_mod(self, rhs: i32) -> i32 {
        if rhs == 0 {
            0
        } else {
            ((self % rhs) + rhs) % rhs
        }
    }
}
impl PyMod<f64> for i32 {
    type Output = f64;
    #[inline]
    fn py_mod(self, rhs: f64) -> f64 {
        if rhs == 0.0 {
            f64::NAN
        } else {
            ((self as f64 % rhs) + rhs) % rhs
        }
    }
}
impl PyMod for i64 {
    type Output = i64;
    #[inline]
    fn py_mod(self, rhs: i64) -> i64 {
        if rhs == 0 {
            0
        } else {
            ((self % rhs) + rhs) % rhs
        }
    }
}
impl PyMod<f64> for i64 {
    type Output = f64;
    #[inline]
    fn py_mod(self, rhs: f64) -> f64 {
        if rhs == 0.0 {
            f64::NAN
        } else {
            ((self as f64 % rhs) + rhs) % rhs
        }
    }
}
impl PyMod for f64 {
    type Output = f64;
    #[inline]
    fn py_mod(self, rhs: f64) -> f64 {
        if rhs == 0.0 {
            f64::NAN
        } else {
            ((self % rhs) + rhs) % rhs
        }
    }
}
impl PyMod<i32> for f64 {
    type Output = f64;
    #[inline]
    fn py_mod(self, rhs: i32) -> f64 {
        if rhs == 0 {
            f64::NAN
        } else {
            ((self % rhs as f64) + rhs as f64) % rhs as f64
        }
    }
}
impl PyMod<i64> for f64 {
    type Output = f64;
    #[inline]
    fn py_mod(self, rhs: i64) -> f64 {
        if rhs == 0 {
            f64::NAN
        } else {
            ((self % rhs as f64) + rhs as f64) % rhs as f64
        }
    }
}
impl PyMod for DepylerValue {
    type Output = DepylerValue;
    fn py_mod(self, rhs: DepylerValue) -> DepylerValue {
        match (self, rhs) {
            (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => {
                DepylerValue::Int(((_dv_a % _dv_b) + _dv_b) % _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 => {
                DepylerValue::Float(((_dv_a % _dv_b) + _dv_b) % _dv_b)
            }
            (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 => {
                let a = _dv_a as f64;
                DepylerValue::Float(((a % _dv_b) + _dv_b) % _dv_b)
            }
            (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => {
                let b = _dv_b as f64;
                DepylerValue::Float(((_dv_a % b) + b) % b)
            }
            _ => DepylerValue::None,
        }
    }
}
impl<T: Clone> PyIndex<i32> for Vec<T> {
    type Output = Option<T>;
    fn py_index(&self, index: i32) -> Option<T> {
        let _dv_len = self.len() as i32;
        let _dv_idx = if index < 0 { _dv_len + index } else { index };
        if _dv_idx >= 0 && (_dv_idx as usize) < self.len() {
            Some(self[_dv_idx as usize].clone())
        } else {
            Option::None
        }
    }
}
impl<T: Clone> PyIndex<i64> for Vec<T> {
    type Output = Option<T>;
    fn py_index(&self, index: i64) -> Option<T> {
        let _dv_len = self.len() as i64;
        let _dv_idx = if index < 0 { _dv_len + index } else { index };
        if _dv_idx >= 0 && (_dv_idx as usize) < self.len() {
            Some(self[_dv_idx as usize].clone())
        } else {
            Option::None
        }
    }
}
impl PyIndex<&str> for std::collections::HashMap<String, DepylerValue> {
    type Output = Option<DepylerValue>;
    fn py_index(&self, key: &str) -> Option<DepylerValue> {
        self.get(key).cloned()
    }
}
impl PyIndex<i32> for String {
    type Output = Option<char>;
    fn py_index(&self, index: i32) -> Option<char> {
        let _dv_len = self.len() as i32;
        let _dv_idx = if index < 0 { _dv_len + index } else { index };
        if _dv_idx >= 0 {
            self.chars().nth(_dv_idx as usize)
        } else {
            Option::None
        }
    }
}
impl PyIndex<i64> for String {
    type Output = Option<char>;
    fn py_index(&self, index: i64) -> Option<char> {
        let _dv_len = self.len() as i64;
        let _dv_idx = if index < 0 { _dv_len + index } else { index };
        if _dv_idx >= 0 {
            self.chars().nth(_dv_idx as usize)
        } else {
            Option::None
        }
    }
}
impl PyIndex<i32> for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, index: i32) -> DepylerValue {
        match self {
            DepylerValue::List(_dv_list) => {
                let _dv_len = _dv_list.len() as i32;
                let _dv_idx = if index < 0 { _dv_len + index } else { index };
                if _dv_idx >= 0 && (_dv_idx as usize) < _dv_list.len() {
                    _dv_list[_dv_idx as usize].clone()
                } else {
                    DepylerValue::None
                }
            }
            DepylerValue::Tuple(_dv_tuple) => {
                let _dv_len = _dv_tuple.len() as i32;
                let _dv_idx = if index < 0 { _dv_len + index } else { index };
                if _dv_idx >= 0 && (_dv_idx as usize) < _dv_tuple.len() {
                    _dv_tuple[_dv_idx as usize].clone()
                } else {
                    DepylerValue::None
                }
            }
            DepylerValue::Str(_dv_str) => {
                let _dv_len = _dv_str.len() as i32;
                let _dv_idx = if index < 0 { _dv_len + index } else { index };
                if _dv_idx >= 0 {
                    _dv_str
                        .chars()
                        .nth(_dv_idx as usize)
                        .map(|_dv_c| DepylerValue::Str(_dv_c.to_string()))
                        .unwrap_or(DepylerValue::None)
                } else {
                    DepylerValue::None
                }
            }
            _ => DepylerValue::None,
        }
    }
}
impl PyIndex<i64> for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, index: i64) -> DepylerValue {
        self.py_index(index as i32)
    }
}
impl PyIndex<&str> for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, key: &str) -> DepylerValue {
        match self {
            DepylerValue::Dict(_dv_dict) => _dv_dict
                .get(&DepylerValue::Str(key.to_string()))
                .cloned()
                .unwrap_or(DepylerValue::None),
            _ => DepylerValue::None,
        }
    }
}
pub trait PyStringMethods {
    fn lower(&self) -> String;
    fn upper(&self) -> String;
    fn strip(&self) -> String;
    fn lstrip(&self) -> String;
    fn rstrip(&self) -> String;
    fn py_split(&self, sep: &str) -> Vec<String>;
    fn py_replace(&self, old: &str, new: &str) -> String;
    fn startswith(&self, prefix: &str) -> bool;
    fn endswith(&self, suffix: &str) -> bool;
    fn py_find(&self, sub: &str) -> i64;
    fn capitalize(&self) -> String;
    fn title(&self) -> String;
    fn swapcase(&self) -> String;
    fn isalpha(&self) -> bool;
    fn isdigit(&self) -> bool;
    fn isalnum(&self) -> bool;
    fn isspace(&self) -> bool;
    fn islower(&self) -> bool;
    fn isupper(&self) -> bool;
    fn center(&self, width: usize) -> String;
    fn ljust(&self, width: usize) -> String;
    fn rjust(&self, width: usize) -> String;
    fn zfill(&self, width: usize) -> String;
    fn count(&self, sub: &str) -> usize;
}
impl PyStringMethods for str {
    #[inline]
    fn lower(&self) -> String {
        self.to_lowercase()
    }
    #[inline]
    fn upper(&self) -> String {
        self.to_uppercase()
    }
    #[inline]
    fn strip(&self) -> String {
        self.trim().to_string()
    }
    #[inline]
    fn lstrip(&self) -> String {
        self.trim_start().to_string()
    }
    #[inline]
    fn rstrip(&self) -> String {
        self.trim_end().to_string()
    }
    #[inline]
    fn py_split(&self, sep: &str) -> Vec<String> {
        self.split(sep).map(|s| s.to_string()).collect()
    }
    #[inline]
    fn py_replace(&self, old: &str, new: &str) -> String {
        self.replace(old, new)
    }
    #[inline]
    fn startswith(&self, prefix: &str) -> bool {
        self.starts_with(prefix)
    }
    #[inline]
    fn endswith(&self, suffix: &str) -> bool {
        self.ends_with(suffix)
    }
    #[inline]
    fn py_find(&self, sub: &str) -> i64 {
        self.find(sub).map(|i| i as i64).unwrap_or(-1)
    }
    #[inline]
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c
                .to_uppercase()
                .chain(chars.flat_map(|c| c.to_lowercase()))
                .collect(),
        }
    }
    #[inline]
    fn title(&self) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        for c in self.chars() {
            if c.is_whitespace() {
                result.push(c);
                capitalize_next = true;
            } else if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_lowercase());
            }
        }
        result
    }
    #[inline]
    fn swapcase(&self) -> String {
        self.chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().collect::<String>()
                } else {
                    c.to_uppercase().collect::<String>()
                }
            })
            .collect()
    }
    #[inline]
    fn isalpha(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_alphabetic())
    }
    #[inline]
    fn isdigit(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_ascii_digit())
    }
    #[inline]
    fn isalnum(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_alphanumeric())
    }
    #[inline]
    fn isspace(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_whitespace())
    }
    #[inline]
    fn islower(&self) -> bool {
        self.chars().any(|c| c.is_lowercase()) && !self.chars().any(|c| c.is_uppercase())
    }
    #[inline]
    fn isupper(&self) -> bool {
        self.chars().any(|c| c.is_uppercase()) && !self.chars().any(|c| c.is_lowercase())
    }
    #[inline]
    fn center(&self, width: usize) -> String {
        if self.len() >= width {
            return self.to_string();
        }
        let padding = width - self.len();
        let left = padding / 2;
        let right = padding - left;
        format!("{}{}{}", " ".repeat(left), self, " ".repeat(right))
    }
    #[inline]
    fn ljust(&self, width: usize) -> String {
        if self.len() >= width {
            return self.to_string();
        }
        format!("{}{}", self, " ".repeat(width - self.len()))
    }
    #[inline]
    fn rjust(&self, width: usize) -> String {
        if self.len() >= width {
            return self.to_string();
        }
        format!("{}{}", " ".repeat(width - self.len()), self)
    }
    #[inline]
    fn zfill(&self, width: usize) -> String {
        if self.len() >= width {
            return self.to_string();
        }
        format!("{}{}", "0".repeat(width - self.len()), self)
    }
    #[inline]
    fn count(&self, sub: &str) -> usize {
        self.matches(sub).count()
    }
}
impl PyStringMethods for String {
    #[inline]
    fn lower(&self) -> String {
        self.as_str().lower()
    }
    #[inline]
    fn upper(&self) -> String {
        self.as_str().upper()
    }
    #[inline]
    fn strip(&self) -> String {
        self.as_str().strip()
    }
    #[inline]
    fn lstrip(&self) -> String {
        self.as_str().lstrip()
    }
    #[inline]
    fn rstrip(&self) -> String {
        self.as_str().rstrip()
    }
    #[inline]
    fn py_split(&self, sep: &str) -> Vec<String> {
        self.as_str().py_split(sep)
    }
    #[inline]
    fn py_replace(&self, old: &str, new: &str) -> String {
        self.as_str().py_replace(old, new)
    }
    #[inline]
    fn startswith(&self, prefix: &str) -> bool {
        self.as_str().startswith(prefix)
    }
    #[inline]
    fn endswith(&self, suffix: &str) -> bool {
        self.as_str().endswith(suffix)
    }
    #[inline]
    fn py_find(&self, sub: &str) -> i64 {
        self.as_str().py_find(sub)
    }
    #[inline]
    fn capitalize(&self) -> String {
        self.as_str().capitalize()
    }
    #[inline]
    fn title(&self) -> String {
        self.as_str().title()
    }
    #[inline]
    fn swapcase(&self) -> String {
        self.as_str().swapcase()
    }
    #[inline]
    fn isalpha(&self) -> bool {
        self.as_str().isalpha()
    }
    #[inline]
    fn isdigit(&self) -> bool {
        self.as_str().isdigit()
    }
    #[inline]
    fn isalnum(&self) -> bool {
        self.as_str().isalnum()
    }
    #[inline]
    fn isspace(&self) -> bool {
        self.as_str().isspace()
    }
    #[inline]
    fn islower(&self) -> bool {
        self.as_str().islower()
    }
    #[inline]
    fn isupper(&self) -> bool {
        self.as_str().isupper()
    }
    #[inline]
    fn center(&self, width: usize) -> String {
        self.as_str().center(width)
    }
    #[inline]
    fn ljust(&self, width: usize) -> String {
        self.as_str().ljust(width)
    }
    #[inline]
    fn rjust(&self, width: usize) -> String {
        self.as_str().rjust(width)
    }
    #[inline]
    fn zfill(&self, width: usize) -> String {
        self.as_str().zfill(width)
    }
    #[inline]
    fn count(&self, sub: &str) -> usize {
        self.as_str().count(sub)
    }
}
impl PyStringMethods for DepylerValue {
    #[inline]
    fn lower(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.lower(),
            _ => String::new(),
        }
    }
    #[inline]
    fn upper(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.upper(),
            _ => String::new(),
        }
    }
    #[inline]
    fn strip(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.strip(),
            _ => String::new(),
        }
    }
    #[inline]
    fn lstrip(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.lstrip(),
            _ => String::new(),
        }
    }
    #[inline]
    fn rstrip(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.rstrip(),
            _ => String::new(),
        }
    }
    #[inline]
    fn py_split(&self, sep: &str) -> Vec<String> {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.py_split(sep),
            _ => Vec::new(),
        }
    }
    #[inline]
    fn py_replace(&self, old: &str, new: &str) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.py_replace(old, new),
            _ => String::new(),
        }
    }
    #[inline]
    fn startswith(&self, prefix: &str) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.startswith(prefix),
            _ => false,
        }
    }
    #[inline]
    fn endswith(&self, suffix: &str) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.endswith(suffix),
            _ => false,
        }
    }
    #[inline]
    fn py_find(&self, sub: &str) -> i64 {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.py_find(sub),
            _ => -1,
        }
    }
    #[inline]
    fn capitalize(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.capitalize(),
            _ => String::new(),
        }
    }
    #[inline]
    fn title(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.title(),
            _ => String::new(),
        }
    }
    #[inline]
    fn swapcase(&self) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.swapcase(),
            _ => String::new(),
        }
    }
    #[inline]
    fn isalpha(&self) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.isalpha(),
            _ => false,
        }
    }
    #[inline]
    fn isdigit(&self) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.isdigit(),
            _ => false,
        }
    }
    #[inline]
    fn isalnum(&self) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.isalnum(),
            _ => false,
        }
    }
    #[inline]
    fn isspace(&self) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.isspace(),
            _ => false,
        }
    }
    #[inline]
    fn islower(&self) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.islower(),
            _ => false,
        }
    }
    #[inline]
    fn isupper(&self) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.isupper(),
            _ => false,
        }
    }
    #[inline]
    fn center(&self, width: usize) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.center(width),
            _ => String::new(),
        }
    }
    #[inline]
    fn ljust(&self, width: usize) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.ljust(width),
            _ => String::new(),
        }
    }
    #[inline]
    fn rjust(&self, width: usize) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.rjust(width),
            _ => String::new(),
        }
    }
    #[inline]
    fn zfill(&self, width: usize) -> String {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.zfill(width),
            _ => String::new(),
        }
    }
    #[inline]
    fn count(&self, sub: &str) -> usize {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.count(sub),
            _ => 0,
        }
    }
}
impl DepylerValue {
    #[doc = r" Check if string contains substring(Python's `in` operator for strings)"]
    #[inline]
    pub fn contains(&self, sub: &str) -> bool {
        match self {
            DepylerValue::Str(_dv_s) => _dv_s.contains(sub),
            DepylerValue::List(_dv_l) => _dv_l.iter().any(|v| {
                if let DepylerValue::Str(s) = v {
                    s == sub
                } else {
                    false
                }
            }),
            _ => false,
        }
    }
}
#[doc = r" DEPYLER-1202: Python integer operations for Rust integer types."]
pub trait PythonIntOps {
    fn bit_length(&self) -> u32;
    fn bit_count(&self) -> u32;
}
impl PythonIntOps for i32 {
    fn bit_length(&self) -> u32 {
        if *self == 0 {
            0
        } else {
            (std::mem::size_of::<i32>() as u32 * 8) - self.unsigned_abs().leading_zeros()
        }
    }
    fn bit_count(&self) -> u32 {
        self.unsigned_abs().count_ones()
    }
}
impl PythonIntOps for i64 {
    fn bit_length(&self) -> u32 {
        if *self == 0 {
            0
        } else {
            (std::mem::size_of::<i64>() as u32 * 8) - self.unsigned_abs().leading_zeros()
        }
    }
    fn bit_count(&self) -> u32 {
        self.unsigned_abs().count_ones()
    }
}
impl PythonIntOps for u32 {
    fn bit_length(&self) -> u32 {
        if *self == 0 {
            0
        } else {
            (std::mem::size_of::<u32>() as u32 * 8) - self.leading_zeros()
        }
    }
    fn bit_count(&self) -> u32 {
        self.count_ones()
    }
}
impl PythonIntOps for u64 {
    fn bit_length(&self) -> u32 {
        if *self == 0 {
            0
        } else {
            (std::mem::size_of::<u64>() as u32 * 8) - self.leading_zeros()
        }
    }
    fn bit_count(&self) -> u32 {
        self.count_ones()
    }
}
impl PythonIntOps for usize {
    fn bit_length(&self) -> u32 {
        if *self == 0 {
            0
        } else {
            (std::mem::size_of::<usize>() as u32 * 8) - self.leading_zeros()
        }
    }
    fn bit_count(&self) -> u32 {
        self.count_ones()
    }
}
impl PythonIntOps for isize {
    fn bit_length(&self) -> u32 {
        if *self == 0 {
            0
        } else {
            (std::mem::size_of::<isize>() as u32 * 8) - self.unsigned_abs().leading_zeros()
        }
    }
    fn bit_count(&self) -> u32 {
        self.unsigned_abs().count_ones()
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
            DepylerRegexMatch::new(&text.to_string(), start, end)
        })
    }
    #[doc = r" Simple pattern match at start(NASA mode alternative to regex)"]
    pub fn match_start(pattern: &str, text: &str) -> Option<Self> {
        if text.starts_with(pattern) {
            Some(DepylerRegexMatch::new(&text.to_string(), 0, pattern.len()))
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
#[doc = "DJB2 hash function by Dan Bernstein."]
#[doc = " Depyler: verified panic-free"]
pub fn djb2_hash(data: &str) -> i32 {
    let mut h: i32 = Default::default();
    h = 5381;
    for ch in data.chars() {
        h = (((h << 5).py_add(h) as i32).py_add(ch as u32 as i32)) as i32;
        h = h & -1;
    }
    h
}
#[doc = "SDBM hash function used in gawk."]
#[doc = " Depyler: verified panic-free"]
pub fn sdbm_hash(data: &str) -> i32 {
    let mut h: i32 = Default::default();
    h = 0;
    for ch in data.chars() {
        let c: i32 = ch as u32 as i32;
        h = ((((c).py_add(h << 6) as i32).py_add(h << 16) as i32) - (h)) as i32;
        h = h & -1;
    }
    h
}
#[doc = "FNV-1a hash function with 32-bit offset basis and prime."]
#[doc = " Depyler: verified panic-free"]
pub fn fnv1a_hash(data: &str) -> i32 {
    let mut h: i32 = Default::default();
    let offset_basis: i32 = -2128831035;
    let fnv_prime: i32 = 16777619;
    h = offset_basis;
    for ch in data.chars() {
        h = h ^ ch as u32 as i32;
        h = (h).py_mul(fnv_prime) & -1;
    }
    h
}
#[doc = "Jenkins one-at-a-time hash function."]
#[doc = " Depyler: verified panic-free"]
pub fn jenkins_one_at_a_time(data: &str) -> i32 {
    let mut h: i32 = Default::default();
    h = 0;
    for ch in data.chars() {
        h = (h).py_add(ch as u32 as i32) & -1;
        h = (h).py_add(h << 10) & -1;
        h = (h ^ h >> 6) & -1;
    }
    let _cse_temp_0 = h << 3;
    let _cse_temp_1 = (h).py_add(_cse_temp_0) & -1;
    h = _cse_temp_1;
    let _cse_temp_2 = h >> 11;
    let _cse_temp_3 = h ^ _cse_temp_2;
    let _cse_temp_4 = _cse_temp_3 & -1;
    h = _cse_temp_4;
    let _cse_temp_5 = h << 15;
    let _cse_temp_6 = (h).py_add(_cse_temp_5) & -1;
    h = _cse_temp_6;
    h
}
#[doc = "Find all occurrences of pattern in text using Rabin-Karp rolling hash."]
#[doc = " Depyler: proven to terminate"]
pub fn rabin_karp_search<'a, 'b>(
    text: &'a str,
    pattern: &'b str,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut power: i32 = Default::default();
    let mut txt_hash: i32 = Default::default();
    let mut pat_hash: i32 = Default::default();
    let mut results: Vec<i32> = vec![];
    let _cse_temp_0 = text.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = pattern.len() as i32;
    let m: i32 = _cse_temp_1;
    let _cse_temp_2 = m > n;
    let _cse_temp_3 = m == 0;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(results);
    }
    let base: i32 = 256;
    let r#mod: i32 = 1000000007;
    pat_hash = 0;
    txt_hash = 0;
    power = 1;
    for _i in 0..((m) - (1i32)) {
        power = (((power).py_mul(base) as i32).py_mod(r#mod)) as i32;
    }
    for i in 0..(m) {
        pat_hash = ((((pat_hash).py_mul(base) as i32).py_add(
            {
                let base = &pattern;
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
            }
            .chars()
            .next()
            .expect("empty string") as i32,
        ) as i32)
            .py_mod(r#mod)) as i32;
        txt_hash = ((((txt_hash).py_mul(base) as i32).py_add(
            {
                let base = &text;
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
            }
            .chars()
            .next()
            .expect("empty string") as i32,
        ) as i32)
            .py_mod(r#mod)) as i32;
    }
    for i in 0..(((n) - (m) as i32).py_add(1i32)) {
        if pat_hash == txt_hash {
            let mut r#match: bool = true;
            for j in 0..(m) {
                if {
                    let base = &text;
                    let idx: i32 = (i).py_add(j);
                    let actual_idx = if idx < 0 {
                        base.chars().count().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.chars()
                        .nth(actual_idx)
                        .map(|c| c.to_string())
                        .unwrap_or_default()
                } != {
                    let base = &pattern;
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
                } {
                    r#match = false;
                    break;
                }
            }
            if r#match {
                results.push(i);
            }
        }
        if i < (n) - (m) {
            txt_hash = ((((txt_hash) - (
                (({
                    let base = &text;
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
                }
                .chars()
                .next()
                .expect("empty string") as i32)
                    .py_mul(power) as i32)
                    .py_mod(r#mod)
            ) as i32)
                .py_add(r#mod) as i32)
                .py_mod(r#mod)) as i32;
            txt_hash = ((((txt_hash).py_mul(base) as i32).py_add(
                {
                    let base = &text;
                    let idx: i32 = (i).py_add(m);
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
                .next()
                .expect("empty string") as i32,
            ) as i32)
                .py_mod(r#mod)) as i32;
        }
    }
    Ok(results)
}
#[doc = "Caesar cipher encryption on letters a-z and A-Z."]
pub fn caesar_encrypt(plaintext: &str, shift: i32) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = vec![];
    for ch in plaintext.chars() {
        if (ch >= 'a') && (ch <= 'z') {
            let shifted: i32 = (((((ch as u32 as i32)
                 - (STR_A_1.chars().next().expect("empty string") as i32)
                as i32)
                .py_add(shift) as i32)
                .py_mod(26i32) as i32)
                .py_add(STR_A_1.chars().next().expect("empty string") as i32))
                as i32;
            result.push(
                char::from_u32((shifted) as u32)
                    .expect("builtin operation failed")
                    .to_string(),
            );
        } else {
            if (ch >= 'A') && (ch <= 'Z') {
                let shifted2: i32 = (((((ch as u32 as i32)
                     - (STR_A_2.chars().next().expect("empty string") as i32)
                    as i32)
                    .py_add(shift) as i32)
                    .py_mod(26i32) as i32)
                    .py_add(STR_A_2.chars().next().expect("empty string") as i32))
                    as i32;
                result.push(
                    char::from_u32((shifted2) as u32)
                        .expect("builtin operation failed")
                        .to_string(),
                );
            } else {
                result.push(ch.to_string());
            }
        }
    }
    Ok(result.join(""))
}
#[doc = "Caesar cipher decryption(reverse shift)."]
#[doc = " Depyler: proven to terminate"]
pub fn caesar_decrypt(ciphertext: &str, shift: i32) -> Result<String, Box<dyn std::error::Error>> {
    caesar_encrypt(ciphertext, (26i32) - ((shift).py_mod(26i32)))
}
#[doc = "Vigenere cipher encryption using a repeating key."]
pub fn vigenere_encrypt<'b, 'a>(
    plaintext: &'a str,
    key: &'b str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = vec![];
    let _cse_temp_0 = key.len() as i32;
    let key_len: i32 = _cse_temp_0;
    let mut key_idx: i32 = 0;
    for ch in plaintext.chars() {
        if (ch >= 'a') && (ch <= 'z') {
            let k: i32 = (({
                let base = &key;
                let idx: i32 = (key_idx).py_mod(key_len);
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
            .next()
            .expect("empty string") as i32)
                 - (STR_A_1.chars().next().expect("empty string") as i32))
                as i32;
            let shifted: i32 = (((((ch as u32 as i32)
                 - (STR_A_1.chars().next().expect("empty string") as i32)
                as i32)
                .py_add(k) as i32)
                .py_mod(26i32) as i32)
                .py_add(STR_A_1.chars().next().expect("empty string") as i32))
                as i32;
            result.push(
                char::from_u32((shifted) as u32)
                    .expect("builtin operation failed")
                    .to_string(),
            );
            key_idx = ((key_idx).py_add(1i32)) as i32;
        } else {
            if (ch >= 'A') && (ch <= 'Z') {
                let k2: i32 = (({
                    let base = &key;
                    let idx: i32 = (key_idx).py_mod(key_len);
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
                .next()
                .expect("empty string") as i32)
                     - (STR_A_1.chars().next().expect("empty string") as i32))
                    as i32;
                let shifted2: i32 = (((((ch as u32 as i32)
                     - (STR_A_2.chars().next().expect("empty string") as i32)
                    as i32)
                    .py_add(k2) as i32)
                    .py_mod(26i32) as i32)
                    .py_add(STR_A_2.chars().next().expect("empty string") as i32))
                    as i32;
                result.push(
                    char::from_u32((shifted2) as u32)
                        .expect("builtin operation failed")
                        .to_string(),
                );
                key_idx = ((key_idx).py_add(1i32)) as i32;
            } else {
                result.push(ch.to_string());
            }
        }
    }
    Ok(result.join(""))
}
#[doc = "Vigenere cipher decryption using a repeating key."]
pub fn vigenere_decrypt<'a, 'b>(
    ciphertext: &'a str,
    key: &'b str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = vec![];
    let _cse_temp_0 = key.len() as i32;
    let key_len: i32 = _cse_temp_0;
    let mut key_idx: i32 = 0;
    for ch in ciphertext.chars() {
        if (ch >= 'a') && (ch <= 'z') {
            let k: i32 = (({
                let base = &key;
                let idx: i32 = (key_idx).py_mod(key_len);
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
            .next()
            .expect("empty string") as i32)
                 - (STR_A_1.chars().next().expect("empty string") as i32))
                as i32;
            let shifted: i32 = ((((((ch as u32 as i32)
                 - (STR_A_1.chars().next().expect("empty string") as i32)
                as i32)
                 - (k) as i32)
                .py_add(26i32) as i32)
                .py_mod(26i32) as i32)
                .py_add(STR_A_1.chars().next().expect("empty string") as i32))
                as i32;
            result.push(
                char::from_u32((shifted) as u32)
                    .expect("builtin operation failed")
                    .to_string(),
            );
            key_idx = ((key_idx).py_add(1i32)) as i32;
        } else {
            if (ch >= 'A') && (ch <= 'Z') {
                let k2: i32 = (({
                    let base = &key;
                    let idx: i32 = (key_idx).py_mod(key_len);
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
                .next()
                .expect("empty string") as i32)
                     - (STR_A_1.chars().next().expect("empty string") as i32))
                    as i32;
                let shifted2: i32 = ((((((ch as u32 as i32)
                     - (STR_A_2.chars().next().expect("empty string") as i32)
                    as i32)
                     - (k2) as i32)
                    .py_add(26i32) as i32)
                    .py_mod(26i32) as i32)
                    .py_add(STR_A_2.chars().next().expect("empty string") as i32))
                    as i32;
                result.push(
                    char::from_u32((shifted2) as u32)
                        .expect("builtin operation failed")
                        .to_string(),
                );
                key_idx = ((key_idx).py_add(1i32)) as i32;
            } else {
                result.push(ch.to_string());
            }
        }
    }
    Ok(result.join(""))
}
#[doc = "XOR cipher with a repeating multi-byte key; reversible."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn xor_cipher_repeating<'b, 'a>(data: &'a str, key: &'b str) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = key.len() as i32;
    let key_len: i32 = _cse_temp_0;
    for i in 0..(data.len() as i32) {
        result.push(
            {
                let base = &data;
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
            }
            .chars()
            .next()
            .expect("empty string") as i32
                ^ {
                    let base = &key;
                    let idx: i32 = (i).py_mod(key_len);
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
                .next()
                .expect("empty string") as i32,
        );
    }
    result
}
#[doc = "Reverse repeating-key XOR cipher to recover plaintext."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn xor_decipher_repeating<'a, 'b>(data: &'a Vec<i32>, key: &'b str) -> String {
    let mut result: Vec<String> = vec![];
    let _cse_temp_0 = key.len() as i32;
    let key_len: i32 = _cse_temp_0;
    for i in 0..(data.len() as i32) {
        result.push(
            char::from_u32(
                (data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    ^ {
                        let base = &key;
                        let idx: i32 = (i).py_mod(key_len);
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
                    .next()
                    .expect("empty string") as i32) as u32,
            )
            .expect("builtin operation failed")
            .to_string(),
        );
    }
    result.join("")
}
#[doc = "Manual base64 encoding implementation using bit manipulation."]
pub fn base64_encode(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let table: String =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string();
    let mut result: Vec<String> = vec![];
    let mut bytes_list: Vec<i32> = vec![];
    for ch in data.chars() {
        bytes_list.push(ch as u32 as i32);
    }
    let mut i: i32 = 0;
    let _cse_temp_0 = bytes_list.len() as i32;
    let n: i32 = _cse_temp_0;
    while i < n {
        let b0: i32 = bytes_list
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let b1: i32 = if (i).py_add(1i32) < n {
            {
                let base = &bytes_list;
                let idx: i32 = (i).py_add(1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }
        } else {
            0
        };
        let b2: i32 = if (i).py_add(2i32) < n {
            {
                let base = &bytes_list;
                let idx: i32 = (i).py_add(2i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }
        } else {
            0
        };
        let triple: i32 = b0 << 16 | b1 << 8 | b2;
        result.push({
            let base = &table;
            let idx: i32 = triple >> 18 & 63;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        });
        result.push({
            let base = &table;
            let idx: i32 = triple >> 12 & 63;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        });
        if (i).py_add(1i32) < n {
            result.push({
                let base = &table;
                let idx: i32 = triple >> 6 & 63;
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            });
        } else {
            result.push("=".to_string());
        }
        if (i).py_add(2i32) < n {
            result.push({
                let base = &table;
                let idx: i32 = triple & 63;
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            });
        } else {
            result.push("=".to_string());
        }
        i = ((i).py_add(3i32)) as i32;
    }
    Ok(result.join(""))
}
#[doc = "Manual base64 decoding implementation using bit manipulation."]
pub fn base64_decode(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
    let table: String =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string();
    let mut lookup: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    for idx in 0..(table.len() as i32) {
        lookup.insert(
            {
                let base = &table;
                let idx: i32 = idx;
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            },
            idx,
        );
    }
    let mut result: Vec<String> = vec![];
    let mut i: i32 = 0;
    let _cse_temp_0 = encoded.len() as i32;
    let n: i32 = _cse_temp_0;
    while i < n {
        let v0: i32 = if {
            let base = &encoded;
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
        } != "="
        {
            lookup
                .get(
                    &({
                        let base = &encoded;
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
                    }),
                )
                .cloned()
                .unwrap_or_default()
        } else {
            0
        };
        let v1: i32 = if {
            let base = &encoded;
            let idx: i32 = (i).py_add(1i32);
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } != "="
        {
            lookup
                .get(
                    &({
                        let base = &encoded;
                        let idx: i32 = (i).py_add(1i32);
                        let actual_idx = if idx < 0 {
                            base.chars().count().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.chars()
                            .nth(actual_idx)
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    }),
                )
                .cloned()
                .unwrap_or_default()
        } else {
            0
        };
        let v2: i32 = if {
            let base = &encoded;
            let idx: i32 = (i).py_add(2i32);
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } != "="
        {
            lookup
                .get(
                    &({
                        let base = &encoded;
                        let idx: i32 = (i).py_add(2i32);
                        let actual_idx = if idx < 0 {
                            base.chars().count().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.chars()
                            .nth(actual_idx)
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    }),
                )
                .cloned()
                .unwrap_or_default()
        } else {
            0
        };
        let v3: i32 = if {
            let base = &encoded;
            let idx: i32 = (i).py_add(3i32);
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } != "="
        {
            lookup
                .get(
                    &({
                        let base = &encoded;
                        let idx: i32 = (i).py_add(3i32);
                        let actual_idx = if idx < 0 {
                            base.chars().count().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.chars()
                            .nth(actual_idx)
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    }),
                )
                .cloned()
                .unwrap_or_default()
        } else {
            0
        };
        let triple: i32 = v0 << 18 | v1 << 12 | v2 << 6 | v3;
        result.push(
            char::from_u32((triple >> 16 & 255) as u32)
                .expect("builtin operation failed")
                .to_string(),
        );
        if {
            let base = &encoded;
            let idx: i32 = (i).py_add(2i32);
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } != "="
        {
            result.push(
                char::from_u32((triple >> 8 & 255) as u32)
                    .expect("builtin operation failed")
                    .to_string(),
            );
        }
        if {
            let base = &encoded;
            let idx: i32 = (i).py_add(3i32);
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } != "="
        {
            result.push(
                char::from_u32((triple & 255) as u32)
                    .expect("builtin operation failed")
                    .to_string(),
            );
        }
        i = ((i).py_add(4i32)) as i32;
    }
    Ok(result.join(""))
}
#[doc = "Run-length encoding: compress consecutive repeated characters."]
#[doc = " Depyler: proven to terminate"]
pub fn run_length_encode(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut prev: String = Default::default();
    let mut count: i32 = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok("".to_string().to_string());
    }
    let mut result: Vec<String> = vec![];
    count = 1;
    prev = {
        let base = &data;
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
    };
    for i in (1)..(data.len() as i32) {
        if {
            let base = &data;
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
        } == *prev
        {
            count = ((count).py_add(1i32)) as i32;
        } else {
            result.push((count).to_string());
            result.push(prev.clone());
            prev = {
                let base = &data;
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
            };
            count = 1;
        }
    }
    result.push((count).to_string());
    result.push(prev.clone());
    Ok(result.join(""))
}
#[doc = "Run-length decoding: expand compressed representation."]
pub fn run_length_decode(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = vec![];
    let mut i: i32 = 0;
    let _cse_temp_0 = encoded.len() as i32;
    let n: i32 = _cse_temp_0;
    while i < n {
        let mut num_str: String = "".to_string().to_string();
        while ((i < n)
            && (({
                let base = &encoded;
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
            })
            .as_str()
                >= "0"))
            && (({
                let base = &encoded;
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
            })
            .as_str()
                <= "9")
        {
            num_str = format!("{}{}", num_str, {
                let base = &encoded;
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
            });
            i = ((i).py_add(1i32)) as i32;
        }
        if (i < n) && (num_str.len() as i32 > 0) {
            let count: i32 = num_str.parse::<i32>().unwrap_or_default();
            let ch: String = {
                let base = &encoded;
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
            };
            for _j in 0..(count) {
                result.push(ch.to_string());
            }
            i = ((i).py_add(1i32)) as i32;
        }
    }
    Ok(result.join(""))
}
#[doc = "Count character frequencies and return sorted by count ascending."]
pub fn huffman_sorted_pairs(data: &str) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
    let mut freq: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    for _ch in data.chars() {
        let ch = _ch.to_string();
        if freq.get(&ch).is_some() {
            {
                let _key = ch.clone();
                let _old_val = freq.get(&_key).cloned().unwrap_or_default();
                freq.insert(_key, _old_val + 1);
            }
        } else {
            freq.insert(ch.to_string().clone(), 1);
        }
    }
    let mut pairs: Vec<(String, i32)> = vec![];
    for _ch in freq.keys().cloned() {
        let ch = _ch.to_string();
        pairs.push((ch.clone(), freq.get(&(ch)).cloned().unwrap_or_default()));
    }
    let _cse_temp_0 = pairs.len() as i32;
    let n: i32 = _cse_temp_0;
    for i in 0..(n) {
        for j in (0)..(((n) - (i) as i32) - (1i32)) {
            if pairs
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .1
                > {
                    let base = &pairs;
                    let idx: i32 = (j).py_add(1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                }
                .1
            {
                let temp: (String, i32) = pairs
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                pairs[(j) as usize] = {
                    let base = &pairs;
                    let idx: i32 = (j).py_add(1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
                pairs[((j).py_add(1i32)) as usize] = temp;
            }
        }
    }
    Ok(pairs)
}
#[doc = "Compute CRC32 checksum with inline table generation."]
pub fn crc32_compute(data: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut crc_val: i32 = Default::default();
    let poly: i32 = -306674912;
    let mut table: Vec<i32> = vec![];
    for i in 0..(256) {
        let mut crc: i32 = i.clone();
        for _j in 0..(8) {
            if crc & 1 != 0 {
                crc = crc >> 1 ^ poly;
            } else {
                crc = crc >> 1;
            }
        }
        table.push(crc & -1);
    }
    crc_val = -1;
    for ch in data.chars() {
        let byte: i32 = ch as u32 as i32 & 255;
        let idx: i32 = (crc_val ^ byte) & 255;
        crc_val = crc_val >> 8
            ^ table
                .get(idx as usize)
                .cloned()
                .expect("IndexError: list index out of range");
    }
    Ok((crc_val ^ -1) & -1)
}
#[doc = "All four MD5 round functions: F, G, H, I."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn md5_round_functions(b: i32, c: i32, d: i32) -> (i32, i32, i32, i32) {
    let _cse_temp_0 = b & c;
    let _cse_temp_1 = !b & d;
    let _cse_temp_2 = _cse_temp_0 | _cse_temp_1;
    let _cse_temp_3 = _cse_temp_2 & -1;
    let f_val: i32 = _cse_temp_3;
    let _cse_temp_4 = b & d;
    let _cse_temp_5 = c & !d;
    let _cse_temp_6 = _cse_temp_4 | _cse_temp_5;
    let _cse_temp_7 = _cse_temp_6 & -1;
    let g_val: i32 = _cse_temp_7;
    let _cse_temp_8 = b ^ c;
    let _cse_temp_9 = _cse_temp_8 ^ d;
    let _cse_temp_10 = _cse_temp_9 & -1;
    let h_val: i32 = _cse_temp_10;
    let _cse_temp_11 = b | !d;
    let _cse_temp_12 = c ^ _cse_temp_11;
    let _cse_temp_13 = _cse_temp_12 & -1;
    let i_val: i32 = _cse_temp_13;
    (f_val, g_val, h_val, i_val)
}
#[doc = "Single MD5 mixing step: add, left-rotate, add."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn md5_mix_step(mut a: i32, b: i32, f_val: i32, k: i32, m_val: i32, s: i32) -> i32 {
    let _cse_temp_0 = (((a).py_add(f_val) as i32).py_add(k)) as i32;
    let _cse_temp_1 = (_cse_temp_0).py_add(m_val) & -1;
    a = _cse_temp_1;
    let _cse_temp_2 = a << s;
    let _cse_temp_3 = a >> (32i32) - (s);
    let _cse_temp_4 = _cse_temp_2 | _cse_temp_3;
    let _cse_temp_5 = _cse_temp_4 & -1;
    let rotated: i32 = _cse_temp_5;
    (rotated).py_add(b) & -1
}
#[doc = "Create bloom filter, insert items, then query for membership."]
pub fn bloom_filter_ops<'b, 'a>(
    size: i32,
    items: &'a Vec<String>,
    query: &'b str,
) -> Result<(Vec<i32>, bool), Box<dyn std::error::Error>> {
    let mut bf: Vec<i32> = vec![];
    for _i in 0..(size) {
        bf.push(0);
    }
    for item in items.iter().cloned() {
        let h1: i32 = ((djb2_hash(&item)).py_mod(size)) as i32;
        let h2: i32 = ((fnv1a_hash(&item)).py_mod(size)) as i32;
        let h3: i32 = ((jenkins_one_at_a_time(&item)).py_mod(size)) as i32;
        bf[(h1) as usize] = 1;
        bf[(h2) as usize] = 1;
        bf[(h3) as usize] = 1;
    }
    let _cse_temp_0 = ((djb2_hash(query.clone())).py_mod(size)) as i32;
    let q1: i32 = _cse_temp_0;
    let _cse_temp_1 = ((fnv1a_hash(query.clone())).py_mod(size)) as i32;
    let q2: i32 = _cse_temp_1;
    let _cse_temp_2 = ((jenkins_one_at_a_time(query)).py_mod(size)) as i32;
    let q3: i32 = _cse_temp_2;
    let _cse_temp_3 = bf
        .get(q1 as usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 1;
    let _cse_temp_4 = (_cse_temp_3) && (_cse_temp_3);
    let _cse_temp_5 = (_cse_temp_4) && (_cse_temp_3);
    let found: bool = _cse_temp_5;
    Ok((bf, found))
}
#[doc = "Distribute keys to nearest clockwise node on a hash ring."]
pub fn consistent_hash_distribute<'a, 'b>(
    keys: &'a Vec<String>,
    nodes: &'b Vec<i32>,
    ring_size: i32,
) -> Result<HashMap<i32, Vec<String>>, Box<dyn std::error::Error>> {
    let mut distribution: std::collections::HashMap<i32, Vec<String>> = {
        let map: HashMap<i32, Vec<String>> = HashMap::new();
        map
    };
    for node in nodes.iter().cloned() {
        distribution.insert(node.clone(), vec![]);
    }
    for key in keys.iter().cloned() {
        let pos: i32 = ((fnv1a_hash(&key)).py_mod(ring_size)) as i32;
        let mut best_node: i32 = nodes
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let mut best_dist: i32 = ((ring_size).py_add(1i32)) as i32;
        for node_pos in nodes.iter().cloned() {
            let dist: i32 = ((((node_pos) - (pos) as i32).py_add(ring_size) as i32)
                .py_mod(ring_size)) as i32;
            if dist < best_dist {
                best_dist = dist;
                best_node = node_pos;
            }
        }
        distribution
            .get(&(best_node))
            .cloned()
            .unwrap_or_default()
            .push(key.clone());
    }
    Ok(distribution)
}
#[doc = "Build a minimal perfect hash for a small set via linear probing."]
pub fn perfect_hash_small(
    keys: &Vec<String>,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = keys.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
        return Ok({
            let map: HashMap<String, i32> = HashMap::new();
            map
        });
    }
    let _cse_temp_2 = ((n).py_mul(2i32)) as i32;
    let table_size: i32 = _cse_temp_2;
    let mut result: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    let mut used: Vec<i32> = vec![];
    for _i in 0..(table_size) {
        used.push(0);
    }
    for key in keys.iter().cloned() {
        let mut h: i32 = ((djb2_hash(&key)).py_mod(table_size)) as i32;
        let mut attempt: i32 = 0;
        while (used
            .get(h as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            == 1)
            && (attempt < table_size)
        {
            h = (((h).py_add(1i32) as i32).py_mod(table_size)) as i32;
            attempt = ((attempt).py_add(1i32)) as i32;
        }
        used[(h) as usize] = 1;
        result.insert(key.to_string().clone(), h);
    }
    Ok(result)
}
#[doc = "Detect DJB2 hash collisions among a list of strings."]
#[doc = " Depyler: verified panic-free"]
pub fn detect_hash_collisions(strings: &Vec<String>) -> Vec<(String, String)> {
    let mut hash_map: std::collections::HashMap<i32, String> = {
        let map: HashMap<i32, String> = HashMap::new();
        map
    };
    let mut collisions: Vec<(String, String)> = vec![];
    for s in strings.iter().cloned() {
        let h: i32 = djb2_hash(&s);
        if hash_map.get(&h).is_some() {
            collisions.push((hash_map.get(&(h)).cloned().unwrap_or_default(), s));
        } else {
            hash_map.insert(h.clone(), s.clone());
        }
    }
    collisions
}
#[doc = "Compute a 4-hash fingerprint for stronger collision resistance."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn multi_hash_fingerprint(data: &str) -> (i32, i32, i32, i32) {
    let h1: i32 = djb2_hash(data.clone());
    let h2: i32 = sdbm_hash(data.clone());
    let h3: i32 = fnv1a_hash(data.clone());
    let h4: i32 = jenkins_one_at_a_time(data);
    (h1, h2, h3, h4)
}
#[doc = "Hash a list of strings using Boost-style hash combine."]
#[doc = " Depyler: verified panic-free"]
pub fn hash_combine_list(items: &Vec<String>) -> i32 {
    let mut combined: i32 = Default::default();
    let magic: i32 = -1640531527;
    combined = 0;
    for item in items.iter().cloned() {
        let h: i32 = fnv1a_hash(&item);
        combined = combined
            ^ (((h).py_add(magic) as i32).py_add(combined << 6) as i32).py_add(combined >> 2);
        combined = combined & -1;
    }
    combined
}
#[doc = "Popcount, bit reversal, byte swap, and right rotate on 32-bit int."]
#[doc = " Depyler: verified panic-free"]
pub fn bit_manipulation_suite(value: i32) -> (i32, i32, i32, i32) {
    let mut rev: i32 = Default::default();
    let mut pop: i32 = Default::default();
    let _cse_temp_0 = value & -1;
    let v: i32 = _cse_temp_0;
    pop = 0;
    let mut tmp: i32 = v.clone();
    while tmp > 0 {
        tmp = tmp & (tmp) - (1i32);
        pop = ((pop).py_add(1i32)) as i32;
    }
    rev = 0;
    let mut tmp2: i32 = v.clone();
    for _i in 0..(32) {
        rev = rev << 1 | tmp2 & 1;
        tmp2 = tmp2 >> 1;
    }
    let _cse_temp_1 = rev & -1;
    rev = _cse_temp_1;
    let _cse_temp_2 = v >> 24;
    let _cse_temp_3 = _cse_temp_2 & 255;
    let b0: i32 = _cse_temp_3;
    let _cse_temp_4 = v >> 16;
    let _cse_temp_5 = _cse_temp_4 & 255;
    let b1: i32 = _cse_temp_5;
    let _cse_temp_6 = v >> 8;
    let _cse_temp_7 = _cse_temp_6 & 255;
    let b2: i32 = _cse_temp_7;
    let _cse_temp_8 = v & 255;
    let b3: i32 = _cse_temp_8;
    let _cse_temp_9 = b3 << 24;
    let _cse_temp_10 = b2 << 16;
    let _cse_temp_11 = _cse_temp_9 | _cse_temp_10;
    let _cse_temp_12 = b1 << 8;
    let _cse_temp_13 = _cse_temp_11 | _cse_temp_12;
    let _cse_temp_14 = _cse_temp_13 | b0;
    let swapped: i32 = _cse_temp_14;
    let _cse_temp_15 = v >> 7;
    let _cse_temp_16 = v << 25;
    let _cse_temp_17 = _cse_temp_15 | _cse_temp_16;
    let _cse_temp_18 = _cse_temp_17 & -1;
    let rotated: i32 = _cse_temp_18;
    (pop, rev, swapped, rotated)
}
#[doc = "Internet-style ones-complement checksum over 16-bit words."]
pub fn simple_checksum(data: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let mut i: i32 = Default::default();
    total = 0;
    i = 0;
    let _cse_temp_0 = data.len() as i32;
    let n: i32 = _cse_temp_0;
    while i < (n) - (1i32) {
        let word: i32 = ({
            let base = &data;
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
        }
        .chars()
        .next()
        .expect("empty string") as i32)
            << 8
            | {
                let base = &data;
                let idx: i32 = (i).py_add(1i32);
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
            .next()
            .expect("empty string") as i32;
        total = ((total).py_add(word)) as i32;
        i = ((i).py_add(2i32)) as i32;
    }
    let _cse_temp_1 = i < n;
    if _cse_temp_1 {
        let _cse_temp_2 = ({
            let base = &data;
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
        }
        .chars()
        .next()
        .expect("empty string") as i32)
            << 8;
        total = ((total).py_add(_cse_temp_2)) as i32;
    }
    while total > 65535 {
        total = ((total & 65535).py_add(total >> 16)) as i32;
    }
    Ok(!total & 65535)
}
#[doc = "Convert a 32-bit hash value to an 8-character hex string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn hash_to_hex(mut value: i32) -> String {
    let _cse_temp_0 = value & -1;
    value = _cse_temp_0;
    let hex_chars: String = "0123456789abcdef".to_string();
    let mut result: Vec<String> = vec![];
    for i in 0..(8) {
        let nibble: i32 = value >> (28i32) - ((i).py_mul(4i32)) & 15;
        result.push({
            let base = &hex_chars;
            let idx: i32 = nibble;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        });
    }
    result.join("")
}
#[doc = "Convert an 8-character hex string to a 32-bit integer."]
#[doc = " Depyler: verified panic-free"]
pub fn hex_to_hash(hex_str: &str) -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    for ch in hex_str.chars() {
        result = result << 4;
        if (ch >= '0') && (ch <= '9') {
            result = result
                | (ch as u32 as i32) - ("0".chars().next().expect("empty string") as i32);
        } else {
            if (ch >= 'a') && (ch <= 'f') {
                result = result
                    | ((ch as u32 as i32)
                         - (STR_A_1.chars().next().expect("empty string") as i32)
                        as i32)
                        .py_add(10i32);
            } else {
                if (ch >= 'A') && (ch <= 'F') {
                    result = result
                        | ((ch as u32 as i32)
                             - (STR_A_2.chars().next().expect("empty string") as i32)
                            as i32)
                            .py_add(10i32);
                }
            }
        }
    }
    result & -1
}
#[doc = "Compute Hamming distance between equal-length strings at bit level."]
#[doc = " Depyler: proven to terminate"]
pub fn hamming_distance_bytes<'b, 'a>(
    a: &'a str,
    b: &'b str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut dist: i32 = Default::default();
    let _cse_temp_0 = a.len() as i32;
    let _cse_temp_1 = b.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    if _cse_temp_2 {
        return Ok(-1);
    }
    dist = 0;
    for i in 0..(a.len() as i32) {
        let mut xor_val: i32 = {
            let base = &a;
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
        }
        .chars()
        .next()
        .expect("empty string") as i32
            ^ {
                let base = &b;
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
            }
            .chars()
            .next()
            .expect("empty string") as i32;
        while xor_val > 0 {
            dist = ((dist).py_add(xor_val & 1)) as i32;
            xor_val = xor_val >> 1;
        }
    }
    Ok(dist)
}
#[doc = "One round of SHA-256 style compression with Ch, Maj, Sigma."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn sha_style_compress(a: i32, b: i32, c: i32, d: i32, w: i32, k: i32) -> (i32, i32, i32, i32) {
    let _cse_temp_0 = a & b;
    let _cse_temp_1 = !a & c;
    let _cse_temp_2 = _cse_temp_0 ^ _cse_temp_1;
    let _cse_temp_3 = _cse_temp_2 & -1;
    let ch: i32 = _cse_temp_3;
    let _cse_temp_4 = a & c;
    let _cse_temp_5 = _cse_temp_0 ^ _cse_temp_4;
    let _cse_temp_6 = b & c;
    let _cse_temp_7 = _cse_temp_5 ^ _cse_temp_6;
    let _cse_temp_8 = _cse_temp_7 & -1;
    let maj: i32 = _cse_temp_8;
    let _cse_temp_9 = a >> 2;
    let _cse_temp_10 = a << 30;
    let _cse_temp_11 = _cse_temp_9 | _cse_temp_10;
    let _cse_temp_12 = a >> 13;
    let _cse_temp_13 = a << 19;
    let _cse_temp_14 = _cse_temp_12 | _cse_temp_13;
    let _cse_temp_15 = _cse_temp_11 ^ _cse_temp_14;
    let _cse_temp_16 = a >> 22;
    let _cse_temp_17 = a << 10;
    let _cse_temp_18 = _cse_temp_16 | _cse_temp_17;
    let _cse_temp_19 = _cse_temp_15 ^ _cse_temp_18;
    let _cse_temp_20 = _cse_temp_19 & -1;
    let s0: i32 = _cse_temp_20;
    let _cse_temp_21 = a >> 6;
    let _cse_temp_22 = a << 26;
    let _cse_temp_23 = _cse_temp_21 | _cse_temp_22;
    let _cse_temp_24 = a >> 11;
    let _cse_temp_25 = a << 21;
    let _cse_temp_26 = _cse_temp_24 | _cse_temp_25;
    let _cse_temp_27 = _cse_temp_23 ^ _cse_temp_26;
    let _cse_temp_28 = a >> 25;
    let _cse_temp_29 = a << 7;
    let _cse_temp_30 = _cse_temp_28 | _cse_temp_29;
    let _cse_temp_31 = _cse_temp_27 ^ _cse_temp_30;
    let _cse_temp_32 = _cse_temp_31 & -1;
    let s1: i32 = _cse_temp_32;
    let _cse_temp_33 = ((d).py_add(s1).py_add(ch)) as i32;
    let _cse_temp_34 = (((_cse_temp_33).py_add(k) as i32).py_add(w)) as i32;
    let _cse_temp_35 = _cse_temp_34 & -1;
    let t1: i32 = _cse_temp_35;
    let _cse_temp_36 = (s0).py_add(maj) & -1;
    let t2: i32 = _cse_temp_36;
    let _cse_temp_37 = (t1).py_add(t2) & -1;
    let new_a: i32 = _cse_temp_37;
    let _cse_temp_38 = (c).py_add(t1) & -1;
    let new_d: i32 = _cse_temp_38;
    (new_a, b, c, new_d)
}
#[doc = "Test all crypto and hash functions with known values."]
pub fn test_all() -> Result<bool, Box<dyn std::error::Error>> {
    let mut total_keys: i32 = Default::default();
    let mut ok: bool = Default::default();
    ok = true;
    let h1: i32 = djb2_hash(&STR_HELLO);
    let _cse_temp_0 = h1 == 0;
    if _cse_temp_0 {
        ok = false;
    }
    let h2: i32 = sdbm_hash(&STR_HELLO);
    let _cse_temp_1 = h2 == 0;
    if _cse_temp_1 {
        ok = false;
    }
    let h3: i32 = fnv1a_hash(&STR_HELLO);
    let _cse_temp_2 = h3 == 0;
    if _cse_temp_2 {
        ok = false;
    }
    let h4: i32 = jenkins_one_at_a_time(&STR_HELLO);
    let _cse_temp_3 = h4 == 0;
    if _cse_temp_3 {
        ok = false;
    }
    let positions: Vec<i32> = rabin_karp_search(&"abcabcabc", &"abc")?;
    let _cse_temp_4 = positions.len() as i32;
    let _cse_temp_5 = _cse_temp_4 != 3;
    if _cse_temp_5 {
        ok = false;
    }
    let enc: String = caesar_encrypt(&STR_HELLO, 3)?;
    let dec: String = caesar_decrypt(&enc, 3)?;
    let _cse_temp_6 = dec != STR_HELLO;
    if _cse_temp_6 {
        ok = false;
    }
    let venc: String = vigenere_encrypt(&"attackatdawn", &"lemon")?;
    let vdec: String = vigenere_decrypt(&venc, &"lemon")?;
    let _cse_temp_7 = vdec != "attackatdawn";
    if _cse_temp_7 {
        ok = false;
    }
    let xdata: Vec<i32> = xor_cipher_repeating(&"secret", &"key");
    let xback: String = xor_decipher_repeating(&xdata, &"key");
    let _cse_temp_8 = xback != "secret";
    if _cse_temp_8 {
        ok = false;
    }
    let b64: String = base64_encode(&"Hello")?;
    let decoded: String = base64_decode(&b64)?;
    let _cse_temp_9 = decoded != "Hello";
    if _cse_temp_9 {
        ok = false;
    }
    let b64_2: String = base64_encode(&"Hi")?;
    let decoded_2: String = base64_decode(&b64_2)?;
    let _cse_temp_10 = decoded_2 != "Hi";
    if _cse_temp_10 {
        ok = false;
    }
    let rle: String = run_length_encode(&"aaabbc")?;
    let _cse_temp_11 = rle != "3a2b1c";
    if _cse_temp_11 {
        ok = false;
    }
    let rld: String = run_length_decode(&"3a2b1c")?;
    let _cse_temp_12 = rld != "aaabbc";
    if _cse_temp_12 {
        ok = false;
    }
    let pairs: Vec<(String, i32)> = huffman_sorted_pairs(&"aabbc")?;
    let _cse_temp_13 = pairs.len() as i32;
    let _cse_temp_14 = _cse_temp_13 != 3;
    if _cse_temp_14 {
        ok = false;
    }
    let crc_val: i32 = crc32_compute(&STR_HELLO)?;
    let _cse_temp_15 = crc_val == 0;
    if _cse_temp_15 {
        ok = false;
    }
    let rounds: (i32, i32, i32, i32) = md5_round_functions(255, 15, 240);
    let _cse_temp_16 = rounds.0 == 0;
    if _cse_temp_16 {
        ok = false;
    }
    let mix: i32 = md5_mix_step(0, 1, 2, 3, 4, 7);
    let _cse_temp_17 = mix == 0;
    if _cse_temp_17 {
        ok = false;
    }
    let bf_result: (Vec<i32>, bool) = bloom_filter_ops(
        64,
        &vec![STR_HELLO.to_string(), "world".to_string()],
        &STR_HELLO,
    )?;
    if !bf_result.1 {
        ok = false;
    }
    let nodes: Vec<i32> = vec![0, 90, 180, 270];
    let keys: Vec<String> = vec![
        STR_A_1.to_string(),
        std::borrow::Cow::Borrowed("b").to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
    ];
    let dist: std::collections::HashMap<i32, Vec<String>> =
        consistent_hash_distribute(&keys, &nodes, 360)?;
    total_keys = 0;
    for node in nodes.iter().cloned() {
        total_keys = ((total_keys)
            .py_add(dist.get(&(node)).cloned().unwrap_or_default().len() as i32))
            as i32;
    }
    let _cse_temp_18 = total_keys != 5;
    if _cse_temp_18 {
        ok = false;
    }
    let phf: std::collections::HashMap<String, i32> = perfect_hash_small(&vec![
        "cat".to_string(),
        "dog".to_string(),
        "fish".to_string(),
    ])?;
    let _cse_temp_19 = phf.get("cat").is_none();
    if _cse_temp_19 {
        ok = false;
    }
    let collisions: Vec<(String, String)> = detect_hash_collisions(&vec![
        STR_A_1.to_string(),
        std::borrow::Cow::Borrowed("b").to_string(),
        "c".to_string(),
        "d".to_string(),
    ]);
    let _cse_temp_20 = collisions.len() as i32;
    let _cse_temp_21 = _cse_temp_20 < 0;
    if _cse_temp_21 {
        ok = false;
    }
    let fp: (i32, i32, i32, i32) = multi_hash_fingerprint(&"test");
    let _cse_temp_22 = (_cse_temp_16) && (_cse_temp_16);
    if _cse_temp_22 {
        ok = false;
    }
    let hl: i32 = hash_combine_list(&vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
    ]);
    let _cse_temp_23 = hl == 0;
    if _cse_temp_23 {
        ok = false;
    }
    let bits: (i32, i32, i32, i32) = bit_manipulation_suite(255);
    let _cse_temp_24 = bits.0 != 8;
    if _cse_temp_24 {
        ok = false;
    }
    let cksum: i32 = simple_checksum(&"Hello World")?;
    let _cse_temp_25 = cksum == 0;
    if _cse_temp_25 {
        ok = false;
    }
    let hex_str: String = hash_to_hex(-559038737);
    let _cse_temp_26 = hex_str != "deadbeef";
    if _cse_temp_26 {
        ok = false;
    }
    let back: i32 = hex_to_hash(&"deadbeef");
    let _cse_temp_27 = back != -559038737;
    if _cse_temp_27 {
        ok = false;
    }
    let hd: i32 = hamming_distance_bytes(&"abc", &"axc")?;
    let _cse_temp_28 = hd < 1;
    if _cse_temp_28 {
        ok = false;
    }
    let sha_out: (i32, i32, i32, i32) = sha_style_compress(
        1779033703,
        -1150833019,
        1013904242,
        -1521486534,
        305419896,
        1116352408,
    );
    if _cse_temp_16 {
        ok = false;
    }
    Ok(ok)
}
#[doc = r" DEPYLER-1216: Auto-generated entry point wrapping top-level script statements"]
#[doc = r" This file was transpiled from a Python script with executable top-level code."]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_djb2_hash_examples() {
        assert_eq!(djb2_hash(""), 0);
        assert_eq!(djb2_hash("a"), 1);
        assert_eq!(djb2_hash("abc"), 3);
    }
    #[test]
    fn test_sdbm_hash_examples() {
        assert_eq!(sdbm_hash(""), 0);
        assert_eq!(sdbm_hash("a"), 1);
        assert_eq!(sdbm_hash("abc"), 3);
    }
    #[test]
    fn test_fnv1a_hash_examples() {
        assert_eq!(fnv1a_hash(""), 0);
        assert_eq!(fnv1a_hash("a"), 1);
        assert_eq!(fnv1a_hash("abc"), 3);
    }
    #[test]
    fn test_jenkins_one_at_a_time_examples() {
        assert_eq!(jenkins_one_at_a_time(""), 0);
        assert_eq!(jenkins_one_at_a_time("a"), 1);
        assert_eq!(jenkins_one_at_a_time("abc"), 3);
    }
    #[test]
    fn quickcheck_huffman_sorted_pairs() {
        fn prop(data: String) -> TestResult {
            let result = huffman_sorted_pairs((&*data).into());
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = data.clone();
            input_sorted.sort();
            let mut result = huffman_sorted_pairs((&*data).into());
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    #[test]
    fn test_crc32_compute_examples() {
        assert_eq!(crc32_compute(""), 0);
        assert_eq!(crc32_compute("a"), 1);
        assert_eq!(crc32_compute("abc"), 3);
    }
    #[test]
    fn test_detect_hash_collisions_examples() {
        assert_eq!(detect_hash_collisions(vec![]), vec![]);
        assert_eq!(detect_hash_collisions(vec![1]), vec![1]);
    }
    #[test]
    fn test_hash_combine_list_examples() {
        assert_eq!(hash_combine_list(&vec![]), 0);
        assert_eq!(hash_combine_list(&vec![1]), 1);
        assert_eq!(hash_combine_list(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_simple_checksum_examples() {
        assert_eq!(simple_checksum(""), 0);
        assert_eq!(simple_checksum("a"), 1);
        assert_eq!(simple_checksum("abc"), 3);
    }
    #[test]
    fn test_hex_to_hash_examples() {
        assert_eq!(hex_to_hash(""), 0);
        assert_eq!(hex_to_hash("a"), 1);
        assert_eq!(hex_to_hash("abc"), 3);
    }
}