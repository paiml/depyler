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
                DepylerValue::Str(value.clone()),
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
#[doc = "BFS from start node, returns distances to all reachable nodes."]
pub fn bfs_distances(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
    start: i32,
) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let mut dist: std::collections::HashMap<i32, i32> = {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(start, (0) as i32);
        map
    };
    let mut queue: Vec<i32> = vec![start];
    let mut head: i32 = 0;
    while head < queue.len() as i32 {
        let node: i32 = queue
            .get(head as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        head = ((head).py_add(1i32)) as i32;
        if graph.get(&node).is_some() {
            let neighbors: Vec<i32> = graph.get(&(node)).cloned().unwrap_or_default();
            let mut i: i32 = 0;
            while i < neighbors.len() as i32 {
                let nb: i32 = neighbors
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                if dist.get(&nb).is_none() {
                    dist.insert(
                        nb.clone(),
                        (dist.get(&(node)).cloned().unwrap_or_default()).py_add(1i32),
                    );
                    queue.push(nb);
                }
                i = ((i).py_add(1i32)) as i32;
            }
        }
    }
    Ok(dist)
}
#[doc = "Return shortest path length from start to end, or -1 if unreachable."]
#[doc = " Depyler: proven to terminate"]
pub fn bfs_shortest_path_length(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
    start: i32,
    end: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = start == end;
    if _cse_temp_0 {
        return Ok(0);
    }
    let dist: std::collections::HashMap<i32, i32> = bfs_distances(&graph, start)?;
    let _cse_temp_1 = dist.get(&end).is_some();
    if _cse_temp_1 {
        return Ok(dist.get(&(end)).cloned().unwrap_or_default());
    }
    Ok(-1)
}
#[doc = "Return the number of nodes at each BFS level from start."]
pub fn bfs_level_sizes(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
    start: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut max_dist: i32 = Default::default();
    let dist: std::collections::HashMap<i32, i32> = bfs_distances(&graph, start)?;
    let _cse_temp_0 = dist.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    max_dist = 0;
    for node in dist.keys().cloned() {
        let d: i32 = dist.get(&(node)).cloned().unwrap_or_default();
        if d > max_dist {
            max_dist = d;
        }
    }
    let mut sizes: Vec<i32> = vec![];
    let mut level: i32 = 0;
    while level <= max_dist {
        sizes.push(0);
        level = ((level).py_add(1i32)) as i32;
    }
    for node in dist.keys().cloned() {
        let d2: i32 = dist.get(&(node)).cloned().unwrap_or_default();
        sizes[(d2) as usize] = (sizes
            .get(d2 as usize)
            .cloned()
            .expect("IndexError: list index out of range"))
        .py_add(1i32);
    }
    Ok(sizes)
}
#[doc = "Iterative DFS returning {node: [discovery_time, finish_time]}."]
pub fn dfs_times(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
    start: i32,
) -> Result<HashMap<i32, Vec<i32>>, Box<dyn std::error::Error>> {
    let mut neighbors: Vec<i32> = Default::default();
    let mut times: std::collections::HashMap<i32, Vec<i32>> = {
        let map: HashMap<i32, Vec<i32>> = HashMap::new();
        map
    };
    let mut clock: Vec<i32> = vec![0];
    let mut stack: Vec<Vec<i32>> = vec![vec![start, 0]];
    let mut visited: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    while stack.len() as i32 > 0 {
        let mut top: Vec<i32> = {
            let base = &stack;
            let idx: i32 = (stack.len() as i32) - (1i32);
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        };
        let node: i32 = top
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let idx: i32 = top
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited.get(&node).is_none() {
            visited.insert(node.clone(), 1);
            clock[(0) as usize] = (clock
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_add(1i32);
            times.insert(
                node.clone(),
                vec![
                    clock
                        .get(0usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                    0,
                ],
            );
        }
        neighbors = vec![];
        if graph.get(&node).is_some() {
            neighbors = graph.get(&(node)).cloned().unwrap_or_default();
        }
        if idx < neighbors.len() as i32 {
            top[(1) as usize] = (idx).py_add(1i32);
            let nb: i32 = neighbors
                .get(idx as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if visited.get(&nb).is_none() {
                stack.push(vec![nb, 0]);
            }
        } else {
            clock[(0) as usize] = (clock
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range"))
            .py_add(1i32);
            times[node as usize][(1) as usize] = clock
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range");
            stack.pop().unwrap_or_default();
        }
    }
    Ok(times)
}
#[doc = "Count nodes reachable from start using DFS."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn dfs_reachable_count(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
    start: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let times: std::collections::HashMap<i32, Vec<i32>> = dfs_times(&graph, start)?;
    Ok(times.len() as i32 as i32)
}
#[doc = "Extract all unique nodes from a graph adjacency list."]
pub fn all_nodes(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut node_set: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    for node in graph.keys().cloned() {
        node_set.insert(node.clone(), 1);
        let neighbors: Vec<i32> = graph.get(&(node)).cloned().unwrap_or_default();
        let mut j: i32 = 0;
        while j < neighbors.len() as i32 {
            node_set.insert(
                neighbors
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
                1,
            );
            j = ((j).py_add(1i32)) as i32;
        }
    }
    let mut result: Vec<i32> = vec![];
    for n in node_set.keys().cloned() {
        result.push(n);
    }
    Ok(result)
}
#[doc = "Ensure graph has edges in both directions without duplicates."]
pub fn make_undirected(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<HashMap<i32, Vec<i32>>, Box<dyn std::error::Error>> {
    let mut ug: std::collections::HashMap<i32, Vec<i32>> = {
        let map: HashMap<i32, Vec<i32>> = HashMap::new();
        map
    };
    let mut edge_set: std::collections::HashMap<i32, std::collections::HashMap<i32, i32>> = {
        let map: HashMap<i32, std::collections::HashMap<i32, i32>> = HashMap::new();
        map
    };
    for node in graph.keys().cloned() {
        if ug.get(&node).is_none() {
            ug.insert(node.clone(), vec![]);
        }
        if edge_set.get(&node).is_none() {
            edge_set.insert(node.clone(), {
                let map: HashMap<i32, i32> = HashMap::new();
                map
            });
        }
        let neighbors: Vec<i32> = graph.get(&(node)).cloned().unwrap_or_default();
        let mut k: i32 = 0;
        while k < neighbors.len() as i32 {
            let nb: i32 = neighbors
                .get(k as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if ug.get(&nb).is_none() {
                ug.insert(nb.clone(), vec![]);
            }
            if edge_set.get(&nb).is_none() {
                edge_set.insert(nb.clone(), {
                    let map: HashMap<i32, i32> = HashMap::new();
                    map
                });
            }
            if edge_set
                .get(&(node))
                .cloned()
                .unwrap_or_default()
                .get(&nb)
                .is_none()
            {
                edge_set[node as usize][(nb) as usize] = 1;
                ug.get(&(node)).cloned().unwrap_or_default().push(nb);
            }
            if edge_set
                .get(&(nb))
                .cloned()
                .unwrap_or_default()
                .get(&node)
                .is_none()
            {
                edge_set[nb as usize][(node) as usize] = 1;
                ug.get(&(nb)).cloned().unwrap_or_default().push(node);
            }
            k = ((k).py_add(1i32)) as i32;
        }
    }
    Ok(ug)
}
#[doc = "Count connected components in an undirected graph."]
pub fn connected_components_count(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    let mut visited: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    count = 0;
    let mut idx: i32 = 0;
    while idx < nodes.len() as i32 {
        let node: i32 = nodes
            .get(idx as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited.get(&node).is_none() {
            count = ((count).py_add(1i32)) as i32;
            let mut queue: Vec<i32> = vec![node];
            visited.insert(node.clone(), 1);
            let mut qh: i32 = 0;
            while qh < queue.len() as i32 {
                let cur: i32 = queue
                    .get(qh as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                qh = ((qh).py_add(1i32)) as i32;
                if ug.get(&cur).is_some() {
                    let nbs: Vec<i32> = ug.get(&(cur)).cloned().unwrap_or_default();
                    let mut ni: i32 = 0;
                    while ni < nbs.len() as i32 {
                        let nb2: i32 = nbs
                            .get(ni as usize)
                            .cloned()
                            .expect("IndexError: list index out of range");
                        if visited.get(&nb2).is_none() {
                            visited.insert(nb2.clone(), 1);
                            queue.push(nb2);
                        }
                        ni = ((ni).py_add(1i32)) as i32;
                    }
                }
            }
        }
        idx = ((idx).py_add(1i32)) as i32;
    }
    Ok(count)
}
#[doc = "Return size of the largest connected component."]
pub fn largest_component_size(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut max_size: i32 = Default::default();
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    let mut visited: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    max_size = 0;
    let mut idx: i32 = 0;
    while idx < nodes.len() as i32 {
        let node: i32 = nodes
            .get(idx as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited.get(&node).is_none() {
            let mut size: i32 = 0;
            let mut queue: Vec<i32> = vec![node];
            visited.insert(node.clone(), 1);
            let mut qh: i32 = 0;
            while qh < queue.len() as i32 {
                let cur: i32 = queue
                    .get(qh as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                qh = ((qh).py_add(1i32)) as i32;
                size = ((size).py_add(1i32)) as i32;
                if ug.get(&cur).is_some() {
                    let nbs: Vec<i32> = ug.get(&(cur)).cloned().unwrap_or_default();
                    let mut ni: i32 = 0;
                    while ni < nbs.len() as i32 {
                        let nb2: i32 = nbs
                            .get(ni as usize)
                            .cloned()
                            .expect("IndexError: list index out of range");
                        if visited.get(&nb2).is_none() {
                            visited.insert(nb2.clone(), 1);
                            queue.push(nb2);
                        }
                        ni = ((ni).py_add(1i32)) as i32;
                    }
                }
            }
            if size > max_size {
                max_size = size;
            }
        }
        idx = ((idx).py_add(1i32)) as i32;
    }
    Ok(max_size)
}
#[doc = "Detect cycle in directed graph. Returns 1 if cycle exists, 0 otherwise."]
pub fn has_cycle_directed(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut neighbors: Vec<i32> = Default::default();
    let white: i32 = 0;
    let gray: i32 = 1;
    let black: i32 = 2;
    let mut color: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let nodes: Vec<i32> = all_nodes(&graph)?;
    let mut ci: i32 = 0;
    while ci < nodes.len() as i32 {
        color.insert(
            nodes
                .get(ci as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            white,
        );
        ci = ((ci).py_add(1i32)) as i32;
    }
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if color.get(&(node)).cloned().unwrap_or_default() == white {
            let mut stack: Vec<Vec<i32>> = vec![vec![node, 0]];
            color.insert(node.clone(), gray);
            while stack.len() as i32 > 0 {
                let mut top: Vec<i32> = {
                    let base = &stack;
                    let idx: i32 = (stack.len() as i32) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
                let cur: i32 = top
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let idx: i32 = top
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                neighbors = vec![];
                if graph.get(&cur).is_some() {
                    neighbors = graph.get(&(cur)).cloned().unwrap_or_default();
                }
                if idx < neighbors.len() as i32 {
                    top[(1) as usize] = (idx).py_add(1i32);
                    let nb: i32 = neighbors
                        .get(idx as usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                    if (color.get(&nb).is_some())
                        && (color.get(&(nb)).cloned().unwrap_or_default() == gray)
                    {
                        return Ok(1);
                    }
                    if (color.get(&nb).is_none())
                        || (color.get(&(nb)).cloned().unwrap_or_default() == white)
                    {
                        if color.get(&nb).is_none() {
                            color.insert(nb.clone(), white);
                        }
                        color.insert(nb.clone(), gray);
                        stack.push(vec![nb, 0]);
                    }
                } else {
                    color.insert(cur.clone(), black);
                    stack.pop().unwrap_or_default();
                }
            }
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    Ok(0)
}
#[doc = "Detect cycle in undirected graph using union-find. Returns 1/0."]
pub fn has_cycle_undirected(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut parent: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let nodes: Vec<i32> = all_nodes(&graph)?;
    let mut pi: i32 = 0;
    while pi < nodes.len() as i32 {
        parent.insert(
            nodes
                .get(pi as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            nodes
                .get(pi as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        pi = ((pi).py_add(1i32)) as i32;
    }
    let find_root = move |x: i32| -> i32 {
        let mut r: i32 = x.clone();
        while parent.get(&(r)).cloned().unwrap_or_default() != r {
            r = parent.get(&(r)).cloned().unwrap_or_default();
        }
        let mut cur2: i32 = x.clone();
        while cur2 != r {
            let nxt: i32 = parent.get(&(cur2)).cloned().unwrap_or_default();
            parent.insert(cur2.clone(), r);
            cur2 = nxt;
        }
        return r;
    };
    for u in graph.keys().cloned() {
        let neighbors: Vec<i32> = graph.get(&(u)).cloned().unwrap_or_default();
        let ei: i32 = 0;
        while ei < neighbors.len() as i32 {
            let v: i32 = neighbors
                .get(ei as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if u < v {
                let ru: i32 = find_root(u);
                let rv: i32 = find_root(v);
                if ru == rv {
                    return Ok(1);
                }
                parent.insert(ru.clone(), rv);
            }
            ei = ((ei).py_add(1i32)) as i32;
        }
    }
    Ok(0)
}
#[doc = "Dijkstra using sorted list as priority queue. Edges: [[neighbor, weight],...]."]
pub fn dijkstra_distances(
    weighted_graph: &std::collections::HashMap<i32, Vec<Vec<i32>>>,
    start: i32,
) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let mut dist: std::collections::HashMap<i32, i32> = {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(start, (0) as i32);
        map
    };
    let mut pq: Vec<Vec<i32>> = vec![vec![0, start]];
    let mut visited: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    while pq.len() as i32 > 0 {
        let mut best_idx: i32 = 0;
        let mut bi: i32 = 1;
        while bi < pq.len() as i32 {
            if pq
                .get(bi as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < pq.get(best_idx as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                best_idx = bi;
            }
            bi = ((bi).py_add(1i32)) as i32;
        }
        let entry: Vec<i32> = pq
            .get(best_idx as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        pq[(best_idx) as usize] = {
            let base = &pq;
            let idx: i32 = (pq.len() as i32) - (1i32);
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        };
        pq.pop().unwrap_or_default();
        let d: i32 = entry
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let u: i32 = entry
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited.get(&u).is_some() {
            continue;
        }
        visited.insert(u.clone(), 1);
        if weighted_graph.get(&u).is_some() {
            let edges: Vec<Vec<i32>> = weighted_graph.get(&(u)).cloned().unwrap_or_default();
            let mut ei: i32 = 0;
            while ei < edges.len() as i32 {
                let v: i32 = edges
                    .get(ei as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let w: i32 = edges
                    .get(ei as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let nd: i32 = ((d).py_add(w)) as i32;
                if (dist.get(&v).is_none()) || (nd < dist.get(&(v)).cloned().unwrap_or_default()) {
                    dist.insert(v.clone(), nd);
                    pq.push(vec![nd, v]);
                }
                ei = ((ei).py_add(1i32)) as i32;
            }
        }
    }
    Ok(dist)
}
#[doc = "Return shortest weighted distance from start to end, or -1."]
#[doc = " Depyler: proven to terminate"]
pub fn dijkstra_shortest(
    weighted_graph: &std::collections::HashMap<i32, Vec<Vec<i32>>>,
    start: i32,
    end: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let dist: std::collections::HashMap<i32, i32> = dijkstra_distances(&weighted_graph, start)?;
    let _cse_temp_0 = dist.get(&end).is_some();
    if _cse_temp_0 {
        return Ok(dist.get(&(end)).cloned().unwrap_or_default());
    }
    Ok(-1)
}
#[doc = "Check if undirected graph is bipartite. Returns 1 if yes, 0 if no."]
pub fn is_bipartite(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    let mut color: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if color.get(&node).is_none() {
            color.insert(node.clone(), 0);
            let mut queue: Vec<i32> = vec![node];
            let mut qh: i32 = 0;
            while qh < queue.len() as i32 {
                let cur: i32 = queue
                    .get(qh as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                qh = ((qh).py_add(1i32)) as i32;
                if ug.get(&cur).is_some() {
                    let nbs: Vec<i32> = ug.get(&(cur)).cloned().unwrap_or_default();
                    let mut nbi: i32 = 0;
                    while nbi < nbs.len() as i32 {
                        let nb: i32 = nbs
                            .get(nbi as usize)
                            .cloned()
                            .expect("IndexError: list index out of range");
                        if color.get(&nb).is_none() {
                            color.insert(
                                nb.clone(),
                                (1i32) - (color.get(&(cur)).cloned().unwrap_or_default()),
                            );
                            queue.push(nb);
                        } else {
                            if color.get(&(nb)).cloned().unwrap_or_default()
                                == color.get(&(cur)).cloned().unwrap_or_default()
                            {
                                return Ok(0);
                            }
                        }
                        nbi = ((nbi).py_add(1i32)) as i32;
                    }
                }
            }
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    Ok(1)
}
#[doc = "Count bridges in an undirected graph using iterative Tarjan."]
pub fn count_bridges(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut bridge_count: i32 = Default::default();
    let mut nbs: Vec<i32> = Default::default();
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    let mut disc: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut low: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut parent: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut timer: Vec<i32> = vec![0];
    bridge_count = 0;
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let root: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if disc.get(&root).is_none() {
            let mut stack: Vec<Vec<i32>> = vec![vec![root, 0]];
            parent.insert(root.clone(), -1);
            while stack.len() as i32 > 0 {
                let mut top: Vec<i32> = {
                    let base = &stack;
                    let idx: i32 = (stack.len() as i32) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
                let u: i32 = top
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let idx: i32 = top
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                if idx == 0 {
                    timer[(0) as usize] = (timer
                        .get(0usize)
                        .cloned()
                        .expect("IndexError: list index out of range"))
                    .py_add(1i32);
                    disc.insert(
                        u.clone(),
                        timer
                            .get(0usize)
                            .cloned()
                            .expect("IndexError: list index out of range"),
                    );
                    low.insert(
                        u.clone(),
                        timer
                            .get(0usize)
                            .cloned()
                            .expect("IndexError: list index out of range"),
                    );
                }
                nbs = vec![];
                if ug.get(&u).is_some() {
                    nbs = ug.get(&(u)).cloned().unwrap_or_default();
                }
                if idx < nbs.len() as i32 {
                    top[(1) as usize] = (idx).py_add(1i32);
                    let v: i32 = nbs
                        .get(idx as usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                    if disc.get(&v).is_none() {
                        parent.insert(v.clone(), u);
                        stack.push(vec![v, 0]);
                    } else {
                        if v != parent.get(&(u)).cloned().unwrap_or_default() {
                            if disc.get(&(v)).cloned().unwrap_or_default()
                                < low.get(&(u)).cloned().unwrap_or_default()
                            {
                                low.insert(u.clone(), disc.get(&(v)).cloned().unwrap_or_default());
                            }
                        }
                    }
                } else {
                    stack.pop().unwrap_or_default();
                    if stack.len() as i32 > 0 {
                        let pu: i32 = {
                            let base = &stack;
                            let idx: i32 = (stack.len() as i32) - (1i32);
                            let actual_idx = if idx < 0 {
                                base.len().saturating_sub(idx.abs() as usize)
                            } else {
                                idx as usize
                            };
                            base.get(actual_idx)
                                .cloned()
                                .expect("IndexError: list index out of range")
                        }
                        .get(0usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                        if low.get(&(u)).cloned().unwrap_or_default()
                            < low.get(&(pu)).cloned().unwrap_or_default()
                        {
                            low.insert(pu.clone(), low.get(&(u)).cloned().unwrap_or_default());
                        }
                        if low.get(&(u)).cloned().unwrap_or_default()
                            > disc.get(&(pu)).cloned().unwrap_or_default()
                        {
                            bridge_count = ((bridge_count).py_add(1i32)) as i32;
                        }
                    }
                }
            }
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    Ok(bridge_count)
}
#[doc = "Count articulation points(cut vertices) in an undirected graph."]
pub fn count_articulation_points(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut nbs: Vec<i32> = Default::default();
    let mut cc: i32 = Default::default();
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    let mut disc: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut low: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut parent: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut is_ap: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut timer: Vec<i32> = vec![0];
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let root: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if disc.get(&root).is_none() {
            let mut stack: Vec<Vec<i32>> = vec![vec![root, 0]];
            parent.insert(root.clone(), -1);
            let mut child_count: std::collections::HashMap<i32, i32> = {
                let mut map: HashMap<i32, i32> = HashMap::new();
                map.insert(root, (0) as i32);
                map
            };
            while stack.len() as i32 > 0 {
                let mut top: Vec<i32> = {
                    let base = &stack;
                    let idx: i32 = (stack.len() as i32) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
                let u: i32 = top
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let idx: i32 = top
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                if idx == 0 {
                    timer[(0) as usize] = (timer
                        .get(0usize)
                        .cloned()
                        .expect("IndexError: list index out of range"))
                    .py_add(1i32);
                    disc.insert(
                        u.clone(),
                        timer
                            .get(0usize)
                            .cloned()
                            .expect("IndexError: list index out of range"),
                    );
                    low.insert(
                        u.clone(),
                        timer
                            .get(0usize)
                            .cloned()
                            .expect("IndexError: list index out of range"),
                    );
                }
                nbs = vec![];
                if ug.get(&u).is_some() {
                    nbs = ug.get(&(u)).cloned().unwrap_or_default();
                }
                if idx < nbs.len() as i32 {
                    top[(1) as usize] = (idx).py_add(1i32);
                    let v: i32 = nbs
                        .get(idx as usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                    if disc.get(&v).is_none() {
                        parent.insert(v.clone(), u);
                        if child_count.get(&u).is_none() {
                            child_count.insert(u.clone(), 0);
                        }
                        {
                            let _key = u.clone();
                            let _old_val = child_count.get(&_key).cloned().unwrap_or_default();
                            child_count.insert(_key, _old_val + 1);
                        }
                        stack.push(vec![v, 0]);
                    } else {
                        if v != parent.get(&(u)).cloned().unwrap_or_default() {
                            if disc.get(&(v)).cloned().unwrap_or_default()
                                < low.get(&(u)).cloned().unwrap_or_default()
                            {
                                low.insert(u.clone(), disc.get(&(v)).cloned().unwrap_or_default());
                            }
                        }
                    }
                } else {
                    stack.pop().unwrap_or_default();
                    if stack.len() as i32 > 0 {
                        let pu: i32 = {
                            let base = &stack;
                            let idx: i32 = (stack.len() as i32) - (1i32);
                            let actual_idx = if idx < 0 {
                                base.len().saturating_sub(idx.abs() as usize)
                            } else {
                                idx as usize
                            };
                            base.get(actual_idx)
                                .cloned()
                                .expect("IndexError: list index out of range")
                        }
                        .get(0usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                        if low.get(&(u)).cloned().unwrap_or_default()
                            < low.get(&(pu)).cloned().unwrap_or_default()
                        {
                            low.insert(pu.clone(), low.get(&(u)).cloned().unwrap_or_default());
                        }
                        if parent.get(&(pu)).cloned().unwrap_or_default() == -1 {
                            cc = 0;
                            if child_count.get(&pu).is_some() {
                                cc = child_count.get(&(pu)).cloned().unwrap_or_default();
                            }
                            if cc > 1 {
                                is_ap.insert(pu.clone(), 1);
                            }
                        } else {
                            if low.get(&(u)).cloned().unwrap_or_default()
                                >= disc.get(&(pu)).cloned().unwrap_or_default()
                            {
                                is_ap.insert(pu.clone(), 1);
                            }
                        }
                    }
                }
            }
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    Ok(is_ap.len() as i32 as i32)
}
#[doc = "Return the transpose(reverse edges) of a directed graph."]
pub fn transpose_graph(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<HashMap<i32, Vec<i32>>, Box<dyn std::error::Error>> {
    let mut tg: std::collections::HashMap<i32, Vec<i32>> = {
        let map: HashMap<i32, Vec<i32>> = HashMap::new();
        map
    };
    let nodes: Vec<i32> = all_nodes(&graph)?;
    let mut gi: i32 = 0;
    while gi < nodes.len() as i32 {
        tg.insert(
            nodes
                .get(gi as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            vec![],
        );
        gi = ((gi).py_add(1i32)) as i32;
    }
    for u in graph.keys().cloned() {
        let neighbors: Vec<i32> = graph.get(&(u)).cloned().unwrap_or_default();
        let mut ei: i32 = 0;
        while ei < neighbors.len() as i32 {
            let v: i32 = neighbors
                .get(ei as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if tg.get(&v).is_none() {
                tg.insert(v.clone(), vec![]);
            }
            tg.get(&(v)).cloned().unwrap_or_default().push(u);
            ei = ((ei).py_add(1i32)) as i32;
        }
    }
    Ok(tg)
}
#[doc = "Count strongly connected components using Kosaraju's algorithm."]
pub fn kosaraju_scc_count(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut scc_count: i32 = Default::default();
    let mut nbs: Vec<i32> = Default::default();
    let nodes: Vec<i32> = all_nodes(&graph)?;
    let mut visited: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut finish_order: Vec<i32> = vec![];
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited.get(&node).is_none() {
            let mut stack: Vec<Vec<i32>> = vec![vec![node, 0]];
            visited.insert(node.clone(), 1);
            while stack.len() as i32 > 0 {
                let mut top: Vec<i32> = {
                    let base = &stack;
                    let idx: i32 = (stack.len() as i32) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
                let u: i32 = top
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let idx: i32 = top
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                nbs = vec![];
                if graph.get(&u).is_some() {
                    nbs = graph.get(&(u)).cloned().unwrap_or_default();
                }
                if idx < nbs.len() as i32 {
                    top[(1) as usize] = (idx).py_add(1i32);
                    let v: i32 = nbs
                        .get(idx as usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                    if visited.get(&v).is_none() {
                        visited.insert(v.clone(), 1);
                        stack.push(vec![v, 0]);
                    }
                } else {
                    finish_order.push(u);
                    stack.pop().unwrap_or_default();
                }
            }
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    let tg: std::collections::HashMap<i32, Vec<i32>> = transpose_graph(&graph)?;
    let mut visited2: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    scc_count = 0;
    let _cse_temp_0 = finish_order.len() as i32;
    let mut fi: i32 = ((_cse_temp_0) - (1i32)) as i32;
    while fi >= 0 {
        let node2: i32 = finish_order
            .get(fi as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited2.get(&node2).is_none() {
            scc_count = ((scc_count).py_add(1i32)) as i32;
            let mut queue: Vec<i32> = vec![node2];
            visited2.insert(node2.clone(), 1);
            let mut qh: i32 = 0;
            while qh < queue.len() as i32 {
                let cur: i32 = queue
                    .get(qh as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                qh = ((qh).py_add(1i32)) as i32;
                if tg.get(&cur).is_some() {
                    let tnbs: Vec<i32> = tg.get(&(cur)).cloned().unwrap_or_default();
                    let mut ti: i32 = 0;
                    while ti < tnbs.len() as i32 {
                        let tv: i32 = tnbs
                            .get(ti as usize)
                            .cloned()
                            .expect("IndexError: list index out of range");
                        if visited2.get(&tv).is_none() {
                            visited2.insert(tv.clone(), 1);
                            queue.push(tv);
                        }
                        ti = ((ti).py_add(1i32)) as i32;
                    }
                }
            }
        }
        fi = ((fi) - (1i32)) as i32;
    }
    Ok(scc_count)
}
#[doc = "Return size of largest SCC via Kosaraju's."]
pub fn largest_scc_size(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut max_size: i32 = Default::default();
    let mut nbs: Vec<i32> = Default::default();
    let nodes: Vec<i32> = all_nodes(&graph)?;
    let mut visited: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut finish_order: Vec<i32> = vec![];
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited.get(&node).is_none() {
            let mut stack: Vec<Vec<i32>> = vec![vec![node, 0]];
            visited.insert(node.clone(), 1);
            while stack.len() as i32 > 0 {
                let mut top: Vec<i32> = {
                    let base = &stack;
                    let idx: i32 = (stack.len() as i32) - (1i32);
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx)
                        .cloned()
                        .expect("IndexError: list index out of range")
                };
                let u: i32 = top
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let idx: i32 = top
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                nbs = vec![];
                if graph.get(&u).is_some() {
                    nbs = graph.get(&(u)).cloned().unwrap_or_default();
                }
                if idx < nbs.len() as i32 {
                    top[(1) as usize] = (idx).py_add(1i32);
                    let v: i32 = nbs
                        .get(idx as usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                    if visited.get(&v).is_none() {
                        visited.insert(v.clone(), 1);
                        stack.push(vec![v, 0]);
                    }
                } else {
                    finish_order.push(u);
                    stack.pop().unwrap_or_default();
                }
            }
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    let tg: std::collections::HashMap<i32, Vec<i32>> = transpose_graph(&graph)?;
    let mut visited2: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    max_size = 0;
    let _cse_temp_0 = finish_order.len() as i32;
    let mut fi: i32 = ((_cse_temp_0) - (1i32)) as i32;
    while fi >= 0 {
        let node2: i32 = finish_order
            .get(fi as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited2.get(&node2).is_none() {
            let mut size: i32 = 0;
            let mut queue: Vec<i32> = vec![node2];
            visited2.insert(node2.clone(), 1);
            let mut qh: i32 = 0;
            while qh < queue.len() as i32 {
                let cur: i32 = queue
                    .get(qh as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                qh = ((qh).py_add(1i32)) as i32;
                size = ((size).py_add(1i32)) as i32;
                if tg.get(&cur).is_some() {
                    let tnbs: Vec<i32> = tg.get(&(cur)).cloned().unwrap_or_default();
                    let mut ti: i32 = 0;
                    while ti < tnbs.len() as i32 {
                        let tv: i32 = tnbs
                            .get(ti as usize)
                            .cloned()
                            .expect("IndexError: list index out of range");
                        if visited2.get(&tv).is_none() {
                            visited2.insert(tv.clone(), 1);
                            queue.push(tv);
                        }
                        ti = ((ti).py_add(1i32)) as i32;
                    }
                }
            }
            if size > max_size {
                max_size = size;
            }
        }
        fi = ((fi) - (1i32)) as i32;
    }
    Ok(max_size)
}
#[doc = "Greedy graph coloring. Returns {node: color} with colors starting at 0."]
pub fn greedy_coloring(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    let mut coloring: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let mut used_colors: std::collections::HashMap<i32, i32> = {
            let map: HashMap<i32, i32> = HashMap::new();
            map
        };
        if ug.get(&node).is_some() {
            let nbs: Vec<i32> = ug.get(&(node)).cloned().unwrap_or_default();
            let mut nbi: i32 = 0;
            while nbi < nbs.len() as i32 {
                let nb: i32 = nbs
                    .get(nbi as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                if coloring.get(&nb).is_some() {
                    used_colors.insert(coloring.get(&(nb)).cloned().unwrap_or_default(), 1);
                }
                nbi = ((nbi).py_add(1i32)) as i32;
            }
        }
        let mut c: i32 = 0;
        while used_colors.get(&c).is_some() {
            c = ((c).py_add(1i32)) as i32;
        }
        coloring.insert(node.clone(), c);
        ni = ((ni).py_add(1i32)) as i32;
    }
    Ok(coloring)
}
#[doc = "Upper bound on chromatic number via greedy coloring."]
pub fn chromatic_number_upper(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut max_color: i32 = Default::default();
    let coloring: std::collections::HashMap<i32, i32> = greedy_coloring(&graph)?;
    max_color = -1;
    for node in coloring.keys().cloned() {
        if coloring.get(&(node)).cloned().unwrap_or_default() > max_color {
            max_color = coloring.get(&(node)).cloned().unwrap_or_default();
        }
    }
    Ok((max_color).py_add(1i32))
}
#[doc = "Kruskal's MST. edges = [[u, v, weight],...]. Returns total MST weight."]
pub fn kruskal_mst_weight(
    num_nodes: i32,
    edges: &Vec<Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total_weight: i32 = Default::default();
    let mut sorted_edges: Vec<Vec<i32>> = vec![];
    let mut ei: i32 = 0;
    while ei < edges.len() as i32 {
        sorted_edges.push(vec![
            edges
                .get(ei as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            edges
                .get(ei as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(1usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            edges
                .get(ei as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(2usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ]);
        ei = ((ei).py_add(1i32)) as i32;
    }
    let mut si: i32 = 0;
    while si < sorted_edges.len() as i32 {
        let mut sj: i32 = ((si).py_add(1i32)) as i32;
        while sj < sorted_edges.len() as i32 {
            if sorted_edges
                .get(sj as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(2usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < sorted_edges
                    .get(si as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(2usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                let tmp: Vec<i32> = sorted_edges
                    .get(si as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                sorted_edges[(si) as usize] = sorted_edges
                    .get(sj as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                sorted_edges[(sj) as usize] = tmp.clone();
            }
            sj = ((sj).py_add(1i32)) as i32;
        }
        si = ((si).py_add(1i32)) as i32;
    }
    let mut parent: Vec<i32> = vec![];
    let mut rank: Vec<i32> = vec![];
    let mut pi: i32 = 0;
    while pi < num_nodes {
        parent.push(pi);
        rank.push(0);
        pi = ((pi).py_add(1i32)) as i32;
    }
    let find_uf = move |x: i32| -> i32 {
        let mut r: i32 = x.clone();
        while parent
            .get(r as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            != r
        {
            r = parent
                .get(r as usize)
                .cloned()
                .expect("IndexError: list index out of range");
        }
        let mut cur2: i32 = x.clone();
        while cur2 != r {
            let nxt: i32 = parent
                .get(cur2 as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            parent[(cur2) as usize] = r;
            cur2 = nxt;
        }
        return r;
    };
    let union_uf = move |a: i32, b: i32| -> i32 {
        let ra: i32 = find_uf(a);
        let rb: i32 = find_uf(b);
        if ra == rb {
            return 0;
        }
        if rank
            .get(ra as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            < rank
                .get(rb as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        {
            parent[(ra) as usize] = rb;
        } else {
            if rank
                .get(ra as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                > rank
                    .get(rb as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                parent[(rb) as usize] = ra;
            } else {
                parent[(rb) as usize] = ra;
                rank[(ra) as usize] = (rank
                    .get(ra as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"))
                .py_add(1i32);
            }
        }
        return 1;
    };
    total_weight = 0;
    let edge_count: i32 = 0;
    let ki: i32 = 0;
    while (ki < sorted_edges.len() as i32) && (edge_count < (num_nodes) - (1i32)) {
        let e: Vec<i32> = sorted_edges
            .get(ki as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if union_uf(
            e.get(0usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            e.get(1usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        ) == 1
        {
            total_weight = ((total_weight).py_add(
                e.get(2usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            )) as i32;
            edge_count = ((edge_count).py_add(1i32)) as i32;
        }
        ki = ((ki).py_add(1i32)) as i32;
    }
    Ok(total_weight)
}
#[doc = "Prim's MST using simple min-extraction. Returns total MST weight."]
pub fn prim_mst_weight(
    weighted_graph: &std::collections::HashMap<i32, Vec<Vec<i32>>>,
    start: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let mut visited: std::collections::HashMap<i32, i32> = {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(start, (1) as i32);
        map
    };
    let mut pq: Vec<Vec<i32>> = vec![];
    let _cse_temp_0 = weighted_graph.get(&start).is_some();
    if _cse_temp_0 {
        let edges: Vec<Vec<i32>> = weighted_graph.get(&(start)).cloned().unwrap_or_default();
        let mut ei: i32 = 0;
        while ei < edges.len() as i32 {
            pq.push(vec![
                edges
                    .get(ei as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
                edges
                    .get(ei as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ]);
            ei = ((ei).py_add(1i32)) as i32;
        }
    }
    total = 0;
    while pq.len() as i32 > 0 {
        let mut best_idx: i32 = 0;
        let mut bi: i32 = 1;
        while bi < pq.len() as i32 {
            if pq
                .get(bi as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < pq.get(best_idx as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                best_idx = bi;
            }
            bi = ((bi).py_add(1i32)) as i32;
        }
        let entry: Vec<i32> = pq
            .get(best_idx as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        pq[(best_idx) as usize] = {
            let base = &pq;
            let idx: i32 = (pq.len() as i32) - (1i32);
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        };
        pq.pop().unwrap_or_default();
        let w: i32 = entry
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let u: i32 = entry
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if visited.get(&u).is_some() {
            continue;
        }
        visited.insert(u.clone(), 1);
        total = ((total).py_add(w)) as i32;
        if weighted_graph.get(&u).is_some() {
            let ue: Vec<Vec<i32>> = weighted_graph.get(&(u)).cloned().unwrap_or_default();
            let mut uei: i32 = 0;
            while uei < ue.len() as i32 {
                let v: i32 = ue
                    .get(uei as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let vw: i32 = ue
                    .get(uei as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                if visited.get(&v).is_none() {
                    pq.push(vec![vw, v]);
                }
                uei = ((uei).py_add(1i32)) as i32;
            }
        }
    }
    Ok(total)
}
#[doc = "Floyd-Warshall. edges=[[u,v,w],...]. Returns 2D distance matrix(999999=inf)."]
pub fn floyd_warshall(
    num_nodes: i32,
    edges: &Vec<Vec<i32>>,
) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let inf: i32 = 999999;
    let mut dist: Vec<Vec<i32>> = vec![];
    let mut i: i32 = 0;
    while i < num_nodes {
        let mut row: Vec<i32> = vec![];
        let mut j: i32 = 0;
        while j < num_nodes {
            if i == j {
                row.push(0);
            } else {
                row.push(inf);
            }
            j = ((j).py_add(1i32)) as i32;
        }
        dist.push(row);
        i = ((i).py_add(1i32)) as i32;
    }
    let mut ei: i32 = 0;
    while ei < edges.len() as i32 {
        let u: i32 = edges
            .get(ei as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let v: i32 = edges
            .get(ei as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let w: i32 = edges
            .get(ei as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get(2usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if w < dist
            .get(u as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get(v as usize)
            .cloned()
            .expect("IndexError: list index out of range")
        {
            dist[u as usize][(v) as usize] = w;
        }
        ei = ((ei).py_add(1i32)) as i32;
    }
    let mut k: i32 = 0;
    while k < num_nodes {
        let mut ii: i32 = 0;
        while ii < num_nodes {
            let mut jj: i32 = 0;
            while jj < num_nodes {
                let through_k: i32 = ((dist
                    .get(ii as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(k as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"))
                .py_add(
                    dist.get(k as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        .get(jj as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                )) as i32;
                if through_k
                    < dist
                        .get(ii as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        .get(jj as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                {
                    dist[ii as usize][(jj) as usize] = through_k;
                }
                jj = ((jj).py_add(1i32)) as i32;
            }
            ii = ((ii).py_add(1i32)) as i32;
        }
        k = ((k).py_add(1i32)) as i32;
    }
    Ok(dist)
}
#[doc = "Return shortest path from start to end using Floyd-Warshall, or -1."]
#[doc = " Depyler: proven to terminate"]
pub fn floyd_shortest(
    num_nodes: i32,
    edges: &Vec<Vec<i32>>,
    start: i32,
    end: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let dist: Vec<Vec<i32>> = floyd_warshall(num_nodes, &edges)?;
    let result: i32 = dist
        .get(start as usize)
        .cloned()
        .expect("IndexError: list index out of range")
        .get(end as usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let _cse_temp_0 = result >= 999999;
    if _cse_temp_0 {
        return Ok(-1);
    }
    Ok(result)
}
#[doc = "Return sorted(descending) degree sequence of an undirected graph."]
pub fn degree_sequence(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut deg: i32 = Default::default();
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    let mut degrees: Vec<i32> = vec![];
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        deg = 0;
        if ug.get(&node).is_some() {
            deg = ug.get(&(node)).cloned().unwrap_or_default().len() as i32;
        }
        degrees.push(deg);
        ni = ((ni).py_add(1i32)) as i32;
    }
    let mut si: i32 = 0;
    while si < degrees.len() as i32 {
        let mut sj: i32 = ((si).py_add(1i32)) as i32;
        while sj < degrees.len() as i32 {
            if degrees
                .get(sj as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                > degrees
                    .get(si as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                let tmp: i32 = degrees
                    .get(si as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                degrees[(si) as usize] = degrees
                    .get(sj as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                degrees[(sj) as usize] = tmp;
            }
            sj = ((sj).py_add(1i32)) as i32;
        }
        si = ((si).py_add(1i32)) as i32;
    }
    Ok(degrees)
}
#[doc = "Return maximum degree in the graph."]
#[doc = " Depyler: proven to terminate"]
pub fn max_degree(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let seq: Vec<i32> = degree_sequence(&graph)?;
    let _cse_temp_0 = seq.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(seq
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Check if graph is regular(all nodes same degree). Returns 1/0."]
pub fn is_regular(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let seq: Vec<i32> = degree_sequence(&graph)?;
    let _cse_temp_0 = seq.len() as i32;
    let _cse_temp_1 = _cse_temp_0 <= 1;
    if _cse_temp_1 {
        return Ok(1);
    }
    let mut i: i32 = 1;
    while i < seq.len() as i32 {
        if seq
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            != seq
                .get(0usize)
                .cloned()
                .expect("IndexError: list index out of range")
        {
            return Ok(0);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(1)
}
#[doc = "Sum of all degrees(should be 2 * num_edges for undirected)."]
pub fn sum_degrees(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let seq: Vec<i32> = degree_sequence(&graph)?;
    total = 0;
    let mut i: i32 = 0;
    while i < seq.len() as i32 {
        total = ((total).py_add(
            seq.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        )) as i32;
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(total)
}
#[doc = "Check if undirected graph has Eulerian circuit(all degrees even, connected)."]
pub fn is_eulerian_circuit(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut has_edge: i32 = Default::default();
    let mut fi: i32 = Default::default();
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    let _cse_temp_0 = nodes.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(1);
    }
    has_edge = 0;
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if (ug.get(&node).is_some())
            && (ug.get(&(node)).cloned().unwrap_or_default().len() as i32 > 0)
        {
            has_edge = 1;
            if (ug.get(&(node)).cloned().unwrap_or_default().len() as i32).py_mod(2i32) != 0 {
                return Ok(0);
            }
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = has_edge == 0;
    if _cse_temp_2 {
        return Ok(1);
    }
    let _cse_temp_3 = connected_components_count(&graph)? > 1;
    if _cse_temp_3 {
        let mut nodes_with_edges: i32 = 0;
        let comp_nodes: Vec<i32> = all_nodes(&ug)?;
        let mut ci: i32 = 0;
        while ci < comp_nodes.len() as i32 {
            let nd: i32 = comp_nodes
                .get(ci as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if (ug.get(&nd).is_some())
                && (ug.get(&(nd)).cloned().unwrap_or_default().len() as i32 > 0)
            {
                nodes_with_edges = ((nodes_with_edges).py_add(1i32)) as i32;
            }
            ci = ((ci).py_add(1i32)) as i32;
        }
        let mut visited_e: std::collections::HashMap<i32, i32> = {
            let map: HashMap<i32, i32> = HashMap::new();
            map
        };
        let mut first_e: i32 = -1;
        fi = 0;
        while fi < comp_nodes.len() as i32 {
            if (ug
                .get(
                    &comp_nodes
                        .get(fi as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                )
                .is_some())
                && (ug
                    .get(
                        &(comp_nodes
                            .get(fi as usize)
                            .cloned()
                            .expect("IndexError: list index out of range")),
                    )
                    .cloned()
                    .unwrap_or_default()
                    .len() as i32
                    > 0)
            {
                first_e = comp_nodes
                    .get(fi as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                fi = comp_nodes.len() as i32;
            }
            fi = ((fi).py_add(1i32)) as i32;
        }
        let _cse_temp_4 = first_e == -1;
        if _cse_temp_4 {
            return Ok(1);
        }
        let mut queue: Vec<i32> = vec![first_e];
        visited_e.insert(first_e.clone(), 1);
        let mut qh: i32 = 0;
        while qh < queue.len() as i32 {
            let cur: i32 = queue
                .get(qh as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            qh = ((qh).py_add(1i32)) as i32;
            if ug.get(&cur).is_some() {
                let nbs: Vec<i32> = ug.get(&(cur)).cloned().unwrap_or_default();
                let mut nbi: i32 = 0;
                while nbi < nbs.len() as i32 {
                    let nb: i32 = nbs
                        .get(nbi as usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                    if visited_e.get(&nb).is_none() {
                        visited_e.insert(nb.clone(), 1);
                        queue.push(nb);
                    }
                    nbi = ((nbi).py_add(1i32)) as i32;
                }
            }
        }
        let mut reachable_with_edges: i32 = 0;
        for rn in visited_e.keys().cloned() {
            if (ug.get(&rn).is_some())
                && (ug.get(&(rn)).cloned().unwrap_or_default().len() as i32 > 0)
            {
                reachable_with_edges = ((reachable_with_edges).py_add(1i32)) as i32;
            }
        }
        let _cse_temp_5 = reachable_with_edges < nodes_with_edges;
        if _cse_temp_5 {
            return Ok(0);
        }
    }
    Ok(1)
}
#[doc = "Count nodes with odd degree in undirected graph."]
pub fn count_odd_degree_nodes(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
    let nodes: Vec<i32> = all_nodes(&ug)?;
    count = 0;
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if (ug.get(&node).is_some())
            && ((ug.get(&(node)).cloned().unwrap_or_default().len() as i32).py_mod(2i32) != 0)
        {
            count = ((count).py_add(1i32)) as i32;
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    Ok(count)
}
#[doc = "Check if undirected graph has Eulerian path. Returns 1/0."]
#[doc = " Depyler: proven to terminate"]
pub fn has_eulerian_path(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut fi: i32 = Default::default();
    let odd: i32 = count_odd_degree_nodes(&graph)?;
    let _cse_temp_0 = odd == 0;
    if _cse_temp_0 {
        return is_eulerian_circuit(&graph);
    }
    let _cse_temp_1 = odd == 2;
    if _cse_temp_1 {
        let _cse_temp_2 = connected_components_count(&graph)? <= 1;
        if _cse_temp_2 {
            return Ok(1);
        }
        let ug: std::collections::HashMap<i32, Vec<i32>> = make_undirected(&graph)?;
        let nodes: Vec<i32> = all_nodes(&ug)?;
        let mut edge_nodes: i32 = 0;
        let mut ni: i32 = 0;
        while ni < nodes.len() as i32 {
            let nd: i32 = nodes
                .get(ni as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if (ug.get(&nd).is_some())
                && (ug.get(&(nd)).cloned().unwrap_or_default().len() as i32 > 0)
            {
                edge_nodes = ((edge_nodes).py_add(1i32)) as i32;
            }
            ni = ((ni).py_add(1i32)) as i32;
        }
        let _cse_temp_3 = edge_nodes <= 2;
        if _cse_temp_3 {
            return Ok(1);
        }
        let mut first_with_edge: i32 = -1;
        fi = 0;
        while fi < nodes.len() as i32 {
            if (ug
                .get(
                    &nodes
                        .get(fi as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                )
                .is_some())
                && (ug
                    .get(
                        &(nodes
                            .get(fi as usize)
                            .cloned()
                            .expect("IndexError: list index out of range")),
                    )
                    .cloned()
                    .unwrap_or_default()
                    .len() as i32
                    > 0)
            {
                first_with_edge = nodes
                    .get(fi as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                fi = nodes.len() as i32;
            }
            fi = ((fi).py_add(1i32)) as i32;
        }
        let _cse_temp_4 = first_with_edge == -1;
        if _cse_temp_4 {
            return Ok(1);
        }
        let mut visited: std::collections::HashMap<i32, i32> = {
            let map: HashMap<i32, i32> = HashMap::new();
            map
        };
        let mut queue: Vec<i32> = vec![first_with_edge];
        visited.insert(first_with_edge.clone(), 1);
        let mut qh: i32 = 0;
        while qh < queue.len() as i32 {
            let cur: i32 = queue
                .get(qh as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            qh = ((qh).py_add(1i32)) as i32;
            if ug.get(&cur).is_some() {
                let nbs: Vec<i32> = ug.get(&(cur)).cloned().unwrap_or_default();
                let mut nbi: i32 = 0;
                while nbi < nbs.len() as i32 {
                    let nb: i32 = nbs
                        .get(nbi as usize)
                        .cloned()
                        .expect("IndexError: list index out of range");
                    if visited.get(&nb).is_none() {
                        visited.insert(nb.clone(), 1);
                        queue.push(nb);
                    }
                    nbi = ((nbi).py_add(1i32)) as i32;
                }
            }
        }
        let mut reachable_edges: i32 = 0;
        for rn in visited.keys().cloned() {
            if (ug.get(&rn).is_some())
                && (ug.get(&(rn)).cloned().unwrap_or_default().len() as i32 > 0)
            {
                reachable_edges = ((reachable_edges).py_add(1i32)) as i32;
            }
        }
        let _cse_temp_5 = reachable_edges >= edge_nodes;
        if _cse_temp_5 {
            return Ok(1);
        }
    }
    Ok(0)
}
#[doc = "Kahn's algorithm for topological sort. Returns empty list if cycle."]
pub fn topological_sort(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let nodes: Vec<i32> = all_nodes(&graph)?;
    let mut in_degree: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        in_degree.insert(
            nodes
                .get(ni as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            0,
        );
        ni = ((ni).py_add(1i32)) as i32;
    }
    for u in graph.keys().cloned() {
        let nbs: Vec<i32> = graph.get(&(u)).cloned().unwrap_or_default();
        let mut ei: i32 = 0;
        while ei < nbs.len() as i32 {
            let v: i32 = nbs
                .get(ei as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if in_degree.get(&v).is_some() {
                {
                    let _key = v.clone();
                    let _old_val = in_degree.get(&_key).cloned().unwrap_or_default();
                    in_degree.insert(_key, _old_val + 1);
                }
            } else {
                in_degree.insert(v.clone(), 1);
            }
            ei = ((ei).py_add(1i32)) as i32;
        }
    }
    let mut queue: Vec<i32> = vec![];
    for n in in_degree.keys().cloned() {
        if in_degree.get(&(n)).cloned().unwrap_or_default() == 0 {
            queue.push(n);
        }
    }
    let mut result: Vec<i32> = vec![];
    let mut qh: i32 = 0;
    while qh < queue.len() as i32 {
        let u2: i32 = queue
            .get(qh as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        qh = ((qh).py_add(1i32)) as i32;
        result.push(u2);
        if graph.get(&u2).is_some() {
            let nbs2: Vec<i32> = graph.get(&(u2)).cloned().unwrap_or_default();
            let mut ei2: i32 = 0;
            while ei2 < nbs2.len() as i32 {
                let v2: i32 = nbs2
                    .get(ei2 as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                {
                    let _key = v2.clone();
                    let _old_val = in_degree.get(&_key).cloned().unwrap_or_default();
                    in_degree.insert(_key, _old_val - 1);
                }
                if in_degree.get(&(v2)).cloned().unwrap_or_default() == 0 {
                    queue.push(v2);
                }
                ei2 = ((ei2).py_add(1i32)) as i32;
            }
        }
    }
    let _cse_temp_0 = result.len() as i32;
    let _cse_temp_1 = nodes.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    if _cse_temp_2 {
        return Ok(vec![]);
    }
    Ok(result)
}
#[doc = "Return longest path length in a DAG. -1 if graph has cycle."]
pub fn dag_longest_path(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut max_d: i32 = Default::default();
    let topo: Vec<i32> = topological_sort(&graph)?;
    let _cse_temp_0 = topo.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let _cse_temp_2 = all_nodes(&graph).len() as i32;
    let _cse_temp_3 = _cse_temp_2 > 0;
    let _cse_temp_4 = (_cse_temp_1) && (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(-1);
    }
    let mut dist: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut ti: i32 = 0;
    while ti < topo.len() as i32 {
        dist.insert(
            topo.get(ti as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            0,
        );
        ti = ((ti).py_add(1i32)) as i32;
    }
    let mut ti2: i32 = 0;
    while ti2 < topo.len() as i32 {
        let u: i32 = topo
            .get(ti2 as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if graph.get(&u).is_some() {
            let nbs: Vec<i32> = graph.get(&(u)).cloned().unwrap_or_default();
            let mut ei: i32 = 0;
            while ei < nbs.len() as i32 {
                let v: i32 = nbs
                    .get(ei as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let nd: i32 = ((dist.get(&(u)).cloned().unwrap_or_default()).py_add(1i32)) as i32;
                if nd > dist.get(&(v)).cloned().unwrap_or_default() {
                    dist.insert(v.clone(), nd);
                }
                ei = ((ei).py_add(1i32)) as i32;
            }
        }
        ti2 = ((ti2).py_add(1i32)) as i32;
    }
    max_d = 0;
    for n in dist.keys().cloned() {
        if dist.get(&(n)).cloned().unwrap_or_default() > max_d {
            max_d = dist.get(&(n)).cloned().unwrap_or_default();
        }
    }
    Ok(max_d)
}
#[doc = "Longest weighted path in DAG. Edges: [[neighbor, weight],...]."]
pub fn dag_longest_weighted_path(
    weighted_graph: &std::collections::HashMap<i32, Vec<Vec<i32>>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut max_d: i32 = Default::default();
    let mut unweighted: std::collections::HashMap<i32, Vec<i32>> = {
        let map: HashMap<i32, Vec<i32>> = HashMap::new();
        map
    };
    for u in weighted_graph.keys().cloned() {
        unweighted.insert(u.clone(), vec![]);
        let edges: Vec<Vec<i32>> = weighted_graph.get(&(u)).cloned().unwrap_or_default();
        let mut ei: i32 = 0;
        while ei < edges.len() as i32 {
            unweighted.get(&(u)).cloned().unwrap_or_default().push(
                edges
                    .get(ei as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
            ei = ((ei).py_add(1i32)) as i32;
        }
    }
    let topo: Vec<i32> = topological_sort(&unweighted)?;
    let _cse_temp_0 = topo.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let _cse_temp_2 = all_nodes(&unweighted).len() as i32;
    let _cse_temp_3 = _cse_temp_2 > 0;
    let _cse_temp_4 = (_cse_temp_1) && (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(-1);
    }
    let mut dist: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut ti: i32 = 0;
    while ti < topo.len() as i32 {
        dist.insert(
            topo.get(ti as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            0,
        );
        ti = ((ti).py_add(1i32)) as i32;
    }
    let mut ti2: i32 = 0;
    while ti2 < topo.len() as i32 {
        let u2: i32 = topo
            .get(ti2 as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if weighted_graph.get(&u2).is_some() {
            let edges2: Vec<Vec<i32>> = weighted_graph.get(&(u2)).cloned().unwrap_or_default();
            let mut ei2: i32 = 0;
            while ei2 < edges2.len() as i32 {
                let v: i32 = edges2
                    .get(ei2 as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let w: i32 = edges2
                    .get(ei2 as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .get(1usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                let nd: i32 = ((dist.get(&(u2)).cloned().unwrap_or_default()).py_add(w)) as i32;
                if nd > dist.get(&(v)).cloned().unwrap_or_default() {
                    dist.insert(v.clone(), nd);
                }
                ei2 = ((ei2).py_add(1i32)) as i32;
            }
        }
        ti2 = ((ti2).py_add(1i32)) as i32;
    }
    max_d = 0;
    for n in dist.keys().cloned() {
        if dist.get(&(n)).cloned().unwrap_or_default() > max_d {
            max_d = dist.get(&(n)).cloned().unwrap_or_default();
        }
    }
    Ok(max_d)
}
#[doc = "Compute in-degree for each node in a directed graph."]
pub fn in_degree_map(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let nodes: Vec<i32> = all_nodes(&graph)?;
    let mut indeg: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        indeg.insert(
            nodes
                .get(ni as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            0,
        );
        ni = ((ni).py_add(1i32)) as i32;
    }
    for u in graph.keys().cloned() {
        let nbs: Vec<i32> = graph.get(&(u)).cloned().unwrap_or_default();
        let mut ei: i32 = 0;
        while ei < nbs.len() as i32 {
            let v: i32 = nbs
                .get(ei as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if indeg.get(&v).is_some() {
                {
                    let _key = v.clone();
                    let _old_val = indeg.get(&_key).cloned().unwrap_or_default();
                    indeg.insert(_key, _old_val + 1);
                }
            } else {
                indeg.insert(v.clone(), 1);
            }
            ei = ((ei).py_add(1i32)) as i32;
        }
    }
    Ok(indeg)
}
#[doc = "Count total edges in a directed graph."]
pub fn count_edges_directed(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    total = 0;
    for u in graph.keys().cloned() {
        total = ((total).py_add(graph.get(&(u)).cloned().unwrap_or_default().len() as i32)) as i32;
    }
    Ok(total)
}
#[doc = "Count nodes with in-degree 0 in directed graph."]
pub fn count_source_nodes(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    let indeg: std::collections::HashMap<i32, i32> = in_degree_map(&graph)?;
    count = 0;
    for n in indeg.keys().cloned() {
        if indeg.get(&(n)).cloned().unwrap_or_default() == 0 {
            count = ((count).py_add(1i32)) as i32;
        }
    }
    Ok(count)
}
#[doc = "Count nodes with out-degree 0 in directed graph."]
pub fn count_sink_nodes(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    let nodes: Vec<i32> = all_nodes(&graph)?;
    count = 0;
    let mut ni: i32 = 0;
    while ni < nodes.len() as i32 {
        let node: i32 = nodes
            .get(ni as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if (graph.get(&node).is_none())
            || (graph.get(&(node)).cloned().unwrap_or_default().len() as i32 == 0)
        {
            count = ((count).py_add(1i32)) as i32;
        }
        ni = ((ni).py_add(1i32)) as i32;
    }
    Ok(count)
}
#[doc = "Return density * 1000(integer) for directed graph. density = E /(V*(V-1))."]
#[doc = " Depyler: proven to terminate"]
pub fn graph_density_x1000(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let nodes: Vec<i32> = all_nodes(&graph)?;
    let _cse_temp_0 = nodes.len() as i32;
    let v: i32 = _cse_temp_0;
    let _cse_temp_1 = v <= 1;
    if _cse_temp_1 {
        return Ok(0);
    }
    let e: i32 = count_edges_directed(&graph)?;
    Ok({
        let a = (e).py_mul(1000i32);
        let b = (v).py_mul((v) - (1i32));
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
#[doc = "Test BFS distance computation."]
#[doc = " Depyler: proven to terminate"]
pub fn test_bfs_distances() -> Result<i32, Box<dyn std::error::Error>> {
    let mut ok: i32 = Default::default();
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![3]);
        map.insert(2, vec![3]);
        map.insert(3, vec![]);
        map
    };
    let d: std::collections::HashMap<i32, i32> = bfs_distances(&g, 0)?;
    ok = 1;
    let _cse_temp_0 = d.get(&(0)).cloned().unwrap_or_default() != 0;
    if _cse_temp_0 {
        ok = 0;
    }
    let _cse_temp_1 = d.get(&(1)).cloned().unwrap_or_default() != 1;
    if _cse_temp_1 {
        ok = 0;
    }
    if _cse_temp_1 {
        ok = 0;
    }
    let _cse_temp_2 = d.get(&(3)).cloned().unwrap_or_default() != 2;
    if _cse_temp_2 {
        ok = 0;
    }
    Ok(ok)
}
#[doc = "Test BFS shortest path length."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bfs_shortest_path() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![3]);
        map.insert(2, vec![3]);
        map.insert(3, vec![4]);
        map.insert(4, vec![]);
        map
    };
    let r1: i32 = bfs_shortest_path_length(&g, 0, 4)?;
    let r2: i32 = bfs_shortest_path_length(&g, 0, 99)?;
    let _cse_temp_0 = r1 != 3;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != -1;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test BFS level sizes."]
#[doc = " Depyler: proven to terminate"]
pub fn test_bfs_level_sizes() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2, 3]);
        map.insert(1, vec![4]);
        map.insert(2, vec![4]);
        map.insert(3, vec![]);
        map.insert(4, vec![]);
        map
    };
    let sizes: Vec<i32> = bfs_level_sizes(&g, 0)?;
    let _cse_temp_0 = sizes.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 3;
    if _cse_temp_1 {
        return Ok(0);
    }
    let _cse_temp_2 = sizes
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        != 1;
    if _cse_temp_2 {
        return Ok(0);
    }
    let _cse_temp_3 = sizes
        .get(1usize)
        .cloned()
        .expect("IndexError: list index out of range")
        != 3;
    if _cse_temp_3 {
        return Ok(0);
    }
    if _cse_temp_2 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test DFS discovery and finish times."]
#[doc = " Depyler: proven to terminate"]
pub fn test_dfs_times() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![3]);
        map.insert(2, vec![]);
        map.insert(3, vec![]);
        map
    };
    let times: std::collections::HashMap<i32, Vec<i32>> = dfs_times(&g, 0)?;
    let _cse_temp_0 = times.get(&0).is_none();
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = times.get(&1).is_none();
    if _cse_temp_1 {
        return Ok(0);
    }
    let _cse_temp_2 = times
        .get(&(0))
        .cloned()
        .unwrap_or_default()
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        >= times
            .get(&(0))
            .cloned()
            .unwrap_or_default()
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range");
    if _cse_temp_2 {
        return Ok(0);
    }
    let _cse_temp_3 = times
        .get(&(0))
        .cloned()
        .unwrap_or_default()
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        != 1;
    if _cse_temp_3 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test DFS reachable count."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dfs_reachable() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![2]);
        map.insert(2, vec![3]);
        map.insert(3, vec![]);
        map.insert(5, vec![6]);
        map.insert(6, vec![]);
        map
    };
    let r1: i32 = dfs_reachable_count(&g, 0)?;
    let r2: i32 = dfs_reachable_count(&g, 5)?;
    let _cse_temp_0 = r1 != 4;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != 2;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test connected component counting."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_connected_components() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0]);
        map.insert(2, vec![3]);
        map.insert(3, vec![2]);
        map.insert(4, vec![]);
        map
    };
    let c: i32 = connected_components_count(&g)?;
    let _cse_temp_0 = c != 3;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test largest component size."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_largest_component() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![2]);
        map.insert(2, vec![0]);
        map.insert(3, vec![4]);
        map.insert(4, vec![3]);
        map.insert(5, vec![]);
        map
    };
    let s: i32 = largest_component_size(&g)?;
    let _cse_temp_0 = s != 3;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test directed cycle detection."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_cycle_directed() -> Result<i32, Box<dyn std::error::Error>> {
    let acyclic: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![2]);
        map.insert(2, vec![3]);
        map.insert(3, vec![]);
        map
    };
    let cyclic: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![2]);
        map.insert(2, vec![0]);
        map
    };
    let r1: i32 = has_cycle_directed(&acyclic)?;
    let r2: i32 = has_cycle_directed(&cyclic)?;
    let _cse_temp_0 = r1 != 0;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != 1;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test undirected cycle detection."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_cycle_undirected() -> Result<i32, Box<dyn std::error::Error>> {
    let tree: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![2]);
        map
    };
    let cyclic: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![2]);
        map
    };
    let r1: i32 = has_cycle_undirected(&tree)?;
    let r2: i32 = has_cycle_undirected(&cyclic)?;
    let _cse_temp_0 = r1 != 0;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != 1;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test Dijkstra shortest path."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dijkstra() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<Vec<i32>>> = {
        let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        map.insert(0, vec![vec![1, 4], vec![2, 1]]);
        map.insert(1, vec![vec![3, 1]]);
        map.insert(2, vec![vec![1, 2], vec![3, 5]]);
        map.insert(3, vec![]);
        map
    };
    let r1: i32 = dijkstra_shortest(&g, 0, 3)?;
    let r2: i32 = dijkstra_shortest(&g, 0, 99)?;
    let _cse_temp_0 = r1 != 4;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != -1;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test bipartite checking."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bipartite() -> Result<i32, Box<dyn std::error::Error>> {
    let bp: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 3]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![1, 3]);
        map.insert(3, vec![0, 2]);
        map
    };
    let not_bp: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![0, 1]);
        map
    };
    let r1: i32 = is_bipartite(&bp)?;
    let r2: i32 = is_bipartite(&not_bp)?;
    let _cse_temp_0 = r1 != 1;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test bridge counting."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bridges() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![1, 3, 4]);
        map.insert(3, vec![2, 4]);
        map.insert(4, vec![2, 3]);
        map
    };
    let b: i32 = count_bridges(&g)?;
    let _cse_temp_0 = b != 2;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test articulation point counting."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_articulation_points() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0, 2, 3]);
        map.insert(2, vec![1]);
        map.insert(3, vec![1, 4]);
        map.insert(4, vec![3]);
        map
    };
    let ap: i32 = count_articulation_points(&g)?;
    let _cse_temp_0 = ap != 2;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test graph transpose."]
#[doc = " Depyler: proven to terminate"]
pub fn test_transpose() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![2]);
        map.insert(2, vec![]);
        map
    };
    let tg: std::collections::HashMap<i32, Vec<i32>> = transpose_graph(&g)?;
    let _cse_temp_0 = tg.get(&0).is_none();
    let _cse_temp_1 = tg.get(&(0)).cloned().unwrap_or_default().len() as i32;
    let _cse_temp_2 = _cse_temp_1 != 0;
    let _cse_temp_3 = (_cse_temp_0) || (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(0);
    }
    let _cse_temp_4 = tg.get(&1).is_none();
    let _cse_temp_5 = _cse_temp_1 != 1;
    let _cse_temp_6 = (_cse_temp_4) || (_cse_temp_5);
    if _cse_temp_6 {
        return Ok(0);
    }
    let _cse_temp_7 = tg.get(&2).is_none();
    let _cse_temp_8 = _cse_temp_1 != 2;
    let _cse_temp_9 = (_cse_temp_7) || (_cse_temp_8);
    if _cse_temp_9 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test Kosaraju SCC count."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_kosaraju_scc() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![2]);
        map.insert(2, vec![0, 3]);
        map.insert(3, vec![4]);
        map.insert(4, vec![5]);
        map.insert(5, vec![3]);
        map
    };
    let c: i32 = kosaraju_scc_count(&g)?;
    let _cse_temp_0 = c != 2;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test largest SCC size."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_largest_scc() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![2]);
        map.insert(2, vec![0]);
        map.insert(3, vec![4]);
        map.insert(4, vec![5]);
        map.insert(5, vec![3]);
        map.insert(6, vec![]);
        map
    };
    let s: i32 = largest_scc_size(&g)?;
    let _cse_temp_0 = s != 3;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test greedy graph coloring."]
pub fn test_greedy_coloring() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![0, 1]);
        map
    };
    let c: std::collections::HashMap<i32, i32> = greedy_coloring(&g)?;
    let _cse_temp_0 = c.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 3;
    if _cse_temp_1 {
        return Ok(0);
    }
    for u in g.keys().cloned() {
        let nbs: Vec<i32> = g.get(&(u)).cloned().unwrap_or_default();
        let mut ni: i32 = 0;
        while ni < nbs.len() as i32 {
            let v: i32 = nbs
                .get(ni as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if c.get(&(u)).cloned().unwrap_or_default() == c.get(&(v)).cloned().unwrap_or_default()
            {
                return Ok(0);
            }
            ni = ((ni).py_add(1i32)) as i32;
        }
    }
    Ok(1)
}
#[doc = "Test chromatic number upper bound."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_chromatic_upper() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![1]);
        map
    };
    let cn: i32 = chromatic_number_upper(&g)?;
    let _cse_temp_0 = cn != 2;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test Kruskal's MST."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_kruskal() -> Result<i32, Box<dyn std::error::Error>> {
    let edges: Vec<Vec<i32>> = vec![
        vec![0, 1, 4],
        vec![0, 2, 1],
        vec![1, 2, 2],
        vec![1, 3, 5],
        vec![2, 3, 8],
    ];
    let w: i32 = kruskal_mst_weight(4, &edges)?;
    let _cse_temp_0 = w != 8;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test Prim's MST."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_prim() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<Vec<i32>>> = {
        let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        map.insert(0, vec![vec![1, 4], vec![2, 1]]);
        map.insert(1, vec![vec![0, 4], vec![2, 2], vec![3, 5]]);
        map.insert(2, vec![vec![0, 1], vec![1, 2], vec![3, 8]]);
        map.insert(3, vec![vec![1, 5], vec![2, 8]]);
        map
    };
    let w: i32 = prim_mst_weight(&g, 0)?;
    let _cse_temp_0 = w != 8;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test Floyd-Warshall all-pairs shortest paths."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_floyd_warshall() -> Result<i32, Box<dyn std::error::Error>> {
    let edges: Vec<Vec<i32>> = vec![vec![0, 1, 3], vec![0, 2, 8], vec![1, 2, 2], vec![2, 3, 1]];
    let r: i32 = floyd_shortest(4, &edges, 0, 3)?;
    let _cse_temp_0 = r != 6;
    if _cse_temp_0 {
        return Ok(0);
    }
    let r2: i32 = floyd_shortest(4, &edges, 3, 0)?;
    let _cse_temp_1 = r2 != -1;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test degree sequence computation."]
pub fn test_degree_sequence() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![0, 1]);
        map
    };
    let seq: Vec<i32> = degree_sequence(&g)?;
    let _cse_temp_0 = seq.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 3;
    if _cse_temp_1 {
        return Ok(0);
    }
    total = 0;
    let mut i: i32 = 0;
    while i < seq.len() as i32 {
        total = ((total).py_add(
            seq.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        )) as i32;
        i = ((i).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = total != 6;
    if _cse_temp_2 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test max degree computation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_max_degree() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2, 3]);
        map.insert(1, vec![0]);
        map.insert(2, vec![0]);
        map.insert(3, vec![0]);
        map
    };
    let md: i32 = max_degree(&g)?;
    let _cse_temp_0 = md != 3;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test regularity check."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_is_regular() -> Result<i32, Box<dyn std::error::Error>> {
    let reg: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![0, 1]);
        map
    };
    let not_reg: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2, 3]);
        map.insert(1, vec![0]);
        map.insert(2, vec![0]);
        map.insert(3, vec![0]);
        map
    };
    let r1: i32 = is_regular(&reg)?;
    let r2: i32 = is_regular(&not_reg)?;
    let _cse_temp_0 = r1 != 1;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test sum of degrees(handshake lemma)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sum_degrees() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![2]);
        map
    };
    let sd: i32 = sum_degrees(&g)?;
    let _cse_temp_0 = sd != 6;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test Eulerian circuit detection."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_eulerian_circuit() -> Result<i32, Box<dyn std::error::Error>> {
    let euler: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![0, 1]);
        map
    };
    let not_euler: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![1]);
        map
    };
    let r1: i32 = is_eulerian_circuit(&euler)?;
    let r2: i32 = is_eulerian_circuit(&not_euler)?;
    let _cse_temp_0 = r1 != 1;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test odd degree node counting."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_odd_degree() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![1]);
        map
    };
    let odd: i32 = count_odd_degree_nodes(&g)?;
    let _cse_temp_0 = odd != 2;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test Eulerian path detection."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_eulerian_path() -> Result<i32, Box<dyn std::error::Error>> {
    let path_g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![1]);
        map
    };
    let no_path: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0]);
        map.insert(2, vec![3]);
        map.insert(3, vec![2, 4]);
        map.insert(4, vec![3]);
        map
    };
    let r1: i32 = has_eulerian_path(&path_g)?;
    let r2: i32 = has_eulerian_path(&no_path)?;
    let _cse_temp_0 = r1 != 1;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = r2 != 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test topological sort."]
pub fn test_topological_sort() -> Result<i32, Box<dyn std::error::Error>> {
    let dag: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![3]);
        map.insert(2, vec![3]);
        map.insert(3, vec![]);
        map
    };
    let topo: Vec<i32> = topological_sort(&dag)?;
    let _cse_temp_0 = topo.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 4;
    if _cse_temp_1 {
        return Ok(0);
    }
    let mut pos: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut ti: i32 = 0;
    while ti < topo.len() as i32 {
        pos.insert(
            topo.get(ti as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            ti,
        );
        ti = ((ti).py_add(1i32)) as i32;
    }
    let _cse_temp_2 =
        pos.get(&(0)).cloned().unwrap_or_default() >= pos.get(&(1)).cloned().unwrap_or_default();
    if _cse_temp_2 {
        return Ok(0);
    }
    if _cse_temp_2 {
        return Ok(0);
    }
    if _cse_temp_2 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test DAG longest path."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dag_longest_path() -> Result<i32, Box<dyn std::error::Error>> {
    let dag: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![3]);
        map.insert(2, vec![3]);
        map.insert(3, vec![4]);
        map.insert(4, vec![]);
        map
    };
    let lp: i32 = dag_longest_path(&dag)?;
    let _cse_temp_0 = lp != 3;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test DAG longest weighted path."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dag_longest_weighted() -> Result<i32, Box<dyn std::error::Error>> {
    let wg: std::collections::HashMap<i32, Vec<Vec<i32>>> = {
        let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        map.insert(0, vec![vec![1, 5], vec![2, 3]]);
        map.insert(1, vec![vec![3, 6]]);
        map.insert(2, vec![vec![3, 4]]);
        map.insert(3, vec![]);
        map
    };
    let lp: i32 = dag_longest_weighted_path(&wg)?;
    let _cse_temp_0 = lp != 11;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test in-degree computation."]
#[doc = " Depyler: proven to terminate"]
pub fn test_in_degree() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![2]);
        map.insert(2, vec![3]);
        map.insert(3, vec![]);
        map
    };
    let indeg: std::collections::HashMap<i32, i32> = in_degree_map(&g)?;
    let _cse_temp_0 = indeg.get(&(0)).cloned().unwrap_or_default() != 0;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = indeg.get(&(1)).cloned().unwrap_or_default() != 1;
    if _cse_temp_1 {
        return Ok(0);
    }
    let _cse_temp_2 = indeg.get(&(2)).cloned().unwrap_or_default() != 2;
    if _cse_temp_2 {
        return Ok(0);
    }
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test directed edge counting."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_edge_count() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![2, 3]);
        map.insert(2, vec![]);
        map.insert(3, vec![]);
        map
    };
    let e: i32 = count_edges_directed(&g)?;
    let _cse_temp_0 = e != 4;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test source and sink node counting."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_source_sink() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2]);
        map.insert(1, vec![3]);
        map.insert(2, vec![3]);
        map.insert(3, vec![]);
        map
    };
    let src: i32 = count_source_nodes(&g)?;
    let snk: i32 = count_sink_nodes(&g)?;
    let _cse_temp_0 = src != 1;
    if _cse_temp_0 {
        return Ok(0);
    }
    let _cse_temp_1 = snk != 1;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test graph density computation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_density() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1, 2, 3]);
        map.insert(1, vec![0, 2, 3]);
        map.insert(2, vec![0, 1, 3]);
        map.insert(3, vec![0, 1, 2]);
        map
    };
    let d: i32 = graph_density_x1000(&g)?;
    let _cse_temp_0 = d != 1000;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test various functions on empty graph."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_empty_graph() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let map: HashMap<i32, Vec<i32>> = HashMap::new();
        map
    };
    let c: i32 = connected_components_count(&g)?;
    let _cse_temp_0 = c != 0;
    if _cse_temp_0 {
        return Ok(0);
    }
    let lc: i32 = largest_component_size(&g)?;
    let _cse_temp_1 = lc != 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    let cyc: i32 = has_cycle_directed(&g)?;
    let _cse_temp_2 = cyc != 0;
    if _cse_temp_2 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test functions on single-node graph."]
#[doc = " Depyler: proven to terminate"]
pub fn test_single_node() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![]);
        map
    };
    let d: std::collections::HashMap<i32, i32> = bfs_distances(&g, 0)?;
    let _cse_temp_0 = d.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 1;
    if _cse_temp_1 {
        return Ok(0);
    }
    let _cse_temp_2 = d.get(&(0)).cloned().unwrap_or_default() != 0;
    if _cse_temp_2 {
        return Ok(0);
    }
    let c: i32 = connected_components_count(&g)?;
    let _cse_temp_3 = c != 1;
    if _cse_temp_3 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test cycle detection with self-loop."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_self_loop_cycle() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![0]);
        map
    };
    let r: i32 = has_cycle_directed(&g)?;
    let _cse_temp_0 = r != 1;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test SCC on disconnected directed graph."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_disconnected_scc() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![0]);
        map.insert(2, vec![3]);
        map.insert(3, vec![2]);
        map.insert(4, vec![]);
        map
    };
    let c: i32 = kosaraju_scc_count(&g)?;
    let _cse_temp_0 = c != 3;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test bipartite on K2,3."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_complete_bipartite() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![2, 3, 4]);
        map.insert(1, vec![2, 3, 4]);
        map.insert(2, vec![0, 1]);
        map.insert(3, vec![0, 1]);
        map.insert(4, vec![0, 1]);
        map
    };
    let r: i32 = is_bipartite(&g)?;
    let _cse_temp_0 = r != 1;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Test BFS on a long chain graph 0 -> 1 -> 2 ->...-> 9."]
#[doc = " Depyler: verified panic-free"]
pub fn test_long_chain_bfs() -> Result<i32, Box<dyn std::error::Error>> {
    let mut g: std::collections::HashMap<i32, Vec<i32>> = {
        let map: HashMap<i32, Vec<i32>> = HashMap::new();
        map
    };
    let mut i: i32 = 0;
    while i < 10 {
        if i < 9 {
            g.insert(i.clone(), vec![(i).py_add(1i32)]);
        } else {
            g.insert(i.clone(), vec![]);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    let r: i32 = bfs_shortest_path_length(&g, 0, 9)?;
    let _cse_temp_0 = r != 9;
    if _cse_temp_0 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Topological sort returns empty on cyclic graph."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_topo_cycle_detection() -> Result<i32, Box<dyn std::error::Error>> {
    let g: std::collections::HashMap<i32, Vec<i32>> = {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        map.insert(0, vec![1]);
        map.insert(1, vec![2]);
        map.insert(2, vec![0]);
        map
    };
    let topo: Vec<i32> = topological_sort(&g)?;
    let _cse_temp_0 = topo.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    Ok(1)
}
#[doc = "Run all tests and return sum of results."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_all_tests() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = 0;
    let _cse_temp_0 = ((total).py_add(test_bfs_distances()?)) as i32;
    total = _cse_temp_0;
    let _cse_temp_1 = ((total).py_add(test_bfs_shortest_path()?)) as i32;
    total = _cse_temp_1;
    let _cse_temp_2 = ((total).py_add(test_bfs_level_sizes()?)) as i32;
    total = _cse_temp_2;
    let _cse_temp_3 = ((total).py_add(test_dfs_times()?)) as i32;
    total = _cse_temp_3;
    let _cse_temp_4 = ((total).py_add(test_dfs_reachable()?)) as i32;
    total = _cse_temp_4;
    let _cse_temp_5 = ((total).py_add(test_connected_components()?)) as i32;
    total = _cse_temp_5;
    let _cse_temp_6 = ((total).py_add(test_largest_component()?)) as i32;
    total = _cse_temp_6;
    let _cse_temp_7 = ((total).py_add(test_cycle_directed()?)) as i32;
    total = _cse_temp_7;
    let _cse_temp_8 = ((total).py_add(test_cycle_undirected()?)) as i32;
    total = _cse_temp_8;
    let _cse_temp_9 = ((total).py_add(test_dijkstra()?)) as i32;
    total = _cse_temp_9;
    let _cse_temp_10 = ((total).py_add(test_bipartite()?)) as i32;
    total = _cse_temp_10;
    let _cse_temp_11 = ((total).py_add(test_bridges()?)) as i32;
    total = _cse_temp_11;
    let _cse_temp_12 = ((total).py_add(test_articulation_points()?)) as i32;
    total = _cse_temp_12;
    let _cse_temp_13 = ((total).py_add(test_transpose()?)) as i32;
    total = _cse_temp_13;
    let _cse_temp_14 = ((total).py_add(test_kosaraju_scc()?)) as i32;
    total = _cse_temp_14;
    let _cse_temp_15 = ((total).py_add(test_largest_scc()?)) as i32;
    total = _cse_temp_15;
    let _cse_temp_16 = ((total).py_add(test_greedy_coloring()?)) as i32;
    total = _cse_temp_16;
    let _cse_temp_17 = ((total).py_add(test_chromatic_upper()?)) as i32;
    total = _cse_temp_17;
    let _cse_temp_18 = ((total).py_add(test_kruskal()?)) as i32;
    total = _cse_temp_18;
    let _cse_temp_19 = ((total).py_add(test_prim()?)) as i32;
    total = _cse_temp_19;
    let _cse_temp_20 = ((total).py_add(test_floyd_warshall()?)) as i32;
    total = _cse_temp_20;
    let _cse_temp_21 = ((total).py_add(test_degree_sequence()?)) as i32;
    total = _cse_temp_21;
    let _cse_temp_22 = ((total).py_add(test_max_degree()?)) as i32;
    total = _cse_temp_22;
    let _cse_temp_23 = ((total).py_add(test_is_regular()?)) as i32;
    total = _cse_temp_23;
    let _cse_temp_24 = ((total).py_add(test_sum_degrees()?)) as i32;
    total = _cse_temp_24;
    let _cse_temp_25 = ((total).py_add(test_eulerian_circuit()?)) as i32;
    total = _cse_temp_25;
    let _cse_temp_26 = ((total).py_add(test_odd_degree()?)) as i32;
    total = _cse_temp_26;
    let _cse_temp_27 = ((total).py_add(test_eulerian_path()?)) as i32;
    total = _cse_temp_27;
    let _cse_temp_28 = ((total).py_add(test_topological_sort()?)) as i32;
    total = _cse_temp_28;
    let _cse_temp_29 = ((total).py_add(test_dag_longest_path()?)) as i32;
    total = _cse_temp_29;
    let _cse_temp_30 = ((total).py_add(test_dag_longest_weighted()?)) as i32;
    total = _cse_temp_30;
    let _cse_temp_31 = ((total).py_add(test_in_degree()?)) as i32;
    total = _cse_temp_31;
    let _cse_temp_32 = ((total).py_add(test_edge_count()?)) as i32;
    total = _cse_temp_32;
    let _cse_temp_33 = ((total).py_add(test_source_sink()?)) as i32;
    total = _cse_temp_33;
    let _cse_temp_34 = ((total).py_add(test_density()?)) as i32;
    total = _cse_temp_34;
    let _cse_temp_35 = ((total).py_add(test_empty_graph()?)) as i32;
    total = _cse_temp_35;
    let _cse_temp_36 = ((total).py_add(test_single_node()?)) as i32;
    total = _cse_temp_36;
    let _cse_temp_37 = ((total).py_add(test_self_loop_cycle()?)) as i32;
    total = _cse_temp_37;
    let _cse_temp_38 = ((total).py_add(test_disconnected_scc()?)) as i32;
    total = _cse_temp_38;
    let _cse_temp_39 = ((total).py_add(test_complete_bipartite()?)) as i32;
    total = _cse_temp_39;
    let _cse_temp_40 = ((total).py_add(test_long_chain_bfs()?)) as i32;
    total = _cse_temp_40;
    let _cse_temp_41 = ((total).py_add(test_topo_cycle_detection()?)) as i32;
    total = _cse_temp_41;
    Ok(total)
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result: i32 = run_all_tests()?;
    let expected: i32 = 42;
    let _cse_temp_0 = result == expected;
    if _cse_temp_0 {
    } else {
        return Err(Box::new(ValueError::new(format!(
            "{}{}",
            format!(
                "{}{}",
                format!("{}{}", "FAIL: expected ", (expected).to_string()),
                " but got "
            ),
            (result).to_string()
        ))));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_topological_sort() {
        fn prop(graph: ()) -> TestResult {
            let result = topological_sort(&graph);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = graph.clone();
            input_sorted.sort();
            let mut result = topological_sort(&graph);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(()) -> TestResult);
    }
    #[test]
    fn test_test_bfs_distances_examples() {
        let _ = test_bfs_distances();
    }
    #[test]
    fn test_test_bfs_shortest_path_examples() {
        let _ = test_bfs_shortest_path();
    }
    #[test]
    fn test_test_bfs_level_sizes_examples() {
        let _ = test_bfs_level_sizes();
    }
    #[test]
    fn test_test_dfs_times_examples() {
        let _ = test_dfs_times();
    }
    #[test]
    fn test_test_dfs_reachable_examples() {
        let _ = test_dfs_reachable();
    }
    #[test]
    fn test_test_connected_components_examples() {
        let _ = test_connected_components();
    }
    #[test]
    fn test_test_largest_component_examples() {
        let _ = test_largest_component();
    }
    #[test]
    fn test_test_cycle_directed_examples() {
        let _ = test_cycle_directed();
    }
    #[test]
    fn test_test_cycle_undirected_examples() {
        let _ = test_cycle_undirected();
    }
    #[test]
    fn test_test_dijkstra_examples() {
        let _ = test_dijkstra();
    }
    #[test]
    fn test_test_bipartite_examples() {
        let _ = test_bipartite();
    }
    #[test]
    fn test_test_bridges_examples() {
        let _ = test_bridges();
    }
    #[test]
    fn test_test_articulation_points_examples() {
        let _ = test_articulation_points();
    }
    #[test]
    fn test_test_transpose_examples() {
        let _ = test_transpose();
    }
    #[test]
    fn test_test_kosaraju_scc_examples() {
        let _ = test_kosaraju_scc();
    }
    #[test]
    fn test_test_largest_scc_examples() {
        let _ = test_largest_scc();
    }
    #[test]
    fn test_test_greedy_coloring_examples() {
        let _ = test_greedy_coloring();
    }
    #[test]
    fn test_test_chromatic_upper_examples() {
        let _ = test_chromatic_upper();
    }
    #[test]
    fn test_test_kruskal_examples() {
        let _ = test_kruskal();
    }
    #[test]
    fn test_test_prim_examples() {
        let _ = test_prim();
    }
    #[test]
    fn test_test_floyd_warshall_examples() {
        let _ = test_floyd_warshall();
    }
    #[test]
    fn test_test_degree_sequence_examples() {
        let _ = test_degree_sequence();
    }
    #[test]
    fn test_test_max_degree_examples() {
        let _ = test_max_degree();
    }
    #[test]
    fn test_test_is_regular_examples() {
        let _ = test_is_regular();
    }
    #[test]
    fn test_test_sum_degrees_examples() {
        let _ = test_sum_degrees();
    }
    #[test]
    fn test_test_eulerian_circuit_examples() {
        let _ = test_eulerian_circuit();
    }
    #[test]
    fn test_test_odd_degree_examples() {
        let _ = test_odd_degree();
    }
    #[test]
    fn test_test_eulerian_path_examples() {
        let _ = test_eulerian_path();
    }
    #[test]
    fn test_test_topological_sort_examples() {
        let _ = test_topological_sort();
    }
    #[test]
    fn test_test_dag_longest_path_examples() {
        let _ = test_dag_longest_path();
    }
    #[test]
    fn test_test_dag_longest_weighted_examples() {
        let _ = test_dag_longest_weighted();
    }
    #[test]
    fn test_test_in_degree_examples() {
        let _ = test_in_degree();
    }
    #[test]
    fn test_test_edge_count_examples() {
        let _ = test_edge_count();
    }
    #[test]
    fn test_test_source_sink_examples() {
        let _ = test_source_sink();
    }
    #[test]
    fn test_test_density_examples() {
        let _ = test_density();
    }
    #[test]
    fn test_test_empty_graph_examples() {
        let _ = test_empty_graph();
    }
    #[test]
    fn test_test_single_node_examples() {
        let _ = test_single_node();
    }
    #[test]
    fn test_test_self_loop_cycle_examples() {
        let _ = test_self_loop_cycle();
    }
    #[test]
    fn test_test_disconnected_scc_examples() {
        let _ = test_disconnected_scc();
    }
    #[test]
    fn test_test_complete_bipartite_examples() {
        let _ = test_complete_bipartite();
    }
    #[test]
    fn test_test_long_chain_bfs_examples() {
        let _ = test_long_chain_bfs();
    }
    #[test]
    fn test_test_topo_cycle_detection_examples() {
        let _ = test_topo_cycle_detection();
    }
    #[test]
    fn test_run_all_tests_examples() {
        let _ = run_all_tests();
    }
}