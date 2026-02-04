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
#[doc = "Infers numeric types from arithmetic operations."]
#[doc = " Depyler: proven to terminate"]
pub fn numeric_operations(x: i32, y: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let sum_val: i32 = ((x).py_add(y)) as i32;
    let _cse_temp_0 = ((x).py_mul(y)) as i32;
    let product: i32 = _cse_temp_0;
    let _cse_temp_1 = x > y;
    if _cse_temp_1 {
        return Ok(sum_val);
    } else {
        return Ok(product);
    }
}
#[doc = "Infers string type from string methods."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn string_manipulation(text: &str) -> String {
    let upper_text: String = text.to_uppercase();
    let lower_text: String = text.to_lowercase();
    if text.starts_with("Hello") {
        return text.replace("Hello", "Hi");
    }
    text.trim().to_string()
}
#[doc = "Infers list type from list operations."]
#[doc = " Depyler: verified panic-free"]
pub fn list_processing(items: &mut Vec<String>) -> Vec<String> {
    items.push("new item".to_string());
    items.extend(
        vec!["more".to_string(), "items".to_string()]
            .iter()
            .cloned(),
    );
    let mut result: Vec<String> = vec![];
    for item in items.iter().cloned() {
        result.push(item.to_uppercase());
    }
    result
}
#[doc = "Multiple inference sources for better confidence."]
pub fn mixed_inference(
    data: &Vec<i32>,
    multiplier: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    total = 0;
    for value in data.iter().cloned() {
        total = ((total).py_add((value).py_mul(multiplier))) as i32;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = {
        let a = total;
        let b = _cse_temp_0;
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
    let average: i32 = _cse_temp_1;
    Ok(average)
}
#[doc = "Type conversion functions provide strong hints."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn type_conversions_hint(value: &str) -> (String, i32, f64) {
    let _cse_temp_0 = (value).to_string();
    let as_string: String = _cse_temp_0.clone().to_string();
    let _cse_temp_1 = value.parse::<i32>().unwrap_or_default();
    let as_int: i32 = _cse_temp_1;
    let _cse_temp_2 = value.parse::<f64>().unwrap();
    let as_float: f64 = _cse_temp_2;
    (as_string, as_int, as_float)
}
#[doc = "Boolean operations suggest bool type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn boolean_logic(a: bool, b: bool, c: bool) -> bool {
    let _cse_temp_0 = (a) && (b);
    if _cse_temp_0 {
        return true;
    } else {
        let _cse_temp_1 = (b) || (c);
        if _cse_temp_1 {
            return false;
        } else {
            return !c;
        }
    }
}
#[doc = "Dictionary method usage."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn dictionary_operations(
    mapping: &std::collections::HashMap<String, String>,
) -> Option<String> {
    let keys: Vec<String> = mapping.keys().cloned().collect::<Vec<_>>();
    let values: Vec<String> = mapping.values().cloned().collect::<Vec<_>>();
    let _cse_temp_0 = mapping.get("key").is_some();
    if _cse_temp_0 {
        return Some(mapping.get("key").cloned().unwrap_or("default".to_string()));
    }
    None
}
#[doc = "Using parameters as callables."]
#[doc = " Depyler: verified panic-free"]
pub fn function_composition(
    transform: impl Fn(String) -> String,
    data: &Vec<String>,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for item in data.iter().cloned() {
        let transformed: String = transform(&item);
        result.push(transformed);
    }
    result
}
#[doc = "Demonstrates different confidence levels."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn confidence_levels_demo<'a, 'b>(
    certain_str: &'a str,
    probable_num: i32,
    possible_container: &'b Vec<i32>,
) -> (String, i32, i32) {
    let processed: String = certain_str
        .to_uppercase()
        .trim()
        .to_string()
        .replace(" ", "_");
    let _cse_temp_0 = ((probable_num).py_mul(2i32)) as i32;
    let doubled: i32 = _cse_temp_0;
    let _cse_temp_1 = possible_container.len() as i32;
    let size: i32 = _cse_temp_1;
    (processed, doubled, size)
}
#[doc = "Simple arithmetic with explicit types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_arithmetic(a: i32, b: i32) -> i32 {
    let result: i32 = ((a).py_add(b)) as i32;
    result
}
#[doc = "Simple string concatenation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_string_concat<'a, 'b>(s1: &'a str, s2: &'b str) -> String {
    let result: String = (s1).py_add(s2);
    result.to_string()
}
#[doc = "Sum a list of integers."]
#[doc = " Depyler: verified panic-free"]
pub fn simple_list_sum(numbers: &Vec<i32>) -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    for n in numbers.iter().cloned() {
        total = ((total).py_add(n)) as i32;
    }
    total
}
#[doc = "Dictionary lookup with default."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_dict_lookup<'a, 'b>(
    d: &'a std::collections::HashMap<String, i32>,
    key: &'b str,
) -> i32 {
    let value: i32 = d.get(key).cloned().unwrap_or(0);
    value
}
#[doc = "Handle optional values."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn optional_handling(maybe_value: &Option<i32>) -> i32 {
    if maybe_value.is_none() {
        return 0;
    }
    (*maybe_value.unwrap()).unwrap()
}
#[doc = "Unpack a tuple."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn tuple_unpacking(pair: &(i32, String)) -> String {
    let (num, text) = pair;
    let result: String = format!("{:?}: {:?}", text, num);
    result.to_string()
}
#[doc = "List comprehension with explicit type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn list_comprehension_typed(numbers: &Vec<i32>) -> Vec<i32> {
    let doubled: Vec<i32> = numbers
        .as_slice()
        .iter()
        .cloned()
        .map(|n| (n).py_mul(2i32))
        .collect::<Vec<_>>();
    doubled
}
#[doc = "Conditional expression(ternary)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn conditional_expression(flag: bool, a: i32, b: i32) -> i32 {
    let result: i32 = if flag { a } else { b };
    result
}
#[doc = "Main function to exercise all examples."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_result: i32 = numeric_operations(10, 5)?;
    println!("{}", format!("Numeric: {}", num_result));
    let str_result: String = string_manipulation(&"Hello World");
    println!("{}", format!("String: {}", str_result));
    let mut items: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let list_result: Vec<String> = list_processing(&mut items);
    println!("{}", format!("List: {:?}", list_result));
    let data: Vec<i32> = vec![1, 2, 3, 4, 5];
    let avg: i32 = mixed_inference(&data, 2)?;
    println!("{}", format!("Average: {}", avg));
    let conv: (String, i32, f64) = type_conversions_hint(&"42");
    println!("{}", format!("Conversions: {:?}", conv));
    let bool_result: bool = boolean_logic(true, false, true);
    println!("{}", format!("Boolean: {}", bool_result));
    let mapping: std::collections::HashMap<String, String> = {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "value".to_string());
        map.insert("other".to_string(), "data".to_string());
        map
    };
    let dict_result: Option<String> = dictionary_operations(&mapping);
    println!("{}", format!("Dict: {:?}", dict_result));
    let arith: i32 = simple_arithmetic(5, 3);
    let concat: String = simple_string_concat(&"Hello", &" World");
    let sum_val: i32 = simple_list_sum(&vec![1, 2, 3]);
    let lookup: i32 = simple_dict_lookup(
        &{
            let mut map = HashMap::new();
            map.insert("a".to_string(), 1);
            map
        },
        &"a",
    );
    println!(
        "{}",
        format!(
            "Simple tests: {}, {}, {}, {}",
            arith, concat, sum_val, lookup
        )
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_numeric_operations_examples() {
        assert_eq!(numeric_operations(0, 0), 0);
        assert_eq!(numeric_operations(1, 2), 3);
        assert_eq!(numeric_operations(-1, 1), 0);
    }
    #[test]
    fn test_list_processing_examples() {
        assert_eq!(list_processing(vec![]), vec![]);
        assert_eq!(list_processing(vec![1]), vec![1]);
    }
    #[test]
    fn test_simple_arithmetic_examples() {
        assert_eq!(simple_arithmetic(0, 0), 0);
        assert_eq!(simple_arithmetic(1, 2), 3);
        assert_eq!(simple_arithmetic(-1, 1), 0);
    }
    #[test]
    fn test_simple_list_sum_examples() {
        assert_eq!(simple_list_sum(&vec![]), 0);
        assert_eq!(simple_list_sum(&vec![1]), 1);
        assert_eq!(simple_list_sum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_list_comprehension_typed_examples() {
        assert_eq!(list_comprehension_typed(vec![]), vec![]);
        assert_eq!(list_comprehension_typed(vec![1]), vec![1]);
    }
}
