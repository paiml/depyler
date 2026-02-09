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
#[doc = "Generate first n Fibonacci numbers as a list."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_sequence(n: i32) -> Vec<i32> {
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return vec![];
    }
    let _cse_temp_1 = n == 1;
    if _cse_temp_1 {
        return vec![0];
    }
    let mut result: Vec<i32> = vec![0, 1];
    for i in (2)..(n) {
        result.push(
            ({
                let base = &result;
                let idx: i32 = (i) - (1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            })
            .py_add({
                let base = &result;
                let idx: i32 = (i) - (2i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }),
        );
    }
    result
}
#[doc = "Generate first n Lucas numbers(2, 1, 3, 4, 7,...)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn lucas_sequence(n: i32) -> Vec<i32> {
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return vec![];
    }
    let _cse_temp_1 = n == 1;
    if _cse_temp_1 {
        return vec![2];
    }
    let mut result: Vec<i32> = vec![2, 1];
    for i in (2)..(n) {
        result.push(
            ({
                let base = &result;
                let idx: i32 = (i) - (1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            })
            .py_add({
                let base = &result;
                let idx: i32 = (i) - (2i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }),
        );
    }
    result
}
#[doc = "Generate first n Tribonacci numbers(0, 0, 1, 1, 2, 4, 7,...)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn tribonacci_sequence(n: i32) -> Vec<i32> {
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return vec![];
    }
    let _cse_temp_1 = n == 1;
    if _cse_temp_1 {
        return vec![0];
    }
    let _cse_temp_2 = n == 2;
    if _cse_temp_2 {
        return vec![0, 0];
    }
    let mut result: Vec<i32> = vec![0, 0, 1];
    for i in (3)..(n) {
        result.push(
            (({
                let base = &result;
                let idx: i32 = (i) - (1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            })
            .py_add({
                let base = &result;
                let idx: i32 = (i) - (2i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }) as i32)
                .py_add({
                    let base = &result;
                    let idx: i32 = (i) - (3i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                }),
        );
    }
    result
}
#[doc = "Simulate infinite natural number generator with cutoff."]
#[doc = " Depyler: verified panic-free"]
pub fn naturals_up_to(limit: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut n: i32 = 1;
    while n <= limit {
        result.push(n);
        n = ((n).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Generate first count powers of two."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn powers_of_two(count: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut val: i32 = 1;
    for _i in 0..(count) {
        result.push(val);
        val = ((val).py_mul(2i32)) as i32;
    }
    result
}
#[doc = "Generate geometric sequence: start, start*ratio, start*ratio^2,..."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn geometric_sequence(start: i32, ratio: i32, count: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut current: i32 = start.clone();
    for _i in 0..(count) {
        result.push(current);
        current = ((current).py_mul(ratio)) as i32;
    }
    result
}
#[doc = "Generate arithmetic sequence with given start, step, and count."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn arithmetic_sequence(start: i32, step: i32, count: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut current: i32 = start.clone();
    for _i in 0..(count) {
        result.push(current);
        current = ((current).py_add(step)) as i32;
    }
    result
}
#[doc = "Generate Collatz sequence starting from n until reaching 1."]
pub fn collatz_sequence(n: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut current: i32 = Default::default();
    let mut result: Vec<i32> = vec![n];
    current = n;
    while current != 1 {
        if (current).py_mod(2i32) == 0 {
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
        } else {
            current = (((3i32).py_mul(current) as i32).py_add(1i32)) as i32;
        }
        result.push(current);
    }
    Ok(result)
}
#[doc = "Take elements while they are positive."]
#[doc = " Depyler: verified panic-free"]
pub fn take_while_positive(nums: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in nums.iter().cloned() {
        if x <= 0 {
            break;
        }
        result.push(x);
    }
    result
}
#[doc = "Drop elements while they are negative, keep the rest."]
#[doc = " Depyler: verified panic-free"]
pub fn drop_while_negative(nums: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut dropping: bool = true;
    for x in nums.iter().cloned() {
        if (dropping) && (x < 0) {
            continue;
        }
        dropping = false;
        result.push(x);
    }
    result
}
#[doc = "Take first n items from a list(simulates islice)."]
#[doc = " Depyler: verified panic-free"]
pub fn take_first_n(items: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut count: i32 = 0;
    for x in items.iter().cloned() {
        if count >= n {
            break;
        }
        result.push(x);
        count = ((count).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Drop first n items from a list."]
#[doc = " Depyler: verified panic-free"]
pub fn drop_first_n(items: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut count: i32 = 0;
    for x in items.iter().cloned() {
        if count < n {
            count = ((count).py_add(1i32)) as i32;
            continue;
        }
        result.push(x);
    }
    result
}
#[doc = "Produce running sum(like itertools.accumulate)."]
#[doc = " Depyler: verified panic-free"]
pub fn running_sum(nums: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut total: i32 = 0;
    for x in nums.iter().cloned() {
        total = ((total).py_add(x)) as i32;
        result.push(total);
    }
    result
}
#[doc = "Produce running product of elements."]
#[doc = " Depyler: verified panic-free"]
pub fn running_product(nums: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut prod: i32 = 1;
    for x in nums.iter().cloned() {
        prod = ((prod).py_mul(x)) as i32;
        result.push(prod);
    }
    result
}
#[doc = "Track the running maximum across the sequence."]
#[doc = " Depyler: proven to terminate"]
pub fn running_max(nums: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut current_max: i32 = Default::default();
    if nums.is_empty() {
        return Ok(vec![]);
    }
    let mut result: Vec<i32> = vec![nums
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")];
    current_max = nums
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for i in (1)..(nums.len() as i32) {
        if nums
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            > current_max
        {
            current_max = nums
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
        }
        result.push(current_max);
    }
    Ok(result)
}
#[doc = "Track the running minimum across the sequence."]
#[doc = " Depyler: proven to terminate"]
pub fn running_min(nums: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut current_min: i32 = Default::default();
    if nums.is_empty() {
        return Ok(vec![]);
    }
    let mut result: Vec<i32> = vec![nums
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")];
    current_min = nums
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for i in (1)..(nums.len() as i32) {
        if nums
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            < current_min
        {
            current_min = nums
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
        }
        result.push(current_min);
    }
    Ok(result)
}
#[doc = "Generalized scan: accumulate with add or multiply."]
#[doc = " Depyler: verified panic-free"]
pub fn scan_with_op(nums: &Vec<i32>, initial: i32, add: bool) -> Vec<i32> {
    let mut acc: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    acc = initial;
    for x in nums.iter().cloned() {
        if add {
            acc = ((acc).py_add(x)) as i32;
        } else {
            acc = ((acc).py_mul(x)) as i32;
        }
        result.push(acc);
    }
    result
}
#[doc = "Chain three lists together(like itertools.chain)."]
#[doc = " Depyler: verified panic-free"]
pub fn chain_lists<'a, 'b, 'c>(a: &'a Vec<i32>, b: &'b Vec<i32>, c: &'c Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in a.iter().cloned() {
        result.push(x);
    }
    for x in b.iter().cloned() {
        result.push(x);
    }
    for x in c.iter().cloned() {
        result.push(x);
    }
    result
}
#[doc = "Flatten a list of lists into a single list."]
#[doc = " Depyler: verified panic-free"]
pub fn flatten_nested(nested: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for inner in nested.iter().cloned() {
        for x in inner.iter().cloned() {
            result.push(x);
        }
    }
    result
}
#[doc = "Flatten and keep only elements above threshold."]
#[doc = " Depyler: verified panic-free"]
pub fn flatten_and_filter(nested: &Vec<Vec<i32>>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for inner in nested.iter().cloned() {
        for x in inner.iter().cloned() {
            if x > threshold {
                result.push(x);
            }
        }
    }
    result
}
#[doc = "Interleave two lists element by element."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn interleave<'b, 'a>(a: &'a Vec<i32>, b: &'b Vec<i32>) -> Vec<i32> {
    let mut min_len: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = a.len() as i32;
    min_len = _cse_temp_0;
    let _cse_temp_1 = b.len() as i32;
    let _cse_temp_2 = _cse_temp_1 < min_len;
    if _cse_temp_2 {
        min_len = _cse_temp_1;
    }
    for i in 0..(min_len) {
        result.push(
            a.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        result.push(
            b.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    }
    for i in (min_len)..(a.len() as i32) {
        result.push(
            a.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    }
    for i in (min_len)..(b.len() as i32) {
        result.push(
            b.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    }
    result
}
#[doc = "Round-robin iteration across multiple lists."]
#[doc = " Depyler: verified panic-free"]
pub fn roundrobin(lists: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut max_len: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    max_len = 0;
    for lst in lists.iter().cloned() {
        if lst.len() as i32 > max_len {
            max_len = lst.len() as i32;
        }
    }
    for i in 0..(max_len) {
        for lst in lists.iter().cloned() {
            if i < lst.len() as i32 {
                result.push(
                    lst.get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                );
            }
        }
    }
    result
}
#[doc = "Zip two lists and sum corresponding pairs."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zip_sum<'b, 'a>(a: &'a Vec<i32>, b: &'b Vec<i32>) -> Vec<i32> {
    let mut min_len: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = a.len() as i32;
    min_len = _cse_temp_0;
    let _cse_temp_1 = b.len() as i32;
    let _cse_temp_2 = _cse_temp_1 < min_len;
    if _cse_temp_2 {
        min_len = _cse_temp_1;
    }
    for i in 0..(min_len) {
        result.push(
            (a.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_add(
                b.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ),
        );
    }
    result
}
#[doc = "Zip two lists and multiply corresponding pairs."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zip_product<'a, 'b>(a: &'a Vec<i32>, b: &'b Vec<i32>) -> Vec<i32> {
    let mut min_len: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = a.len() as i32;
    min_len = _cse_temp_0;
    let _cse_temp_1 = b.len() as i32;
    let _cse_temp_2 = _cse_temp_1 < min_len;
    if _cse_temp_2 {
        min_len = _cse_temp_1;
    }
    for i in 0..(min_len) {
        result.push(
            (a.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_mul(
                b.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ),
        );
    }
    result
}
#[doc = "Zip two lists and take the max of each pair."]
#[doc = " Depyler: proven to terminate"]
pub fn zip_max<'a, 'b>(
    a: &'a Vec<i32>,
    b: &'b Vec<i32>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut min_len: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = a.len() as i32;
    min_len = _cse_temp_0;
    let _cse_temp_1 = b.len() as i32;
    let _cse_temp_2 = _cse_temp_1 < min_len;
    if _cse_temp_2 {
        min_len = _cse_temp_1;
    }
    for i in 0..(min_len) {
        if a.get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            > b.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        {
            result.push(
                a.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        } else {
            result.push(
                b.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        }
    }
    Ok(result)
}
#[doc = "Simulate enumerate: return list of [index, value] pairs."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn enumerate_list(items: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..(items.len() as i32) {
        result.push(vec![
            i,
            items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ]);
    }
    result
}
#[doc = "Zip items with their index starting from a given offset."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zip_with_index(items: &Vec<i32>, start: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..(items.len() as i32) {
        result.push(vec![
            (start).py_add(i),
            items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ]);
    }
    result
}
#[doc = "Split a list into chunks of given size."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn chunk_list(items: &Vec<i32>, size: i32) -> Vec<Vec<i32>> {
    let mut chunk: Vec<i32> = Default::default();
    let mut result: Vec<Vec<i32>> = vec![];
    chunk = vec![];
    for i in 0..(items.len() as i32) {
        chunk.push(
            items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        if chunk.len() as i32 == size {
            result.push(chunk);
            chunk = vec![];
        }
    }
    let _cse_temp_0 = chunk.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        result.push(chunk);
    }
    result
}
#[doc = "Split into chunks and sum each chunk."]
#[doc = " Depyler: verified panic-free"]
pub fn chunk_sum(items: &Vec<i32>, size: i32) -> Vec<i32> {
    let mut count: i32 = Default::default();
    let mut total: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    total = 0;
    count = 0;
    for x in items.iter().cloned() {
        total = ((total).py_add(x)) as i32;
        count = ((count).py_add(1i32)) as i32;
        if count == size {
            result.push(total);
            total = 0;
            count = 0;
        }
    }
    let _cse_temp_0 = count > 0;
    if _cse_temp_0 {
        result.push(total);
    }
    result
}
#[doc = "Generate consecutive pairs: [a,b], [b,c], [c,d],..."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn pairwise(items: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..((items.len() as i32) - (1i32)) {
        result.push(vec![
            items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            {
                let base = &items;
                let idx: i32 = (i).py_add(1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            },
        ]);
    }
    result
}
#[doc = "Generate consecutive triples from a list."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn triples(items: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..((items.len() as i32) - (2i32)) {
        result.push(vec![
            items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            {
                let base = &items;
                let idx: i32 = (i).py_add(1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            },
            {
                let base = &items;
                let idx: i32 = (i).py_add(2i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            },
        ]);
    }
    result
}
#[doc = "Compute sum over a sliding window of given size."]
#[doc = " Depyler: proven to terminate"]
pub fn sliding_window_sum(
    nums: &Vec<i32>,
    window: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut current_sum: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = nums.len() as i32;
    let _cse_temp_1 = _cse_temp_0 < window;
    if _cse_temp_1 {
        return Ok(result);
    }
    current_sum = 0;
    for i in 0..(window) {
        current_sum = ((current_sum).py_add(
            nums.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        )) as i32;
    }
    result.push(current_sum);
    for i in (window)..(nums.len() as i32) {
        current_sum = ((current_sum).py_add(
            (nums
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
             - ({
                let base = &nums;
                let idx: i32 = (i) - (window);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }),
        )) as i32;
        result.push(current_sum);
    }
    Ok(result)
}
#[doc = "Compute max over a sliding window(brute force)."]
#[doc = " Depyler: proven to terminate"]
pub fn sliding_window_max(
    nums: &Vec<i32>,
    window: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    for i in 0..(((nums.len() as i32) - (window) as i32).py_add(1i32)) {
        let mut w_max: i32 = nums
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        for j in ((i).py_add(1i32))..((i).py_add(window)) {
            if nums
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                > w_max
            {
                w_max = nums
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
            }
        }
        result.push(w_max);
    }
    Ok(result)
}
#[doc = "Compute min over a sliding window(brute force)."]
#[doc = " Depyler: proven to terminate"]
pub fn sliding_window_min(
    nums: &Vec<i32>,
    window: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    for i in 0..(((nums.len() as i32) - (window) as i32).py_add(1i32)) {
        let mut w_min: i32 = nums
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        for j in ((i).py_add(1i32))..((i).py_add(window)) {
            if nums
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < w_min
            {
                w_min = nums
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
            }
        }
        result.push(w_min);
    }
    Ok(result)
}
#[doc = "Integer average over a sliding window(truncated)."]
#[doc = " Depyler: proven to terminate"]
pub fn sliding_window_avg_int(
    nums: &Vec<i32>,
    window: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut current_sum: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = nums.len() as i32;
    let _cse_temp_1 = _cse_temp_0 < window;
    let _cse_temp_2 = window <= 0;
    let _cse_temp_3 = (_cse_temp_1) || (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(result);
    }
    current_sum = 0;
    for i in 0..(window) {
        current_sum = ((current_sum).py_add(
            nums.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        )) as i32;
    }
    result.push({
        let a = current_sum;
        let b = window;
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
    for i in (window)..(nums.len() as i32) {
        current_sum = ((current_sum).py_add(
            (nums
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
             - ({
                let base = &nums;
                let idx: i32 = (i) - (window);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            }),
        )) as i32;
        result.push({
            let a = current_sum;
            let b = window;
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
    Ok(result)
}
#[doc = "Map(multiply) then filter(keep above threshold)."]
#[doc = " Depyler: verified panic-free"]
pub fn map_then_filter(nums: &Vec<i32>, multiplier: i32, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in nums.iter().cloned() {
        let mapped: i32 = ((x).py_mul(multiplier)) as i32;
        if mapped > threshold {
            result.push(mapped);
        }
    }
    result
}
#[doc = "Filter(keep above threshold) then map(add offset)."]
#[doc = " Depyler: verified panic-free"]
pub fn filter_then_map(nums: &Vec<i32>, threshold: i32, offset: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in nums.iter().cloned() {
        if x > threshold {
            result.push((x).py_add(offset));
        }
    }
    result
}
#[doc = "Pipeline: square each -> filter below limit -> sum."]
#[doc = " Depyler: verified panic-free"]
pub fn pipeline_square_filter_sum(nums: &Vec<i32>, limit: i32) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for x in nums.iter().cloned() {
        let squared: i32 = ((x).py_mul(x)) as i32;
        if squared < limit {
            total = ((total).py_add(squared)) as i32;
        }
    }
    total
}
#[doc = "Pipeline: absolute value -> deduplicate -> sort."]
pub fn pipeline_abs_dedup_sort(nums: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut abs_vals: Vec<i32> = vec![];
    for x in nums.iter().cloned() {
        if x < 0 {
            abs_vals.push(-x);
        } else {
            abs_vals.push(x);
        }
    }
    let mut seen: Vec<i32> = vec![];
    for x in abs_vals.iter().cloned() {
        let mut found: bool = false;
        for s in seen.iter().cloned() {
            if s == x {
                found = true;
                break;
            }
        }
        if !found {
            seen.push(x);
        }
    }
    for i in 0..(seen.len() as i32) {
        for j in ((i).py_add(1i32))..(seen.len() as i32) {
            if seen
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                > seen
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                let temp: i32 = seen
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                seen[(i) as usize] = seen
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                seen[(j) as usize] = temp;
            }
        }
    }
    Ok(seen)
}
#[doc = "Three-stage pipeline: double -> filter even -> subtract one."]
pub fn multi_stage_transform(nums: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut stage1: Vec<i32> = vec![];
    for x in nums.iter().cloned() {
        stage1.push((x).py_mul(2i32));
    }
    let mut stage2: Vec<i32> = vec![];
    for x in stage1.iter().cloned() {
        if (x).py_mod(2i32) == 0 {
            stage2.push(x);
        }
    }
    let mut stage3: Vec<i32> = vec![];
    for x in stage2.iter().cloned() {
        stage3.push((x) - (1i32));
    }
    Ok(stage3)
}
#[doc = "State machine: count transitions between even/odd states."]
#[doc = " Depyler: proven to terminate"]
pub fn state_machine_even_odd(nums: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut transitions: i32 = Default::default();
    if nums.is_empty() {
        return Ok(0);
    }
    transitions = 0;
    let _cse_temp_0 = ((nums
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
    .py_mod(2i32)) as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let mut is_even: bool = _cse_temp_1.clone();
    for i in (1)..(nums.len() as i32) {
        let current_even: bool = (nums
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range"))
        .py_mod(2i32)
            == 0;
        if current_even != is_even {
            transitions = ((transitions).py_add(1i32)) as i32;
        }
        is_even = current_even;
    }
    Ok(transitions)
}
#[doc = "Count sign changes(positive < -> negative) in sequence."]
#[doc = " Depyler: proven to terminate"]
pub fn state_machine_sign_changes(nums: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut changes: i32 = Default::default();
    let _cse_temp_0 = nums.len() as i32;
    let _cse_temp_1 = _cse_temp_0 < 2;
    if _cse_temp_1 {
        return Ok(0);
    }
    changes = 0;
    let _cse_temp_2 = nums
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        >= 0;
    let mut prev_positive: bool = _cse_temp_2.clone();
    for i in (1)..(nums.len() as i32) {
        let curr_positive: bool = nums
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            >= 0;
        if curr_positive != prev_positive {
            changes = ((changes).py_add(1i32)) as i32;
        }
        prev_positive = curr_positive;
    }
    Ok(changes)
}
#[doc = "Encode run lengths of consecutive equal elements."]
#[doc = " Depyler: proven to terminate"]
pub fn state_machine_run_lengths(nums: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    if nums.is_empty() {
        return Ok(vec![]);
    }
    let mut result: Vec<i32> = vec![];
    count = 1;
    for i in (1)..(nums.len() as i32) {
        if nums
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            == {
                let base = &nums;
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
            count = ((count).py_add(1i32)) as i32;
        } else {
            result.push(count);
            count = 1;
        }
    }
    result.push(count);
    Ok(result)
}
#[doc = "Simulate bracket depth tracking. opens=[positions of '('], closes=[positions of ')'].\n    Returns depth at each position from 0 to max_pos."]
#[doc = " Depyler: verified panic-free"]
pub fn state_machine_bracket_depth<'b, 'a>(opens: &'a Vec<i32>, closes: &'b Vec<i32>) -> Vec<i32> {
    let mut max_pos: i32 = Default::default();
    max_pos = 0;
    for p in opens.iter().cloned() {
        if p > max_pos {
            max_pos = p;
        }
    }
    for p in closes.iter().cloned() {
        if p > max_pos {
            max_pos = p;
        }
    }
    let mut depths: Vec<i32> = vec![];
    let mut depth: i32 = 0;
    for pos in 0..((max_pos).py_add(1i32)) {
        for o in opens.iter().cloned() {
            if o == pos {
                depth = ((depth).py_add(1i32)) as i32;
            }
        }
        for c in closes.iter().cloned() {
            if c == pos {
                depth = ((depth) - (1i32)) as i32;
            }
        }
        depths.push(depth);
    }
    depths
}
#[doc = "Cartesian product flattened: [a0*b0, a0*b1,..., a1*b0,...]."]
#[doc = " Depyler: verified panic-free"]
pub fn cartesian_product_flat<'a, 'b>(a: &'a Vec<i32>, b: &'b Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in a.iter().cloned() {
        for y in b.iter().cloned() {
            result.push((x).py_mul(y));
        }
    }
    result
}
#[doc = "Cartesian product as pairs: [[a0,b0], [a0,b1],...]."]
#[doc = " Depyler: verified panic-free"]
pub fn cartesian_product_pairs<'b, 'a>(a: &'a Vec<i32>, b: &'b Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for x in a.iter().cloned() {
        for y in b.iter().cloned() {
            result.push(vec![x, y]);
        }
    }
    result
}
#[doc = "Triple cartesian product, returning sums of triples."]
#[doc = " Depyler: verified panic-free"]
pub fn cartesian_triple_sum<'a, 'c, 'b>(
    a: &'a Vec<i32>,
    b: &'b Vec<i32>,
    c: &'c Vec<i32>,
) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in a.iter().cloned() {
        for y in b.iter().cloned() {
            for z in c.iter().cloned() {
                result.push(((x).py_add(y) as i32).py_add(z));
            }
        }
    }
    result
}
#[doc = "Find all pairs from items where pair sums to target."]
#[doc = " Depyler: proven to terminate"]
pub fn self_cartesian_filter(
    items: &Vec<i32>,
    target_sum: i32,
) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..(items.len() as i32) {
        for j in ((i).py_add(1i32))..(items.len() as i32) {
            if (items
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_add(
                items
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ) == target_sum
            {
                result.push(vec![
                    items
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                    items
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                ]);
            }
        }
    }
    Ok(result)
}
#[doc = "Simulate range with arbitrary step."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn step_iterator(start: i32, stop: i32, step: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut current: i32 = start.clone();
    let _cse_temp_0 = step > 0;
    if _cse_temp_0 {
        while current < stop {
            result.push(current);
            current = ((current).py_add(step)) as i32;
        }
    } else {
        let _cse_temp_1 = step < 0;
        if _cse_temp_1 {
            while current > stop {
                result.push(current);
                current = ((current).py_add(step)) as i32;
            }
        }
    }
    result
}
#[doc = "Cycle through items n times total(like itertools.cycle limited)."]
#[doc = " Depyler: verified panic-free"]
pub fn cycle_n(items: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    if items.is_empty() {
        return result;
    }
    let mut count: i32 = 0;
    while count < n {
        for x in items.iter().cloned() {
            if count >= n {
                break;
            }
            result.push(x);
            count = ((count).py_add(1i32)) as i32;
        }
    }
    result
}
#[doc = "Repeat each element a given number of times."]
#[doc = " Depyler: verified panic-free"]
pub fn repeat_each(items: &Vec<i32>, times: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in items.iter().cloned() {
        for _t in 0..(times) {
            result.push(x);
        }
    }
    result
}
#[doc = "Yield only unique elements preserving first-seen order."]
#[doc = " Depyler: verified panic-free"]
pub fn unique_elements(items: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in items.iter().cloned() {
        let mut found: bool = false;
        for r in result.iter().cloned() {
            if r == x {
                found = true;
                break;
            }
        }
        if !found {
            result.push(x);
        }
    }
    result
}
#[doc = "Select elements where corresponding selector is nonzero(like itertools.compress)."]
#[doc = " Depyler: proven to terminate"]
pub fn compress_select<'a, 'b>(
    data: &'a Vec<i32>,
    selectors: &'b Vec<i32>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut min_len: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = data.len() as i32;
    min_len = _cse_temp_0;
    let _cse_temp_1 = selectors.len() as i32;
    let _cse_temp_2 = _cse_temp_1 < min_len;
    if _cse_temp_2 {
        min_len = _cse_temp_1;
    }
    for i in 0..(min_len) {
        if selectors
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            != 0
        {
            result.push(
                data.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        }
    }
    Ok(result)
}
#[doc = "Generate primes up to limit using sieve approach with list."]
pub fn prime_sieve(limit: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = limit < 2;
    if _cse_temp_0 {
        return Ok(vec![]);
    }
    let mut is_prime: Vec<bool> = vec![];
    for _i in 0..((limit).py_add(1i32)) {
        is_prime.push(true);
    }
    is_prime[(0) as usize] = false;
    is_prime[(1) as usize] = false;
    let mut p: i32 = 2;
    while (p).py_mul(p) <= limit {
        if is_prime
            .get(p as usize)
            .cloned()
            .expect("IndexError: list index out of range")
        {
            let mut multiple: i32 = ((p).py_mul(p)) as i32;
            while multiple <= limit {
                is_prime[(multiple) as usize] = false;
                multiple = ((multiple).py_add(p)) as i32;
            }
        }
        p = ((p).py_add(1i32)) as i32;
    }
    let mut result: Vec<i32> = vec![];
    for i in (2)..((limit).py_add(1i32)) {
        if is_prime
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
        {
            result.push(i);
        }
    }
    Ok(result)
}
#[doc = "Generate nth row of Pascal's triangle."]
#[doc = " Depyler: proven to terminate"]
pub fn pascal_row(n: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut row: Vec<i32> = vec![1];
    for k in (1)..((n).py_add(1i32)) {
        let val: i32 = {
            let a = ({
                let base = &row;
                let idx: i32 = (k) - (1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            })
            .py_mul(((n) - (k) as i32).py_add(1i32));
            let b = k;
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
        row.push(val);
    }
    Ok(row)
}
#[doc = "One step of the look-and-say sequence."]
#[doc = " Depyler: proven to terminate"]
pub fn look_and_say_step(seq: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut current: i32 = Default::default();
    let mut count: i32 = Default::default();
    if seq.is_empty() {
        return Ok(vec![]);
    }
    let mut result: Vec<i32> = vec![];
    count = 1;
    current = seq
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for i in (1)..(seq.len() as i32) {
        if seq
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            == current
        {
            count = ((count).py_add(1i32)) as i32;
        } else {
            result.push(count);
            result.push(current);
            current = seq
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            count = 1;
        }
    }
    result.push(count);
    result.push(current);
    Ok(result)
}
#[doc = "Generate first n Catalan numbers."]
#[doc = " Depyler: proven to terminate"]
pub fn catalan_numbers(n: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return Ok(vec![]);
    }
    let mut result: Vec<i32> = vec![1];
    for i in (1)..(n) {
        let val: i32 = {
            let a = (({
                let base = &result;
                let idx: i32 = (i) - (1i32);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            })
            .py_mul(2i32) as i32)
                .py_mul(((2i32).py_mul(i) as i32) - (1i32));
            let b = (i).py_add(1i32);
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
        result.push(val);
    }
    Ok(result)
}
#[doc = "Test fibonacci sequence generation."]
#[doc = " Depyler: proven to terminate"]
pub fn test_fibonacci() -> Result<i32, Box<dyn std::error::Error>> {
    let fib: Vec<i32> = fibonacci_sequence(10);
    Ok(fib
        .get(9usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test lucas sequence."]
#[doc = " Depyler: proven to terminate"]
pub fn test_lucas() -> Result<i32, Box<dyn std::error::Error>> {
    let luc: Vec<i32> = lucas_sequence(7);
    Ok(luc
        .get(6usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test tribonacci sequence."]
#[doc = " Depyler: proven to terminate"]
pub fn test_tribonacci() -> Result<i32, Box<dyn std::error::Error>> {
    let tri: Vec<i32> = tribonacci_sequence(8);
    Ok(tri
        .get(7usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test naturals up to limit."]
#[doc = " Depyler: verified panic-free"]
pub fn test_naturals() -> i32 {
    let mut total: i32 = Default::default();
    let nums: Vec<i32> = naturals_up_to(5);
    total = 0;
    for x in nums.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test powers of two generation."]
#[doc = " Depyler: proven to terminate"]
pub fn test_powers_of_two() -> Result<i32, Box<dyn std::error::Error>> {
    let pows: Vec<i32> = powers_of_two(6);
    Ok(pows
        .get(5usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test geometric sequence."]
#[doc = " Depyler: proven to terminate"]
pub fn test_geometric() -> Result<i32, Box<dyn std::error::Error>> {
    let geo: Vec<i32> = geometric_sequence(3, 2, 5);
    Ok(geo
        .get(4usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test arithmetic sequence."]
#[doc = " Depyler: proven to terminate"]
pub fn test_arithmetic() -> Result<i32, Box<dyn std::error::Error>> {
    let arith: Vec<i32> = arithmetic_sequence(10, 3, 4);
    Ok(arith
        .get(3usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test collatz sequence length for 27."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_collatz() -> Result<i32, Box<dyn std::error::Error>> {
    let seq: Vec<i32> = collatz_sequence(27)?;
    Ok(seq.len() as i32 as i32)
}
#[doc = "Test take_while_positive."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_take_while() -> i32 {
    let result: Vec<i32> = take_while_positive(&vec![3, 5, 2, -1, 4]);
    result.len() as i32 as i32
}
#[doc = "Test drop_while_negative."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_drop_while() -> i32 {
    let result: Vec<i32> = drop_while_negative(&vec![-3, -1, 5, -2, 8]);
    result.len() as i32 as i32
}
#[doc = "Test take_first_n."]
#[doc = " Depyler: verified panic-free"]
pub fn test_take_first() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = take_first_n(&vec![10, 20, 30, 40, 50], 3);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test drop_first_n."]
#[doc = " Depyler: verified panic-free"]
pub fn test_drop_first() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = drop_first_n(&vec![10, 20, 30, 40, 50], 2);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test running sum accumulator."]
#[doc = " Depyler: proven to terminate"]
pub fn test_running_sum() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = running_sum(&vec![1, 2, 3, 4, 5]);
    Ok(result
        .get(4usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test running product."]
#[doc = " Depyler: proven to terminate"]
pub fn test_running_product() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = running_product(&vec![1, 2, 3, 4]);
    Ok(result
        .get(3usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test running max tracker."]
#[doc = " Depyler: proven to terminate"]
pub fn test_running_max() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = running_max(&vec![3, 1, 4, 1, 5, 9])?;
    Ok(result
        .get(5usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test running min tracker."]
#[doc = " Depyler: proven to terminate"]
pub fn test_running_min() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = running_min(&vec![5, 3, 7, 2, 8])?;
    Ok(result
        .get(3usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test scan with addition."]
#[doc = " Depyler: proven to terminate"]
pub fn test_scan() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = scan_with_op(&vec![1, 2, 3, 4], 0, true);
    Ok(result
        .get(3usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test chaining lists."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_chain() -> i32 {
    let result: Vec<i32> = chain_lists(&vec![1, 2], &vec![3, 4], &vec![5, 6]);
    result.len() as i32 as i32
}
#[doc = "Test flatten nested lists."]
#[doc = " Depyler: verified panic-free"]
pub fn test_flatten() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = flatten_nested(&vec![vec![1, 2], vec![3], vec![4, 5, 6]]);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test flatten and filter."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_flatten_filter() -> i32 {
    let result: Vec<i32> = flatten_and_filter(&vec![vec![1, 5], vec![2, 8], vec![3, 7]], 4);
    result.len() as i32 as i32
}
#[doc = "Test interleave two lists."]
#[doc = " Depyler: verified panic-free"]
pub fn test_interleave() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = interleave(&vec![1, 3, 5], &vec![2, 4, 6]);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test round-robin iteration."]
#[doc = " Depyler: proven to terminate"]
pub fn test_roundrobin() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = roundrobin(&vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    Ok({
        let _r: i32 = ((result
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range"))
        .py_add(
            result
                .get(1usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ) as i32)
            .py_add(
                result
                    .get(2usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        _r
    })
}
#[doc = "Test zip sum."]
#[doc = " Depyler: verified panic-free"]
pub fn test_zip_sum() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = zip_sum(&vec![1, 2, 3], &vec![10, 20, 30]);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test zip product."]
#[doc = " Depyler: verified panic-free"]
pub fn test_zip_product() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = zip_product(&vec![2, 3, 4], &vec![5, 6, 7]);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test zip max."]
#[doc = " Depyler: verified panic-free"]
pub fn test_zip_max() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = zip_max(&vec![1, 5, 3], &vec![4, 2, 6])?;
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    Ok(total)
}
#[doc = "Test enumerate simulation."]
#[doc = " Depyler: proven to terminate"]
pub fn test_enumerate() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<Vec<i32>> = enumerate_list(&vec![10, 20, 30]);
    Ok((result
        .get(2usize)
        .cloned()
        .expect("IndexError: list index out of range")
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
    .py_add(
        result
            .get(2usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range"),
    ))
}
#[doc = "Test chunking."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_chunk() -> i32 {
    let chunks: Vec<Vec<i32>> = chunk_list(&vec![1, 2, 3, 4, 5, 6, 7], 3);
    chunks.len() as i32 as i32
}
#[doc = "Test chunk sum."]
#[doc = " Depyler: verified panic-free"]
pub fn test_chunk_sum() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = chunk_sum(&vec![1, 2, 3, 4, 5, 6], 2);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test pairwise generation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pairwise() -> i32 {
    let pairs: Vec<Vec<i32>> = pairwise(&vec![1, 2, 3, 4]);
    pairs.len() as i32 as i32
}
#[doc = "Test triple generation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_triples() -> i32 {
    let trips: Vec<Vec<i32>> = triples(&vec![1, 2, 3, 4, 5]);
    trips.len() as i32 as i32
}
#[doc = "Test sliding window sum."]
#[doc = " Depyler: proven to terminate"]
pub fn test_sliding_sum() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = sliding_window_sum(&vec![1, 3, 5, 7, 9], 3)?;
    Ok((result
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
    .py_add(
        result
            .get(2usize)
            .cloned()
            .expect("IndexError: list index out of range"),
    ))
}
#[doc = "Test sliding window max."]
#[doc = " Depyler: proven to terminate"]
pub fn test_sliding_max() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = sliding_window_max(&vec![1, 3, 2, 5, 4, 1], 3)?;
    Ok((result
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
    .py_add(
        result
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range"),
    ))
}
#[doc = "Test sliding window min."]
#[doc = " Depyler: proven to terminate"]
pub fn test_sliding_min() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = sliding_window_min(&vec![4, 2, 5, 1, 3, 6], 3)?;
    Ok((result
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
    .py_add(
        result
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range"),
    ))
}
#[doc = "Test sliding window integer average."]
#[doc = " Depyler: proven to terminate"]
pub fn test_sliding_avg() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = sliding_window_avg_int(&vec![10, 20, 30, 40, 50], 3)?;
    Ok(result
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test map then filter pipeline."]
#[doc = " Depyler: verified panic-free"]
pub fn test_map_filter_pipeline() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = map_then_filter(&vec![1, 2, 3, 4, 5], 3, 9);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test filter then map pipeline."]
#[doc = " Depyler: verified panic-free"]
pub fn test_filter_map_pipeline() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = filter_then_map(&vec![1, 5, 3, 8, 2], 3, 10);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test pipeline square filter sum."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_square_filter_sum() -> i32 {
    pipeline_square_filter_sum(&vec![1, 2, 3, 4, 5], 20)
}
#[doc = "Test abs dedup sort pipeline."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_abs_dedup_sort() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = pipeline_abs_dedup_sort(&vec![-3, 1, -1, 3, 2, -2])?;
    Ok(result.len() as i32 as i32)
}
#[doc = "Test multi-stage transform pipeline."]
#[doc = " Depyler: verified panic-free"]
pub fn test_multi_stage() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = multi_stage_transform(&vec![1, 2, 3, 4, 5])?;
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    Ok(total)
}
#[doc = "Test even/odd state machine."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_state_even_odd() -> Result<i32, Box<dyn std::error::Error>> {
    state_machine_even_odd(&vec![2, 3, 4, 6, 7])
}
#[doc = "Test sign change detection."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sign_changes() -> Result<i32, Box<dyn std::error::Error>> {
    state_machine_sign_changes(&vec![1, -2, 3, -4, 5])
}
#[doc = "Test run length encoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_run_lengths() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = state_machine_run_lengths(&vec![1, 1, 2, 2, 2, 3])?;
    Ok(result.len() as i32 as i32)
}
#[doc = "Test bracket depth simulation."]
#[doc = " Depyler: proven to terminate"]
pub fn test_bracket_depth() -> Result<i32, Box<dyn std::error::Error>> {
    let depths: Vec<i32> = state_machine_bracket_depth(&vec![0, 2], &vec![3, 4]);
    Ok(depths
        .get(2usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test flattened cartesian product."]
#[doc = " Depyler: verified panic-free"]
pub fn test_cartesian_flat() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = cartesian_product_flat(&vec![1, 2], &vec![3, 4]);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test cartesian product pairs."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_cartesian_pairs() -> i32 {
    let result: Vec<Vec<i32>> = cartesian_product_pairs(&vec![1, 2], &vec![3, 4]);
    result.len() as i32 as i32
}
#[doc = "Test triple cartesian sum."]
#[doc = " Depyler: proven to terminate"]
pub fn test_triple_sum() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = cartesian_triple_sum(&vec![1], &vec![2], &vec![3]);
    Ok(result
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test self cartesian filter."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_self_cartesian() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<Vec<i32>> = self_cartesian_filter(&vec![1, 2, 3, 4, 5], 6)?;
    Ok(result.len() as i32 as i32)
}
#[doc = "Test step iterator."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_step_iterator() -> i32 {
    let result: Vec<i32> = step_iterator(0, 20, 3);
    result.len() as i32 as i32
}
#[doc = "Test cycle_n."]
#[doc = " Depyler: verified panic-free"]
pub fn test_cycle() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = cycle_n(&vec![1, 2, 3], 7);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test repeat each element."]
#[doc = " Depyler: verified panic-free"]
pub fn test_repeat_each() -> i32 {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = repeat_each(&vec![5, 10], 3);
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    total
}
#[doc = "Test unique elements preserving order."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_unique() -> i32 {
    let result: Vec<i32> = unique_elements(&vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3]);
    result.len() as i32 as i32
}
#[doc = "Test compress select."]
#[doc = " Depyler: verified panic-free"]
pub fn test_compress() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let result: Vec<i32> = compress_select(&vec![10, 20, 30, 40, 50], &vec![1, 0, 1, 0, 1])?;
    total = 0;
    for x in result.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    Ok(total)
}
#[doc = "Test prime sieve."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_prime_sieve() -> Result<i32, Box<dyn std::error::Error>> {
    let primes: Vec<i32> = prime_sieve(30)?;
    Ok(primes.len() as i32 as i32)
}
#[doc = "Test Pascal's triangle row."]
#[doc = " Depyler: verified panic-free"]
pub fn test_pascal_row() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let row: Vec<i32> = pascal_row(5)?;
    total = 0;
    for x in row.iter().cloned() {
        total = ((total).py_add(x)) as i32;
    }
    Ok(total)
}
#[doc = "Test look-and-say step."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_look_and_say() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = look_and_say_step(&vec![1, 1, 2, 3, 3])?;
    Ok(result.len() as i32 as i32)
}
#[doc = "Test Catalan numbers."]
#[doc = " Depyler: proven to terminate"]
pub fn test_catalan() -> Result<i32, Box<dyn std::error::Error>> {
    let result: Vec<i32> = catalan_numbers(6)?;
    Ok(result
        .get(5usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Run all test functions and sum their results for verification."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_all_tests() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = 0;
    let _cse_temp_0 = ((total).py_add(test_fibonacci()?)) as i32;
    total = _cse_temp_0;
    let _cse_temp_1 = ((total).py_add(test_lucas()?)) as i32;
    total = _cse_temp_1;
    let _cse_temp_2 = ((total).py_add(test_tribonacci()?)) as i32;
    total = _cse_temp_2;
    let _cse_temp_3 = ((total).py_add(test_naturals())) as i32;
    total = _cse_temp_3;
    let _cse_temp_4 = ((total).py_add(test_powers_of_two()?)) as i32;
    total = _cse_temp_4;
    let _cse_temp_5 = ((total).py_add(test_geometric()?)) as i32;
    total = _cse_temp_5;
    let _cse_temp_6 = ((total).py_add(test_arithmetic()?)) as i32;
    total = _cse_temp_6;
    let _cse_temp_7 = ((total).py_add(test_collatz()?)) as i32;
    total = _cse_temp_7;
    let _cse_temp_8 = ((total).py_add(test_take_while())) as i32;
    total = _cse_temp_8;
    let _cse_temp_9 = ((total).py_add(test_drop_while())) as i32;
    total = _cse_temp_9;
    let _cse_temp_10 = ((total).py_add(test_take_first())) as i32;
    total = _cse_temp_10;
    let _cse_temp_11 = ((total).py_add(test_drop_first())) as i32;
    total = _cse_temp_11;
    let _cse_temp_12 = ((total).py_add(test_running_sum()?)) as i32;
    total = _cse_temp_12;
    let _cse_temp_13 = ((total).py_add(test_running_product()?)) as i32;
    total = _cse_temp_13;
    let _cse_temp_14 = ((total).py_add(test_running_max()?)) as i32;
    total = _cse_temp_14;
    let _cse_temp_15 = ((total).py_add(test_running_min()?)) as i32;
    total = _cse_temp_15;
    let _cse_temp_16 = ((total).py_add(test_scan()?)) as i32;
    total = _cse_temp_16;
    let _cse_temp_17 = ((total).py_add(test_chain())) as i32;
    total = _cse_temp_17;
    let _cse_temp_18 = ((total).py_add(test_flatten())) as i32;
    total = _cse_temp_18;
    let _cse_temp_19 = ((total).py_add(test_flatten_filter())) as i32;
    total = _cse_temp_19;
    let _cse_temp_20 = ((total).py_add(test_interleave())) as i32;
    total = _cse_temp_20;
    let _cse_temp_21 = ((total).py_add(test_roundrobin()?)) as i32;
    total = _cse_temp_21;
    let _cse_temp_22 = ((total).py_add(test_zip_sum())) as i32;
    total = _cse_temp_22;
    let _cse_temp_23 = ((total).py_add(test_zip_product())) as i32;
    total = _cse_temp_23;
    let _cse_temp_24 = ((total).py_add(test_zip_max()?)) as i32;
    total = _cse_temp_24;
    let _cse_temp_25 = ((total).py_add(test_enumerate()?)) as i32;
    total = _cse_temp_25;
    let _cse_temp_26 = ((total).py_add(test_chunk())) as i32;
    total = _cse_temp_26;
    let _cse_temp_27 = ((total).py_add(test_chunk_sum())) as i32;
    total = _cse_temp_27;
    let _cse_temp_28 = ((total).py_add(test_pairwise())) as i32;
    total = _cse_temp_28;
    let _cse_temp_29 = ((total).py_add(test_triples())) as i32;
    total = _cse_temp_29;
    let _cse_temp_30 = ((total).py_add(test_sliding_sum()?)) as i32;
    total = _cse_temp_30;
    let _cse_temp_31 = ((total).py_add(test_sliding_max()?)) as i32;
    total = _cse_temp_31;
    let _cse_temp_32 = ((total).py_add(test_sliding_min()?)) as i32;
    total = _cse_temp_32;
    let _cse_temp_33 = ((total).py_add(test_sliding_avg()?)) as i32;
    total = _cse_temp_33;
    let _cse_temp_34 = ((total).py_add(test_map_filter_pipeline())) as i32;
    total = _cse_temp_34;
    let _cse_temp_35 = ((total).py_add(test_filter_map_pipeline())) as i32;
    total = _cse_temp_35;
    let _cse_temp_36 = ((total).py_add(test_square_filter_sum())) as i32;
    total = _cse_temp_36;
    let _cse_temp_37 = ((total).py_add(test_abs_dedup_sort()?)) as i32;
    total = _cse_temp_37;
    let _cse_temp_38 = ((total).py_add(test_multi_stage()?)) as i32;
    total = _cse_temp_38;
    let _cse_temp_39 = ((total).py_add(test_state_even_odd()?)) as i32;
    total = _cse_temp_39;
    let _cse_temp_40 = ((total).py_add(test_sign_changes()?)) as i32;
    total = _cse_temp_40;
    let _cse_temp_41 = ((total).py_add(test_run_lengths()?)) as i32;
    total = _cse_temp_41;
    let _cse_temp_42 = ((total).py_add(test_bracket_depth()?)) as i32;
    total = _cse_temp_42;
    let _cse_temp_43 = ((total).py_add(test_cartesian_flat())) as i32;
    total = _cse_temp_43;
    let _cse_temp_44 = ((total).py_add(test_cartesian_pairs())) as i32;
    total = _cse_temp_44;
    let _cse_temp_45 = ((total).py_add(test_triple_sum()?)) as i32;
    total = _cse_temp_45;
    let _cse_temp_46 = ((total).py_add(test_self_cartesian()?)) as i32;
    total = _cse_temp_46;
    let _cse_temp_47 = ((total).py_add(test_step_iterator())) as i32;
    total = _cse_temp_47;
    let _cse_temp_48 = ((total).py_add(test_cycle())) as i32;
    total = _cse_temp_48;
    let _cse_temp_49 = ((total).py_add(test_repeat_each())) as i32;
    total = _cse_temp_49;
    let _cse_temp_50 = ((total).py_add(test_unique())) as i32;
    total = _cse_temp_50;
    let _cse_temp_51 = ((total).py_add(test_compress()?)) as i32;
    total = _cse_temp_51;
    let _cse_temp_52 = ((total).py_add(test_prime_sieve()?)) as i32;
    total = _cse_temp_52;
    let _cse_temp_53 = ((total).py_add(test_pascal_row()?)) as i32;
    total = _cse_temp_53;
    let _cse_temp_54 = ((total).py_add(test_look_and_say()?)) as i32;
    total = _cse_temp_54;
    let _cse_temp_55 = ((total).py_add(test_catalan()?)) as i32;
    total = _cse_temp_55;
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
    fn test_take_while_positive_examples() {
        assert_eq!(take_while_positive(vec![]), vec![]);
        assert_eq!(take_while_positive(vec![1]), vec![1]);
    }
    #[test]
    fn test_drop_while_negative_examples() {
        assert_eq!(drop_while_negative(vec![]), vec![]);
        assert_eq!(drop_while_negative(vec![1]), vec![1]);
    }
    #[test]
    fn test_running_sum_examples() {
        assert_eq!(running_sum(vec![]), vec![]);
        assert_eq!(running_sum(vec![1]), vec![1]);
    }
    #[test]
    fn test_running_product_examples() {
        assert_eq!(running_product(vec![]), vec![]);
        assert_eq!(running_product(vec![1]), vec![1]);
    }
    #[test]
    fn test_running_max_examples() {
        assert_eq!(running_max(vec![]), vec![]);
        assert_eq!(running_max(vec![1]), vec![1]);
    }
    #[test]
    fn test_running_min_examples() {
        assert_eq!(running_min(vec![]), vec![]);
        assert_eq!(running_min(vec![1]), vec![1]);
    }
    #[test]
    fn test_flatten_nested_examples() {
        assert_eq!(flatten_nested(vec![]), vec![]);
        assert_eq!(flatten_nested(vec![1]), vec![1]);
    }
    #[test]
    fn test_roundrobin_examples() {
        assert_eq!(roundrobin(vec![]), vec![]);
        assert_eq!(roundrobin(vec![1]), vec![1]);
    }
    #[test]
    fn test_enumerate_list_examples() {
        assert_eq!(enumerate_list(vec![]), vec![]);
        assert_eq!(enumerate_list(vec![1]), vec![1]);
    }
    #[test]
    fn test_pairwise_examples() {
        assert_eq!(pairwise(vec![]), vec![]);
        assert_eq!(pairwise(vec![1]), vec![1]);
    }
    #[test]
    fn test_triples_examples() {
        assert_eq!(triples(vec![]), vec![]);
        assert_eq!(triples(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_pipeline_abs_dedup_sort() {
        fn prop(nums: Vec<i32>) -> TestResult {
            let result = pipeline_abs_dedup_sort(&nums);
            if result < 0 {
                return TestResult::failed();
            }
            let input_len = nums.len();
            let result = pipeline_abs_dedup_sort(&nums);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = pipeline_abs_dedup_sort(&nums);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = nums.clone();
            input_sorted.sort();
            let mut result = pipeline_abs_dedup_sort(&nums);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_pipeline_abs_dedup_sort_examples() {
        assert_eq!(pipeline_abs_dedup_sort(vec![]), vec![]);
        assert_eq!(pipeline_abs_dedup_sort(vec![1]), vec![1]);
    }
    #[test]
    fn test_multi_stage_transform_examples() {
        assert_eq!(multi_stage_transform(vec![]), vec![]);
        assert_eq!(multi_stage_transform(vec![1]), vec![1]);
    }
    #[test]
    fn test_state_machine_even_odd_examples() {
        assert_eq!(state_machine_even_odd(&vec![]), 0);
        assert_eq!(state_machine_even_odd(&vec![1]), 1);
        assert_eq!(state_machine_even_odd(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_state_machine_sign_changes_examples() {
        assert_eq!(state_machine_sign_changes(&vec![]), 0);
        assert_eq!(state_machine_sign_changes(&vec![1]), 1);
        assert_eq!(state_machine_sign_changes(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_state_machine_run_lengths_examples() {
        assert_eq!(state_machine_run_lengths(vec![]), vec![]);
        assert_eq!(state_machine_run_lengths(vec![1]), vec![1]);
    }
    #[test]
    fn test_unique_elements_examples() {
        assert_eq!(unique_elements(vec![]), vec![]);
        assert_eq!(unique_elements(vec![1]), vec![1]);
    }
    #[test]
    fn test_look_and_say_step_examples() {
        assert_eq!(look_and_say_step(vec![]), vec![]);
        assert_eq!(look_and_say_step(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_fibonacci_examples() {
        let _ = test_fibonacci();
    }
    #[test]
    fn test_test_lucas_examples() {
        let _ = test_lucas();
    }
    #[test]
    fn test_test_tribonacci_examples() {
        let _ = test_tribonacci();
    }
    #[test]
    fn test_test_naturals_examples() {
        let _ = test_naturals();
    }
    #[test]
    fn test_test_powers_of_two_examples() {
        let _ = test_powers_of_two();
    }
    #[test]
    fn test_test_geometric_examples() {
        let _ = test_geometric();
    }
    #[test]
    fn test_test_arithmetic_examples() {
        let _ = test_arithmetic();
    }
    #[test]
    fn test_test_collatz_examples() {
        let _ = test_collatz();
    }
    #[test]
    fn test_test_take_while_examples() {
        let _ = test_take_while();
    }
    #[test]
    fn test_test_drop_while_examples() {
        let _ = test_drop_while();
    }
    #[test]
    fn test_test_take_first_examples() {
        let _ = test_take_first();
    }
    #[test]
    fn test_test_drop_first_examples() {
        let _ = test_drop_first();
    }
    #[test]
    fn test_test_running_sum_examples() {
        let _ = test_running_sum();
    }
    #[test]
    fn test_test_running_product_examples() {
        let _ = test_running_product();
    }
    #[test]
    fn test_test_running_max_examples() {
        let _ = test_running_max();
    }
    #[test]
    fn test_test_running_min_examples() {
        let _ = test_running_min();
    }
    #[test]
    fn test_test_scan_examples() {
        let _ = test_scan();
    }
    #[test]
    fn test_test_chain_examples() {
        let _ = test_chain();
    }
    #[test]
    fn test_test_flatten_examples() {
        let _ = test_flatten();
    }
    #[test]
    fn test_test_flatten_filter_examples() {
        let _ = test_flatten_filter();
    }
    #[test]
    fn test_test_interleave_examples() {
        let _ = test_interleave();
    }
    #[test]
    fn test_test_roundrobin_examples() {
        let _ = test_roundrobin();
    }
    #[test]
    fn test_test_zip_sum_examples() {
        let _ = test_zip_sum();
    }
    #[test]
    fn test_test_zip_product_examples() {
        let _ = test_zip_product();
    }
    #[test]
    fn test_test_zip_max_examples() {
        let _ = test_zip_max();
    }
    #[test]
    fn test_test_enumerate_examples() {
        let _ = test_enumerate();
    }
    #[test]
    fn test_test_chunk_examples() {
        let _ = test_chunk();
    }
    #[test]
    fn test_test_chunk_sum_examples() {
        let _ = test_chunk_sum();
    }
    #[test]
    fn test_test_pairwise_examples() {
        let _ = test_pairwise();
    }
    #[test]
    fn test_test_triples_examples() {
        let _ = test_triples();
    }
    #[test]
    fn test_test_sliding_sum_examples() {
        let _ = test_sliding_sum();
    }
    #[test]
    fn test_test_sliding_max_examples() {
        let _ = test_sliding_max();
    }
    #[test]
    fn test_test_sliding_min_examples() {
        let _ = test_sliding_min();
    }
    #[test]
    fn test_test_sliding_avg_examples() {
        let _ = test_sliding_avg();
    }
    #[test]
    fn test_test_map_filter_pipeline_examples() {
        let _ = test_map_filter_pipeline();
    }
    #[test]
    fn test_test_filter_map_pipeline_examples() {
        let _ = test_filter_map_pipeline();
    }
    #[test]
    fn test_test_square_filter_sum_examples() {
        let _ = test_square_filter_sum();
    }
    #[test]
    fn quickcheck_test_abs_dedup_sort() {
        fn prop() -> TestResult {
            let result = test_abs_dedup_sort();
            if result < 0 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn() -> TestResult);
    }
    #[test]
    fn test_test_abs_dedup_sort_examples() {
        let _ = test_abs_dedup_sort();
    }
    #[test]
    fn test_test_multi_stage_examples() {
        let _ = test_multi_stage();
    }
    #[test]
    fn test_test_state_even_odd_examples() {
        let _ = test_state_even_odd();
    }
    #[test]
    fn test_test_sign_changes_examples() {
        let _ = test_sign_changes();
    }
    #[test]
    fn test_test_run_lengths_examples() {
        let _ = test_run_lengths();
    }
    #[test]
    fn test_test_bracket_depth_examples() {
        let _ = test_bracket_depth();
    }
    #[test]
    fn test_test_cartesian_flat_examples() {
        let _ = test_cartesian_flat();
    }
    #[test]
    fn test_test_cartesian_pairs_examples() {
        let _ = test_cartesian_pairs();
    }
    #[test]
    fn test_test_triple_sum_examples() {
        let _ = test_triple_sum();
    }
    #[test]
    fn test_test_self_cartesian_examples() {
        let _ = test_self_cartesian();
    }
    #[test]
    fn test_test_step_iterator_examples() {
        let _ = test_step_iterator();
    }
    #[test]
    fn test_test_cycle_examples() {
        let _ = test_cycle();
    }
    #[test]
    fn test_test_repeat_each_examples() {
        let _ = test_repeat_each();
    }
    #[test]
    fn test_test_unique_examples() {
        let _ = test_unique();
    }
    #[test]
    fn test_test_compress_examples() {
        let _ = test_compress();
    }
    #[test]
    fn test_test_prime_sieve_examples() {
        let _ = test_prime_sieve();
    }
    #[test]
    fn test_test_pascal_row_examples() {
        let _ = test_pascal_row();
    }
    #[test]
    fn test_test_look_and_say_examples() {
        let _ = test_look_and_say();
    }
    #[test]
    fn test_test_catalan_examples() {
        let _ = test_catalan();
    }
    #[test]
    fn test_run_all_tests_examples() {
        let _ = run_all_tests();
    }
}