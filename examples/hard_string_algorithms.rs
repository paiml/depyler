#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR__: &'static str = "*";
const STR_HELLO: &'static str = "hello";
const STR_EMPTY: &'static str = "";
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
                DepylerValue::Str(key.to_string()),
                DepylerValue::Str(value.to_string()),
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
        Self::new(total_days.to_string(), total_secs.to_string(), total_us.to_string())
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
            self.days + other.days.to_string(),
            self.seconds + other.seconds.to_string(),
            self.microseconds + other.microseconds.to_string(),
        )
    }
}
impl std::ops::Sub for DepylerTimeDelta {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.days - other.days.to_string(),
            self.seconds - other.seconds.to_string(),
            self.microseconds - other.microseconds.to_string(),
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
#[doc = "Compute the KMP failure function(partial match table)."]
pub fn kmp_failure_function(pattern: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = pattern.len() as i32;
    let m: i32 = _cse_temp_0;
    let _cse_temp_1 = (vec![0]).py_mul(m);
    let mut fail: Vec<i32> = _cse_temp_1.clone();
    let mut k: i32 = 0;
    let mut i: i32 = 1;
    while i < m {
        if {
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
        } == {
            let base = &pattern;
            let idx: i32 = k;
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
            k = ((k).py_add(1i32)) as i32;
            fail[(i) as usize] = k;
            i = ((i).py_add(1i32)) as i32;
        } else {
            if k > 0 {
                k = {
                    let base = &fail;
                    let idx: i32 = (k) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
            } else {
                fail[(i) as usize] = 0;
                i = ((i).py_add(1i32)) as i32;
            }
        }
    }
    Ok(fail)
}
#[doc = "Find all occurrences of pattern in text using KMP algorithm."]
pub fn kmp_search<'a, 'b>(
    text: &'a str,
    pattern: &'b str,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = text.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = pattern.len() as i32;
    let m: i32 = _cse_temp_1;
    let _cse_temp_2 = m == 0;
    if _cse_temp_2 {
        return Ok(vec![]);
    }
    let fail: Vec<i32> = kmp_failure_function(pattern.clone())?;
    let mut matches: Vec<i32> = vec![];
    let mut j: i32 = 0;
    let mut i: i32 = 0;
    while i < n {
        if {
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
        } == {
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
            i = ((i).py_add(1i32)) as i32;
            j = ((j).py_add(1i32)) as i32;
            if j == m {
                matches.push((i) - (m));
                j = {
                    let base = &fail;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
            }
        } else {
            if j > 0 {
                j = {
                    let base = &fail;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
            } else {
                i = ((i).py_add(1i32)) as i32;
            }
        }
    }
    Ok(matches)
}
#[doc = "Find pattern occurrences using Rabin-Karp rolling hash."]
pub fn rabin_karp_search<'a, 'b>(
    text: &'a str,
    pattern: &'b str,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut h: i32 = Default::default();
    let mut i: i32 = Default::default();
    let mut t_hash: i32 = Default::default();
    let mut p_hash: i32 = Default::default();
    let _cse_temp_0 = text.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = pattern.len() as i32;
    let m: i32 = _cse_temp_1;
    let _cse_temp_2 = m == 0;
    let _cse_temp_3 = m > n;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(vec![]);
    }
    let base: i32 = 256;
    let r#mod: i32 = 1000000007;
    let mut matches: Vec<i32> = vec![];
    p_hash = 0;
    t_hash = 0;
    h = 1;
    let mut k: i32 = 0;
    while k < (m) - (1i32) {
        h = (((h).py_mul(base) as i32).py_mod(r#mod)) as i32;
        k = ((k).py_add(1i32)) as i32;
    }
    i = 0;
    while i < m {
        p_hash = ((((base).py_mul(p_hash) as i32).py_add(
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
        t_hash = ((((base).py_mul(t_hash) as i32).py_add(
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
        i = ((i).py_add(1i32)) as i32;
    }
    i = 0;
    while i <= (n) - (m) {
        if p_hash == t_hash {
            let mut r#match: bool = true;
            let mut j: i32 = 0;
            while j < m {
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
                    j = m;
                } else {
                    j = ((j).py_add(1i32)) as i32;
                }
            }
            if r#match {
                matches.push(i);
            }
        }
        if i < (n) - (m) {
            t_hash = ((((base).py_mul(
                (t_hash) - (
                    ({
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
                        .py_mul(h)
                ),
            ) as i32)
                .py_add(
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
            if t_hash < 0 {
                t_hash = ((t_hash).py_add(r#mod)) as i32;
            }
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(matches)
}
#[doc = "Expand around center indices to find palindrome."]
pub fn expand_around_center(
    s: &str,
    mut left: i32,
    mut right: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let _cse_temp_0 = s.len() as i32;
    let n: i32 = _cse_temp_0;
    while ((left >= 0) && (right < n))
        && ({
            let base = &s;
            let idx: i32 = left;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == {
            let base = &s;
            let idx: i32 = right;
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
    {
        left = ((left) - (1i32)) as i32;
        right = ((right).py_add(1i32)) as i32;
    }
    Ok({
        let base = (s).clone();
        let start_idx: i32 = (left).py_add(1i32);
        let stop_idx: i32 = right;
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
    })
}
#[doc = "Find the longest palindromic substring using expand-around-center."]
#[doc = " Depyler: verified panic-free"]
pub fn longest_palindromic_substring(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut best: String = Default::default();
    let _cse_temp_0 = s.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(STR_EMPTY.to_string());
    }
    best = {
        let base = (s).clone();
        let start_idx: i32 = 0;
        let stop_idx: i32 = 1;
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
    let mut i: i32 = 0;
    while i < s.len() as i32 {
        let odd: String = expand_around_center(s, i, i)?;
        if odd.len() as i32 > best.len() as i32 {
            best = odd.clone();
        }
        let even: String = expand_around_center(s, i, (i).py_add(1i32))?;
        if even.len() as i32 > best.len() as i32 {
            best = even.clone();
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(best.to_string())
}
#[doc = "Check if s2 is a rotation of s1."]
pub fn is_rotation<'a, 'b>(s1: &'a str, s2: &'b str) -> Result<bool, Box<dyn std::error::Error>> {
    let _cse_temp_0 = s1.len() as i32;
    let _cse_temp_1 = s2.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    if _cse_temp_2 {
        return Ok(false);
    }
    let _cse_temp_3 = _cse_temp_0 == 0;
    if _cse_temp_3 {
        return Ok(true);
    }
    let doubled: String = format!("{}{}", s1, s1);
    let _cse_temp_4 = doubled.len() as i32;
    let n: i32 = _cse_temp_4;
    let m: i32 = _cse_temp_1;
    let mut i: i32 = 0;
    while i <= (n) - (m) {
        let mut found: bool = true;
        let mut j: i32 = 0;
        while j < m {
            if {
                let base = &doubled;
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
                let base = &s2;
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
                found = false;
                j = m;
            } else {
                j = ((j).py_add(1i32)) as i32;
            }
        }
        if found {
            return Ok(true);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(false)
}
#[doc = "Count character frequencies in a string."]
pub fn char_frequency(s: &str) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut freq: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    let mut i: i32 = 0;
    while i < s.len() as i32 {
        let c: String = {
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
        };
        if freq.get(&c).is_some() {
            {
                let _key = c.clone();
                let _old_val = freq.get(&_key).cloned().unwrap_or_default();
                freq.insert(_key, _old_val + 1);
            }
        } else {
            freq.insert(c.to_string().clone(), 1);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(freq)
}
#[doc = "Check if two strings are anagrams of each other."]
pub fn are_anagrams<'b, 'a>(s1: &'a str, s2: &'b str) -> Result<bool, Box<dyn std::error::Error>> {
    let _cse_temp_0 = s1.len() as i32;
    let _cse_temp_1 = s2.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    if _cse_temp_2 {
        return Ok(false);
    }
    let freq1: std::collections::HashMap<String, i32> = char_frequency(s1)?;
    let freq2: std::collections::HashMap<String, i32> = char_frequency(s2)?;
    for key in freq1.keys().cloned() {
        if freq2.get(&key).is_none() {
            return Ok(false);
        }
        if freq1.get(&(key)).cloned().unwrap_or_default()
            != freq2.get(&(key)).cloned().unwrap_or_default()
        {
            return Ok(false);
        }
    }
    for key in freq2.keys().cloned() {
        if freq1.get(&key).is_none() {
            return Ok(false);
        }
    }
    Ok(true)
}
#[doc = "Group a list of words into anagram groups."]
pub fn group_anagrams(words: &Vec<String>) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut seen: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    let mut groups: Vec<Vec<String>> = vec![];
    let mut i: i32 = 0;
    while i < words.len() as i32 {
        let w: String = words
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let key: String = sort_string(&w)?;
        if seen.get(&key).is_some() {
            let idx: i32 = seen.get(&(key)).cloned().unwrap_or_default();
            groups
                .get(idx as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .push(w);
        } else {
            seen.insert(key.to_string().clone(), groups.len() as i32);
            groups.push(vec![w]);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(groups)
}
#[doc = "Sort characters of a string alphabetically(selection sort)."]
pub fn sort_string(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut i: i32 = Default::default();
    let mut result: String = Default::default();
    let mut chars: Vec<String> = vec![];
    i = 0;
    while i < s.len() as i32 {
        chars.push({
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
        });
        i = ((i).py_add(1i32)) as i32;
    }
    let _cse_temp_0 = chars.len() as i32;
    let n: i32 = _cse_temp_0;
    i = 0;
    while i < n {
        let mut min_idx: i32 = i.clone();
        let mut j: i32 = ((i).py_add(1i32)) as i32;
        while j < n {
            if chars
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < chars
                    .get(min_idx as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                min_idx = j;
            }
            j = ((j).py_add(1i32)) as i32;
        }
        let tmp: String = chars
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        chars[(i) as usize] = chars
            .get(min_idx as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        chars[(min_idx) as usize] = tmp.clone();
        i = ((i).py_add(1i32)) as i32;
    }
    result = STR_EMPTY.to_string().to_string();
    i = 0;
    while i < n {
        result = format!(
            "{}{}",
            result,
            chars
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        );
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(result.to_string())
}
#[doc = "Run-length encode a string: 'aaabbc' -> 'a3b2c1'."]
pub fn run_length_encode(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    let mut result: String = Default::default();
    let _cse_temp_0 = s.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(STR_EMPTY.to_string());
    }
    result = STR_EMPTY.to_string().to_string();
    count = 1;
    let mut i: i32 = 1;
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
        } == {
            let base = &s;
            let idx: i32 = (i) - (1i32);
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
            count = ((count).py_add(1i32)) as i32;
        } else {
            result = format!(
                "{}{}",
                format!("{}{}", result, {
                    let base = &s;
                    let idx: i32 = (i) - (1i32);
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
                int_to_str(count)?
            );
            count = 1;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = format!("{}{}", result, {
        let base = &s;
        let idx: i32 = (s.len() as i32) - (1i32);
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
    let _cse_temp_3 = format!("{}{}", _cse_temp_2, int_to_str(count)?);
    result = _cse_temp_3.clone();
    Ok(result.to_string())
}
#[doc = "Convert a non-negative integer to its string representation."]
pub fn int_to_str(n: i32) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String = Default::default();
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
        return Ok("0".to_string().to_string());
    }
    result = STR_EMPTY.to_string().to_string();
    let mut val: i32 = n.clone();
    while val > 0 {
        let digit: i32 = ((val).py_mod(10i32)) as i32;
        if digit == 0 {
            result = format!("{}{}", "0".to_string(), result);
        } else {
            if digit == 1 {
                result = format!("{}{}", "1", result);
            } else {
                if digit == 2 {
                    result = format!("{}{}", "2", result);
                } else {
                    if digit == 3 {
                        result = format!("{}{}", "3", result);
                    } else {
                        if digit == 4 {
                            result = format!("{}{}", "4", result);
                        } else {
                            if digit == 5 {
                                result = format!("{}{}", "5", result);
                            } else {
                                if digit == 6 {
                                    result = format!("{}{}", "6", result);
                                } else {
                                    if digit == 7 {
                                        result = format!("{}{}", "7", result);
                                    } else {
                                        if digit == 8 {
                                            result = format!("{}{}", "8", result);
                                        } else {
                                            result = format!("{}{}", "9", result);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        val = {
            let a = val;
            let b = 10;
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
    }
    Ok(result.to_string())
}
#[doc = "Decode a run-length encoded string: 'a3b2c1' -> 'aaabbc'."]
pub fn run_length_decode(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string().to_string();
    let mut i: i32 = 0;
    let _cse_temp_0 = encoded.len() as i32;
    let n: i32 = _cse_temp_0;
    while i < n {
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
        i = ((i).py_add(1i32)) as i32;
        let mut num_str: String = STR_EMPTY.to_string();
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
                >= "0".to_string()))
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
        let count: i32 = str_to_int(&num_str)?;
        let mut j: i32 = 0;
        while j < count {
            result = format!("{}{}", result, ch);
            j = ((j).py_add(1i32)) as i32;
        }
    }
    Ok(result.to_string())
}
#[doc = "Convert a numeric string to an integer."]
pub fn str_to_int(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut result: i32 = Default::default();
    let mut d: i32 = Default::default();
    result = 0;
    let mut i: i32 = 0;
    while i < s.len() as i32 {
        let c: String = {
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
        };
        d = 0;
        if c == "1" {
            d = 1;
        } else {
            if c == "2" {
                d = 2;
            } else {
                if c == "3" {
                    d = 3;
                } else {
                    if c == "4" {
                        d = 4;
                    } else {
                        if c == "5" {
                            d = 5;
                        } else {
                            if c == "6" {
                                d = 6;
                            } else {
                                if c == "7" {
                                    d = 7;
                                } else {
                                    if c == "8" {
                                        d = 8;
                                    } else {
                                        if c == "9" {
                                            d = 9;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        result = (((result).py_mul(10i32) as i32).py_add(d)) as i32;
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Compute the Levenshtein (edit) distance between two strings."]
pub fn levenshtein_distance<'a, 'b>(
    s1: &'a str,
    s2: &'b str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut j: i32 = Default::default();
    let mut prev: Vec<i32> = Default::default();
    let mut min_cost: i32 = Default::default();
    let _cse_temp_0 = s1.len() as i32;
    let m: i32 = _cse_temp_0;
    let _cse_temp_1 = s2.len() as i32;
    let n: i32 = _cse_temp_1;
    prev = vec![];
    j = 0;
    while j <= n {
        prev.push(j);
        j = ((j).py_add(1i32)) as i32;
    }
    let mut i: i32 = 1;
    while i <= m {
        let mut curr: Vec<i32> = vec![i];
        j = 1;
        while j <= n {
            if {
                let base = &s1;
                let idx: i32 = (i) - (1i32);
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            } == {
                let base = &s2;
                let idx: i32 = (j) - (1i32);
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
                curr.push({
                    let base = &prev;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                });
            } else {
                let replace_cost: i32 = (({
                    let base = &prev;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                })
                .py_add(1i32)) as i32;
                let insert_cost: i32 = (({
                    let base = &curr;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                })
                .py_add(1i32)) as i32;
                let delete_cost: i32 = ((prev
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"))
                .py_add(1i32)) as i32;
                min_cost = replace_cost;
                if insert_cost < min_cost {
                    min_cost = insert_cost;
                }
                if delete_cost < min_cost {
                    min_cost = delete_cost;
                }
                curr.push(min_cost);
            }
            j = ((j).py_add(1i32)) as i32;
        }
        prev = curr.clone();
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(prev
        .get(n as usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Compute the Z-array for the given string."]
pub fn z_function(s: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = s.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    let _cse_temp_2 = (vec![0]).py_mul(n);
    let mut z: Vec<i32> = _cse_temp_2.clone();
    z[(0) as usize] = n;
    let mut l: i32 = 0;
    let mut r: i32 = 0;
    let mut i: i32 = 1;
    while i < n {
        if i < r {
            z[(i) as usize] = (r) - (i);
            if {
                let base = &z;
                let idx: i32 = (i) - (l);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            } < z
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
            {
                z[(i) as usize] = {
                    let base = &z;
                    let idx: i32 = (i) - (l);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
            }
        }
        while ((i).py_add(
            z.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ) < n)
            && ({
                let base = &s;
                let idx: i32 = z
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            } == {
                let base = &s;
                let idx: i32 = (i).py_add(
                    z.get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                );
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
        {
            z[(i) as usize] = (z
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_add(1i32);
        }
        if (i).py_add(
            z.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ) > r
        {
            l = i;
            r = ((i).py_add(
                z.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            )) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(z)
}
#[doc = "Find all occurrences of pattern in text using Z-algorithm."]
pub fn z_search<'b, 'a>(
    text: &'a str,
    pattern: &'b str,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = pattern.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    let _cse_temp_2 = format!("{}{}", format!("{}{}", pattern, "$"), text);
    let combined: String = _cse_temp_2.clone().to_string();
    let z: Vec<i32> = z_function(&combined)?;
    let m: i32 = _cse_temp_0;
    let mut matches: Vec<i32> = vec![];
    let mut i: i32 = ((m).py_add(1i32)) as i32;
    while i < combined.len() as i32 {
        if z.get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            == m
        {
            matches.push(((i) - (m) as i32) - (1i32));
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(matches)
}
#[doc = "Build a suffix array using naive O(n^2 log n) construction."]
pub fn build_suffix_array(s: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut i: i32 = Default::default();
    let _cse_temp_0 = s.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut suffixes: Vec<i32> = vec![];
    i = 0;
    while i < n {
        suffixes.push(i);
        i = ((i).py_add(1i32)) as i32;
    }
    i = 0;
    while i < n {
        let mut min_idx: i32 = i.clone();
        let mut j: i32 = ((i).py_add(1i32)) as i32;
        while j < n {
            if compare_suffixes(
                s.to_string(),
                suffixes
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
                suffixes
                    .get(min_idx as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            )? < 0
            {
                min_idx = j;
            }
            j = ((j).py_add(1i32)) as i32;
        }
        let tmp: i32 = suffixes
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        suffixes[(i) as usize] = suffixes
            .get(min_idx as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        suffixes[(min_idx) as usize] = tmp;
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(suffixes)
}
#[doc = "Compare two suffixes of s starting at positions i and j."]
pub fn compare_suffixes(
    s: &str,
    mut i: i32,
    mut j: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = s.len() as i32;
    let n: i32 = _cse_temp_0;
    while (i < n) && (j < n) {
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
        })
        .as_str()
            < ({
                let base = &s;
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
            })
            .as_str()
        {
            return Ok(-1);
        } else {
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
            })
            .as_str()
                > ({
                    let base = &s;
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
                })
                .as_str()
            {
                return Ok(1);
            }
        }
        i = ((i).py_add(1i32)) as i32;
        j = ((j).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = i == n;
    let _cse_temp_2 = j == n;
    let _cse_temp_3 = (_cse_temp_1) && (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(0);
    } else {
        if _cse_temp_1 {
            return Ok(-1);
        } else {
            return Ok(1);
        }
    }
}
#[doc = "Compute odd-length palindrome radii using Manacher's algorithm."]
pub fn manacher_odd(s: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = s.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    let _cse_temp_2 = (vec![0]).py_mul(n);
    let mut p: Vec<i32> = _cse_temp_2.clone();
    let mut center: i32 = 0;
    let mut right: i32 = 0;
    let mut i: i32 = 0;
    while i < n {
        let mirror: i32 = (((2i32).py_mul(center) as i32) - (i)) as i32;
        if (i < right) && (mirror >= 0) {
            p[(i) as usize] = (right) - (i);
            if p.get(mirror as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < p.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                p[(i) as usize] = p
                    .get(mirror as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
            }
        }
        while ((((i) - (
            p.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        ) as i32)
             - (1i32)
            >= 0)
            && (((i).py_add(
                p.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ) as i32)
                .py_add(1i32)
                < n))
            && ({
                let base = &s;
                let idx: i32 = ((i) - (
                    p.get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                ) as i32)
                     - (1i32);
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            } == {
                let base = &s;
                let idx: i32 = ((i).py_add(
                    p.get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                ) as i32)
                    .py_add(1i32);
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
        {
            p[(i) as usize] = (p
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_add(1i32);
        }
        if (i).py_add(
            p.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ) > right
        {
            center = i;
            right = ((i).py_add(
                p.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            )) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(p)
}
#[doc = "Find the longest palindromic substring using Manacher's algorithm."]
pub fn longest_palindrome_manacher(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut i: i32 = Default::default();
    let mut t: String = Default::default();
    let mut max_len: i32 = Default::default();
    let mut best_center: i32 = Default::default();
    let _cse_temp_0 = s.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(STR_EMPTY.to_string());
    }
    t = "^".to_string();
    i = 0;
    while i < s.len() as i32 {
        t = format!("{}{}", format!("{}{}", t, "#"), {
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
        });
        i = ((i).py_add(1i32)) as i32;
    }
    t = format!("{}{}", t, "#$");
    let _cse_temp_2 = t.len() as i32;
    let n: i32 = _cse_temp_2;
    let _cse_temp_3 = (vec![0]).py_mul(n);
    let mut p: Vec<i32> = _cse_temp_3.clone();
    let mut center: i32 = 0;
    let mut right: i32 = 0;
    i = 1;
    while i < (n) - (1i32) {
        let mirror: i32 = (((2i32).py_mul(center) as i32) - (i)) as i32;
        if i < right {
            p[(i) as usize] = (right) - (i);
            if (mirror >= 0)
                && (p
                    .get(mirror as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    < p.get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"))
            {
                p[(i) as usize] = p
                    .get(mirror as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
            }
        }
        while {
            let base = &t;
            let idx: i32 = ((i).py_add(
                p.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ) as i32)
                .py_add(1i32);
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == {
            let base = &t;
            let idx: i32 = ((i) - (
                p.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            ) as i32)
                 - (1i32);
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
            p[(i) as usize] = (p
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_add(1i32);
        }
        if (i).py_add(
            p.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ) > right
        {
            center = i;
            right = ((i).py_add(
                p.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            )) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    max_len = 0;
    best_center = 0;
    i = 1;
    while i < (n) - (1i32) {
        if p.get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            > max_len
        {
            max_len = p
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            best_center = i;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    let _cse_temp_4 = {
        let a = (best_center) - (max_len);
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
    let start: i32 = _cse_temp_4;
    Ok({
        let base = (s).clone();
        let start_idx: i32 = start;
        let stop_idx: i32 = (start).py_add(max_len);
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
    })
}
#[doc = "Compress a string using run-length encoding; return original if no savings."]
pub fn compress_string(s: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    let mut compressed: String = Default::default();
    let _cse_temp_0 = s.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(STR_EMPTY.to_string());
    }
    compressed = STR_EMPTY.to_string().to_string();
    count = 1;
    let mut i: i32 = 1;
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
        } == {
            let base = &s;
            let idx: i32 = (i) - (1i32);
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
            count = ((count).py_add(1i32)) as i32;
        } else {
            compressed = format!("{}{}", compressed, {
                let base = &s;
                let idx: i32 = (i) - (1i32);
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
            if count > 1 {
                compressed = format!("{}{}", compressed, int_to_str(count)?);
            }
            count = 1;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = format!("{}{}", compressed, {
        let base = &s;
        let idx: i32 = (s.len() as i32) - (1i32);
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
    compressed = _cse_temp_2.clone();
    let _cse_temp_3 = count > 1;
    if _cse_temp_3 {
        let _cse_temp_4 = format!("{}{}", compressed, int_to_str(count)?);
        compressed = _cse_temp_4.clone();
    }
    let _cse_temp_5 = compressed.len() as i32;
    let _cse_temp_6 = _cse_temp_5 < _cse_temp_0;
    if _cse_temp_6 {
        return Ok(compressed.to_string());
    }
    Ok(s.to_string())
}
#[doc = "Match text against pattern with '?'(any char) and '*'(any sequence)."]
pub fn wildcard_match<'b, 'a>(
    text: &'a str,
    pattern: &'b str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut j: i32 = Default::default();
    let mut prev: Vec<bool> = Default::default();
    let _cse_temp_0 = text.len() as i32;
    let m: i32 = _cse_temp_0;
    let _cse_temp_1 = pattern.len() as i32;
    let n: i32 = _cse_temp_1;
    prev = vec![];
    j = 0;
    while j <= n {
        prev.push(false);
        j = ((j).py_add(1i32)) as i32;
    }
    prev[(0) as usize] = true;
    j = 1;
    while j <= n {
        if {
            let base = &pattern;
            let idx: i32 = (j) - (1i32);
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == STR__
        {
            prev[(j) as usize] = {
                let base = &prev;
                let idx: i32 = (j) - (1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            };
        }
        j = ((j).py_add(1i32)) as i32;
    }
    let mut i: i32 = 1;
    while i <= m {
        let mut curr: Vec<bool> = vec![];
        j = 0;
        while j <= n {
            curr.push(false);
            j = ((j).py_add(1i32)) as i32;
        }
        j = 1;
        while j <= n {
            if {
                let base = &pattern;
                let idx: i32 = (j) - (1i32);
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            } == STR__
            {
                curr[(j) as usize] = ({
                    let base = &curr;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                }) || (prev
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"));
            } else {
                if ({
                    let base = &pattern;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.chars().count().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.chars()
                        .nth(actual_idx)
                        .map(|c| c.to_string())
                        .unwrap_or_default()
                } == "?")
                    || ({
                        let base = &pattern;
                        let idx: i32 = (j) - (1i32);
                        let actual_idx = if idx < 0 {
                            base.chars().count().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.chars()
                            .nth(actual_idx)
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    } == {
                        let base = &text;
                        let idx: i32 = (i) - (1i32);
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
                {
                    curr[(j) as usize] = {
                        let base = &prev;
                        let idx: i32 = (j) - (1i32);
                        let actual_idx = if idx < 0 {
                            base.len().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.get(actual_idx)
                            .cloned()
                            .expect("IndexError: list index out of range")
                    };
                }
            }
            j = ((j).py_add(1i32)) as i32;
        }
        prev = curr.clone();
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(prev
        .get(n as usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Match text against a simple regex with '.'(any char) and '*'(zero or more of prev)."]
pub fn simple_regex_match<'a, 'b>(
    text: &'a str,
    pattern: &'b str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut j: i32 = Default::default();
    let mut prev: Vec<bool> = Default::default();
    let _cse_temp_0 = text.len() as i32;
    let m: i32 = _cse_temp_0;
    let _cse_temp_1 = pattern.len() as i32;
    let n: i32 = _cse_temp_1;
    prev = vec![];
    j = 0;
    while j <= n {
        prev.push(false);
        j = ((j).py_add(1i32)) as i32;
    }
    prev[(0) as usize] = true;
    j = 2;
    while j <= n {
        if {
            let base = &pattern;
            let idx: i32 = (j) - (1i32);
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == STR__
        {
            prev[(j) as usize] = {
                let base = &prev;
                let idx: i32 = (j) - (2i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            };
        }
        j = ((j).py_add(1i32)) as i32;
    }
    let mut i: i32 = 1;
    while i <= m {
        let mut curr: Vec<bool> = vec![];
        j = 0;
        while j <= n {
            curr.push(false);
            j = ((j).py_add(1i32)) as i32;
        }
        j = 1;
        while j <= n {
            if {
                let base = &pattern;
                let idx: i32 = (j) - (1i32);
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            } == STR__
            {
                curr[(j) as usize] = {
                    let base = &curr;
                    let idx: i32 = (j) - (2i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
                if ({
                    let base = &pattern;
                    let idx: i32 = (j) - (2i32);
                    let actual_idx = if idx < 0 {
                        base.chars().count().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.chars()
                        .nth(actual_idx)
                        .map(|c| c.to_string())
                        .unwrap_or_default()
                } == ".")
                    || ({
                        let base = &pattern;
                        let idx: i32 = (j) - (2i32);
                        let actual_idx = if idx < 0 {
                            base.chars().count().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.chars()
                            .nth(actual_idx)
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    } == {
                        let base = &text;
                        let idx: i32 = (i) - (1i32);
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
                {
                    curr[(j) as usize] = (curr
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"))
                        || (prev
                            .get(j as usize)
                            .cloned()
                            .expect("IndexError: list index out of range"));
                }
            } else {
                if ({
                    let base = &pattern;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.chars().count().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.chars()
                        .nth(actual_idx)
                        .map(|c| c.to_string())
                        .unwrap_or_default()
                } == ".")
                    || ({
                        let base = &pattern;
                        let idx: i32 = (j) - (1i32);
                        let actual_idx = if idx < 0 {
                            base.chars().count().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.chars()
                            .nth(actual_idx)
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    } == {
                        let base = &text;
                        let idx: i32 = (i) - (1i32);
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
                {
                    curr[(j) as usize] = {
                        let base = &prev;
                        let idx: i32 = (j) - (1i32);
                        let actual_idx = if idx < 0 {
                            base.len().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        base.get(actual_idx)
                            .cloned()
                            .expect("IndexError: list index out of range")
                    };
                }
            }
            j = ((j).py_add(1i32)) as i32;
        }
        prev = curr.clone();
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(prev
        .get(n as usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Find the longest common prefix of a list of strings."]
pub fn longest_common_prefix(strings: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut prefix: String = Default::default();
    let mut j: i32 = Default::default();
    let _cse_temp_0 = strings.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(STR_EMPTY.to_string());
    }
    prefix = strings
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let mut i: i32 = 1;
    while i < strings.len() as i32 {
        let mut new_prefix: String = STR_EMPTY.to_string();
        j = 0;
        let s: String = strings
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        while (j < prefix.len() as i32) && (j < s.len() as i32) {
            if {
                let base = &prefix;
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
            } == {
                let base = &s;
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
                new_prefix = format!("{}{}", new_prefix, {
                    let base = &prefix;
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
                });
            } else {
                j = prefix.len() as i32;
            }
            j = ((j).py_add(1i32)) as i32;
        }
        prefix = new_prefix.clone();
        if prefix.len() as i32 == 0 {
            return Ok(STR_EMPTY.to_string());
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(prefix.to_string())
}
#[doc = "Encode text using Caesar cipher with given shift."]
pub fn caesar_cipher_encode(text: &str, shift: i32) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string().to_string();
    let mut i: i32 = 0;
    while i < text.len() as i32 {
        let c: String = {
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
        };
        let mut code: i32 = c.chars().next().expect("empty string") as i32;
        if (code >= 65) && (code <= 90) {
            code = (((((code) - (65i32) as i32).py_add(shift) as i32).py_mod(26i32) as i32)
                .py_add(65i32)) as i32;
            result = format!(
                "{}{}",
                result,
                char::from_u32((code) as u32)
                    .expect("builtin operation failed")
                    .to_string()
            );
        } else {
            if (code >= 97) && (code <= 122) {
                code = (((((code) - (97i32) as i32).py_add(shift) as i32).py_mod(26i32) as i32)
                    .py_add(97i32)) as i32;
                result = format!(
                    "{}{}",
                    result,
                    char::from_u32((code) as u32)
                        .expect("builtin operation failed")
                        .to_string()
                );
            } else {
                result = format!("{}{}", result, c);
            }
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(result.to_string())
}
#[doc = "Decode text using Caesar cipher with given shift."]
#[doc = " Depyler: proven to terminate"]
pub fn caesar_cipher_decode(text: &str, shift: i32) -> Result<String, Box<dyn std::error::Error>> {
    caesar_cipher_encode(text, (26i32) - ((shift).py_mod(26i32)))
}
#[doc = "Compute the Hamming distance between two equal-length strings."]
pub fn hamming_distance<'b, 'a>(
    s1: &'a str,
    s2: &'b str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut dist: i32 = Default::default();
    let _cse_temp_0 = s1.len() as i32;
    let _cse_temp_1 = s2.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    if _cse_temp_2 {
        return Ok(-1);
    }
    dist = 0;
    let mut i: i32 = 0;
    while i < s1.len() as i32 {
        if {
            let base = &s1;
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
        } != {
            let base = &s2;
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
        } {
            dist = ((dist).py_add(1i32)) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(dist)
}
#[doc = "Find the longest common substring of two strings."]
pub fn longest_common_substring<'a, 'b>(
    s1: &'a str,
    s2: &'b str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut j: i32 = Default::default();
    let _cse_temp_0 = s1.len() as i32;
    let m: i32 = _cse_temp_0;
    let _cse_temp_1 = s2.len() as i32;
    let n: i32 = _cse_temp_1;
    let mut best_len: i32 = 0;
    let mut best_end: i32 = 0;
    let mut prev: Vec<i32> = vec![];
    j = 0;
    while j <= n {
        prev.push(0);
        j = ((j).py_add(1i32)) as i32;
    }
    let mut i: i32 = 1;
    while i <= m {
        let mut curr: Vec<i32> = vec![0];
        j = 1;
        while j <= n {
            if {
                let base = &s1;
                let idx: i32 = (i) - (1i32);
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            } == {
                let base = &s2;
                let idx: i32 = (j) - (1i32);
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
                let val: i32 = (({
                    let base = &prev;
                    let idx: i32 = (j) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                })
                .py_add(1i32)) as i32;
                curr.push(val);
                if val > best_len {
                    best_len = val;
                    best_end = i;
                }
            } else {
                curr.push(0);
            }
            j = ((j).py_add(1i32)) as i32;
        }
        prev = curr.clone();
        i = ((i).py_add(1i32)) as i32;
    }
    Ok({
        let base = (s1).clone();
        let start_idx: i32 = (best_end) - (best_len);
        let stop_idx: i32 = best_end;
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
    })
}
#[doc = "Check if s is a subsequence of t."]
pub fn is_subsequence<'a, 'b>(s: &'a str, t: &'b str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut si: i32 = Default::default();
    si = 0;
    let mut ti: i32 = 0;
    while (si < s.len() as i32) && (ti < t.len() as i32) {
        if {
            let base = &s;
            let idx: i32 = si;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == {
            let base = &t;
            let idx: i32 = ti;
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
            si = ((si).py_add(1i32)) as i32;
        }
        ti = ((ti).py_add(1i32)) as i32;
    }
    Ok(si == s.len() as i32)
}
#[doc = "Count the number of distinct substrings using a brute-force set approach."]
#[doc = " Depyler: verified panic-free"]
pub fn count_distinct_substrings(s: &str) -> i32 {
    let mut count: i32 = Default::default();
    let _cse_temp_0 = s.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut seen: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    let mut i: i32 = 0;
    while i < n {
        let mut j: i32 = ((i).py_add(1i32)) as i32;
        while j <= n {
            let sub: String = {
                let base = (s).clone();
                let start_idx: i32 = i;
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
            if seen.get(&sub).is_none() {
                seen.insert(sub.to_string().clone(), 1);
            }
            j = ((j).py_add(1i32)) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    count = 0;
    for _key in seen.keys().cloned() {
        count = ((count).py_add(1i32)) as i32;
    }
    count
}
#[doc = "Find minimum repeats of a such that b is a substring of the repeated a. Return -1 if impossible."]
pub fn repeated_string_match<'a, 'b>(
    a: &'a str,
    b: &'b str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = a.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(-1);
    }
    let mut repeated: String = STR_EMPTY.to_string();
    let mut count: i32 = 0;
    while (repeated.len() as i32) < (b.len() as i32).py_add((2i32).py_mul(a.len() as i32)) {
        repeated = format!("{}{}", repeated, a);
        count = ((count).py_add(1i32)) as i32;
        if repeated.len() as i32 >= b.len() as i32 {
            let mut found: bool = false;
            let mut k: i32 = 0;
            while k <= (repeated.len() as i32) - (b.len() as i32) {
                let mut r#match: bool = true;
                let mut j: i32 = 0;
                while j < b.len() as i32 {
                    if {
                        let base = &repeated;
                        let idx: i32 = (k).py_add(j);
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
                        let base = &b;
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
                        j = b.len() as i32;
                    } else {
                        j = ((j).py_add(1i32)) as i32;
                    }
                }
                if r#match {
                    found = true;
                    k = repeated.len() as i32;
                } else {
                    k = ((k).py_add(1i32)) as i32;
                }
            }
            if found {
                return Ok(count);
            }
        }
    }
    Ok(-1)
}
#[doc = "Interleave two strings character by character."]
pub fn interleave_strings<'b, 'a>(
    s1: &'a str,
    s2: &'b str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String = Default::default();
    let mut i: i32 = Default::default();
    let mut j: i32 = Default::default();
    result = STR_EMPTY.to_string().to_string();
    i = 0;
    j = 0;
    while (i < s1.len() as i32) && (j < s2.len() as i32) {
        result = format!(
            "{}{}",
            format!("{}{}", result, {
                let base = &s1;
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
            {
                let base = &s2;
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
        );
        i = ((i).py_add(1i32)) as i32;
        j = ((j).py_add(1i32)) as i32;
    }
    while i < s1.len() as i32 {
        result = format!("{}{}", result, {
            let base = &s1;
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
    while j < s2.len() as i32 {
        result = format!("{}{}", result, {
            let base = &s2;
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
        });
        j = ((j).py_add(1i32)) as i32;
    }
    Ok(result.to_string())
}
#[doc = "Find the longest substring that appears at least twice."]
#[doc = " Depyler: verified panic-free"]
pub fn longest_repeating_substring(s: &str) -> String {
    let mut best: String = Default::default();
    let _cse_temp_0 = s.len() as i32;
    let n: i32 = _cse_temp_0;
    best = STR_EMPTY.to_string().to_string();
    let mut length: i32 = 1;
    while length < n {
        let mut i: i32 = 0;
        while i <= (n) - (length) {
            let sub: String = {
                let base = (s).clone();
                let start_idx: i32 = i;
                let stop_idx: i32 = (i).py_add(length);
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
            let mut j: i32 = ((i).py_add(1i32)) as i32;
            while j <= (n) - (length) {
                if {
                    let base = (s).clone();
                    let start_idx: i32 = j;
                    let stop_idx: i32 = (j).py_add(length);
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
                } == sub
                {
                    if length > best.len() as i32 {
                        best = sub.clone();
                    }
                    j = n;
                } else {
                    j = ((j).py_add(1i32)) as i32;
                }
            }
            i = ((i).py_add(1i32)) as i32;
        }
        length = ((length).py_add(1i32)) as i32;
    }
    best.to_string()
}
#[doc = "Compute a polynomial rolling hash of a string."]
pub fn string_multiply_hash(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut h: i32 = Default::default();
    h = 0;
    let base: i32 = 31;
    let r#mod: i32 = 1000000007;
    let mut i: i32 = 0;
    while i < s.len() as i32 {
        h = ((((h).py_mul(base) as i32).py_add(
            {
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
            }
            .chars()
            .next()
            .expect("empty string") as i32,
        ) as i32)
            .py_mod(r#mod)) as i32;
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(h)
}
#[doc = "Test all string algorithm functions."]
#[doc = " Depyler: proven to terminate"]
pub fn test_all() -> Result<bool, Box<dyn std::error::Error>> {
    let fail: Vec<i32> = kmp_failure_function(&"abcabd")?;
    let _cse_temp_0 = fail
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 0;
    let _cse_temp_1 = fail
        .get(3usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 1;
    let _cse_temp_2 = (_cse_temp_0) && (_cse_temp_1);
    let _cse_temp_3 = fail
        .get(4usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 2;
    let _cse_temp_4 = (_cse_temp_2) && (_cse_temp_3);
    let mut ok: bool = _cse_temp_4.clone();
    let matches: Vec<i32> = kmp_search(&"abcabcabd", &"abcabd")?;
    let _cse_temp_5 = matches.len() as i32;
    let _cse_temp_6 = _cse_temp_5 == 1;
    let _cse_temp_7 = (ok) && (_cse_temp_6);
    let _cse_temp_8 = matches
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 3;
    let _cse_temp_9 = (_cse_temp_7) && (_cse_temp_8);
    ok = _cse_temp_9;
    let rk: Vec<i32> = rabin_karp_search(&"ababababab", &"abab")?;
    let _cse_temp_10 = rk.len() as i32;
    let _cse_temp_11 = _cse_temp_10 >= 1;
    let _cse_temp_12 = (ok) && (_cse_temp_11);
    let _cse_temp_13 = (_cse_temp_12) && (_cse_temp_0);
    ok = _cse_temp_13;
    let pal: String = longest_palindromic_substring(&"babad")?;
    let _cse_temp_14 = pal == "bab";
    let _cse_temp_15 = pal == "aba";
    let _cse_temp_16 = (_cse_temp_14) || (_cse_temp_15);
    let _cse_temp_17 = (ok) && (_cse_temp_16);
    ok = _cse_temp_17;
    let _cse_temp_18 = (ok) && (is_rotation(&"abcde", &"cdeab")?);
    ok = _cse_temp_18;
    let _cse_temp_19 = (ok) && (!is_rotation(&"abcde", &"abced"));
    ok = _cse_temp_19;
    let _cse_temp_20 = (ok) && (are_anagrams(&"listen", &"silent")?);
    ok = _cse_temp_20;
    ok = _cse_temp_19;
    let groups: Vec<Vec<String>> = group_anagrams(&vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ])?;
    let _cse_temp_21 = groups.len() as i32;
    let _cse_temp_22 = _cse_temp_21 == 3;
    let _cse_temp_23 = (ok) && (_cse_temp_22);
    ok = _cse_temp_23;
    let encoded: String = run_length_encode(&"aaabbc")?;
    let _cse_temp_24 = encoded == "a3b2c1";
    let _cse_temp_25 = (ok) && (_cse_temp_24);
    ok = _cse_temp_25;
    let decoded: String = run_length_decode(&encoded)?;
    let _cse_temp_26 = decoded == "aaabbc";
    let _cse_temp_27 = (ok) && (_cse_temp_26);
    ok = _cse_temp_27;
    let dist: i32 = levenshtein_distance(&"kitten", &"sitting")?;
    let _cse_temp_28 = dist == 3;
    let _cse_temp_29 = (ok) && (_cse_temp_28);
    ok = _cse_temp_29;
    let z_matches: Vec<i32> = z_search(&"ababababab", &"abab")?;
    let _cse_temp_30 = z_matches.len() as i32;
    let _cse_temp_31 = _cse_temp_30 >= 1;
    let _cse_temp_32 = (ok) && (_cse_temp_31);
    let _cse_temp_33 = (_cse_temp_32) && (_cse_temp_0);
    ok = _cse_temp_33;
    let sa: Vec<i32> = build_suffix_array(&"banana")?;
    let _cse_temp_34 = sa
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 5;
    let _cse_temp_35 = (ok) && (_cse_temp_34);
    ok = _cse_temp_35;
    let man_pal: String = longest_palindrome_manacher(&"cbbd")?;
    let _cse_temp_36 = man_pal == "bb";
    let _cse_temp_37 = (ok) && (_cse_temp_36);
    ok = _cse_temp_37;
    let comp: String = compress_string("aabcccccaaa".to_string())?;
    let _cse_temp_38 = comp.len() as i32;
    let _cse_temp_39 = "aabcccccaaa".len() as i32;
    let _cse_temp_40 = _cse_temp_38 < _cse_temp_39;
    let _cse_temp_41 = (ok) && (_cse_temp_40);
    ok = _cse_temp_41;
    let _cse_temp_42 = (ok) && (wildcard_match(&"adceb", &"*a*b")?);
    ok = _cse_temp_42;
    ok = _cse_temp_19;
    let _cse_temp_43 = (ok) && (simple_regex_match(&"aab", &"c*a*b")?);
    ok = _cse_temp_43;
    ok = _cse_temp_19;
    let lcp: String = longest_common_prefix(&vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ])?;
    let _cse_temp_44 = lcp == "fl";
    let _cse_temp_45 = (ok) && (_cse_temp_44);
    ok = _cse_temp_45;
    let encrypted: String = caesar_cipher_encode(&STR_HELLO, 3)?;
    let _cse_temp_46 = encrypted == "khoor";
    let _cse_temp_47 = (ok) && (_cse_temp_46);
    ok = _cse_temp_47;
    let decrypted: String = caesar_cipher_decode(&encrypted, 3)?;
    let _cse_temp_48 = decrypted == STR_HELLO;
    let _cse_temp_49 = (ok) && (_cse_temp_48);
    ok = _cse_temp_49;
    let hd: i32 = hamming_distance(&"karolin", &"kathrin")?;
    let _cse_temp_50 = hd == 3;
    let _cse_temp_51 = (ok) && (_cse_temp_50);
    ok = _cse_temp_51;
    let lcs: String = longest_common_substring(&"abcdef", &"zbcdf")?;
    let _cse_temp_52 = lcs == "bcd";
    let _cse_temp_53 = (ok) && (_cse_temp_52);
    ok = _cse_temp_53;
    let _cse_temp_54 = (ok) && (is_subsequence(&"ace", &"abcde")?);
    ok = _cse_temp_54;
    ok = _cse_temp_19;
    let dc: i32 = count_distinct_substrings(&"abc");
    let _cse_temp_55 = dc == 6;
    let _cse_temp_56 = (ok) && (_cse_temp_55);
    ok = _cse_temp_56;
    let rsm: i32 = repeated_string_match(&"abcd", &"cdabcdab")?;
    let _cse_temp_57 = rsm == 3;
    let _cse_temp_58 = (ok) && (_cse_temp_57);
    ok = _cse_temp_58;
    let inter: String = interleave_strings(&"abc", &"xyz")?;
    let _cse_temp_59 = inter == "axbycz";
    let _cse_temp_60 = (ok) && (_cse_temp_59);
    ok = _cse_temp_60;
    let lrs: String = longest_repeating_substring(&"banana");
    let _cse_temp_61 = lrs == "ana";
    let _cse_temp_62 = (ok) && (_cse_temp_61);
    ok = _cse_temp_62;
    let h1: i32 = string_multiply_hash(&STR_HELLO)?;
    let h2: i32 = string_multiply_hash(&STR_HELLO)?;
    let _cse_temp_63 = h1 == h2;
    let _cse_temp_64 = (ok) && (_cse_temp_63);
    let _cse_temp_65 = h1 > 0;
    let _cse_temp_66 = (_cse_temp_64) && (_cse_temp_65);
    ok = _cse_temp_66;
    Ok(ok)
}
#[doc = r" DEPYLER-1216: Auto-generated entry point for standalone compilation"]
#[doc = r" This file was transpiled from a Python module without an explicit main."]
#[doc = r#" Add a main () function or `if __name__ == "__main__":` block to customize."#]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_group_anagrams_examples() {
        assert_eq!(group_anagrams(vec![]), vec![]);
        assert_eq!(group_anagrams(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_sort_string() {
        fn prop(s: String) -> TestResult {
            let result = sort_string((&*s).into());
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = s.clone();
            input_sorted.sort();
            let mut result = sort_string((&*s).into());
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
    #[test]
    fn test_str_to_int_examples() {
        assert_eq!(str_to_int(""), 0);
        assert_eq!(str_to_int("a"), 1);
        assert_eq!(str_to_int("abc"), 3);
    }
    #[test]
    fn test_count_distinct_substrings_examples() {
        assert_eq!(count_distinct_substrings(""), 0);
        assert_eq!(count_distinct_substrings("a"), 1);
        assert_eq!(count_distinct_substrings("abc"), 3);
    }
    #[test]
    fn test_string_multiply_hash_examples() {
        assert_eq!(string_multiply_hash(""), 0);
        assert_eq!(string_multiply_hash("a"), 1);
        assert_eq!(string_multiply_hash("abc"), 3);
    }
}