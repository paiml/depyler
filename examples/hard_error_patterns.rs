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
#[doc = "Division with zero guard returning sentinel -1."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(-1);
    }
    Ok({
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
    })
}
#[doc = "Division with caller-supplied default on zero divisor."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide_with_default(
    a: i32,
    b: i32,
    default: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(default);
    }
    Ok({
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
    })
}
#[doc = "Chained division: a / b / c with guards at each step."]
#[doc = " Depyler: proven to terminate"]
pub fn chained_division(a: i32, b: i32, c: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(-1);
    }
    let _cse_temp_1 = {
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
    let intermediate: i32 = _cse_temp_1;
    let _cse_temp_2 = c == 0;
    if _cse_temp_2 {
        return Ok(-1);
    }
    Ok({
        let a = intermediate;
        let b = c;
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
    })
}
#[doc = "Simulate wrapping addition within [0, max_val]."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn wrapping_add(a: i32, b: i32, max_val: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = ((a).py_add(b)) as i32;
    let _cse_temp_0 = result > max_val;
    if _cse_temp_0 {
        let _cse_temp_1 = (((result) - (max_val) as i32) - (1i32)) as i32;
        result = _cse_temp_1;
    }
    let _cse_temp_2 = result < 0;
    if _cse_temp_2 {
        let _cse_temp_3 = (((result).py_add(max_val) as i32).py_add(1i32)) as i32;
        result = _cse_temp_3;
    }
    result
}
#[doc = "Saturating addition: clamp at max_val."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn saturating_add(a: i32, b: i32, max_val: i32) -> i32 {
    let result: i32 = ((a).py_add(b)) as i32;
    let _cse_temp_0 = result > max_val;
    if _cse_temp_0 {
        return max_val;
    }
    let _cse_temp_1 = result < 0;
    if _cse_temp_1 {
        return 0;
    }
    result
}
#[doc = "Saturating subtraction: floor at zero."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn saturating_sub(a: i32, b: i32) -> i32 {
    let result: i32 = ((a) - (b)) as i32;
    let _cse_temp_0 = result < 0;
    if _cse_temp_0 {
        return 0;
    }
    result
}
#[doc = "Saturating multiplication clamped to max_val."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn saturating_mul(a: i32, b: i32, max_val: i32) -> i32 {
    let _cse_temp_0 = ((a).py_mul(b)) as i32;
    let result: i32 = _cse_temp_0;
    let _cse_temp_1 = result > max_val;
    if _cse_temp_1 {
        return max_val;
    }
    let _cse_temp_2 = result < 0;
    if _cse_temp_2 {
        return 0;
    }
    result
}
#[doc = "Clamp value to [lo, hi] range."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn clamp(value: i32, lo: i32, hi: i32) -> i32 {
    let _cse_temp_0 = value < lo;
    if _cse_temp_0 {
        return lo;
    }
    let _cse_temp_1 = value > hi;
    if _cse_temp_1 {
        return hi;
    }
    value
}
#[doc = "Absolute difference without overflow risk."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn abs_diff(a: i32, b: i32) -> i32 {
    let _cse_temp_0 = a > b;
    if _cse_temp_0 {
        return (a) - (b);
    }
    (b) - (a)
}
#[doc = "Overflow-safe midpoint calculation."]
#[doc = " Depyler: proven to terminate"]
pub fn midpoint_safe(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = a > b;
    if _cse_temp_0 {
        return Ok((b).py_add({
            let a = (a) - (b);
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
        }));
    }
    Ok((a).py_add({
        let a = (b) - (a);
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
    }))
}
#[doc = "Classify value: 0=zero, 1=min boundary, 2=max boundary, 3=interior."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn boundary_classify(value: i32, max_val: i32) -> i32 {
    let _cse_temp_0 = value == 0;
    if _cse_temp_0 {
        return 0;
    }
    let _cse_temp_1 = value == 1;
    if _cse_temp_1 {
        return 1;
    }
    let _cse_temp_2 = value == max_val;
    if _cse_temp_2 {
        return 2;
    }
    3
}
#[doc = "Factorial with depth limit, returns -1 on exceeded depth."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn limited_factorial(n: i32, max_depth: i32) -> i32 {
    let _cse_temp_0 = max_depth <= 0;
    if _cse_temp_0 {
        return -1;
    }
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
        return 1;
    }
    let sub: i32 = limited_factorial((n) - (1i32), (max_depth) - (1i32));
    let _cse_temp_2 = sub == -1;
    if _cse_temp_2 {
        return -1;
    }
    (n).py_mul(sub)
}
#[doc = "Fibonacci with depth limit."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn limited_fibonacci(n: i32, max_depth: i32) -> i32 {
    let _cse_temp_0 = max_depth <= 0;
    if _cse_temp_0 {
        return -1;
    }
    let _cse_temp_1 = n <= 0;
    if _cse_temp_1 {
        return 0;
    }
    let _cse_temp_2 = n == 1;
    if _cse_temp_2 {
        return 1;
    }
    let a: i32 = limited_fibonacci((n) - (1i32), (max_depth) - (1i32));
    let _cse_temp_3 = a == -1;
    if _cse_temp_3 {
        return -1;
    }
    let b: i32 = limited_fibonacci((n) - (2i32), (max_depth) - (1i32));
    let _cse_temp_4 = b == -1;
    if _cse_temp_4 {
        return -1;
    }
    (a).py_add(b)
}
#[doc = "Iterative power with result limit. Returns -1 if exceeded."]
#[doc = " Depyler: verified panic-free"]
pub fn bounded_power(base: i32, exp: i32, limit: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = 1;
    let mut i: i32 = 0;
    while i < exp {
        result = ((result).py_mul(base)) as i32;
        if result > limit {
            return -1;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Return first element or -1 for empty list."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_first(lst: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(-1);
    }
    Ok(lst
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Return last element or -1 for empty list."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_last(lst: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(-1);
    }
    Ok({
        let base = &lst;
        let idx: i32 = (lst.len() as i32) - (1i32);
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx)
            .cloned()
            .expect("IndexError: list index out of range")
    })
}
#[doc = "Sum with empty list guard."]
#[doc = " Depyler: verified panic-free"]
pub fn safe_sum(lst: &Vec<i32>) -> i32 {
    let mut total: i32 = Default::default();
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return 0;
    }
    total = 0;
    for x in lst.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Max with empty list guard, returns sentinel -999999."]
pub fn safe_max(lst: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut best: i32 = Default::default();
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(-999999);
    }
    best = lst
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for x in lst.iter().cloned() {
        if x > best {
            best = x;
        }
    }
    Ok(best)
}
#[doc = "Min with empty list guard, returns sentinel 999999."]
pub fn safe_min(lst: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut best: i32 = Default::default();
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(999999);
    }
    best = lst
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for x in lst.iter().cloned() {
        if x < best {
            best = x;
        }
    }
    Ok(best)
}
#[doc = "Integer average with empty guard."]
pub fn safe_average_int(lst: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    total = 0;
    for x in lst.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    Ok({
        let a = total;
        let b = lst.len() as i32;
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
    } as i32)
}
#[doc = "Bounds-checked index access, returns -1 on out of bounds."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_index(lst: &Vec<i32>, idx: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = idx < 0;
    if _cse_temp_0 {
        return Ok(-1);
    }
    let _cse_temp_1 = lst.len() as i32;
    let _cse_temp_2 = idx >= _cse_temp_1;
    if _cse_temp_2 {
        return Ok(-1);
    }
    Ok(lst
        .get(idx as usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Returns 1 if set succeeded, 0 if out of bounds."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn safe_set(lst: &mut Vec<i32>, idx: i32, val: i32) -> i32 {
    let _cse_temp_0 = idx < 0;
    if _cse_temp_0 {
        return 0;
    }
    let _cse_temp_1 = lst.len() as i32;
    let _cse_temp_2 = idx >= _cse_temp_1;
    if _cse_temp_2 {
        return 0;
    }
    lst[(idx) as usize] = val;
    1
}
#[doc = "Linear search returning index or default."]
pub fn find_or_default(
    lst: &Vec<i32>,
    target: i32,
    default: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut i: i32 = 0;
    while i < lst.len() as i32 {
        if lst
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            == target
        {
            return Ok(i);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(default)
}
#[doc = "If either input is sentinel -1, propagate it."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn propagate_sentinel(a: i32, b: i32) -> i32 {
    let _cse_temp_0 = a == -1;
    if _cse_temp_0 {
        return -1;
    }
    let _cse_temp_1 = b == -1;
    if _cse_temp_1 {
        return -1;
    }
    (a).py_add(b)
}
#[doc = "Chain operations, propagating -1 sentinel at each step."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn chain_sentinel_ops(a: i32, b: i32, c: i32) -> i32 {
    let step1: i32 = propagate_sentinel(a, b);
    let _cse_temp_0 = step1 == -1;
    if _cse_temp_0 {
        return -1;
    }
    let step2: i32 = propagate_sentinel(step1, c);
    step2
}
#[doc = "Sum list but propagate sentinel -1 from any element."]
#[doc = " Depyler: verified panic-free"]
pub fn sentinel_map(lst: &Vec<i32>) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for x in lst.iter().cloned() {
        if x == -1 {
            return -1;
        }
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "0=invalid, 1=equilateral, 2=isosceles, 3=scalene."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn classify_triangle(a: i32, b: i32, c: i32) -> i32 {
    let _cse_temp_0 = a <= 0;
    let _cse_temp_1 = b <= 0;
    let _cse_temp_2 = (_cse_temp_0) || (_cse_temp_1);
    let _cse_temp_3 = c <= 0;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return 0;
    }
    let _cse_temp_5 = (a).py_add(b) <= c;
    let _cse_temp_6 = (a).py_add(c) <= b;
    let _cse_temp_7 = (_cse_temp_5) || (_cse_temp_6);
    let _cse_temp_8 = (b).py_add(c) <= a;
    let _cse_temp_9 = (_cse_temp_7) || (_cse_temp_8);
    if _cse_temp_9 {
        return 0;
    }
    let _cse_temp_10 = a == b;
    let _cse_temp_11 = b == c;
    let _cse_temp_12 = (_cse_temp_10) && (_cse_temp_11);
    if _cse_temp_12 {
        return 1;
    }
    let _cse_temp_13 = (_cse_temp_10) || (_cse_temp_11);
    let _cse_temp_14 = a == c;
    let _cse_temp_15 = (_cse_temp_13) || (_cse_temp_14);
    if _cse_temp_15 {
        return 2;
    }
    3
}
#[doc = "Classify into ranges: 0=negative, 1=[0,10), 2=[10,100), 3=[100,1000), 4=large."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn multi_range_classify(x: i32) -> i32 {
    let _cse_temp_0 = x < 0;
    if _cse_temp_0 {
        return 0;
    }
    let _cse_temp_1 = x < 10;
    if _cse_temp_1 {
        return 1;
    }
    let _cse_temp_2 = x < 100;
    if _cse_temp_2 {
        return 2;
    }
    let _cse_temp_3 = x < 1000;
    if _cse_temp_3 {
        return 3;
    }
    4
}
#[doc = "Map score to grade: 5=A, 4=B, 3=C, 2=D, 1=F, 0=invalid."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn grade_score(score: i32) -> i32 {
    let _cse_temp_0 = score < 0;
    let _cse_temp_1 = score > 100;
    let _cse_temp_2 = (_cse_temp_0) || (_cse_temp_1);
    if _cse_temp_2 {
        return 0;
    }
    let _cse_temp_3 = score >= 90;
    if _cse_temp_3 {
        return 5;
    }
    let _cse_temp_4 = score >= 80;
    if _cse_temp_4 {
        return 4;
    }
    let _cse_temp_5 = score >= 70;
    if _cse_temp_5 {
        return 3;
    }
    let _cse_temp_6 = score >= 60;
    if _cse_temp_6 {
        return 2;
    }
    1
}
#[doc = "Count how many elements are negative(error indicators)."]
#[doc = " Depyler: verified panic-free"]
pub fn count_errors(lst: &Vec<i32>) -> i32 {
    let mut errors: i32 = Default::default();
    errors = 0;
    for x in lst.iter().cloned() {
        if x < 0 {
            errors = ((errors).py_add(1i32)) as i32;
        }
    }
    errors
}
#[doc = "Count elements within valid range [lo, hi]."]
#[doc = " Depyler: verified panic-free"]
pub fn count_valid(lst: &Vec<i32>, lo: i32, hi: i32) -> i32 {
    let mut valid: i32 = Default::default();
    valid = 0;
    for x in lst.iter().cloned() {
        if (x >= lo) && (x <= hi) {
            valid = ((valid).py_add(1i32)) as i32;
        }
    }
    valid
}
#[doc = "Sum only non-negative elements, skip errors."]
#[doc = " Depyler: verified panic-free"]
pub fn sum_valid_only(lst: &Vec<i32>) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for x in lst.iter().cloned() {
        if x >= 0 {
            total = ((total).py_add(x)) as i32;
        }
    }
    total
}
#[doc = "Return index of first negative element, or -1 if none."]
pub fn first_error_index(lst: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut i: i32 = 0;
    while i < lst.len() as i32 {
        if lst
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            < 0
        {
            return Ok(i);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(-1)
}
#[doc = "Try a/b, then a/c, then a/d, then return 0."]
#[doc = " Depyler: proven to terminate"]
pub fn fallback_divide(a: i32, b: i32, c: i32, d: i32) -> Result<i32, Box<dyn std::error::Error>> {
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
    }
    let _cse_temp_1 = c != 0;
    if _cse_temp_1 {
        return Ok({
            let a = a;
            let b = c;
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
    }
    let _cse_temp_2 = d != 0;
    if _cse_temp_2 {
        return Ok({
            let a = a;
            let b = d;
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
    }
    Ok(0)
}
#[doc = "Try index i1, then i2, then i3, then default."]
#[doc = " Depyler: proven to terminate"]
pub fn fallback_lookup(
    lst: &Vec<i32>,
    i1: i32,
    i2: i32,
    i3: i32,
    default: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = i1 >= 0;
    let _cse_temp_1 = lst.len() as i32;
    let _cse_temp_2 = i1 < _cse_temp_1;
    let _cse_temp_3 = (_cse_temp_0) && (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(lst
            .get(i1 as usize)
            .cloned()
            .expect("IndexError: list index out of range"));
    }
    let _cse_temp_4 = i2 >= 0;
    let _cse_temp_5 = i2 < _cse_temp_1;
    let _cse_temp_6 = (_cse_temp_4) && (_cse_temp_5);
    if _cse_temp_6 {
        return Ok(lst
            .get(i2 as usize)
            .cloned()
            .expect("IndexError: list index out of range"));
    }
    let _cse_temp_7 = i3 >= 0;
    let _cse_temp_8 = i3 < _cse_temp_1;
    let _cse_temp_9 = (_cse_temp_7) && (_cse_temp_8);
    if _cse_temp_9 {
        return Ok(lst
            .get(i3 as usize)
            .cloned()
            .expect("IndexError: list index out of range"));
    }
    Ok(default)
}
#[doc = "Return first non-sentinel value, or sentinel if all are sentinel."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn coalesce(a: i32, b: i32, c: i32, sentinel: i32) -> i32 {
    let _cse_temp_0 = a != sentinel;
    if _cse_temp_0 {
        return a;
    }
    let _cse_temp_1 = b != sentinel;
    if _cse_temp_1 {
        return b;
    }
    let _cse_temp_2 = c != sentinel;
    if _cse_temp_2 {
        return c;
    }
    sentinel
}
#[doc = "0=valid, 1=negative, 2=too_large, 3=zero."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_age(age: i32) -> i32 {
    let _cse_temp_0 = age < 0;
    if _cse_temp_0 {
        return 1;
    }
    let _cse_temp_1 = age == 0;
    if _cse_temp_1 {
        return 3;
    }
    let _cse_temp_2 = age > 150;
    if _cse_temp_2 {
        return 2;
    }
    0
}
#[doc = "0=valid, 1=below, 2=above."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_range(val: i32, lo: i32, hi: i32) -> i32 {
    let _cse_temp_0 = val < lo;
    if _cse_temp_0 {
        return 1;
    }
    let _cse_temp_1 = val > hi;
    if _cse_temp_1 {
        return 2;
    }
    0
}
#[doc = "Validate pair: 0=ok, 1=a_negative, 2=b_negative, 3=both_negative, 4=a_equals_b."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_pair(a: i32, b: i32) -> i32 {
    let _cse_temp_0 = a < 0;
    let _cse_temp_1 = b < 0;
    let _cse_temp_2 = (_cse_temp_0) && (_cse_temp_1);
    if _cse_temp_2 {
        return 3;
    }
    if _cse_temp_0 {
        return 1;
    }
    if _cse_temp_1 {
        return 2;
    }
    let _cse_temp_3 = a == b;
    if _cse_temp_3 {
        return 4;
    }
    0
}
#[doc = "Run multiple validations, return first failure code or 0."]
#[doc = " Depyler: proven to terminate"]
pub fn validation_pipeline(val: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = val < 0;
    if _cse_temp_0 {
        return Ok(1);
    }
    let _cse_temp_1 = val > 10000;
    if _cse_temp_1 {
        return Ok(2);
    }
    let _cse_temp_2 = ((val).py_mod(2i32)) as i32;
    let _cse_temp_3 = _cse_temp_2 != 0;
    if _cse_temp_3 {
        return Ok(3);
    }
    let _cse_temp_4 = val == 0;
    if _cse_temp_4 {
        return Ok(4);
    }
    Ok(0)
}
#[doc = "Sum elements after clamping each to [lo, hi]."]
#[doc = " Depyler: verified panic-free"]
pub fn sum_of_clamped(lst: &Vec<i32>, lo: i32, hi: i32) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for x in lst.iter().cloned() {
        let clamped: i32 = clamp(x, lo, hi);
        total = ((total).py_add(clamped)) as i32;
    }
    total
}
#[doc = "Count elements within [center - width, center + width]."]
#[doc = " Depyler: verified panic-free"]
pub fn count_in_band(lst: &Vec<i32>, center: i32, width: i32) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    let lo: i32 = ((center) - (width)) as i32;
    let hi: i32 = ((center).py_add(width)) as i32;
    for x in lst.iter().cloned() {
        if (x >= lo) && (x <= hi) {
            count = ((count).py_add(1i32)) as i32;
        }
    }
    count
}
#[doc = "Increment from start by step until positive. Return value or -1."]
#[doc = " Depyler: verified panic-free"]
pub fn retry_until_positive(start: i32, step: i32, max_tries: i32) -> i32 {
    let mut current: i32 = start.clone();
    let mut tries: i32 = 0;
    while tries < max_tries {
        if current > 0 {
            return current;
        }
        current = ((current).py_add(step)) as i32;
        tries = ((tries).py_add(1i32)) as i32;
    }
    -1
}
#[doc = "Decrement from start toward target. Return steps taken or -1."]
#[doc = " Depyler: verified panic-free"]
pub fn countdown_to_target(start: i32, target: i32, max_steps: i32) -> i32 {
    let mut current: i32 = start.clone();
    let mut steps: i32 = 0;
    while steps < max_steps {
        if current <= target {
            return steps;
        }
        current = ((current) - (1i32)) as i32;
        steps = ((steps).py_add(1i32)) as i32;
    }
    -1
}
#[doc = "Halve value toward zero. Return iterations or -1."]
pub fn converge_to_zero(value: i32, max_iters: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut current: i32 = Default::default();
    let mut iters: i32 = Default::default();
    current = value;
    let _cse_temp_0 = current < 0;
    if _cse_temp_0 {
        current = -current;
    }
    iters = 0;
    while iters < max_iters {
        if current == 0 {
            return Ok(iters);
        }
        current = {
            let a = current;
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
        iters = ((iters).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = current == 0;
    if _cse_temp_1 {
        return Ok(iters);
    }
    Ok(-1)
}
#[doc = "Simple state machine: returns next state or -1 for invalid."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn state_machine_step(state: i32, input_val: i32) -> i32 {
    let _cse_temp_0 = state == 0;
    if _cse_temp_0 {
        let _cse_temp_1 = input_val > 0;
        if _cse_temp_1 {
            return 1;
        }
        return 0;
    }
    let _cse_temp_2 = state == 1;
    if _cse_temp_2 {
        let _cse_temp_3 = input_val > 10;
        if _cse_temp_3 {
            return 2;
        }
        let _cse_temp_4 = input_val < 0;
        if _cse_temp_4 {
            return 0;
        }
        return 1;
    }
    let _cse_temp_5 = state == 2;
    if _cse_temp_5 {
        let _cse_temp_6 = input_val == 0;
        if _cse_temp_6 {
            return 0;
        }
        return 2;
    }
    -1
}
#[doc = "Run state machine over input list, return final state."]
#[doc = " Depyler: verified panic-free"]
pub fn run_state_machine(inputs: &Vec<i32>) -> i32 {
    let mut state: i32 = Default::default();
    state = 0;
    for inp in inputs.iter().cloned() {
        state = state_machine_step(state, inp);
        if state == -1 {
            return -1;
        }
    }
    state
}
#[doc = "0=valid ascending, 1=has_duplicate, 2=has_descent, 3=empty."]
pub fn validate_sequence(lst: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(3);
    }
    let mut i: i32 = 1;
    while i < lst.len() as i32 {
        if lst
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            == {
                let base = &lst;
                let idx: i32 = (i) - (1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }
        {
            return Ok(1);
        }
        if lst
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            < {
                let base = &lst;
                let idx: i32 = (i) - (1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }
        {
            return Ok(2);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(0)
}
#[doc = "Multiple guard clauses before computation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn guarded_compute(a: i32, b: i32, c: i32) -> i32 {
    let _cse_temp_0 = a == 0;
    if _cse_temp_0 {
        return 0;
    }
    let _cse_temp_1 = b == 0;
    if _cse_temp_1 {
        return a;
    }
    let _cse_temp_2 = c == 0;
    if _cse_temp_2 {
        return (a).py_add(b);
    }
    let _cse_temp_3 = a < 0;
    if _cse_temp_3 {
        return -1;
    }
    let _cse_temp_4 = b < 0;
    if _cse_temp_4 {
        return -2;
    }
    let _cse_temp_5 = c < 0;
    if _cse_temp_5 {
        return -3;
    }
    {
        let _r: i32 = ((a).py_mul(b) as i32).py_add(c);
        _r
    }
}
#[doc = "Four-parameter guarded computation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn deeply_guarded(x: i32, y: i32, z: i32, w: i32) -> i32 {
    let _cse_temp_0 = x <= 0;
    if _cse_temp_0 {
        return 0;
    }
    let _cse_temp_1 = y <= 0;
    if _cse_temp_1 {
        return x;
    }
    let _cse_temp_2 = z <= 0;
    if _cse_temp_2 {
        return (x).py_add(y);
    }
    let _cse_temp_3 = w <= 0;
    if _cse_temp_3 {
        return {
            let _r: i32 = ((x).py_add(y) as i32).py_add(z);
            _r
        };
    }
    {
        let _r: i32 = ((x).py_mul(y) as i32).py_add((z).py_mul(w));
        _r
    }
}
#[doc = "Treat empty list as null-equivalent, return 0."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn null_safe_length(lst: &Vec<i32>) -> i32 {
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return 0;
    }
    lst.len() as i32 as i32
}
#[doc = "Product with empty = 1(identity) and zero short-circuit."]
#[doc = " Depyler: verified panic-free"]
pub fn null_safe_product(lst: &Vec<i32>) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return 1;
    }
    result = 1;
    for x in lst.iter().cloned() {
        if x == 0 {
            return 0;
        }
        result = ((result).py_mul(x)) as i32;
    }
    result
}
#[doc = "Get element at index or return default."]
#[doc = " Depyler: proven to terminate"]
pub fn get_or_default(
    lst: &Vec<i32>,
    idx: i32,
    default: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = idx < 0;
    let _cse_temp_1 = lst.len() as i32;
    let _cse_temp_2 = idx >= _cse_temp_1;
    let _cse_temp_3 = (_cse_temp_0) || (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(default);
    }
    Ok(lst
        .get(idx as usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Return first positive element or default."]
#[doc = " Depyler: verified panic-free"]
pub fn first_positive_or_default(lst: &Vec<i32>, default: i32) -> i32 {
    for x in lst.iter().cloned() {
        if x > 0 {
            return x;
        }
    }
    default
}
#[doc = "Return max of list or default if empty."]
pub fn max_or_default(lst: &Vec<i32>, default: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut best: i32 = Default::default();
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(default);
    }
    best = lst
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for x in lst.iter().cloned() {
        if x > best {
            best = x;
        }
    }
    Ok(best)
}
#[doc = "Simulate work with timeout. Returns completed units or -1."]
#[doc = " Depyler: verified panic-free"]
pub fn simulate_timeout(work: i32, timeout: i32) -> i32 {
    let mut done: i32 = Default::default();
    done = 0;
    let mut elapsed: i32 = 0;
    while done < work {
        if elapsed >= timeout {
            return -1;
        }
        done = ((done).py_add(1i32)) as i32;
        elapsed = ((elapsed).py_add(1i32)) as i32;
    }
    done
}
#[doc = "Simulate work with exponential backoff. Returns rounds used."]
#[doc = " Depyler: verified panic-free"]
pub fn work_with_backoff(total: i32, max_rounds: i32) -> i32 {
    let mut remaining: i32 = Default::default();
    let mut rounds: i32 = Default::default();
    let mut chunk: i32 = Default::default();
    remaining = total;
    rounds = 0;
    chunk = 1;
    while (remaining > 0) && (rounds < max_rounds) {
        if chunk > remaining {
            chunk = remaining;
        }
        remaining = ((remaining) - (chunk)) as i32;
        chunk = ((chunk).py_mul(2i32)) as i32;
        rounds = ((rounds).py_add(1i32)) as i32;
    }
    let _cse_temp_0 = remaining > 0;
    if _cse_temp_0 {
        return -1;
    }
    rounds
}
#[doc = "Untyped division guard."]
#[doc = " Depyler: proven to terminate"]
pub fn untyped_safe_divide(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(-1);
    }
    Ok({
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
    })
}
#[doc = "Untyped clamping."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn untyped_clamp(value: i32, lo: i32, hi: i32) -> String {
    let _cse_temp_0 = value < lo;
    if _cse_temp_0 {
        return lo.to_string();
    }
    let _cse_temp_1 = value > hi;
    if _cse_temp_1 {
        return hi.to_string();
    }
    value.to_string()
}
#[doc = "Untyped coalesce."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn untyped_fallback(a: DepylerValue, b: DepylerValue, c: i32, sentinel: DepylerValue) -> i32 {
    let _cse_temp_0 = a != sentinel;
    if _cse_temp_0 {
        return a;
    }
    let _cse_temp_1 = b != sentinel;
    if _cse_temp_1 {
        return b;
    }
    let _cse_temp_2 = c != sentinel;
    if _cse_temp_2 {
        return c;
    }
    sentinel
}
#[doc = "Untyped retry loop."]
#[doc = " Depyler: verified panic-free"]
pub fn untyped_retry(start: &DepylerValue, step: i32, max_tries: i32) -> i32 {
    let mut current = start.clone();
    let mut tries = 0;
    while tries < max_tries {
        if current > 0 {
            return current;
        }
        current = (current).py_add(step);
        tries = ((tries).py_add(1i32)) as i32;
    }
    -1
}
#[doc = "Untyped guard clause function."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn untyped_guard_compute(a: i32, b: i32, c: i32) -> i32 {
    let _cse_temp_0 = a == 0;
    if _cse_temp_0 {
        return 0;
    }
    let _cse_temp_1 = b == 0;
    if _cse_temp_1 {
        return a;
    }
    let _cse_temp_2 = c == 0;
    if _cse_temp_2 {
        return (a).py_add(b);
    }
    {
        let _r: i32 = ((a).py_mul(b) as i32).py_add(c);
        _r
    }
}
#[doc = "Weighted average with mismatched-length and zero-weight guards."]
pub fn safe_weighted_average<'a, 'b>(
    values: &'a Vec<i32>,
    weights: &'b Vec<i32>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total_weight: i32 = Default::default();
    let mut weighted_sum: i32 = Default::default();
    let mut length: i32 = Default::default();
    let mut w: i32 = Default::default();
    let _cse_temp_0 = values.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    let _cse_temp_2 = weights.len() as i32;
    let _cse_temp_3 = _cse_temp_2 == 0;
    if _cse_temp_3 {
        return Ok(0);
    }
    length = _cse_temp_0;
    let _cse_temp_4 = _cse_temp_2 < length;
    if _cse_temp_4 {
        length = _cse_temp_2;
    }
    total_weight = 0;
    weighted_sum = 0;
    let mut i: i32 = 0;
    while i < length {
        w = weights
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if w < 0 {
            w = 0;
        }
        total_weight = ((total_weight).py_add(w)) as i32;
        weighted_sum = ((weighted_sum).py_add(
            (values
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_mul(w),
        )) as i32;
        i = ((i).py_add(1i32)) as i32;
    }
    let _cse_temp_5 = total_weight == 0;
    if _cse_temp_5 {
        return Ok(0);
    }
    Ok({
        let a = weighted_sum;
        let b = total_weight;
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
    })
}
#[doc = "Binary search with full bounds protection."]
pub fn robust_binary_search(
    lst: &Vec<i32>,
    target: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = lst.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(-1);
    }
    let mut lo: i32 = 0;
    let mut hi: i32 = ((_cse_temp_0) - (1i32)) as i32;
    while lo <= hi {
        let mid: i32 = ((lo).py_add({
            let a = (hi) - (lo);
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
        })) as i32;
        if (mid < 0) || (mid >= lst.len() as i32) {
            return Ok(-1);
        }
        let val: i32 = lst
            .get(mid as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if val == target {
            return Ok(mid);
        }
        if val < target {
            lo = ((mid).py_add(1i32)) as i32;
        } else {
            hi = ((mid) - (1i32)) as i32;
        }
    }
    Ok(-1)
}
#[doc = "Cascading validation: each param validated against previous.\n    Returns 0 on success, error code 1-7 on failure."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn cascading_validation(a: i32, b: i32, c: i32, d: i32) -> i32 {
    let _cse_temp_0 = a < 0;
    if _cse_temp_0 {
        return 1;
    }
    let _cse_temp_1 = b < a;
    if _cse_temp_1 {
        return 2;
    }
    let _cse_temp_2 = c < b;
    if _cse_temp_2 {
        return 3;
    }
    let _cse_temp_3 = d < c;
    if _cse_temp_3 {
        return 4;
    }
    let _cse_temp_4 = (d) - (a) > 1000;
    if _cse_temp_4 {
        return 5;
    }
    let _cse_temp_5 = (((b) - (a) as i32).py_add((d) - (c))) as i32;
    let _cse_temp_6 = _cse_temp_5 > 500;
    if _cse_temp_6 {
        return 6;
    }
    let _cse_temp_7 = (((a).py_add(b) as i32).py_add(c)) as i32;
    let _cse_temp_8 = (_cse_temp_7).py_add(d) == 0;
    if _cse_temp_8 {
        return 7;
    }
    0
}
#[doc = "Process list with error recovery at each step.\n    Negative = error, skip and try next. Returns sum of valid * count_valid."]
#[doc = " Depyler: verified panic-free"]
pub fn error_recovery_chain(values: &Vec<i32>) -> i32 {
    let mut valid_sum: i32 = Default::default();
    let mut valid_count: i32 = Default::default();
    valid_count = 0;
    valid_sum = 0;
    let mut consecutive_errors: i32 = 0;
    for v in values.iter().cloned() {
        if v < 0 {
            consecutive_errors = ((consecutive_errors).py_add(1i32)) as i32;
            if consecutive_errors >= 3 {
                return -1;
            }
        } else {
            consecutive_errors = 0;
            valid_count = ((valid_count).py_add(1i32)) as i32;
            valid_sum = ((valid_sum).py_add(v)) as i32;
        }
    }
    let _cse_temp_0 = valid_count == 0;
    if _cse_temp_0 {
        return 0;
    }
    (valid_sum).py_mul(valid_count)
}
#[doc = "Execute all test cases and return total passed count."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_all_tests() -> Result<i32, Box<dyn std::error::Error>> {
    let mut passed: i32 = Default::default();
    passed = 0;
    let _cse_temp_0 = safe_divide(10, 3)? == 3;
    if _cse_temp_0 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = safe_divide(10, 0)? == -1;
    if _cse_temp_1 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = safe_divide(0, 5)? == 0;
    if _cse_temp_2 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_3 = safe_divide_with_default(10, 0, 42)? == 42;
    if _cse_temp_3 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_4 = safe_divide_with_default(10, 2, 42)? == 5;
    if _cse_temp_4 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_5 = chained_division(100, 5, 2)? == 10;
    if _cse_temp_5 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_6 = chained_division(100, 0, 2)? == -1;
    if _cse_temp_6 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_7 = chained_division(100, 5, 0)? == -1;
    if _cse_temp_7 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_8 = wrapping_add(250, 10, 255) == 4;
    if _cse_temp_8 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_9 = saturating_add(250, 10, 255) == 255;
    if _cse_temp_9 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_10 = saturating_add(5, 3, 255) == 8;
    if _cse_temp_10 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_11 = saturating_sub(10, 3) == 7;
    if _cse_temp_11 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_12 = saturating_sub(3, 10) == 0;
    if _cse_temp_12 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_13 = saturating_mul(10, 10, 50) == 50;
    if _cse_temp_13 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_14 = clamp(5, 0, 10) == 5;
    if _cse_temp_14 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_15 = clamp(-5, 0, 10) == 0;
    if _cse_temp_15 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_16 = clamp(15, 0, 10) == 10;
    if _cse_temp_16 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_17 = abs_diff(10, 3) == 7;
    if _cse_temp_17 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_18 = abs_diff(3, 10) == 7;
    if _cse_temp_18 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_19 = midpoint_safe(0, 10)? == 5;
    if _cse_temp_19 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_20 = midpoint_safe(10, 0)? == 5;
    if _cse_temp_20 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_21 = boundary_classify(0, 100) == 0;
    if _cse_temp_21 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_22 = boundary_classify(1, 100) == 1;
    if _cse_temp_22 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_23 = boundary_classify(100, 100) == 2;
    if _cse_temp_23 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_24 = boundary_classify(50, 100) == 3;
    if _cse_temp_24 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_25 = limited_factorial(5, 10) == 120;
    if _cse_temp_25 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_26 = limited_factorial(5, 2) == -1;
    if _cse_temp_26 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_27 = limited_fibonacci(6, 20) == 8;
    if _cse_temp_27 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_28 = bounded_power(2, 8, 1000) == 256;
    if _cse_temp_28 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_29 = bounded_power(2, 20, 1000) == -1;
    if _cse_temp_29 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_30 = safe_first(&vec![10, 20, 30])? == 10;
    if _cse_temp_30 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_31 = safe_first(&vec![])? == -1;
    if _cse_temp_31 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_32 = safe_last(&vec![10, 20, 30])? == 30;
    if _cse_temp_32 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_33 = safe_last(&vec![])? == -1;
    if _cse_temp_33 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_34 = safe_sum(&vec![1, 2, 3, 4]) == 10;
    if _cse_temp_34 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_35 = safe_sum(&vec![]) == 0;
    if _cse_temp_35 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_36 = safe_max(&vec![3, 1, 4, 1, 5])? == 5;
    if _cse_temp_36 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_37 = safe_max(&vec![])? == -999999;
    if _cse_temp_37 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_38 = safe_min(&vec![3, 1, 4, 1, 5])? == 1;
    if _cse_temp_38 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_39 = safe_min(&vec![])? == 999999;
    if _cse_temp_39 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_40 = safe_average_int(&vec![10, 20, 30])? == 20;
    if _cse_temp_40 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_41 = safe_average_int(&vec![])? == 0;
    if _cse_temp_41 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_42 = safe_index(&vec![10, 20, 30], 1)? == 20;
    if _cse_temp_42 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_43 = safe_index(&vec![10, 20, 30], -1)? == -1;
    if _cse_temp_43 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_44 = safe_index(&vec![10, 20, 30], 5)? == -1;
    if _cse_temp_44 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_45 = find_or_default(&vec![10, 20, 30], 20, -1)? == 1;
    if _cse_temp_45 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_46 = find_or_default(&vec![10, 20, 30], 99, -1)? == -1;
    if _cse_temp_46 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_47 = propagate_sentinel(5, 3) == 8;
    if _cse_temp_47 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_48 = propagate_sentinel(-1, 3) == -1;
    if _cse_temp_48 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_49 = propagate_sentinel(5, -1) == -1;
    if _cse_temp_49 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_50 = chain_sentinel_ops(1, 2, 3) == 6;
    if _cse_temp_50 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_51 = chain_sentinel_ops(-1, 2, 3) == -1;
    if _cse_temp_51 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_52 = sentinel_map(&vec![1, 2, 3]) == 6;
    if _cse_temp_52 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_53 = sentinel_map(&vec![1, -1, 3]) == -1;
    if _cse_temp_53 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_54 = classify_triangle(3, 3, 3) == 1;
    if _cse_temp_54 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_55 = classify_triangle(3, 3, 4) == 2;
    if _cse_temp_55 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_56 = classify_triangle(3, 4, 5) == 3;
    if _cse_temp_56 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_57 = classify_triangle(1, 1, 10) == 0;
    if _cse_temp_57 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_58 = classify_triangle(-1, 3, 3) == 0;
    if _cse_temp_58 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_59 = multi_range_classify(-5) == 0;
    if _cse_temp_59 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_60 = multi_range_classify(5) == 1;
    if _cse_temp_60 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_61 = multi_range_classify(50) == 2;
    if _cse_temp_61 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_62 = multi_range_classify(500) == 3;
    if _cse_temp_62 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_63 = multi_range_classify(5000) == 4;
    if _cse_temp_63 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_64 = grade_score(95) == 5;
    if _cse_temp_64 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_65 = grade_score(85) == 4;
    if _cse_temp_65 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_66 = grade_score(55) == 1;
    if _cse_temp_66 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_67 = grade_score(-1) == 0;
    if _cse_temp_67 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_68 = count_errors(&vec![1, -1, 2, -2, 3]) == 2;
    if _cse_temp_68 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_69 = count_valid(&vec![1, 5, 10, 15, 20], 5, 15) == 3;
    if _cse_temp_69 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_70 = sum_valid_only(&vec![1, -1, 2, -2, 3]) == 6;
    if _cse_temp_70 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_71 = first_error_index(&vec![1, 2, -3, 4])? == 2;
    if _cse_temp_71 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_72 = first_error_index(&vec![1, 2, 3])? == -1;
    if _cse_temp_72 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_73 = fallback_divide(10, 0, 0, 2)? == 5;
    if _cse_temp_73 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_74 = fallback_divide(10, 0, 0, 0)? == 0;
    if _cse_temp_74 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_75 = fallback_lookup(&vec![10, 20, 30], -1, 5, 1, 99)? == 20;
    if _cse_temp_75 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_76 = coalesce(-1, -1, 42, -1) == 42;
    if _cse_temp_76 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_77 = coalesce(-1, -1, -1, -1) == -1;
    if _cse_temp_77 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_78 = validate_age(25) == 0;
    if _cse_temp_78 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_79 = validate_age(-5) == 1;
    if _cse_temp_79 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_80 = validate_age(200) == 2;
    if _cse_temp_80 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_81 = validate_age(0) == 3;
    if _cse_temp_81 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_82 = validate_range(5, 0, 10) == 0;
    if _cse_temp_82 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_83 = validate_range(-1, 0, 10) == 1;
    if _cse_temp_83 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_84 = validate_range(11, 0, 10) == 2;
    if _cse_temp_84 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_85 = validate_pair(1, 2) == 0;
    if _cse_temp_85 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_86 = validate_pair(-1, 2) == 1;
    if _cse_temp_86 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_87 = validate_pair(1, -1) == 2;
    if _cse_temp_87 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_88 = validate_pair(-1, -1) == 3;
    if _cse_temp_88 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_89 = validate_pair(5, 5) == 4;
    if _cse_temp_89 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_90 = validation_pipeline(10)? == 0;
    if _cse_temp_90 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_91 = validation_pipeline(-1)? == 1;
    if _cse_temp_91 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_92 = validation_pipeline(20000)? == 2;
    if _cse_temp_92 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_93 = validation_pipeline(7)? == 3;
    if _cse_temp_93 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_94 = validation_pipeline(0)? == 4;
    if _cse_temp_94 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_95 = sum_of_clamped(&vec![-5, 3, 15, 7], 0, 10) == 20;
    if _cse_temp_95 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_96 = count_in_band(&vec![1, 5, 10, 15, 20], 10, 5) == 3;
    if _cse_temp_96 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_97 = retry_until_positive(-5, 2, 10) == 1;
    if _cse_temp_97 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_98 = retry_until_positive(-100, 1, 5) == -1;
    if _cse_temp_98 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_99 = countdown_to_target(10, 5, 100) == 5;
    if _cse_temp_99 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_100 = converge_to_zero(16, 100)? == 5;
    if _cse_temp_100 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_101 = converge_to_zero(0, 100)? == 0;
    if _cse_temp_101 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_102 = state_machine_step(0, 5) == 1;
    if _cse_temp_102 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_103 = state_machine_step(0, -1) == 0;
    if _cse_temp_103 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_104 = state_machine_step(1, 15) == 2;
    if _cse_temp_104 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_105 = state_machine_step(5, 0) == -1;
    if _cse_temp_105 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_106 = run_state_machine(&vec![5, 15, 0]) == 0;
    if _cse_temp_106 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    if _cse_temp_106 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_107 = validate_sequence(&vec![1, 2, 3, 4])? == 0;
    if _cse_temp_107 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_108 = validate_sequence(&vec![1, 2, 2, 3])? == 1;
    if _cse_temp_108 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_109 = validate_sequence(&vec![1, 3, 2, 4])? == 2;
    if _cse_temp_109 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_110 = validate_sequence(&vec![])? == 3;
    if _cse_temp_110 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_111 = guarded_compute(0, 5, 5) == 0;
    if _cse_temp_111 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_112 = guarded_compute(3, 0, 5) == 3;
    if _cse_temp_112 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_113 = guarded_compute(3, 4, 0) == 7;
    if _cse_temp_113 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_114 = guarded_compute(3, 4, 5) == 17;
    if _cse_temp_114 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_115 = guarded_compute(-1, 4, 5) == -1;
    if _cse_temp_115 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_116 = deeply_guarded(0, 1, 1, 1) == 0;
    if _cse_temp_116 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_117 = deeply_guarded(5, 0, 1, 1) == 5;
    if _cse_temp_117 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_118 = deeply_guarded(5, 3, 0, 1) == 8;
    if _cse_temp_118 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_119 = deeply_guarded(5, 3, 2, 0) == 10;
    if _cse_temp_119 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_120 = deeply_guarded(2, 3, 4, 5) == 26;
    if _cse_temp_120 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_121 = null_safe_length(&vec![]) == 0;
    if _cse_temp_121 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_122 = null_safe_length(&vec![1, 2, 3]) == 3;
    if _cse_temp_122 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_123 = null_safe_product(&vec![]) == 1;
    if _cse_temp_123 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_124 = null_safe_product(&vec![2, 3, 4]) == 24;
    if _cse_temp_124 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_125 = null_safe_product(&vec![2, 0, 4]) == 0;
    if _cse_temp_125 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_126 = get_or_default(&vec![10, 20, 30], 1, -1)? == 20;
    if _cse_temp_126 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_127 = get_or_default(&vec![10, 20, 30], 5, -1)? == -1;
    if _cse_temp_127 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_128 = first_positive_or_default(&vec![-1, -2, 3, 4], 0) == 3;
    if _cse_temp_128 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_129 = first_positive_or_default(&vec![-1, -2], 0) == 0;
    if _cse_temp_129 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_130 = max_or_default(&vec![3, 1, 4], 0)? == 4;
    if _cse_temp_130 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_131 = max_or_default(&vec![], 0)? == 0;
    if _cse_temp_131 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_132 = simulate_timeout(5, 10) == 5;
    if _cse_temp_132 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_133 = simulate_timeout(10, 5) == -1;
    if _cse_temp_133 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_134 = work_with_backoff(7, 10) == 3;
    if _cse_temp_134 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_135 = work_with_backoff(1000, 3) == -1;
    if _cse_temp_135 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_136 = untyped_safe_divide(10, 0)? == -1;
    if _cse_temp_136 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_137 = untyped_safe_divide(10, 3)? == 3;
    if _cse_temp_137 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_138 = untyped_clamp(5, 0, 10) == 5;
    if _cse_temp_138 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_139 = untyped_clamp(-5, 0, 10) == 0;
    if _cse_temp_139 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_140 = untyped_fallback(-1, -1, 42, -1) == 42;
    if _cse_temp_140 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_141 = untyped_retry(&-5, 2, 10) == 1;
    if _cse_temp_141 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_142 = untyped_guard_compute(3, 4, 5) == 17;
    if _cse_temp_142 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_143 = safe_weighted_average(&vec![10, 20, 30], &vec![1, 2, 3])? == 23;
    if _cse_temp_143 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_144 = safe_weighted_average(&vec![], &vec![1, 2])? == 0;
    if _cse_temp_144 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    if _cse_temp_144 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_145 = robust_binary_search(&vec![1, 3, 5, 7, 9], 5)? == 2;
    if _cse_temp_145 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_146 = robust_binary_search(&vec![1, 3, 5, 7, 9], 4)? == -1;
    if _cse_temp_146 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_147 = robust_binary_search(&vec![], 5)? == -1;
    if _cse_temp_147 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_148 = cascading_validation(1, 2, 3, 4) == 0;
    if _cse_temp_148 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_149 = cascading_validation(-1, 2, 3, 4) == 1;
    if _cse_temp_149 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_150 = cascading_validation(5, 3, 4, 5) == 2;
    if _cse_temp_150 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_151 = error_recovery_chain(&vec![1, 2, 3]) == 18;
    if _cse_temp_151 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    let _cse_temp_152 = error_recovery_chain(&vec![-1, -1, -1, 5]) == -1;
    if _cse_temp_152 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    if _cse_temp_151 {
        passed = ((passed).py_add(1i32)) as i32;
    }
    Ok(passed)
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result: i32 = run_all_tests()?;
    println!("{}", result);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_safe_divide_examples() {
        assert_eq!(safe_divide(0, 0), 0);
        assert_eq!(safe_divide(1, 2), 3);
        assert_eq!(safe_divide(-1, 1), 0);
    }
    #[test]
    fn test_saturating_sub_examples() {
        assert_eq!(saturating_sub(0, 0), 0);
        assert_eq!(saturating_sub(1, 2), 3);
        assert_eq!(saturating_sub(-1, 1), 0);
    }
    #[test]
    fn quickcheck_abs_diff() {
        fn prop(a: i32, b: i32) -> TestResult {
            let result = abs_diff(a.clone(), b.clone());
            if result < 0 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_abs_diff_examples() {
        assert_eq!(abs_diff(0, 0), 0);
        assert_eq!(abs_diff(1, 2), 3);
        assert_eq!(abs_diff(-1, 1), 0);
    }
    #[test]
    fn test_midpoint_safe_examples() {
        assert_eq!(midpoint_safe(0, 0), 0);
        assert_eq!(midpoint_safe(1, 2), 3);
        assert_eq!(midpoint_safe(-1, 1), 0);
    }
    #[test]
    fn test_boundary_classify_examples() {
        assert_eq!(boundary_classify(0, 0), 0);
        assert_eq!(boundary_classify(1, 2), 3);
        assert_eq!(boundary_classify(-1, 1), 0);
    }
    #[test]
    fn test_limited_factorial_examples() {
        assert_eq!(limited_factorial(0, 0), 0);
        assert_eq!(limited_factorial(1, 2), 3);
        assert_eq!(limited_factorial(-1, 1), 0);
    }
    #[test]
    fn test_limited_fibonacci_examples() {
        assert_eq!(limited_fibonacci(0, 0), 0);
        assert_eq!(limited_fibonacci(1, 2), 3);
        assert_eq!(limited_fibonacci(-1, 1), 0);
    }
    #[test]
    fn test_safe_first_examples() {
        assert_eq!(safe_first(&vec![]), 0);
        assert_eq!(safe_first(&vec![1]), 1);
        assert_eq!(safe_first(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_safe_last_examples() {
        assert_eq!(safe_last(&vec![]), 0);
        assert_eq!(safe_last(&vec![1]), 1);
        assert_eq!(safe_last(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_safe_sum_examples() {
        assert_eq!(safe_sum(&vec![]), 0);
        assert_eq!(safe_sum(&vec![1]), 1);
        assert_eq!(safe_sum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_safe_max_examples() {
        assert_eq!(safe_max(&vec![]), 0);
        assert_eq!(safe_max(&vec![1]), 1);
        assert_eq!(safe_max(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_safe_min_examples() {
        assert_eq!(safe_min(&vec![]), 0);
        assert_eq!(safe_min(&vec![1]), 1);
        assert_eq!(safe_min(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_safe_average_int_examples() {
        assert_eq!(safe_average_int(&vec![]), 0);
        assert_eq!(safe_average_int(&vec![1]), 1);
        assert_eq!(safe_average_int(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_propagate_sentinel_examples() {
        assert_eq!(propagate_sentinel(0, 0), 0);
        assert_eq!(propagate_sentinel(1, 2), 3);
        assert_eq!(propagate_sentinel(-1, 1), 0);
    }
    #[test]
    fn test_sentinel_map_examples() {
        assert_eq!(sentinel_map(&vec![]), 0);
        assert_eq!(sentinel_map(&vec![1]), 1);
        assert_eq!(sentinel_map(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_multi_range_classify_examples() {
        assert_eq!(multi_range_classify(0), 0);
        assert_eq!(multi_range_classify(1), 1);
        assert_eq!(multi_range_classify(-1), -1);
    }
    #[test]
    fn test_grade_score_examples() {
        assert_eq!(grade_score(0), 0);
        assert_eq!(grade_score(1), 1);
        assert_eq!(grade_score(-1), -1);
    }
    #[test]
    fn test_count_errors_examples() {
        assert_eq!(count_errors(&vec![]), 0);
        assert_eq!(count_errors(&vec![1]), 1);
        assert_eq!(count_errors(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_sum_valid_only_examples() {
        assert_eq!(sum_valid_only(&vec![]), 0);
        assert_eq!(sum_valid_only(&vec![1]), 1);
        assert_eq!(sum_valid_only(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_first_error_index_examples() {
        assert_eq!(first_error_index(&vec![]), 0);
        assert_eq!(first_error_index(&vec![1]), 1);
        assert_eq!(first_error_index(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_validate_age_examples() {
        assert_eq!(validate_age(0), 0);
        assert_eq!(validate_age(1), 1);
        assert_eq!(validate_age(-1), -1);
    }
    #[test]
    fn test_validate_pair_examples() {
        assert_eq!(validate_pair(0, 0), 0);
        assert_eq!(validate_pair(1, 2), 3);
        assert_eq!(validate_pair(-1, 1), 0);
    }
    #[test]
    fn test_validation_pipeline_examples() {
        assert_eq!(validation_pipeline(0), 0);
        assert_eq!(validation_pipeline(1), 1);
        assert_eq!(validation_pipeline(-1), -1);
    }
    #[test]
    fn test_converge_to_zero_examples() {
        assert_eq!(converge_to_zero(0, 0), 0);
        assert_eq!(converge_to_zero(1, 2), 3);
        assert_eq!(converge_to_zero(-1, 1), 0);
    }
    #[test]
    fn test_state_machine_step_examples() {
        assert_eq!(state_machine_step(0, 0), 0);
        assert_eq!(state_machine_step(1, 2), 3);
        assert_eq!(state_machine_step(-1, 1), 0);
    }
    #[test]
    fn test_run_state_machine_examples() {
        assert_eq!(run_state_machine(&vec![]), 0);
        assert_eq!(run_state_machine(&vec![1]), 1);
        assert_eq!(run_state_machine(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_validate_sequence_examples() {
        assert_eq!(validate_sequence(&vec![]), 0);
        assert_eq!(validate_sequence(&vec![1]), 1);
        assert_eq!(validate_sequence(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_null_safe_length_examples() {
        assert_eq!(null_safe_length(&vec![]), 0);
        assert_eq!(null_safe_length(&vec![1]), 1);
        assert_eq!(null_safe_length(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_null_safe_product_examples() {
        assert_eq!(null_safe_product(&vec![]), 0);
        assert_eq!(null_safe_product(&vec![1]), 1);
        assert_eq!(null_safe_product(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_simulate_timeout_examples() {
        assert_eq!(simulate_timeout(0, 0), 0);
        assert_eq!(simulate_timeout(1, 2), 3);
        assert_eq!(simulate_timeout(-1, 1), 0);
    }
    #[test]
    fn test_work_with_backoff_examples() {
        assert_eq!(work_with_backoff(0, 0), 0);
        assert_eq!(work_with_backoff(1, 2), 3);
        assert_eq!(work_with_backoff(-1, 1), 0);
    }
    #[test]
    fn test_error_recovery_chain_examples() {
        assert_eq!(error_recovery_chain(&vec![]), 0);
        assert_eq!(error_recovery_chain(&vec![1]), 1);
        assert_eq!(error_recovery_chain(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_run_all_tests_examples() {
        let _ = run_all_tests();
    }
}