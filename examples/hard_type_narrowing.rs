#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::HashSet;
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
#[doc = "Return x if positive, else return 0 as default."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn clamp_positive(x: i32) -> i32 {
    let _cse_temp_0 = x > 0;
    if _cse_temp_0 {
        return x;
    } else {
        return 0;
    }
}
#[doc = "Test conditional return with positive and negative inputs."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_clamp_positive() -> i32 {
    let a: i32 = clamp_positive(42);
    let b: i32 = clamp_positive(-7);
    let c: i32 = clamp_positive(0);
    {
        let _r: i32 = ((a).py_add(b) as i32).py_add(c);
        _r
    }
}
#[doc = "Divide a by b, returning 0 when b is zero."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = b != 0;
    if _cse_temp_0 {
        return Ok({
            let a = a;
            let b = b;
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
        });
    } else {
        return Ok(0);
    }
}
#[doc = "Test division with zero and non-zero divisors."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_safe_divide() -> Result<i32, Box<dyn std::error::Error>> {
    let r1: i32 = safe_divide(100, 3)?;
    let r2: i32 = safe_divide(50, 0)?;
    let r3: i32 = safe_divide(-20, 4)?;
    Ok({
        let _r: i32 = ((r1).py_add(r2) as i32).py_add(r3);
        _r
    })
}
#[doc = "Map score to grade bucket via chained ternary expressions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn classify_score(score: i32) -> i32 {
    if score >= 90 {
        4
    } else {
        if score >= 80 {
            3
        } else {
            if score >= 70 {
                2
            } else {
                if score >= 60 {
                    1
                } else {
                    0
                }
            }
        }
    }
}
#[doc = "Test all five ternary branches."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_classify_score() -> i32 {
    let a: i32 = classify_score(95);
    let b: i32 = classify_score(85);
    let c: i32 = classify_score(75);
    let d: i32 = classify_score(65);
    let e: i32 = classify_score(50);
    {
        let _r: i32 = ((((a).py_add(b) as i32).py_add(c) as i32).py_add(d) as i32).py_add(e);
        _r
    }
}
#[doc = "Weighted average mixing int and float types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn weighted_average(a: i32, b: i32, w: f64) -> f64 {
    let _cse_temp_0 = (a).py_mul(w);
    let left: f64 = _cse_temp_0;
    let _cse_temp_1 = (b).py_mul((1.0) - (w));
    let right: f64 = _cse_temp_1;
    (left).py_add(right)
}
#[doc = "Test mixed int/float arithmetic returns expected value."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_weighted_average() -> i32 {
    let result: f64 = weighted_average(10, 20, 0.5);
    (result) as i32
}
#[doc = "Sum the digit values in a string of digits."]
#[doc = " Depyler: verified panic-free"]
pub fn parse_digit_sum(s: &str) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for ch in s.chars() {
        if ch.is_numeric() {
            total = ((total).py_add(ch.to_digit(10).unwrap_or(0) as i32)) as i32;
        }
    }
    total
}
#[doc = "Test digit parsing from mixed string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_parse_digit_sum() -> i32 {
    let r1: i32 = parse_digit_sum(&"12345");
    let r2: i32 = parse_digit_sum(&"a1b2c3");
    (r1).py_add(r2)
}
#[doc = "Check if lo<x<hi using chained comparison."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn in_range_exclusive(x: i32, lo: i32, hi: i32) -> bool {
    (lo < x) && (x < hi)
}
#[doc = "Count values strictly between lo and hi."]
#[doc = " Depyler: verified panic-free"]
pub fn count_in_band(values: &Vec<i32>, lo: i32, hi: i32) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    for v in values.iter().cloned() {
        if (lo < v) && (v < hi) {
            count = ((count).py_add(1i32)) as i32;
        }
    }
    count
}
#[doc = "Test chained comparison logic."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_chained_comparisons() -> i32 {
    let nums: Vec<i32> = vec![1, 5, 10, 15, 20, 25, 30];
    let c: i32 = count_in_band(&nums, 5, 25);
    let b1: i32 = if in_range_exclusive(10, 0, 20) { 1 } else { 0 };
    let b2: i32 = if in_range_exclusive(0, 0, 20) { 1 } else { 0 };
    {
        let _r: i32 = ((c).py_add(b1) as i32).py_add(b2);
        _r
    }
}
#[doc = "Return quotient and remainder as a two-element list."]
#[doc = " Depyler: proven to terminate"]
pub fn divmod_pair(a: i32, b: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = {
        let a = a;
        let b = b;
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
    let q: i32 = _cse_temp_0;
    let _cse_temp_1 = ((a).py_mod(b)) as i32;
    let r: i32 = _cse_temp_1;
    Ok(vec![q, r])
}
#[doc = "Test multiple return via list unpacking."]
#[doc = " Depyler: proven to terminate"]
pub fn test_divmod_pair() -> Result<i32, Box<dyn std::error::Error>> {
    let pair: Vec<i32> = divmod_pair(17, 5)?;
    let q: i32 = pair
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let r: i32 = pair
        .get(1usize)
        .cloned()
        .expect("IndexError: list index out of range");
    Ok({
        let _r: i32 = ((q).py_mul(10i32) as i32).py_add(r);
        _r
    })
}
#[doc = "Filter list to values in [lo, hi) using compound condition."]
#[doc = " Depyler: verified panic-free"]
pub fn filter_in_range(lst: &Vec<i32>, lo: i32, hi: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in lst.iter().cloned() {
        if (x >= lo) && (x < hi) {
            result.push(x);
        }
    }
    result
}
#[doc = "Test list filtering with compound conditions."]
#[doc = " Depyler: verified panic-free"]
pub fn test_filter_in_range() -> i32 {
    let mut total: i32 = Default::default();
    let data: Vec<i32> = vec![3, 7, 15, 22, 50, 75, 99, 100];
    let filtered: Vec<i32> = filter_in_range(&data, 10, 80);
    total = 0;
    for v in filtered.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Build dict mapping index to value for values above threshold."]
#[doc = " Depyler: proven to terminate"]
pub fn index_above_threshold(
    values: &Vec<i32>,
    threshold: i32,
) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let mut result: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    for i in 0..(values.len() as i32) {
        if values
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            > threshold
        {
            result.insert(
                i.clone(),
                values
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        }
    }
    Ok(result)
}
#[doc = "Test dict building with conditional filtering."]
pub fn test_index_above_threshold() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let data: Vec<i32> = vec![5, 12, 3, 18, 7, 25, 1];
    let d: std::collections::HashMap<i32, i32> = index_above_threshold(&data, 10)?;
    total = 0;
    for k in d.keys().cloned() {
        total = ((total).py_add(d.get(&(k)).cloned().unwrap_or_default())) as i32;
    }
    Ok(total)
}
#[doc = "Compute |(a union b) intersect c| using set operations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn set_algebra<'c, 'b, 'a>(a: &'a Vec<i32>, b: &'b Vec<i32>, c: &'c Vec<i32>) -> i32 {
    let sa: std::collections::HashSet<i32> =
        a.iter().cloned().collect::<std::collections::HashSet<_>>();
    let sb: std::collections::HashSet<i32> =
        b.iter().cloned().collect::<std::collections::HashSet<_>>();
    let sc: std::collections::HashSet<i32> =
        c.iter().cloned().collect::<std::collections::HashSet<_>>();
    let _cse_temp_0 = sa
        .union(&sb)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let union_ab: std::collections::HashSet<i32> = _cse_temp_0.clone();
    let _cse_temp_1 = union_ab
        .intersection(&sc)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let result: std::collections::HashSet<i32> = _cse_temp_1.clone();
    result.len() as i32 as i32
}
#[doc = "Test set union, intersection chain."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_algebra() -> i32 {
    let a: Vec<i32> = vec![1, 2, 3, 4, 5];
    let b: Vec<i32> = vec![4, 5, 6, 7, 8];
    let c: Vec<i32> = vec![2, 4, 6, 8, 10];
    set_algebra(&a, &b, &c)
}
#[doc = "Repeat a string n times."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn repeat_string(s: &str, n: i32) -> String {
    (s).py_mul(n)
}
#[doc = "Build a separator line of given width."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn build_separator(char: &str, width: i32) -> String {
    (char).py_mul(width)
}
#[doc = "Test string repetition patterns."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_string_multiplication() -> i32 {
    let r1: String = repeat_string(&"ab", 3);
    let r2: String = build_separator(&"-", 10);
    (r1.len() as i32).py_add(r2.len() as i32) as i32
}
#[doc = "Compare floor division and integer division results."]
#[doc = " Depyler: proven to terminate"]
pub fn floor_vs_true(a: i32, b: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = {
        let a = a;
        let b = b;
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
    let floor_result: i32 = _cse_temp_0;
    let _cse_temp_1 = ((a).py_mod(b)) as i32;
    let remainder: i32 = _cse_temp_1;
    let _cse_temp_2 = ((floor_result).py_mul(b)) as i32;
    let reconstructed: i32 = ((_cse_temp_2).py_add(remainder)) as i32;
    Ok(vec![floor_result, remainder, reconstructed])
}
#[doc = "Test floor division with positive and negative operands."]
#[doc = " Depyler: proven to terminate"]
pub fn test_floor_division() -> Result<i32, Box<dyn std::error::Error>> {
    let pos: Vec<i32> = floor_vs_true(17, 5)?;
    let neg: Vec<i32> = floor_vs_true(-17, 5)?;
    Ok({
        let _r: i32 = (((pos
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range"))
        .py_add(
            pos.get(1usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ) as i32)
            .py_add(
                neg.get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ) as i32)
            .py_add(
                neg.get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        _r
    })
}
#[doc = "Compute base raised to exp using the power operator."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn power_chain(base: i32, exp: i32) -> i32 {
    {
        if exp >= 0 && (exp as i64) <= (u32::MAX as i64) {
            ({ base } as i32)
                .checked_pow({ exp } as u32)
                .expect("Power operation overflowed")
        } else {
            ({ base } as f64).powf({ exp } as f64) as i32
        }
    }
}
#[doc = "Sum of squares from 1 to n using power operator."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn sum_of_squares(n: i32) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for i in (1)..((n).py_add(1i32)) {
        total = ((total).py_add({
            if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
                ({ i } as i32)
                    .checked_pow({ 2 } as u32)
                    .expect("Power operation overflowed")
            } else {
                ({ i } as f64).powf({ 2 } as f64) as i32
            }
        })) as i32;
    }
    total
}
#[doc = "Test power operator with various combinations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_power_operator() -> i32 {
    let a: i32 = power_chain(2, 10);
    let b: i32 = power_chain(3, 4);
    let c: i32 = sum_of_squares(5);
    {
        let _r: i32 = ((a).py_add(b) as i32).py_add(c);
        _r
    }
}
#[doc = "Extract bits from x using AND mask."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn extract_bits(x: i32, mask: i32) -> i32 {
    x & mask
}
#[doc = "Set flag bits in x using OR."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn set_flags(x: i32, flag: i32) -> i32 {
    x | flag
}
#[doc = "Toggle bits in x using XOR."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn toggle_bits(x: i32, toggle: i32) -> i32 {
    x ^ toggle
}
#[doc = "Pack two values by shifting high left and ORing with low."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn shift_pack(high: i32, low: i32, bits: i32) -> i32 {
    high << bits | low
}
#[doc = "Unpack the high portion by shifting right."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn shift_unpack_high(packed: i32, bits: i32) -> i32 {
    packed >> bits
}
#[doc = "Test AND, OR, XOR, left shift, right shift."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bitwise_operations() -> i32 {
    let a: i32 = extract_bits(255, 15);
    let b: i32 = set_flags(240, 15);
    let c: i32 = toggle_bits(255, 15);
    let packed: i32 = shift_pack(5, 3, 4);
    let high: i32 = shift_unpack_high(packed, 4);
    {
        let _r: i32 =
            ((((a).py_add(b) as i32).py_add(c) as i32).py_add(packed) as i32).py_add(high);
        _r
    }
}
#[doc = "Return True if x is above threshold."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn threshold_check(x: i32) -> bool {
    x > 50
}
#[doc = "Return True if x is even."]
#[doc = " Depyler: proven to terminate"]
pub fn parity_check(x: i32) -> Result<bool, Box<dyn std::error::Error>> {
    Ok((x).py_mod(2i32) == 0)
}
#[doc = "Return True if x is in [10, 100]."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn range_check(x: i32) -> bool {
    (x >= 10) && (x <= 100)
}
#[doc = "Evaluate compound boolean with short-circuit semantics.\n\n    Returns 1 if(threshold_check(a) and parity_check(b)) or range_check(c),\n    otherwise returns 0.\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn compound_logic(a: i32, b: i32, c: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = (threshold_check(a)) && (parity_check(b)?);
    if _cse_temp_0 {
        return Ok(1);
    }
    if range_check(c) {
        return Ok(1);
    }
    Ok(0)
}
#[doc = "Test complex boolean short-circuit evaluation paths."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_boolean_short_circuit() -> Result<i32, Box<dyn std::error::Error>> {
    let r1: i32 = compound_logic(60, 4, 200)?;
    let r2: i32 = compound_logic(30, 4, 50)?;
    let r3: i32 = compound_logic(30, 3, 5)?;
    let r4: i32 = compound_logic(80, 7, 200)?;
    Ok({
        let _r: i32 = (((r1).py_add(r2) as i32).py_add(r3) as i32).py_add(r4);
        _r
    })
}
#[doc = "Run all test functions and return sum of results."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_all_tests() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = 0;
    let _cse_temp_0 = ((total).py_add(test_clamp_positive())) as i32;
    total = _cse_temp_0;
    let _cse_temp_1 = ((total).py_add(test_safe_divide()?)) as i32;
    total = _cse_temp_1;
    let _cse_temp_2 = ((total).py_add(test_classify_score())) as i32;
    total = _cse_temp_2;
    let _cse_temp_3 = ((total).py_add(test_weighted_average())) as i32;
    total = _cse_temp_3;
    let _cse_temp_4 = ((total).py_add(test_parse_digit_sum())) as i32;
    total = _cse_temp_4;
    let _cse_temp_5 = ((total).py_add(test_chained_comparisons())) as i32;
    total = _cse_temp_5;
    let _cse_temp_6 = ((total).py_add(test_divmod_pair()?)) as i32;
    total = _cse_temp_6;
    let _cse_temp_7 = ((total).py_add(test_filter_in_range())) as i32;
    total = _cse_temp_7;
    let _cse_temp_8 = ((total).py_add(test_index_above_threshold()?)) as i32;
    total = _cse_temp_8;
    let _cse_temp_9 = ((total).py_add(test_set_algebra())) as i32;
    total = _cse_temp_9;
    let _cse_temp_10 = ((total).py_add(test_string_multiplication())) as i32;
    total = _cse_temp_10;
    let _cse_temp_11 = ((total).py_add(test_floor_division()?)) as i32;
    total = _cse_temp_11;
    let _cse_temp_12 = ((total).py_add(test_power_operator())) as i32;
    total = _cse_temp_12;
    let _cse_temp_13 = ((total).py_add(test_bitwise_operations())) as i32;
    total = _cse_temp_13;
    let _cse_temp_14 = ((total).py_add(test_boolean_short_circuit()?)) as i32;
    total = _cse_temp_14;
    Ok(total)
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
    fn test_clamp_positive_examples() {
        assert_eq!(clamp_positive(0), 0);
        assert_eq!(clamp_positive(1), 1);
        assert_eq!(clamp_positive(-1), -1);
    }
    #[test]
    fn test_test_clamp_positive_examples() {
        let _ = test_clamp_positive();
    }
    #[test]
    fn test_safe_divide_examples() {
        assert_eq!(safe_divide(0, 0), 0);
        assert_eq!(safe_divide(1, 2), 3);
        assert_eq!(safe_divide(-1, 1), 0);
    }
    #[test]
    fn test_test_safe_divide_examples() {
        let _ = test_safe_divide();
    }
    #[test]
    fn test_classify_score_examples() {
        assert_eq!(classify_score(0), 0);
        assert_eq!(classify_score(1), 1);
        assert_eq!(classify_score(-1), -1);
    }
    #[test]
    fn test_test_classify_score_examples() {
        let _ = test_classify_score();
    }
    #[test]
    fn test_test_weighted_average_examples() {
        let _ = test_weighted_average();
    }
    #[test]
    fn test_parse_digit_sum_examples() {
        assert_eq!(parse_digit_sum(""), 0);
        assert_eq!(parse_digit_sum("a"), 1);
        assert_eq!(parse_digit_sum("abc"), 3);
    }
    #[test]
    fn test_test_parse_digit_sum_examples() {
        let _ = test_parse_digit_sum();
    }
    #[test]
    fn test_test_chained_comparisons_examples() {
        let _ = test_chained_comparisons();
    }
    #[test]
    fn test_test_divmod_pair_examples() {
        let _ = test_divmod_pair();
    }
    #[test]
    fn test_test_filter_in_range_examples() {
        let _ = test_filter_in_range();
    }
    #[test]
    fn test_test_index_above_threshold_examples() {
        let _ = test_index_above_threshold();
    }
    #[test]
    fn test_test_set_algebra_examples() {
        let _ = test_set_algebra();
    }
    #[test]
    fn quickcheck_repeat_string() {
        fn prop(s: String, n: i32) -> TestResult {
            let result1 = repeat_string((&*s).into(), n.clone());
            let result2 = repeat_string(n.clone(), (&*s).into());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(String, i32) -> TestResult);
    }
    #[test]
    fn quickcheck_build_separator() {
        fn prop(char: String, width: i32) -> TestResult {
            let result1 = build_separator((&*char).into(), width.clone());
            let result2 = build_separator(width.clone(), (&*char).into());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(String, i32) -> TestResult);
    }
    #[test]
    fn test_test_string_multiplication_examples() {
        let _ = test_string_multiplication();
    }
    #[test]
    fn test_test_floor_division_examples() {
        let _ = test_floor_division();
    }
    #[test]
    fn test_power_chain_examples() {
        assert_eq!(power_chain(0, 0), 0);
        assert_eq!(power_chain(1, 2), 3);
        assert_eq!(power_chain(-1, 1), 0);
    }
    #[test]
    fn test_sum_of_squares_examples() {
        assert_eq!(sum_of_squares(0), 0);
        assert_eq!(sum_of_squares(1), 1);
        assert_eq!(sum_of_squares(-1), -1);
    }
    #[test]
    fn test_test_power_operator_examples() {
        let _ = test_power_operator();
    }
    #[test]
    fn quickcheck_extract_bits() {
        fn prop(x: i32, mask: i32) -> TestResult {
            if (x > 0 && mask > i32::MAX - x) || (x < 0 && mask < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = extract_bits(x.clone(), mask.clone());
            let result2 = extract_bits(mask.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_extract_bits_examples() {
        assert_eq!(extract_bits(0, 0), 0);
        assert_eq!(extract_bits(1, 2), 3);
        assert_eq!(extract_bits(-1, 1), 0);
    }
    #[test]
    fn quickcheck_set_flags() {
        fn prop(x: i32, flag: i32) -> TestResult {
            if (x > 0 && flag > i32::MAX - x) || (x < 0 && flag < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = set_flags(x.clone(), flag.clone());
            let result2 = set_flags(flag.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_set_flags_examples() {
        assert_eq!(set_flags(0, 0), 0);
        assert_eq!(set_flags(1, 2), 3);
        assert_eq!(set_flags(-1, 1), 0);
    }
    #[test]
    fn quickcheck_toggle_bits() {
        fn prop(x: i32, toggle: i32) -> TestResult {
            if (x > 0 && toggle > i32::MAX - x) || (x < 0 && toggle < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = toggle_bits(x.clone(), toggle.clone());
            let result2 = toggle_bits(toggle.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_toggle_bits_examples() {
        assert_eq!(toggle_bits(0, 0), 0);
        assert_eq!(toggle_bits(1, 2), 3);
        assert_eq!(toggle_bits(-1, 1), 0);
    }
    #[test]
    fn test_shift_unpack_high_examples() {
        assert_eq!(shift_unpack_high(0, 0), 0);
        assert_eq!(shift_unpack_high(1, 2), 3);
        assert_eq!(shift_unpack_high(-1, 1), 0);
    }
    #[test]
    fn test_test_bitwise_operations_examples() {
        let _ = test_bitwise_operations();
    }
    #[test]
    fn test_threshold_check_examples() {
        let _ = threshold_check(Default::default());
    }
    #[test]
    fn test_parity_check_examples() {
        let _ = parity_check(Default::default());
    }
    #[test]
    fn test_range_check_examples() {
        let _ = range_check(Default::default());
    }
    #[test]
    fn test_test_boolean_short_circuit_examples() {
        let _ = test_boolean_short_circuit();
    }
    #[test]
    fn test_run_all_tests_examples() {
        let _ = run_all_tests();
    }
}