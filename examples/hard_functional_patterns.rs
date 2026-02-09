#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR_X: &'static str = "x";
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
#[doc = "Apply transformation to each element(simulates map with closure)."]
#[doc = " Depyler: verified panic-free"]
pub fn manual_map(vals: &Vec<i32>, addend: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        result.push((v).py_add(addend));
    }
    result
}
#[doc = "Test manual map over a list."]
#[doc = " Depyler: verified panic-free"]
pub fn test_manual_map() -> i32 {
    let mut total: i32 = Default::default();
    let mapped: Vec<i32> = manual_map(&vec![1, 2, 3, 4, 5], 10);
    total = 0;
    for v in mapped.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Filter elements keeping only positives."]
#[doc = " Depyler: verified panic-free"]
pub fn manual_filter_positive(vals: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        if v > 0 {
            result.push(v);
        }
    }
    result
}
#[doc = "Test filtering negative values out."]
#[doc = " Depyler: verified panic-free"]
pub fn test_manual_filter() -> i32 {
    let mut total: i32 = Default::default();
    let filtered: Vec<i32> = manual_filter_positive(&vec![-3, -1, 0, 2, 5, -7, 8]);
    total = 0;
    for v in filtered.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Left fold with addition as the combining operation."]
#[doc = " Depyler: verified panic-free"]
pub fn fold_left_sum(vals: &Vec<i32>, init: i32) -> i32 {
    let mut acc: i32 = Default::default();
    acc = init;
    for v in vals.iter().cloned() {
        acc = ((acc).py_add(v)) as i32;
    }
    acc
}
#[doc = "Left fold with multiplication as the combining operation."]
#[doc = " Depyler: verified panic-free"]
pub fn fold_left_product(vals: &Vec<i32>, init: i32) -> i32 {
    let mut acc: i32 = Default::default();
    acc = init;
    for v in vals.iter().cloned() {
        acc = ((acc).py_mul(v)) as i32;
    }
    acc
}
#[doc = "Test fold left with sum and product."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_fold_left() -> i32 {
    let s: i32 = fold_left_sum(&vec![1, 2, 3, 4, 5], 0);
    let p: i32 = fold_left_product(&vec![1, 2, 3, 4], 1);
    (s).py_add(p)
}
#[doc = "Running prefix sum(scan operation)."]
#[doc = " Depyler: verified panic-free"]
pub fn scan_sum(vals: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut acc: i32 = 0;
    for v in vals.iter().cloned() {
        acc = ((acc).py_add(v)) as i32;
        result.push(acc);
    }
    result
}
#[doc = "Test scan produces correct running totals."]
#[doc = " Depyler: proven to terminate"]
pub fn test_scan() -> Result<i32, Box<dyn std::error::Error>> {
    let scanned: Vec<i32> = scan_sum(&vec![1, 2, 3, 4, 5]);
    Ok({
        let base = &scanned;
        let idx: i32 = (scanned.len() as i32) - (1i32);
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
#[doc = "Apply an increment operation twice(simulates compose(f,f))."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn apply_twice(x: i32, step: i32) -> i32 {
    let first: i32 = ((x).py_add(step)) as i32;
    let second: i32 = ((first).py_add(step)) as i32;
    second
}
#[doc = "Apply an increment operation three times."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn apply_thrice(x: i32, step: i32) -> i32 {
    let first: i32 = ((x).py_add(step)) as i32;
    let second: i32 = ((first).py_add(step)) as i32;
    let third: i32 = ((second).py_add(step)) as i32;
    third
}
#[doc = "Test function composition simulation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_composition() -> i32 {
    let a: i32 = apply_twice(10, 3);
    let b: i32 = apply_thrice(10, 3);
    (a).py_add(b)
}
#[doc = "Simulates partial application: add_five = partial(add, 5)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add_partial(a: i32, b: i32) -> i32 {
    (a).py_add(b)
}
#[doc = "Simulates partial application for multiplication."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn multiply_partial(a: i32, b: i32) -> i32 {
    (a).py_mul(b)
}
#[doc = "Dispatch to simulate partial application of binary ops."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn apply_binary_op(x: i32, y: i32, use_add: bool) -> i32 {
    if use_add {
        return add_partial(x, y);
    }
    multiply_partial(x, y)
}
#[doc = "Test partial application simulation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_partial_application() -> i32 {
    let r1: i32 = apply_binary_op(5, 10, true);
    let r2: i32 = apply_binary_op(3, 7, false);
    (r1).py_add(r2)
}
#[doc = "Simulate curried addition: curry(add)(a)(b)(c) = a+b+c."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn curried_add_step1(a: i32, b: i32, c: i32) -> i32 {
    {
        let _r: i32 = ((a).py_add(b) as i32).py_add(c);
        _r
    }
}
#[doc = "Simulate curried multiplication."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn curried_multiply_step1(a: i32, b: i32, c: i32) -> i32 {
    {
        let _r: i32 = ((a).py_mul(b) as i32).py_mul(c);
        _r
    }
}
#[doc = "Test currying simulation with 3-arg functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_currying() -> i32 {
    let r1: i32 = curried_add_step1(1, 2, 3);
    let r2: i32 = curried_multiply_step1(2, 3, 4);
    (r1).py_add(r2)
}
#[doc = "Chain: filter positives -> double each -> sum all."]
#[doc = " Depyler: verified panic-free"]
pub fn pipeline_transform(vals: &Vec<i32>) -> i32 {
    let mut total: i32 = Default::default();
    let mut positives: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        if v > 0 {
            positives.push(v);
        }
    }
    let mut doubled: Vec<i32> = vec![];
    for v in positives.iter().cloned() {
        doubled.push((v).py_mul(2i32));
    }
    total = 0;
    for v in doubled.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Chain: filter>threshold -> square -> take sum -> add offset."]
#[doc = " Depyler: verified panic-free"]
pub fn pipeline_nested(vals: &Vec<i32>, threshold: i32) -> i32 {
    let mut total: i32 = Default::default();
    let mut filtered: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        if v > threshold {
            filtered.push(v);
        }
    }
    let mut squared: Vec<i32> = vec![];
    for v in filtered.iter().cloned() {
        squared.push((v).py_mul(v));
    }
    total = 0;
    for v in squared.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    (total).py_add(filtered.len() as i32) as i32
}
#[doc = "Test pipeline transformations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pipeline() -> i32 {
    let r1: i32 = pipeline_transform(&vec![-3, 1, -2, 4, 5]);
    let r2: i32 = pipeline_nested(&vec![1, 5, 3, 8, 2, 7], 4);
    (r1).py_add(r2)
}
#[doc = "Zip two lists by summing corresponding elements."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zip_sum<'b, 'a>(a: &'a Vec<i32>, b: &'b Vec<i32>) -> Vec<i32> {
    let mut length: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = a.len() as i32;
    length = _cse_temp_0;
    let _cse_temp_1 = b.len() as i32;
    let _cse_temp_2 = _cse_temp_1 < length;
    if _cse_temp_2 {
        length = _cse_temp_1;
    }
    for i in 0..(length) {
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
#[doc = "Zip two lists by multiplying corresponding elements(dot-product style)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zip_product<'b, 'a>(a: &'a Vec<i32>, b: &'b Vec<i32>) -> Vec<i32> {
    let mut length: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = a.len() as i32;
    length = _cse_temp_0;
    let _cse_temp_1 = b.len() as i32;
    let _cse_temp_2 = _cse_temp_1 < length;
    if _cse_temp_2 {
        length = _cse_temp_1;
    }
    for i in 0..(length) {
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
#[doc = "Test zip operations."]
#[doc = " Depyler: verified panic-free"]
pub fn test_zip() -> i32 {
    let mut total: i32 = Default::default();
    let sums: Vec<i32> = zip_sum(&vec![1, 2, 3], &vec![10, 20, 30]);
    let prods: Vec<i32> = zip_product(&vec![2, 3, 4], &vec![5, 6, 7]);
    total = 0;
    for v in sums.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    for v in prods.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Unzip list of pairs into two separate lists packed as [firsts..., seconds...]."]
#[doc = " Depyler: verified panic-free"]
pub fn unzip_pairs(pairs: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut firsts: Vec<i32> = vec![];
    let mut seconds: Vec<i32> = vec![];
    for pair in pairs.iter().cloned() {
        firsts.push(
            pair.get(0usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        seconds.push(
            pair.get(1usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    }
    let result: Vec<Vec<i32>> = vec![firsts, seconds];
    result
}
#[doc = "Test unzip of pairs."]
pub fn test_unzip() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let pairs: Vec<Vec<i32>> = vec![vec![1, 10], vec![2, 20], vec![3, 30]];
    let unzipped: Vec<Vec<i32>> = unzip_pairs(&pairs);
    total = 0;
    for v in unzipped
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
    {
        total = ((total).py_add(v)) as i32;
    }
    for v in unzipped
        .get(1usize)
        .cloned()
        .expect("IndexError: list index out of range")
    {
        total = ((total).py_add(v)) as i32;
    }
    Ok(total)
}
#[doc = "Partition list into [less_than_pivot, greater_or_equal]."]
#[doc = " Depyler: verified panic-free"]
pub fn partition(vals: &Vec<i32>, pivot: i32) -> Vec<Vec<i32>> {
    let mut less: Vec<i32> = vec![];
    let mut greater_eq: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        if v < pivot {
            less.push(v);
        } else {
            greater_eq.push(v);
        }
    }
    vec![less, greater_eq]
}
#[doc = "Test partitioning a list around a pivot."]
#[doc = " Depyler: proven to terminate"]
pub fn test_partition() -> Result<i32, Box<dyn std::error::Error>> {
    let parts: Vec<Vec<i32>> = partition(&vec![5, 1, 8, 3, 9, 2, 7], 5);
    let _cse_temp_0 = parts
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        .len() as i32;
    let less_count: i32 = _cse_temp_0;
    let geq_count: i32 = _cse_temp_0;
    Ok({
        let _r: i32 = ((less_count).py_mul(10i32) as i32).py_add(geq_count);
        _r
    })
}
#[doc = "Flat map: each element n expands to [n, n*n]."]
#[doc = " Depyler: verified panic-free"]
pub fn flat_map_expand(vals: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        result.push(v);
        result.push((v).py_mul(v));
    }
    result
}
#[doc = "Flat map: each element n expands to range(n)."]
#[doc = " Depyler: verified panic-free"]
pub fn flat_map_range(vals: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        for i in 0..(v) {
            result.push(i);
        }
    }
    result
}
#[doc = "Test flat map operations."]
#[doc = " Depyler: verified panic-free"]
pub fn test_flat_map() -> i32 {
    let mut total: i32 = Default::default();
    let expanded: Vec<i32> = flat_map_expand(&vec![1, 2, 3]);
    let ranged: Vec<i32> = flat_map_range(&vec![2, 3, 4]);
    total = 0;
    for v in expanded.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    for v in ranged.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Check if all elements are positive."]
#[doc = " Depyler: verified panic-free"]
pub fn all_positive(vals: &Vec<i32>) -> bool {
    for v in vals.iter().cloned() {
        if v <= 0 {
            return false;
        }
    }
    true
}
#[doc = "Check if any element is negative."]
#[doc = " Depyler: verified panic-free"]
pub fn any_negative(vals: &Vec<i32>) -> bool {
    for v in vals.iter().cloned() {
        if v < 0 {
            return true;
        }
    }
    false
}
#[doc = "Check that no element is zero."]
#[doc = " Depyler: verified panic-free"]
pub fn none_zero(vals: &Vec<i32>) -> bool {
    for v in vals.iter().cloned() {
        if v == 0 {
            return false;
        }
    }
    true
}
#[doc = "Count elements above threshold(predicate combinator)."]
#[doc = " Depyler: verified panic-free"]
pub fn count_matching(vals: &Vec<i32>, threshold: i32) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    for v in vals.iter().cloned() {
        if v > threshold {
            count = ((count).py_add(1i32)) as i32;
        }
    }
    count
}
#[doc = "Test predicate combinators."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_predicates() -> i32 {
    let mut score: i32 = Default::default();
    score = 0;
    if all_positive(&vec![1, 2, 3, 4]) {
        score = ((score).py_add(10i32)) as i32;
    }
    if any_negative(&vec![1, -2, 3]) {
        score = ((score).py_add(20i32)) as i32;
    }
    if none_zero(&vec![1, 2, 3]) {
        score = ((score).py_add(30i32)) as i32;
    }
    let _cse_temp_0 = ((score).py_add(count_matching(&vec![1, 5, 3, 8, 2, 7], 4))) as i32;
    score = _cse_temp_0;
    score
}
#[doc = "Transducer: filter(>min_val) then map(*2) then reduce(+, 0).\n\n    Composed in a single pass for efficiency.\n    "]
#[doc = " Depyler: verified panic-free"]
pub fn transduce_filter_double_sum(vals: &Vec<i32>, min_val: i32) -> i32 {
    let mut acc: i32 = Default::default();
    acc = 0;
    for v in vals.iter().cloned() {
        if v > min_val {
            acc = ((acc).py_add((v).py_mul(2i32))) as i32;
        }
    }
    acc
}
#[doc = "Transducer: map(x^2) then filter(<max_square) then reduce(+, 0)."]
#[doc = " Depyler: verified panic-free"]
pub fn transduce_square_filter_sum(vals: &Vec<i32>, max_square: i32) -> i32 {
    let mut acc: i32 = Default::default();
    acc = 0;
    for v in vals.iter().cloned() {
        let sq: i32 = ((v).py_mul(v)) as i32;
        if sq < max_square {
            acc = ((acc).py_add(sq)) as i32;
        }
    }
    acc
}
#[doc = "Test transducer-style composed transformations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_transducers() -> i32 {
    let r1: i32 = transduce_filter_double_sum(&vec![1, 5, 3, 8, 2, 7], 4);
    let r2: i32 = transduce_square_filter_sum(&vec![1, 2, 3, 4, 5], 20);
    (r1).py_add(r2)
}
#[doc = "Fibonacci with memoization using dict."]
#[doc = " Depyler: proven to terminate"]
pub fn fib_memo(n: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut cache: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    cache.insert(0, 0);
    cache.insert(1, 1);
    for i in (2)..((n).py_add(1i32)) {
        cache.insert(
            i.clone(),
            (cache.get(&((i) - (1i32))).cloned().unwrap_or_default())
                .py_add(cache.get(&((i) - (2i32))).cloned().unwrap_or_default()),
        );
    }
    Ok(cache.get(&(n)).cloned().unwrap_or_default())
}
#[doc = "Tribonacci with memoization using dict."]
#[doc = " Depyler: proven to terminate"]
pub fn tribonacci_memo(n: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut cache: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    cache.insert(0, 0);
    cache.insert(1, 0);
    cache.insert(2, 1);
    for i in (3)..((n).py_add(1i32)) {
        cache.insert(
            i.clone(),
            ((cache.get(&((i) - (1i32))).cloned().unwrap_or_default())
                .py_add(cache.get(&((i) - (2i32))).cloned().unwrap_or_default())
                as i32)
                .py_add(cache.get(&((i) - (3i32))).cloned().unwrap_or_default()),
        );
    }
    Ok(cache.get(&(n)).cloned().unwrap_or_default())
}
#[doc = "Test memoized functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_memoization() -> Result<i32, Box<dyn std::error::Error>> {
    let f10: i32 = fib_memo(10)?;
    let t10: i32 = tribonacci_memo(10)?;
    Ok((f10).py_add(t10))
}
#[doc = "Count steps to reach fixed point 1 via Collatz sequence."]
pub fn fixed_point_collatz(n: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut steps: i32 = Default::default();
    steps = 0;
    let mut current: i32 = n.clone();
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
        steps = ((steps).py_add(1i32)) as i32;
    }
    Ok(steps)
}
#[doc = "Repeatedly sum digits until single digit(digital root via iteration)."]
pub fn fixed_point_digit_sum(n: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut current: i32 = Default::default();
    current = n;
    while current >= 10 {
        let mut total: i32 = 0;
        let mut temp: i32 = current.clone();
        while temp > 0 {
            total = ((total).py_add((temp).py_mod(10i32))) as i32;
            temp = {
                let a = temp;
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
        current = total;
    }
    Ok(current)
}
#[doc = "Test fixed-point iterations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_fixed_point() -> Result<i32, Box<dyn std::error::Error>> {
    let collatz_27: i32 = fixed_point_collatz(27)?;
    let digit_root: i32 = fixed_point_digit_sum(9999)?;
    Ok((collatz_27).py_add(digit_root))
}
#[doc = "Sum list using accumulator-passing style."]
#[doc = " Depyler: verified panic-free"]
pub fn sum_acc(vals: &Vec<i32>, mut acc: i32) -> i32 {
    for v in vals.iter().cloned() {
        acc = ((acc).py_add(v)) as i32;
    }
    acc
}
#[doc = "Factorial using accumulator-passing style(tail-recursive form)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn factorial_acc(n: i32, acc: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = acc;
    for i in (1)..((n).py_add(1i32)) {
        result = ((result).py_mul(i)) as i32;
    }
    result
}
#[doc = "Power using accumulator-passing style."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn power_acc(base: i32, exp: i32, acc: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = acc;
    for _i in 0..(exp) {
        result = ((result).py_mul(base)) as i32;
    }
    result
}
#[doc = "Test accumulator-passing style functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_accumulator_passing() -> i32 {
    let s: i32 = sum_acc(&vec![1, 2, 3, 4, 5], 0);
    let f: i32 = factorial_acc(6, 1);
    let p: i32 = power_acc(2, 10, 1);
    {
        let _r: i32 = ((s).py_add(f) as i32).py_add(p);
        _r
    }
}
#[doc = "CPS add: compute a+b then apply continuation(multiply by k)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn cps_add(a: i32, b: i32, continuation_mul: i32) -> i32 {
    let intermediate: i32 = ((a).py_add(b)) as i32;
    (intermediate).py_mul(continuation_mul)
}
#[doc = "CPS chain: add(x, y) -> multiply result by z -> add 1."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn cps_chain(x: i32, y: i32, z: i32) -> i32 {
    let step1: i32 = cps_add(x, y, z);
    (step1).py_add(1i32)
}
#[doc = "Factorial using CPS simulation with explicit continuation stack."]
#[doc = " Depyler: verified panic-free"]
pub fn cps_factorial(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = 1;
    let mut i: i32 = n.clone();
    while i > 0 {
        result = ((result).py_mul(i)) as i32;
        i = ((i) - (1i32)) as i32;
    }
    result
}
#[doc = "Test continuation-passing style simulation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_cps() -> i32 {
    let r1: i32 = cps_add(3, 4, 2);
    let r2: i32 = cps_chain(2, 3, 4);
    let r3: i32 = cps_factorial(5);
    {
        let _r: i32 = ((r1).py_add(r2) as i32).py_add(r3);
        _r
    }
}
#[doc = "Church boolean TRUE: select first argument."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_true(a: i32, _b: i32) -> i32 {
    a
}
#[doc = "Church boolean FALSE: select second argument."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_false(_a: i32, b: i32) -> i32 {
    b
}
#[doc = "Church AND: if p then q else false."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_and(p: bool, q: bool, a: i32, b: i32) -> i32 {
    if p {
        if q {
            return church_true(a, b);
        }
        return church_false(a, b);
    }
    church_false(a, b)
}
#[doc = "Church OR: if p then true else q."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_or(p: bool, q: bool, a: i32, b: i32) -> i32 {
    if p {
        return church_true(a, b);
    }
    if q {
        return church_true(a, b);
    }
    church_false(a, b)
}
#[doc = "Church NOT: if p then false else true."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_not(p: bool, a: i32, b: i32) -> i32 {
    if p {
        return church_false(a, b);
    }
    church_true(a, b)
}
#[doc = "Test Church-encoded boolean operations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_church_booleans() -> i32 {
    let t1: i32 = church_and(true, true, 10, 0);
    let t2: i32 = church_and(true, false, 10, 0);
    let t3: i32 = church_or(false, true, 20, 0);
    let t4: i32 = church_or(false, false, 20, 0);
    let t5: i32 = church_not(false, 30, 0);
    {
        let _r: i32 = ((((t1).py_add(t2) as i32).py_add(t3) as i32).py_add(t4) as i32).py_add(t5);
        _r
    }
}
#[doc = "Church numeral 0: apply f zero times(identity)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_zero(x: i32) -> i32 {
    x
}
#[doc = "Church successor: apply f one more time than n."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_succ(n: i32, x: i32, step: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = x;
    for _i in 0..((n).py_add(1i32)) {
        result = ((result).py_add(step)) as i32;
    }
    result
}
#[doc = "Church addition: apply f(a+b) times."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_add_nums(a: i32, b: i32, x: i32, step: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = x;
    for _i in 0..((a).py_add(b)) {
        result = ((result).py_add(step)) as i32;
    }
    result
}
#[doc = "Church multiplication: apply f(a*b) times."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn church_mul_nums(a: i32, b: i32, x: i32, step: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = x;
    for _i in 0..((a).py_mul(b)) {
        result = ((result).py_add(step)) as i32;
    }
    result
}
#[doc = "Test Church numeral operations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_church_numerals() -> i32 {
    let z: i32 = church_zero(42);
    let s: i32 = church_succ(3, 0, 1);
    let a: i32 = church_add_nums(3, 4, 0, 1);
    let m: i32 = church_mul_nums(3, 4, 0, 1);
    {
        let _r: i32 = (((z).py_add(s) as i32).py_add(a) as i32).py_add(m);
        _r
    }
}
#[doc = "Y-combinator-style factorial using explicit loop(no lambda needed)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn y_factorial(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    result = 1;
    for i in (2)..((n).py_add(1i32)) {
        result = ((result).py_mul(i)) as i32;
    }
    result
}
#[doc = "Y-combinator-style fibonacci using explicit iteration."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn y_fibonacci(n: i32) -> i32 {
    let mut b: i32 = Default::default();
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    let mut a: i32 = 0;
    b = 1;
    for _i in (2)..((n).py_add(1i32)) {
        let temp: i32 = ((a).py_add(b)) as i32;
        a = b;
        b = temp;
    }
    b
}
#[doc = "Y-combinator-style power using explicit iteration."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn y_power(base: i32, exp: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = 1;
    for _i in 0..(exp) {
        result = ((result).py_mul(base)) as i32;
    }
    result
}
#[doc = "Test Y-combinator style recursive functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_y_combinator() -> i32 {
    let f: i32 = y_factorial(7);
    let fib: i32 = y_fibonacci(12);
    let p: i32 = y_power(3, 5);
    {
        let _r: i32 = ((f).py_add(fib) as i32).py_add(p);
        _r
    }
}
#[doc = "Lens getter: extract value at key."]
#[doc = " Depyler: proven to terminate"]
pub fn lens_get<'b, 'a>(
    data: &'a std::collections::HashMap<String, i32>,
    key: &'b str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = data.get(key).is_some();
    if _cse_temp_0 {
        return Ok(data.get(key).cloned().unwrap_or_default());
    }
    Ok(0)
}
#[doc = "Lens setter: return new dict with key set to value."]
pub fn lens_set(
    data: &std::collections::HashMap<String, i32>,
    key: String,
    value: i32,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut result: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    for k in data.keys().cloned() {
        result.insert(
            k.to_string().clone(),
            data.get(&(k)).cloned().unwrap_or_default(),
        );
    }
    result.insert(key.to_string().clone(), value);
    Ok(result)
}
#[doc = "Lens modify: apply transformation to value at key."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn lens_modify<'a, 'b>(
    data: &'a std::collections::HashMap<String, i32>,
    key: &'b str,
    delta: i32,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let current: i32 = lens_get(&data, key.clone())?;
    lens_set(&data, key.to_string(), (current).py_add(delta))
}
#[doc = "Test lens-style get/set/modify on dicts."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_lens() -> Result<i32, Box<dyn std::error::Error>> {
    let data: std::collections::HashMap<String, i32> = {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert(STR_X.to_string(), (10) as i32);
        map.insert("y".to_string(), (20) as i32);
        map.insert("z".to_string(), (30) as i32);
        map
    };
    let got: i32 = lens_get(&data, &STR_X)?;
    let updated: std::collections::HashMap<String, i32> = lens_set(&data, STR_X.to_string(), 100)?;
    let modified: std::collections::HashMap<String, i32> = lens_modify(&data, &"y", 5)?;
    Ok({
        let _r: i32 =
            ((got).py_add(lens_get(&updated, &STR_X)?) as i32).py_add(lens_get(&modified, &"y")?);
        _r
    })
}
#[doc = "Group values by their remainder when divided by modulus."]
pub fn group_by_mod(
    vals: &Vec<i32>,
    modulus: i32,
) -> Result<HashMap<i32, Vec<i32>>, Box<dyn std::error::Error>> {
    let mut groups: std::collections::HashMap<i32, Vec<i32>> = {
        let map: HashMap<i32, Vec<i32>> = HashMap::new();
        map
    };
    for v in vals.iter().cloned() {
        let key: i32 = ((v).py_mod(modulus)) as i32;
        if groups.get(&key).is_some() {
            groups.get(&(key)).cloned().unwrap_or_default().push(v);
        } else {
            groups.insert(key.clone(), vec![v]);
        }
    }
    Ok(groups)
}
#[doc = "Group by sign and count elements in each group.\n\n    Returns dict with keys: -1(negative), 0(zero), 1(positive).\n    "]
pub fn group_by_sign(vals: &Vec<i32>) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let mut counts: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    counts.insert(-1, 0);
    counts.insert(0, 0);
    counts.insert(1, 0);
    for v in vals.iter().cloned() {
        if v < 0 {
            counts.insert(
                -1,
                (counts.get(&(-1)).cloned().unwrap_or_default()).py_add(1i32),
            );
        } else {
            if v == 0 {
                counts.insert(
                    0,
                    (counts.get(&(0)).cloned().unwrap_or_default()).py_add(1i32),
                );
            } else {
                counts.insert(
                    1,
                    (counts.get(&(1)).cloned().unwrap_or_default()).py_add(1i32),
                );
            }
        }
    }
    Ok(counts)
}
#[doc = "Test groupBy implementations."]
#[doc = " Depyler: proven to terminate"]
pub fn test_group_by() -> Result<i32, Box<dyn std::error::Error>> {
    let groups: std::collections::HashMap<i32, Vec<i32>> =
        group_by_mod(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3)?;
    let _cse_temp_0 = groups.get(&(0)).cloned().unwrap_or_default().len() as i32;
    let count_mod0: i32 = _cse_temp_0;
    let count_mod1: i32 = _cse_temp_0;
    let signs: std::collections::HashMap<i32, i32> = group_by_sign(&vec![-5, -3, 0, 1, 4, 7])?;
    let neg: i32 = signs.get(&(-1)).cloned().unwrap_or_default();
    let pos: i32 = signs.get(&(1)).cloned().unwrap_or_default();
    Ok({
        let _r: i32 = ((((count_mod0).py_mul(10i32) as i32).py_add((count_mod1).py_mul(10i32))
            as i32)
            .py_add(neg) as i32)
            .py_add(pos);
        _r
    })
}
#[doc = "Take first n elements."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn take_n(vals: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for i in 0..(n) {
        if i < vals.len() as i32 {
            result.push(
                vals.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        }
    }
    result
}
#[doc = "Drop first n elements."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn drop_n(vals: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for i in (n)..(vals.len() as i32) {
        result.push(
            vals.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
    }
    result
}
#[doc = "Take elements while they are positive."]
#[doc = " Depyler: verified panic-free"]
pub fn take_while_positive(vals: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        if v <= 0 {
            return result;
        }
        result.push(v);
    }
    result
}
#[doc = "Drop elements while they are positive, return the rest."]
#[doc = " Depyler: verified panic-free"]
pub fn drop_while_positive(vals: &Vec<i32>) -> Vec<i32> {
    let mut dropping: bool = true;
    let mut result: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        if (dropping) && (v > 0) {
            continue;
        }
        dropping = false;
        result.push(v);
    }
    result
}
#[doc = "Test take/drop/take_while/drop_while."]
#[doc = " Depyler: verified panic-free"]
pub fn test_take_drop() -> i32 {
    let mut total: i32 = Default::default();
    let taken: Vec<i32> = take_n(&vec![10, 20, 30, 40, 50], 3);
    let dropped: Vec<i32> = drop_n(&vec![10, 20, 30, 40, 50], 2);
    let tw: Vec<i32> = take_while_positive(&vec![3, 5, 7, -1, 9, 11]);
    let dw: Vec<i32> = drop_while_positive(&vec![3, 5, 7, -1, 9, 11]);
    total = 0;
    for v in taken.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    for v in dropped.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    let _cse_temp_0 = tw.len() as i32;
    let _cse_temp_1 = ((_cse_temp_0).py_mul(10i32)) as i32;
    total = ((total).py_add(_cse_temp_1)) as i32;
    let _cse_temp_2 = dw.len() as i32;
    let _cse_temp_3 = ((_cse_temp_2).py_mul(10i32)) as i32;
    total = ((total).py_add(_cse_temp_3)) as i32;
    total
}
#[doc = "Compute sum for each sliding window of given size."]
#[doc = " Depyler: proven to terminate"]
pub fn sliding_window_sum(
    vals: &Vec<i32>,
    window: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    for i in 0..(((vals.len() as i32) - (window) as i32).py_add(1i32)) {
        let mut w_sum: i32 = 0;
        for j in 0..(window) {
            w_sum = ((w_sum).py_add({
                let base = &vals;
                let idx: i32 = (i).py_add(j);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            })) as i32;
        }
        result.push(w_sum);
    }
    Ok(result)
}
#[doc = "Compute max for each sliding window of given size."]
#[doc = " Depyler: proven to terminate"]
pub fn sliding_window_max(
    vals: &Vec<i32>,
    window: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    for i in 0..(((vals.len() as i32) - (window) as i32).py_add(1i32)) {
        let mut w_max: i32 = vals
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        for j in (1)..(window) {
            if {
                let base = &vals;
                let idx: i32 = (i).py_add(j);
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx)
                    .cloned()
                    .expect("IndexError: list index out of range")
            } > w_max
            {
                w_max = {
                    let base = &vals;
                    let idx: i32 = (i).py_add(j);
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
        result.push(w_max);
    }
    Ok(result)
}
#[doc = "Test sliding window operations."]
#[doc = " Depyler: verified panic-free"]
pub fn test_sliding_window() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let sums: Vec<i32> = sliding_window_sum(&vec![1, 3, 5, 7, 9], 3)?;
    let maxes: Vec<i32> = sliding_window_max(&vec![1, 3, 5, 7, 9], 3)?;
    total = 0;
    for v in sums.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    for v in maxes.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    Ok(total)
}
#[doc = "Interleave two lists element by element."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn interleave<'a, 'b>(a: &'a Vec<i32>, b: &'b Vec<i32>) -> Vec<i32> {
    let mut length: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = a.len() as i32;
    length = _cse_temp_0;
    let _cse_temp_1 = b.len() as i32;
    let _cse_temp_2 = _cse_temp_1 > length;
    if _cse_temp_2 {
        length = _cse_temp_1;
    }
    for i in 0..(length) {
        if i < a.len() as i32 {
            result.push(
                a.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        }
        if i < b.len() as i32 {
            result.push(
                b.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        }
    }
    result
}
#[doc = "Test interleaving two lists."]
#[doc = " Depyler: verified panic-free"]
pub fn test_interleave() -> i32 {
    let mut total: i32 = Default::default();
    let merged: Vec<i32> = interleave(&vec![1, 3, 5], &vec![2, 4, 6]);
    total = 0;
    for v in merged.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Split list into chunks of given size."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn chunk(vals: &Vec<i32>, size: i32) -> Vec<Vec<i32>> {
    let mut current: Vec<i32> = Default::default();
    let mut result: Vec<Vec<i32>> = vec![];
    current = vec![];
    for (_i, v) in vals.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) {
        current.push(v);
        if current.len() as i32 == size {
            result.push(current);
            current = vec![];
        }
    }
    let _cse_temp_0 = current.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        result.push(current);
    }
    result
}
#[doc = "Test chunking a list."]
pub fn test_chunk() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let chunks: Vec<Vec<i32>> = chunk(&vec![1, 2, 3, 4, 5, 6, 7], 3);
    let _cse_temp_0 = chunks.len() as i32;
    let num_chunks: i32 = _cse_temp_0;
    let _cse_temp_1 = {
        let base = &chunks;
        let idx: i32 = (num_chunks) - (1i32);
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx)
            .cloned()
            .expect("IndexError: list index out of range")
    }
    .len() as i32;
    let last_chunk_size: i32 = _cse_temp_1;
    total = 0;
    for c in chunks.iter().cloned() {
        for v in c.iter().cloned() {
            total = ((total).py_add(v)) as i32;
        }
    }
    Ok({
        let _r: i32 = ((total).py_add((num_chunks).py_mul(100i32)) as i32).py_add(last_chunk_size);
        _r
    })
}
#[doc = "Remove duplicates while preserving insertion order."]
#[doc = " Depyler: verified panic-free"]
pub fn unique_preserve_order(vals: &Vec<i32>) -> Vec<i32> {
    let mut seen: std::collections::HashMap<i32, bool> = {
        let map: HashMap<i32, bool> = HashMap::new();
        map
    };
    let mut result: Vec<i32> = vec![];
    for v in vals.iter().cloned() {
        if seen.get(&v).is_none() {
            seen.insert(v.clone(), true);
            result.push(v);
        }
    }
    result
}
#[doc = "Test deduplication preserving order."]
#[doc = " Depyler: verified panic-free"]
pub fn test_unique() -> i32 {
    let mut total: i32 = Default::default();
    let deduped: Vec<i32> = unique_preserve_order(&vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3]);
    total = 0;
    for v in deduped.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    {
        let _r: i32 = (total).py_add((deduped.len() as i32).py_mul(10i32)) as i32;
        _r
    }
}
#[doc = "Build frequency map of values."]
pub fn frequency_map(vals: &Vec<i32>) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let mut freq: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    for v in vals.iter().cloned() {
        if freq.get(&v).is_some() {
            {
                let _key = v.clone();
                let _old_val = freq.get(&_key).cloned().unwrap_or_default();
                freq.insert(_key, _old_val + 1);
            }
        } else {
            freq.insert(v.clone(), 1);
        }
    }
    Ok(freq)
}
#[doc = "Find the most frequently occurring element."]
pub fn most_frequent(vals: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut best_val: i32 = Default::default();
    let freq: std::collections::HashMap<i32, i32> = frequency_map(&vals)?;
    best_val = vals
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let mut best_count: i32 = 0;
    for v in vals.iter().cloned() {
        if freq.get(&(v)).cloned().unwrap_or_default() > best_count {
            best_count = freq.get(&(v)).cloned().unwrap_or_default();
            best_val = v;
        }
    }
    Ok(best_val)
}
#[doc = "Test frequency map and most frequent."]
#[doc = " Depyler: proven to terminate"]
pub fn test_frequency() -> Result<i32, Box<dyn std::error::Error>> {
    let freq: std::collections::HashMap<i32, i32> = frequency_map(&vec![1, 2, 2, 3, 3, 3, 4])?;
    let mf: i32 = most_frequent(&vec![1, 2, 2, 3, 3, 3, 4])?;
    let _cse_temp_0 = ((freq.get(&(3)).cloned().unwrap_or_default()).py_mul(10i32)) as i32;
    let total: i32 = ((_cse_temp_0).py_add(mf)) as i32;
    Ok(total)
}
#[doc = "Compute [count, sum, min, max] in single pass."]
pub fn tally_stats(vals: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut hi: i32 = Default::default();
    let mut lo: i32 = Default::default();
    let mut count: i32 = Default::default();
    let mut total: i32 = Default::default();
    let _cse_temp_0 = vals.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![0, 0, 0, 0]);
    }
    count = 0;
    total = 0;
    lo = vals
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    hi = vals
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for v in vals.iter().cloned() {
        count = ((count).py_add(1i32)) as i32;
        total = ((total).py_add(v)) as i32;
        if v < lo {
            lo = v;
        }
        if v > hi {
            hi = v;
        }
    }
    Ok(vec![count, total, lo, hi])
}
#[doc = "Test multi-accumulator tally."]
#[doc = " Depyler: proven to terminate"]
pub fn test_tally() -> Result<i32, Box<dyn std::error::Error>> {
    let stats: Vec<i32> = tally_stats(&vec![5, 2, 8, 1, 9, 3, 7])?;
    Ok({
        let _r: i32 = (((stats
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range"))
        .py_add(
            stats
                .get(1usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ) as i32)
            .py_add(
                stats
                    .get(2usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ) as i32)
            .py_add(
                stats
                    .get(3usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        _r
    })
}
#[doc = "Flatten a 2D list into 1D."]
#[doc = " Depyler: verified panic-free"]
pub fn flatten_2d(nested: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for row in nested.iter().cloned() {
        for v in row.iter().cloned() {
            result.push(v);
        }
    }
    result
}
#[doc = "Test flattening nested list."]
#[doc = " Depyler: verified panic-free"]
pub fn test_flatten() -> i32 {
    let mut total: i32 = Default::default();
    let flat: Vec<i32> = flatten_2d(&vec![vec![1, 2], vec![3, 4, 5], vec![6]]);
    total = 0;
    for v in flat.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Map each element to element * its index."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn map_with_index(vals: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for (i, v) in vals.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) {
        let i = i as i32;
        result.push((v).py_mul(i));
    }
    result
}
#[doc = "Test map with index."]
#[doc = " Depyler: verified panic-free"]
pub fn test_map_with_index() -> i32 {
    let mut total: i32 = Default::default();
    let mapped: Vec<i32> = map_with_index(&vec![10, 20, 30, 40]);
    total = 0;
    for v in mapped.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Generate sequence of powers of 2 via unfold."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn unfold_powers_of_two(count: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut current: i32 = 1;
    for _i in 0..(count) {
        result.push(current);
        current = ((current).py_mul(2i32)) as i32;
    }
    result
}
#[doc = "Generate triangular numbers via unfold."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn unfold_triangular(count: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut acc: i32 = 0;
    for i in (1)..((count).py_add(1i32)) {
        acc = ((acc).py_add(i)) as i32;
        result.push(acc);
    }
    result
}
#[doc = "Test unfold / sequence generation."]
#[doc = " Depyler: verified panic-free"]
pub fn test_unfold() -> i32 {
    let mut total: i32 = Default::default();
    let powers: Vec<i32> = unfold_powers_of_two(8);
    let triangles: Vec<i32> = unfold_triangular(5);
    total = 0;
    for v in powers.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    for v in triangles.iter().cloned() {
        total = ((total).py_add(v)) as i32;
    }
    total
}
#[doc = "Count iterations of halving until reaching 1."]
pub fn iterate_halve(n: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    count = 0;
    let mut current: i32 = n.clone();
    while current > 1 {
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
        count = ((count).py_add(1i32)) as i32;
    }
    Ok(count)
}
#[doc = "Iterate 3n+1 rule up to max_steps, return final value."]
#[doc = " Depyler: proven to terminate"]
pub fn iterate_triple_plus_one(n: i32, max_steps: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut current: i32 = Default::default();
    current = n;
    for _i in 0..(max_steps) {
        if current <= 1 {
            return Ok(current);
        }
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
    }
    Ok(current)
}
#[doc = "Test iterative transformations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_iterate() -> Result<i32, Box<dyn std::error::Error>> {
    let h: i32 = iterate_halve(256)?;
    let t: i32 = iterate_triple_plus_one(7, 100)?;
    Ok((h).py_add(t))
}
#[doc = "Safe head of list, returns default if empty(Option pattern)."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_head(vals: &Vec<i32>, default: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = vals.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(default);
    }
    Ok(vals
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Safe last of list, returns default if empty."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_last(vals: &Vec<i32>, default: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = vals.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(default);
    }
    Ok({
        let base = &vals;
        let idx: i32 = (vals.len() as i32) - (1i32);
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
#[doc = "Safe index access, returns default if out of bounds."]
#[doc = " Depyler: proven to terminate"]
pub fn safe_index(
    vals: &Vec<i32>,
    idx: i32,
    default: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = idx < 0;
    let _cse_temp_1 = vals.len() as i32;
    let _cse_temp_2 = idx >= _cse_temp_1;
    let _cse_temp_3 = (_cse_temp_0) || (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(default);
    }
    Ok(vals
        .get(idx as usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Test Option/Maybe pattern with safe accessors."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_maybe() -> Result<i32, Box<dyn std::error::Error>> {
    let h: i32 = safe_head(&vec![10, 20, 30], -1)?;
    let he: i32 = safe_head(&vec![], -1)?;
    let l: i32 = safe_last(&vec![10, 20, 30], -1)?;
    let le: i32 = safe_last(&vec![], -1)?;
    let i: i32 = safe_index(&vec![10, 20, 30], 1, -1)?;
    let ie: i32 = safe_index(&vec![10, 20, 30], 5, -1)?;
    Ok({
        let _r: i32 = (((((h).py_add(he) as i32).py_add(l) as i32).py_add(le) as i32).py_add(i)
            as i32)
            .py_add(ie);
        _r
    })
}
#[doc = "Run all test functions and return sum of results."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_all_tests() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = 0;
    let _cse_temp_0 = ((total).py_add(test_manual_map())) as i32;
    total = _cse_temp_0;
    let _cse_temp_1 = ((total).py_add(test_manual_filter())) as i32;
    total = _cse_temp_1;
    let _cse_temp_2 = ((total).py_add(test_fold_left())) as i32;
    total = _cse_temp_2;
    let _cse_temp_3 = ((total).py_add(test_scan()?)) as i32;
    total = _cse_temp_3;
    let _cse_temp_4 = ((total).py_add(test_composition())) as i32;
    total = _cse_temp_4;
    let _cse_temp_5 = ((total).py_add(test_partial_application())) as i32;
    total = _cse_temp_5;
    let _cse_temp_6 = ((total).py_add(test_currying())) as i32;
    total = _cse_temp_6;
    let _cse_temp_7 = ((total).py_add(test_pipeline())) as i32;
    total = _cse_temp_7;
    let _cse_temp_8 = ((total).py_add(test_zip())) as i32;
    total = _cse_temp_8;
    let _cse_temp_9 = ((total).py_add(test_unzip()?)) as i32;
    total = _cse_temp_9;
    let _cse_temp_10 = ((total).py_add(test_partition()?)) as i32;
    total = _cse_temp_10;
    let _cse_temp_11 = ((total).py_add(test_flat_map())) as i32;
    total = _cse_temp_11;
    let _cse_temp_12 = ((total).py_add(test_predicates())) as i32;
    total = _cse_temp_12;
    let _cse_temp_13 = ((total).py_add(test_transducers())) as i32;
    total = _cse_temp_13;
    let _cse_temp_14 = ((total).py_add(test_memoization()?)) as i32;
    total = _cse_temp_14;
    let _cse_temp_15 = ((total).py_add(test_fixed_point()?)) as i32;
    total = _cse_temp_15;
    let _cse_temp_16 = ((total).py_add(test_accumulator_passing())) as i32;
    total = _cse_temp_16;
    let _cse_temp_17 = ((total).py_add(test_cps())) as i32;
    total = _cse_temp_17;
    let _cse_temp_18 = ((total).py_add(test_church_booleans())) as i32;
    total = _cse_temp_18;
    let _cse_temp_19 = ((total).py_add(test_church_numerals())) as i32;
    total = _cse_temp_19;
    let _cse_temp_20 = ((total).py_add(test_y_combinator())) as i32;
    total = _cse_temp_20;
    let _cse_temp_21 = ((total).py_add(test_lens()?)) as i32;
    total = _cse_temp_21;
    let _cse_temp_22 = ((total).py_add(test_group_by()?)) as i32;
    total = _cse_temp_22;
    let _cse_temp_23 = ((total).py_add(test_take_drop())) as i32;
    total = _cse_temp_23;
    let _cse_temp_24 = ((total).py_add(test_sliding_window()?)) as i32;
    total = _cse_temp_24;
    let _cse_temp_25 = ((total).py_add(test_interleave())) as i32;
    total = _cse_temp_25;
    let _cse_temp_26 = ((total).py_add(test_chunk()?)) as i32;
    total = _cse_temp_26;
    let _cse_temp_27 = ((total).py_add(test_unique())) as i32;
    total = _cse_temp_27;
    let _cse_temp_28 = ((total).py_add(test_frequency()?)) as i32;
    total = _cse_temp_28;
    let _cse_temp_29 = ((total).py_add(test_tally()?)) as i32;
    total = _cse_temp_29;
    let _cse_temp_30 = ((total).py_add(test_flatten())) as i32;
    total = _cse_temp_30;
    let _cse_temp_31 = ((total).py_add(test_map_with_index())) as i32;
    total = _cse_temp_31;
    let _cse_temp_32 = ((total).py_add(test_unfold())) as i32;
    total = _cse_temp_32;
    let _cse_temp_33 = ((total).py_add(test_iterate()?)) as i32;
    total = _cse_temp_33;
    let _cse_temp_34 = ((total).py_add(test_maybe()?)) as i32;
    total = _cse_temp_34;
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
    fn test_test_manual_map_examples() {
        let _ = test_manual_map();
    }
    #[test]
    fn test_manual_filter_positive_examples() {
        assert_eq!(manual_filter_positive(vec![]), vec![]);
        assert_eq!(manual_filter_positive(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_manual_filter_examples() {
        let _ = test_manual_filter();
    }
    #[test]
    fn test_test_fold_left_examples() {
        let _ = test_fold_left();
    }
    #[test]
    fn test_scan_sum_examples() {
        assert_eq!(scan_sum(vec![]), vec![]);
        assert_eq!(scan_sum(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_scan_examples() {
        let _ = test_scan();
    }
    #[test]
    fn test_apply_twice_examples() {
        assert_eq!(apply_twice(0, 0), 0);
        assert_eq!(apply_twice(1, 2), 3);
        assert_eq!(apply_twice(-1, 1), 0);
    }
    #[test]
    fn test_apply_thrice_examples() {
        assert_eq!(apply_thrice(0, 0), 0);
        assert_eq!(apply_thrice(1, 2), 3);
        assert_eq!(apply_thrice(-1, 1), 0);
    }
    #[test]
    fn test_test_composition_examples() {
        let _ = test_composition();
    }
    #[test]
    fn quickcheck_add_partial() {
        fn prop(a: i32, b: i32) -> TestResult {
            if (a > 0 && b > i32::MAX - a) || (a < 0 && b < i32::MIN - a) {
                return TestResult::discard();
            }
            let result1 = add_partial(a.clone(), b.clone());
            let result2 = add_partial(b.clone(), a.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_add_partial_examples() {
        assert_eq!(add_partial(0, 0), 0);
        assert_eq!(add_partial(1, 2), 3);
        assert_eq!(add_partial(-1, 1), 0);
    }
    #[test]
    fn quickcheck_multiply_partial() {
        fn prop(a: i32, b: i32) -> TestResult {
            if (a > 0 && b > i32::MAX - a) || (a < 0 && b < i32::MIN - a) {
                return TestResult::discard();
            }
            let result1 = multiply_partial(a.clone(), b.clone());
            let result2 = multiply_partial(b.clone(), a.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_multiply_partial_examples() {
        assert_eq!(multiply_partial(0, 0), 0);
        assert_eq!(multiply_partial(1, 2), 3);
        assert_eq!(multiply_partial(-1, 1), 0);
    }
    #[test]
    fn test_test_partial_application_examples() {
        let _ = test_partial_application();
    }
    #[test]
    fn test_test_currying_examples() {
        let _ = test_currying();
    }
    #[test]
    fn test_pipeline_transform_examples() {
        assert_eq!(pipeline_transform(&vec![]), 0);
        assert_eq!(pipeline_transform(&vec![1]), 1);
        assert_eq!(pipeline_transform(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_test_pipeline_examples() {
        let _ = test_pipeline();
    }
    #[test]
    fn test_test_zip_examples() {
        let _ = test_zip();
    }
    #[test]
    fn test_unzip_pairs_examples() {
        assert_eq!(unzip_pairs(vec![]), vec![]);
        assert_eq!(unzip_pairs(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_unzip_examples() {
        let _ = test_unzip();
    }
    #[test]
    fn test_test_partition_examples() {
        let _ = test_partition();
    }
    #[test]
    fn quickcheck_flat_map_expand() {
        fn prop(vals: Vec<i32>) -> TestResult {
            let input_len = vals.len();
            let result = flat_map_expand(&vals);
            if result.len() != input_len {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_flat_map_expand_examples() {
        assert_eq!(flat_map_expand(vec![]), vec![]);
        assert_eq!(flat_map_expand(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_flat_map_range() {
        fn prop(vals: Vec<i32>) -> TestResult {
            let input_len = vals.len();
            let result = flat_map_range(&vals);
            if result.len() != input_len {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_flat_map_range_examples() {
        assert_eq!(flat_map_range(vec![]), vec![]);
        assert_eq!(flat_map_range(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_flat_map_examples() {
        let _ = test_flat_map();
    }
    #[test]
    fn test_all_positive_examples() {
        let _ = all_positive(Default::default());
    }
    #[test]
    fn test_any_negative_examples() {
        let _ = any_negative(Default::default());
    }
    #[test]
    fn test_none_zero_examples() {
        let _ = none_zero(Default::default());
    }
    #[test]
    fn test_test_predicates_examples() {
        let _ = test_predicates();
    }
    #[test]
    fn test_test_transducers_examples() {
        let _ = test_transducers();
    }
    #[test]
    fn test_fib_memo_examples() {
        assert_eq!(fib_memo(0), 0);
        assert_eq!(fib_memo(1), 1);
        assert_eq!(fib_memo(-1), -1);
    }
    #[test]
    fn test_tribonacci_memo_examples() {
        assert_eq!(tribonacci_memo(0), 0);
        assert_eq!(tribonacci_memo(1), 1);
        assert_eq!(tribonacci_memo(-1), -1);
    }
    #[test]
    fn test_test_memoization_examples() {
        let _ = test_memoization();
    }
    #[test]
    fn test_fixed_point_collatz_examples() {
        assert_eq!(fixed_point_collatz(0), 0);
        assert_eq!(fixed_point_collatz(1), 1);
        assert_eq!(fixed_point_collatz(-1), -1);
    }
    #[test]
    fn test_fixed_point_digit_sum_examples() {
        assert_eq!(fixed_point_digit_sum(0), 0);
        assert_eq!(fixed_point_digit_sum(1), 1);
        assert_eq!(fixed_point_digit_sum(-1), -1);
    }
    #[test]
    fn test_test_fixed_point_examples() {
        let _ = test_fixed_point();
    }
    #[test]
    fn test_factorial_acc_examples() {
        assert_eq!(factorial_acc(0, 0), 0);
        assert_eq!(factorial_acc(1, 2), 3);
        assert_eq!(factorial_acc(-1, 1), 0);
    }
    #[test]
    fn test_test_accumulator_passing_examples() {
        let _ = test_accumulator_passing();
    }
    #[test]
    fn test_cps_factorial_examples() {
        assert_eq!(cps_factorial(0), 0);
        assert_eq!(cps_factorial(1), 1);
        assert_eq!(cps_factorial(-1), -1);
    }
    #[test]
    fn test_test_cps_examples() {
        let _ = test_cps();
    }
    #[test]
    fn test_church_true_examples() {
        assert_eq!(church_true(0, 0), 0);
        assert_eq!(church_true(1, 2), 3);
        assert_eq!(church_true(-1, 1), 0);
    }
    #[test]
    fn test_church_false_examples() {
        assert_eq!(church_false(0, 0), 0);
        assert_eq!(church_false(1, 2), 3);
        assert_eq!(church_false(-1, 1), 0);
    }
    #[test]
    fn test_test_church_booleans_examples() {
        let _ = test_church_booleans();
    }
    #[test]
    fn quickcheck_church_zero() {
        fn prop(x: i32) -> TestResult {
            let result = church_zero(x.clone());
            if result != x {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32) -> TestResult);
    }
    #[test]
    fn test_church_zero_examples() {
        assert_eq!(church_zero(0), 0);
        assert_eq!(church_zero(1), 1);
        assert_eq!(church_zero(-1), -1);
    }
    #[test]
    fn test_test_church_numerals_examples() {
        let _ = test_church_numerals();
    }
    #[test]
    fn test_y_factorial_examples() {
        assert_eq!(y_factorial(0), 0);
        assert_eq!(y_factorial(1), 1);
        assert_eq!(y_factorial(-1), -1);
    }
    #[test]
    fn test_y_fibonacci_examples() {
        assert_eq!(y_fibonacci(0), 0);
        assert_eq!(y_fibonacci(1), 1);
        assert_eq!(y_fibonacci(-1), -1);
    }
    #[test]
    fn test_y_power_examples() {
        assert_eq!(y_power(0, 0), 0);
        assert_eq!(y_power(1, 2), 3);
        assert_eq!(y_power(-1, 1), 0);
    }
    #[test]
    fn test_test_y_combinator_examples() {
        let _ = test_y_combinator();
    }
    #[test]
    fn test_test_lens_examples() {
        let _ = test_lens();
    }
    #[test]
    fn test_test_group_by_examples() {
        let _ = test_group_by();
    }
    #[test]
    fn test_take_while_positive_examples() {
        assert_eq!(take_while_positive(vec![]), vec![]);
        assert_eq!(take_while_positive(vec![1]), vec![1]);
    }
    #[test]
    fn test_drop_while_positive_examples() {
        assert_eq!(drop_while_positive(vec![]), vec![]);
        assert_eq!(drop_while_positive(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_take_drop_examples() {
        let _ = test_take_drop();
    }
    #[test]
    fn test_test_sliding_window_examples() {
        let _ = test_sliding_window();
    }
    #[test]
    fn test_test_interleave_examples() {
        let _ = test_interleave();
    }
    #[test]
    fn test_test_chunk_examples() {
        let _ = test_chunk();
    }
    #[test]
    fn test_unique_preserve_order_examples() {
        assert_eq!(unique_preserve_order(vec![]), vec![]);
        assert_eq!(unique_preserve_order(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_unique_examples() {
        let _ = test_unique();
    }
    #[test]
    fn test_most_frequent_examples() {
        assert_eq!(most_frequent(&vec![]), 0);
        assert_eq!(most_frequent(&vec![1]), 1);
        assert_eq!(most_frequent(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_test_frequency_examples() {
        let _ = test_frequency();
    }
    #[test]
    fn test_tally_stats_examples() {
        assert_eq!(tally_stats(vec![]), vec![]);
        assert_eq!(tally_stats(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_tally_examples() {
        let _ = test_tally();
    }
    #[test]
    fn test_flatten_2d_examples() {
        assert_eq!(flatten_2d(vec![]), vec![]);
        assert_eq!(flatten_2d(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_flatten_examples() {
        let _ = test_flatten();
    }
    #[test]
    fn quickcheck_map_with_index() {
        fn prop(vals: Vec<i32>) -> TestResult {
            let input_len = vals.len();
            let result = map_with_index(&vals);
            if result.len() != input_len {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_map_with_index_examples() {
        assert_eq!(map_with_index(vec![]), vec![]);
        assert_eq!(map_with_index(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_map_with_index_examples() {
        let _ = test_map_with_index();
    }
    #[test]
    fn test_test_unfold_examples() {
        let _ = test_unfold();
    }
    #[test]
    fn test_iterate_halve_examples() {
        assert_eq!(iterate_halve(0), 0);
        assert_eq!(iterate_halve(1), 1);
        assert_eq!(iterate_halve(-1), -1);
    }
    #[test]
    fn test_iterate_triple_plus_one_examples() {
        assert_eq!(iterate_triple_plus_one(0, 0), 0);
        assert_eq!(iterate_triple_plus_one(1, 2), 3);
        assert_eq!(iterate_triple_plus_one(-1, 1), 0);
    }
    #[test]
    fn test_test_iterate_examples() {
        let _ = test_iterate();
    }
    #[test]
    fn test_test_maybe_examples() {
        let _ = test_maybe();
    }
    #[test]
    fn test_run_all_tests_examples() {
        let _ = run_all_tests();
    }
}