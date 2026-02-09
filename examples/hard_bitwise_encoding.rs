#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
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
#[doc = "Count the number of set bits in a non-negative integer."]
#[doc = " Depyler: verified panic-free"]
pub fn popcount(n: i32) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    let mut val: i32 = n.clone();
    while val > 0 {
        count = ((count).py_add(val & 1)) as i32;
        val = val >> 1;
    }
    count
}
#[doc = "Return 0 if even number of set bits, 1 if odd."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parity(n: i32) -> i32 {
    let mut val: i32 = n.clone();
    let _cse_temp_0 = val >> 16;
    let _cse_temp_1 = val ^ _cse_temp_0;
    val = _cse_temp_1;
    let _cse_temp_2 = val >> 8;
    let _cse_temp_3 = val ^ _cse_temp_2;
    val = _cse_temp_3;
    let _cse_temp_4 = val >> 4;
    let _cse_temp_5 = val ^ _cse_temp_4;
    val = _cse_temp_5;
    let _cse_temp_6 = val >> 2;
    let _cse_temp_7 = val ^ _cse_temp_6;
    val = _cse_temp_7;
    let _cse_temp_8 = val >> 1;
    let _cse_temp_9 = val ^ _cse_temp_8;
    val = _cse_temp_9;
    val & 1
}
#[doc = "Reverse 16 bits of an integer(fits in i32)."]
#[doc = " Depyler: verified panic-free"]
pub fn reverse_bits_32(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let mut i: i32 = 0;
    let _cse_temp_0 = n & 65535;
    let mut val: i32 = _cse_temp_0.clone();
    while i < 16 {
        result = result << 1 | val & 1;
        val = val >> 1;
        i = ((i).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Isolate the lowest set bit. Returns 0 if n is 0."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn isolate_lowest_set_bit(n: i32) -> i32 {
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
        return 0;
    }
    n & (-n)
}
#[doc = "Clear the lowest set bit of n."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn clear_lowest_set_bit(n: i32) -> i32 {
    n & (n) - (1i32)
}
#[doc = "Return the position of the highest set bit, or -1 if n is 0."]
#[doc = " Depyler: verified panic-free"]
pub fn highest_set_bit_pos(n: i32) -> i32 {
    let mut pos: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return -1;
    }
    pos = 0;
    let mut val: i32 = n.clone();
    while val > 1 {
        val = val >> 1;
        pos = ((pos).py_add(1i32)) as i32;
    }
    pos
}
#[doc = "Return the smallest power of two>= n. Assumes n>0."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn next_power_of_two(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    let mut val: i32 = ((n) - (1i32)) as i32;
    let _cse_temp_1 = val >> 1;
    let _cse_temp_2 = val | _cse_temp_1;
    val = _cse_temp_2;
    let _cse_temp_3 = val >> 2;
    let _cse_temp_4 = val | _cse_temp_3;
    val = _cse_temp_4;
    let _cse_temp_5 = val >> 4;
    let _cse_temp_6 = val | _cse_temp_5;
    val = _cse_temp_6;
    let _cse_temp_7 = val >> 8;
    let _cse_temp_8 = val | _cse_temp_7;
    val = _cse_temp_8;
    let _cse_temp_9 = val >> 16;
    let _cse_temp_10 = val | _cse_temp_9;
    val = _cse_temp_10;
    (val).py_add(1i32)
}
#[doc = "Swap bits at positions i and j in n."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn swap_bits(n: i32, i: i32, j: i32) -> i32 {
    let _cse_temp_0 = n >> i;
    let _cse_temp_1 = _cse_temp_0 & 1;
    let bit_i: i32 = _cse_temp_1;
    let _cse_temp_2 = n >> j;
    let _cse_temp_3 = _cse_temp_2 & 1;
    let bit_j: i32 = _cse_temp_3;
    let _cse_temp_4 = bit_i == bit_j;
    if _cse_temp_4 {
        return n;
    }
    let _cse_temp_5 = 1 << i;
    let _cse_temp_6 = 1 << j;
    let _cse_temp_7 = _cse_temp_5 | _cse_temp_6;
    let mask: i32 = _cse_temp_7;
    n ^ mask
}
#[doc = "Pack three 8-bit color channels into a single 24-bit integer."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn pack_rgb(r: i32, g: i32, b: i32) -> i32 {
    (r & 255) << 16 | (g & 255) << 8 | b & 255
}
#[doc = "Extract the red channel from a packed RGB value."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn unpack_r(packed: i32) -> i32 {
    packed >> 16 & 255
}
#[doc = "Extract the green channel from a packed RGB value."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn unpack_g(packed: i32) -> i32 {
    packed >> 8 & 255
}
#[doc = "Extract the blue channel from a packed RGB value."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn unpack_b(packed: i32) -> i32 {
    packed & 255
}
#[doc = "Pack four 8-bit fields into a 32-bit integer."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn pack_fields(a: i32, b: i32, c: i32, d: i32) -> i32 {
    (a & 255) << 24 | (b & 255) << 16 | (c & 255) << 8 | d & 255
}
#[doc = "Extract a bitfield of given width at given bit offset."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn extract_field(packed: i32, offset: i32, width: i32) -> i32 {
    let _cse_temp_0 = 1 << width;
    let mask: i32 = ((_cse_temp_0) - (1i32)) as i32;
    packed >> offset & mask
}
#[doc = "Set a bitfield of given width at given bit offset to value."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn set_field(packed: i32, offset: i32, width: i32, value: i32) -> i32 {
    let _cse_temp_0 = 1 << width;
    let mask: i32 = ((_cse_temp_0) - (1i32)) as i32;
    let _cse_temp_1 = packed & !mask << offset;
    let cleared: i32 = _cse_temp_1;
    cleared | (value & mask) << offset
}
#[doc = "Encode a list of integers by XORing each with the key."]
#[doc = " Depyler: verified panic-free"]
pub fn xor_encode(data: &Vec<i32>, key: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        result.push(
            data.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                ^ key & 255,
        );
        i = ((i).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Encode with a rolling XOR key that shifts after each byte."]
pub fn xor_encode_rolling(
    data: &Vec<i32>,
    key: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    let _cse_temp_0 = key & 255;
    let mut current_key: i32 = _cse_temp_0.clone();
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        let encoded: i32 = data
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            ^ current_key;
        result.push(encoded);
        current_key = (current_key << 1 | current_key >> 7) & 255;
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Run-length encode a list into [value, count, value, count,...] pairs."]
pub fn rle_encode(data: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut current: i32 = Default::default();
    let mut count: i32 = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    let mut result: Vec<i32> = vec![];
    current = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    count = 1;
    let mut i: i32 = 1;
    while i < data.len() as i32 {
        if data
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            == current
        {
            count = ((count).py_add(1i32)) as i32;
        } else {
            result.push(current);
            result.push(count);
            current = data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            count = 1;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    result.push(current);
    result.push(count);
    Ok(result)
}
#[doc = "Decode a run-length encoded list back to original data."]
pub fn rle_decode(encoded: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    let mut i: i32 = 0;
    while i < (encoded.len() as i32) - (1i32) {
        let value: i32 = encoded
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        let count: i32 = {
            let base = &encoded;
            let idx: i32 = (i).py_add(1i32);
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        };
        let mut j: i32 = 0;
        while j < count {
            result.push(value);
            j = ((j).py_add(1i32)) as i32;
        }
        i = ((i).py_add(2i32)) as i32;
    }
    Ok(result)
}
#[doc = "Delta-encode: first element as-is, then differences."]
#[doc = " Depyler: verified panic-free"]
pub fn delta_encode(data: &Vec<i32>) -> Vec<i32> {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return vec![];
    }
    let mut result: Vec<i32> = vec![data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")];
    let mut i: i32 = 1;
    while i < data.len() as i32 {
        result.push(
            (data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"))
             - ({
                let base = &data;
                let idx: i32 = (i) - (1i32);
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
        i = ((i).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Decode a delta-encoded list back to original values."]
#[doc = " Depyler: verified panic-free"]
pub fn delta_decode(encoded: &Vec<i32>) -> Vec<i32> {
    let _cse_temp_0 = encoded.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return vec![];
    }
    let mut result: Vec<i32> = vec![encoded
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")];
    let mut i: i32 = 1;
    while i < encoded.len() as i32 {
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
            .py_add(
                encoded
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            ),
        );
        i = ((i).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Encode a non-negative integer into 7-bit chunks with high-bit continuation."]
#[doc = " Depyler: verified panic-free"]
pub fn varint_encode(n: i32) -> Vec<i32> {
    let mut chunk: i32 = Default::default();
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
        return vec![0];
    }
    let mut result: Vec<i32> = vec![];
    let mut val: i32 = n.clone();
    while val > 0 {
        chunk = val & 127;
        val = val >> 7;
        if val > 0 {
            chunk = chunk | 128;
        }
        result.push(chunk);
    }
    result
}
#[doc = "Decode a varint-encoded list back to an integer."]
pub fn varint_decode(encoded: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut result: i32 = Default::default();
    result = 0;
    let mut shift: i32 = 0;
    let mut i: i32 = 0;
    while i < encoded.len() as i32 {
        let chunk: i32 = encoded
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            & 127;
        result = result | chunk << shift;
        if encoded
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            & 128
            == 0
        {
            break;
        }
        shift = ((shift).py_add(7i32)) as i32;
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Encode a list of non-negative integers as concatenated varints."]
pub fn varint_encode_list(data: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        let encoded: Vec<i32> = varint_encode(
            data.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
        );
        let mut j: i32 = 0;
        while j < encoded.len() as i32 {
            result.push(
                encoded
                    .get(j as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
            j = ((j).py_add(1i32)) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Convert a binary number to Gray code."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn to_gray(n: i32) -> i32 {
    n ^ n >> 1
}
#[doc = "Convert a Gray code back to binary."]
#[doc = " Depyler: verified panic-free"]
pub fn from_gray(gray: i32) -> i32 {
    let mut n: i32 = Default::default();
    n = gray;
    let _cse_temp_0 = n >> 1;
    let mut mask: i32 = _cse_temp_0.clone();
    while mask > 0 {
        n = n ^ mask;
        mask = mask >> 1;
    }
    n
}
#[doc = "Count the number of bit positions where a and b differ."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn hamming_distance(a: i32, b: i32) -> i32 {
    popcount(a ^ b)
}
#[doc = "Rotate a 16-bit value left by amount positions(fits in i32)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn rotate_left_32(n: i32, amount: i32) -> i32 {
    let _cse_temp_0 = n & 65535;
    let val: i32 = _cse_temp_0;
    let _cse_temp_1 = amount & 15;
    let shift: i32 = _cse_temp_1;
    (val << shift | val >> (16i32) - (shift)) & 65535
}
#[doc = "Rotate a 16-bit value right by amount positions(fits in i32)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn rotate_right_32(n: i32, amount: i32) -> i32 {
    let _cse_temp_0 = n & 65535;
    let val: i32 = _cse_temp_0;
    let _cse_temp_1 = amount & 15;
    let shift: i32 = _cse_temp_1;
    (val >> shift | val << (16i32) - (shift)) & 65535
}
#[doc = "Compute an 8-bit CRC-like checksum using polynomial division over GF(2)."]
pub fn crc8_simple(data: &Vec<i32>, poly: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut crc: i32 = Default::default();
    crc = 0;
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        crc = crc
            ^ data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                & 255;
        let mut bit: i32 = 0;
        while bit < 8 {
            if crc & 128 != 0 {
                crc = (crc << 1 ^ poly) & 255;
            } else {
                crc = crc << 1 & 255;
            }
            bit = ((bit).py_add(1i32)) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(crc)
}
#[doc = "Compute a 16-bit CRC-like checksum."]
pub fn crc16_simple(data: &Vec<i32>, poly: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut crc: i32 = Default::default();
    crc = 65535;
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        crc = crc
            ^ data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                & 255;
        let mut bit: i32 = 0;
        while bit < 8 {
            if crc & 1 != 0 {
                crc = crc >> 1 ^ poly;
            } else {
                crc = crc >> 1;
            }
            bit = ((bit).py_add(1i32)) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(crc & 65535)
}
#[doc = "Convert list of 8-bit values to list of 6-bit values(base64-style grouping)."]
pub fn base64_like_encode(data: &Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut buffer: i32 = Default::default();
    let mut bits_in_buffer: i32 = Default::default();
    let mut result: Vec<i32> = vec![];
    buffer = 0;
    bits_in_buffer = 0;
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        buffer = buffer << 8
            | data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                & 255;
        bits_in_buffer = ((bits_in_buffer).py_add(8i32)) as i32;
        while bits_in_buffer >= 6 {
            bits_in_buffer = ((bits_in_buffer) - (6i32)) as i32;
            result.push(buffer >> bits_in_buffer & 63);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    let _cse_temp_0 = bits_in_buffer > 0;
    if _cse_temp_0 {
        result.push(buffer << (6i32) - (bits_in_buffer) & 63);
    }
    Ok(result)
}
#[doc = "Convert list of 6-bit values back to 8-bit values."]
pub fn base64_like_decode(
    encoded: &Vec<i32>,
    original_len: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    let mut buffer: i32 = 0;
    let mut bits_in_buffer: i32 = 0;
    let mut i: i32 = 0;
    while i < encoded.len() as i32 {
        buffer = buffer << 6
            | encoded
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                & 63;
        bits_in_buffer = ((bits_in_buffer).py_add(6i32)) as i32;
        while bits_in_buffer >= 8 {
            bits_in_buffer = ((bits_in_buffer) - (8i32)) as i32;
            result.push(buffer >> bits_in_buffer & 255);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    while result.len() as i32 > original_len {
        result.pop().unwrap_or_default();
    }
    Ok(result)
}
#[doc = "Build a frequency table mapping values to their counts."]
pub fn frequency_table(data: &Vec<i32>) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let mut table: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        let val: i32 = data
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range");
        if table.get(&val).is_some() {
            {
                let _key = val.clone();
                let _old_val = table.get(&_key).cloned().unwrap_or_default();
                table.insert(_key, _old_val + 1);
            }
        } else {
            table.insert(val.clone(), 1);
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(table)
}
#[doc = "Assign bit lengths by frequency rank: most frequent gets 1 bit, next 2, etc."]
#[doc = " Depyler: verified panic-free"]
pub fn assign_bit_lengths(freq: &std::collections::HashMap<i32, i32>) -> HashMap<i32, i32> {
    let mut bit_len: i32 = Default::default();
    let _cse_temp_0 = freq.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return {
            let map: HashMap<i32, i32> = HashMap::new();
            map
        };
    }
    let sorted_keys: Vec<i32> = {
        let mut __sorted_result = freq.keys().cloned().collect::<Vec<_>>().clone();
        __sorted_result.sort_by_key(|k| freq.get(&(k)).cloned().unwrap_or_default());
        if true {
            __sorted_result.reverse();
        }
        __sorted_result
    };
    let mut lengths: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    let mut rank: i32 = 1;
    let mut i: i32 = 0;
    while i < sorted_keys.len() as i32 {
        bit_len = ((highest_set_bit_pos(rank)).py_add(1i32)) as i32;
        if bit_len < 1 {
            bit_len = 1;
        }
        lengths.insert(
            sorted_keys
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range"),
            bit_len,
        );
        rank = ((rank).py_add(1i32)) as i32;
        i = ((i).py_add(1i32)) as i32;
    }
    lengths
}
#[doc = "Compute total bits needed to encode data with given bit lengths."]
pub fn total_encoded_bits<'a, 'b>(
    data: &'a Vec<i32>,
    bit_lengths: &'b std::collections::HashMap<i32, i32>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = Default::default();
    total = 0;
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        if bit_lengths
            .get(
                &data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            )
            .is_some()
        {
            total = ((total).py_add(
                bit_lengths
                    .get(
                        &(data
                            .get(i as usize)
                            .cloned()
                            .expect("IndexError: list index out of range")),
                    )
                    .cloned()
                    .unwrap_or_default(),
            )) as i32;
        } else {
            total = ((total).py_add(8i32)) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(total)
}
#[doc = "Map signed integer to unsigned: 0 -> 0, -1 -> 1, 1 -> 2, -2 -> 3, 2 -> 4, etc."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zigzag_encode(n: i32) -> i32 {
    let _cse_temp_0 = n >= 0;
    if _cse_temp_0 {
        return (n).py_mul(2i32);
    }
    {
        let _r: i32 = ((-n).py_mul(2i32) as i32) - (1i32);
        _r
    }
}
#[doc = "Decode a zigzag-encoded unsigned integer back to signed."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zigzag_decode(n: i32) -> i32 {
    let _cse_temp_0 = n & 1;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return n >> 1;
    }
    -(n).py_add(1i32) >> 1
}
#[doc = "Check if code_a(with len_a bits) is a prefix of code_b(with len_b bits). Returns 1 or 0."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_prefix_of(code_a: i32, len_a: i32, code_b: i32, len_b: i32) -> i32 {
    let _cse_temp_0 = len_a > len_b;
    if _cse_temp_0 {
        return 0;
    }
    let shift: i32 = ((len_b) - (len_a)) as i32;
    let _cse_temp_1 = code_b >> shift;
    let _cse_temp_2 = _cse_temp_1 == code_a;
    if _cse_temp_2 {
        return 1;
    }
    0
}
#[doc = "Check if a set of codes with given bit lengths is prefix-free. Returns 1 if valid, 0 if not."]
pub fn validate_prefix_free<'a, 'b>(
    codes: &'a Vec<i32>,
    lengths: &'b Vec<i32>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = codes.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut i: i32 = 0;
    while i < n {
        let mut j: i32 = 0;
        while j < n {
            if i != j {
                if is_prefix_of(
                    codes
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                    lengths
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                    codes
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                    lengths
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                ) == 1
                {
                    return Ok(0);
                }
            }
            j = ((j).py_add(1i32)) as i32;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    Ok(1)
}
#[doc = "Interleave the lower 16 bits of x and y into a 32-bit Morton code."]
#[doc = " Depyler: verified panic-free"]
pub fn interleave_bits(x: i32, y: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let mut i: i32 = 0;
    while i < 16 {
        result = result | (x >> i & 1) << (2i32).py_mul(i);
        result = result | (y >> i & 1) << ((2i32).py_mul(i) as i32).py_add(1i32);
        i = ((i).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Extract the x component from a Morton code."]
#[doc = " Depyler: verified panic-free"]
pub fn deinterleave_x(morton: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let mut i: i32 = 0;
    while i < 16 {
        result = result | (morton >> (2i32).py_mul(i) & 1) << i;
        i = ((i).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Extract the y component from a Morton code."]
#[doc = " Depyler: verified panic-free"]
pub fn deinterleave_y(morton: i32) -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let mut i: i32 = 0;
    while i < 16 {
        result = result | (morton >> ((2i32).py_mul(i) as i32).py_add(1i32) & 1) << i;
        i = ((i).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Compute the absolute difference of Morton codes for two 2D points."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn morton_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let m1: i32 = interleave_bits(x1, y1);
    let m2: i32 = interleave_bits(x2, y2);
    let _cse_temp_0 = m1 > m2;
    if _cse_temp_0 {
        return (m1) - (m2);
    }
    (m2) - (m1)
}
#[doc = "Test popcount and parity on known values."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_popcount_and_parity() -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let _cse_temp_0 = popcount(0) == 0;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = popcount(255) == 8;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = popcount(170) == 4;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_3 = parity(15) == 0;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_4 = parity(7) == 1;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_5 = parity(0) == 0;
    if _cse_temp_5 {
        result = ((result).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Test 16-bit bit reversal."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_reverse_bits() -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let _cse_temp_0 = reverse_bits_32(0) == 0;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = reverse_bits_32(1) == 32768;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let rev: i32 = reverse_bits_32(32768);
    let _cse_temp_2 = rev == 1;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_3 = reverse_bits_32(reverse_bits_32(12345)) == 12345;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Test isolate lowest set bit and clear lowest set bit."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_isolate_and_clear() -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let _cse_temp_0 = isolate_lowest_set_bit(12) == 4;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = isolate_lowest_set_bit(0) == 0;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = isolate_lowest_set_bit(8) == 8;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_3 = clear_lowest_set_bit(12) == 8;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_4 = clear_lowest_set_bit(8) == 0;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_5 = clear_lowest_set_bit(0) == -1;
    if _cse_temp_5 {
        result = ((result).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Test highest_set_bit_pos, next_power_of_two, swap_bits."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bit_utilities() -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let _cse_temp_0 = highest_set_bit_pos(1) == 0;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = highest_set_bit_pos(8) == 3;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = highest_set_bit_pos(0) == -1;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_3 = next_power_of_two(5) == 8;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_4 = next_power_of_two(8) == 8;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_5 = next_power_of_two(1) == 1;
    if _cse_temp_5 {
        result = ((result).py_add(1i32)) as i32;
    }
    let swapped: i32 = swap_bits(10, 1, 2);
    let _cse_temp_6 = swapped == 12;
    if _cse_temp_6 {
        result = ((result).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Test RGB packing and unpacking."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pack_unpack_rgb() -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let packed: i32 = pack_rgb(255, 128, 0);
    let _cse_temp_0 = unpack_r(packed) == 255;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = unpack_g(packed) == 128;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = unpack_b(packed) == 0;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let packed2: i32 = pack_rgb(0, 0, 0);
    let _cse_temp_3 = packed2 == 0;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Test pack_fields, extract_field, set_field."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bitfield_ops() -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let packed: i32 = pack_fields(171, 205, 239, 18);
    let _cse_temp_0 = extract_field(packed, 24, 8) == 171;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = extract_field(packed, 16, 8) == 205;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = extract_field(packed, 8, 8) == 239;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_3 = extract_field(packed, 0, 8) == 18;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let modified: i32 = set_field(packed, 8, 8, 153);
    let _cse_temp_4 = extract_field(modified, 8, 8) == 153;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Test XOR encode is reversible."]
pub fn test_xor_cipher() -> Result<i32, Box<dyn std::error::Error>> {
    let mut r#match: i32 = Default::default();
    let mut result: i32 = Default::default();
    result = 0;
    let data: Vec<i32> = vec![72, 101, 108, 108, 111];
    let key: i32 = 42;
    let encoded: Vec<i32> = xor_encode(&data, key);
    let decoded: Vec<i32> = xor_encode(&encoded, key);
    let _cse_temp_0 = decoded.len() as i32;
    let _cse_temp_1 = data.len() as i32;
    let _cse_temp_2 = _cse_temp_0 == _cse_temp_1;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    r#match = 1;
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        if decoded
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            != data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        {
            r#match = 0;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    result = ((result).py_add(r#match)) as i32;
    let rolling_enc: Vec<i32> = xor_encode_rolling(&data, key)?;
    let _cse_temp_3 = rolling_enc.len() as i32;
    let _cse_temp_4 = _cse_temp_3 == _cse_temp_1;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Test run-length encoding roundtrip."]
pub fn test_rle() -> Result<i32, Box<dyn std::error::Error>> {
    let mut r#match: i32 = Default::default();
    let mut result: i32 = Default::default();
    result = 0;
    let data: Vec<i32> = vec![1, 1, 1, 2, 2, 3, 3, 3, 3];
    let encoded: Vec<i32> = rle_encode(&data)?;
    let _cse_temp_0 = encoded
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 1;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = encoded
        .get(1usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 3;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = encoded
        .get(2usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 2;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let decoded: Vec<i32> = rle_decode(&encoded)?;
    let _cse_temp_3 = decoded.len() as i32;
    let _cse_temp_4 = data.len() as i32;
    let _cse_temp_5 = _cse_temp_3 == _cse_temp_4;
    if _cse_temp_5 {
        result = ((result).py_add(1i32)) as i32;
    }
    r#match = 1;
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        if decoded
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            != data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        {
            r#match = 0;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    result = ((result).py_add(r#match)) as i32;
    Ok(result)
}
#[doc = "Test delta encoding roundtrip."]
pub fn test_delta_encoding() -> Result<i32, Box<dyn std::error::Error>> {
    let mut r#match: i32 = Default::default();
    let mut result: i32 = Default::default();
    result = 0;
    let data: Vec<i32> = vec![10, 13, 17, 20, 25];
    let encoded: Vec<i32> = delta_encode(&data);
    let _cse_temp_0 = encoded
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 10;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = encoded
        .get(1usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 3;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = encoded
        .get(2usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 4;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let decoded: Vec<i32> = delta_decode(&encoded);
    r#match = 1;
    let mut i: i32 = 0;
    while i < data.len() as i32 {
        if decoded
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            != data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        {
            r#match = 0;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    result = ((result).py_add(r#match)) as i32;
    let empty_enc: Vec<i32> = delta_encode(&vec![]);
    let _cse_temp_3 = empty_enc.len() as i32;
    let _cse_temp_4 = _cse_temp_3 == 0;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Test variable-length integer encoding roundtrip."]
#[doc = " Depyler: proven to terminate"]
pub fn test_varint() -> Result<i32, Box<dyn std::error::Error>> {
    let mut result: i32 = Default::default();
    result = 0;
    let enc0: Vec<i32> = varint_encode(0);
    let _cse_temp_0 = enc0
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        == 0;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = varint_decode(&enc0)? == 0;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let enc300: Vec<i32> = varint_encode(300);
    let _cse_temp_2 = varint_decode(&enc300)? == 300;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let enc_big: Vec<i32> = varint_encode(123456789);
    let _cse_temp_3 = varint_decode(&enc_big)? == 123456789;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_4 = enc_big.len() as i32;
    let _cse_temp_5 = _cse_temp_4 > 1;
    if _cse_temp_5 {
        result = ((result).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Test Gray code conversion roundtrip."]
#[doc = " Depyler: verified panic-free"]
pub fn test_gray_code() -> i32 {
    let mut all_roundtrip: i32 = Default::default();
    let mut gray_diff: i32 = Default::default();
    let mut result: i32 = Default::default();
    result = 0;
    let _cse_temp_0 = to_gray(0) == 0;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = to_gray(1) == 1;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = to_gray(2) == 3;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_3 = to_gray(3) == 2;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let mut i: i32 = 0;
    all_roundtrip = 1;
    while i < 256 {
        if from_gray(to_gray(i)) != i {
            all_roundtrip = 0;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    result = ((result).py_add(all_roundtrip)) as i32;
    gray_diff = 1;
    let mut j: i32 = 1;
    while j < 64 {
        let diff: i32 = to_gray(j) ^ to_gray((j) - (1i32));
        if popcount(diff) != 1 {
            gray_diff = 0;
        }
        j = ((j).py_add(1i32)) as i32;
    }
    result = ((result).py_add(gray_diff)) as i32;
    result
}
#[doc = "Test Hamming distance and bit rotation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_hamming_and_rotation() -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let _cse_temp_0 = hamming_distance(0, 0) == 0;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = hamming_distance(255, 0) == 8;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = hamming_distance(10, 5) == 4;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let rot: i32 = rotate_left_32(1, 4);
    let _cse_temp_3 = rot == 16;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let rot_back: i32 = rotate_right_32(rot, 4);
    let _cse_temp_4 = rot_back == 1;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    let full_rot: i32 = rotate_left_32(24237, 16);
    let _cse_temp_5 = full_rot == 24237;
    if _cse_temp_5 {
        result = ((result).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Test CRC checksum computation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_crc() -> Result<i32, Box<dyn std::error::Error>> {
    let mut result: i32 = Default::default();
    result = 0;
    let data1: Vec<i32> = vec![1, 2, 3];
    let crc1: i32 = crc8_simple(&data1, 7)?;
    let _cse_temp_0 = crc1 >= 0;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = crc1 <= 255;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let data2: Vec<i32> = vec![1, 2, 3];
    let crc2: i32 = crc8_simple(&data2, 7)?;
    let _cse_temp_2 = crc1 == crc2;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let crc16_val: i32 = crc16_simple(&data1, 40961)?;
    let _cse_temp_3 = crc16_val >= 0;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_4 = crc16_val <= 65535;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Test base64-like encoding roundtrip."]
pub fn test_base64_like() -> Result<i32, Box<dyn std::error::Error>> {
    let mut r#match: i32 = Default::default();
    let mut all_valid: i32 = Default::default();
    let mut result: i32 = Default::default();
    result = 0;
    let data: Vec<i32> = vec![65, 66, 67];
    let encoded: Vec<i32> = base64_like_encode(&data)?;
    let _cse_temp_0 = encoded.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 4;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let mut i: i32 = 0;
    all_valid = 1;
    while i < encoded.len() as i32 {
        if encoded
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            < 0
        {
            all_valid = 0;
        }
        if encoded
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            > 63
        {
            all_valid = 0;
        }
        i = ((i).py_add(1i32)) as i32;
    }
    result = ((result).py_add(all_valid)) as i32;
    let decoded: Vec<i32> = base64_like_decode(&encoded, 3)?;
    let _cse_temp_2 = decoded.len() as i32;
    let _cse_temp_3 = _cse_temp_2 == 3;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    r#match = 1;
    let mut j: i32 = 0;
    while j < data.len() as i32 {
        if decoded
            .get(j as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            != data
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        {
            r#match = 0;
        }
        j = ((j).py_add(1i32)) as i32;
    }
    result = ((result).py_add(r#match)) as i32;
    Ok(result)
}
#[doc = "Test zigzag encoding for signed-to-unsigned mapping."]
#[doc = " Depyler: verified panic-free"]
pub fn test_zigzag() -> i32 {
    let mut roundtrip_ok: i32 = Default::default();
    let mut result: i32 = Default::default();
    result = 0;
    let _cse_temp_0 = zigzag_encode(0) == 0;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = zigzag_encode(-1) == 1;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = zigzag_encode(1) == 2;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_3 = zigzag_encode(-2) == 3;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_4 = zigzag_encode(2) == 4;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    roundtrip_ok = 1;
    let mut n: i32 = -100;
    while n <= 100 {
        if zigzag_decode(zigzag_encode(n)) != n {
            roundtrip_ok = 0;
        }
        n = ((n).py_add(1i32)) as i32;
    }
    result = ((result).py_add(roundtrip_ok)) as i32;
    result
}
#[doc = "Test prefix-free code validation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_prefix_free() -> Result<i32, Box<dyn std::error::Error>> {
    let mut result: i32 = Default::default();
    result = 0;
    let codes_good: Vec<i32> = vec![0, 2, 3];
    let lens_good: Vec<i32> = vec![1, 2, 2];
    let _cse_temp_0 = validate_prefix_free(&codes_good, &lens_good)? == 1;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let codes_bad: Vec<i32> = vec![0, 1, 1];
    let lens_bad: Vec<i32> = vec![1, 2, 1];
    let _cse_temp_1 = validate_prefix_free(&codes_bad, &lens_bad)? == 0;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let codes_single: Vec<i32> = vec![5];
    let lens_single: Vec<i32> = vec![3];
    let _cse_temp_2 = validate_prefix_free(&codes_single, &lens_single)? == 1;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Test bit interleaving / Morton codes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_morton_codes() -> i32 {
    let mut result: i32 = Default::default();
    result = 0;
    let morton: i32 = interleave_bits(5, 3);
    let _cse_temp_0 = deinterleave_x(morton) == 5;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = deinterleave_y(morton) == 3;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = interleave_bits(0, 0) == 0;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let m1: i32 = interleave_bits(1, 0);
    let _cse_temp_3 = m1 == 1;
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let m2: i32 = interleave_bits(0, 1);
    let _cse_temp_4 = m2 == 2;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    let dist: i32 = morton_distance(0, 0, 1, 1);
    let _cse_temp_5 = dist > 0;
    if _cse_temp_5 {
        result = ((result).py_add(1i32)) as i32;
    }
    result
}
#[doc = "Test frequency table and bit length assignment."]
#[doc = " Depyler: proven to terminate"]
pub fn test_huffman_like() -> Result<i32, Box<dyn std::error::Error>> {
    let mut result: i32 = Default::default();
    result = 0;
    let data: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
    let freq: std::collections::HashMap<i32, i32> = frequency_table(&data)?;
    let _cse_temp_0 = freq.get(&(1)).cloned().unwrap_or_default() == 3;
    if _cse_temp_0 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_1 = freq.get(&(2)).cloned().unwrap_or_default() == 2;
    if _cse_temp_1 {
        result = ((result).py_add(1i32)) as i32;
    }
    let _cse_temp_2 = freq.get(&(3)).cloned().unwrap_or_default() == 1;
    if _cse_temp_2 {
        result = ((result).py_add(1i32)) as i32;
    }
    let lengths: std::collections::HashMap<i32, i32> = assign_bit_lengths(&freq);
    let _cse_temp_3 = lengths.get(&(1)).cloned().unwrap_or_default()
        <= lengths.get(&(3)).cloned().unwrap_or_default();
    if _cse_temp_3 {
        result = ((result).py_add(1i32)) as i32;
    }
    let total: i32 = total_encoded_bits(&data, &lengths)?;
    let _cse_temp_4 = total > 0;
    if _cse_temp_4 {
        result = ((result).py_add(1i32)) as i32;
    }
    Ok(result)
}
#[doc = "Run all test functions and return the sum of passed checks."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_all_tests() -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = 0;
    let _cse_temp_0 = ((total).py_add(test_popcount_and_parity())) as i32;
    total = _cse_temp_0;
    let _cse_temp_1 = ((total).py_add(test_reverse_bits())) as i32;
    total = _cse_temp_1;
    let _cse_temp_2 = ((total).py_add(test_isolate_and_clear())) as i32;
    total = _cse_temp_2;
    let _cse_temp_3 = ((total).py_add(test_bit_utilities())) as i32;
    total = _cse_temp_3;
    let _cse_temp_4 = ((total).py_add(test_pack_unpack_rgb())) as i32;
    total = _cse_temp_4;
    let _cse_temp_5 = ((total).py_add(test_bitfield_ops())) as i32;
    total = _cse_temp_5;
    let _cse_temp_6 = ((total).py_add(test_xor_cipher()?)) as i32;
    total = _cse_temp_6;
    let _cse_temp_7 = ((total).py_add(test_rle()?)) as i32;
    total = _cse_temp_7;
    let _cse_temp_8 = ((total).py_add(test_delta_encoding()?)) as i32;
    total = _cse_temp_8;
    let _cse_temp_9 = ((total).py_add(test_varint()?)) as i32;
    total = _cse_temp_9;
    let _cse_temp_10 = ((total).py_add(test_gray_code())) as i32;
    total = _cse_temp_10;
    let _cse_temp_11 = ((total).py_add(test_hamming_and_rotation())) as i32;
    total = _cse_temp_11;
    let _cse_temp_12 = ((total).py_add(test_crc()?)) as i32;
    total = _cse_temp_12;
    let _cse_temp_13 = ((total).py_add(test_base64_like()?)) as i32;
    total = _cse_temp_13;
    let _cse_temp_14 = ((total).py_add(test_zigzag())) as i32;
    total = _cse_temp_14;
    let _cse_temp_15 = ((total).py_add(test_prefix_free()?)) as i32;
    total = _cse_temp_15;
    let _cse_temp_16 = ((total).py_add(test_morton_codes())) as i32;
    total = _cse_temp_16;
    let _cse_temp_17 = ((total).py_add(test_huffman_like()?)) as i32;
    total = _cse_temp_17;
    Ok(total)
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passed: i32 = run_all_tests()?;
    println!("{}", passed);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_popcount_examples() {
        assert_eq!(popcount(0), 0);
        assert_eq!(popcount(1), 1);
        assert_eq!(popcount(-1), -1);
    }
    #[test]
    fn test_parity_examples() {
        assert_eq!(parity(0), 0);
        assert_eq!(parity(1), 1);
        assert_eq!(parity(-1), -1);
    }
    #[test]
    fn test_reverse_bits_32_examples() {
        assert_eq!(reverse_bits_32(0), 0);
        assert_eq!(reverse_bits_32(1), 1);
        assert_eq!(reverse_bits_32(-1), -1);
    }
    #[test]
    fn test_isolate_lowest_set_bit_examples() {
        assert_eq!(isolate_lowest_set_bit(0), 0);
        assert_eq!(isolate_lowest_set_bit(1), 1);
        assert_eq!(isolate_lowest_set_bit(-1), -1);
    }
    #[test]
    fn test_clear_lowest_set_bit_examples() {
        assert_eq!(clear_lowest_set_bit(0), 0);
        assert_eq!(clear_lowest_set_bit(1), 1);
        assert_eq!(clear_lowest_set_bit(-1), -1);
    }
    #[test]
    fn test_highest_set_bit_pos_examples() {
        assert_eq!(highest_set_bit_pos(0), 0);
        assert_eq!(highest_set_bit_pos(1), 1);
        assert_eq!(highest_set_bit_pos(-1), -1);
    }
    #[test]
    fn test_next_power_of_two_examples() {
        assert_eq!(next_power_of_two(0), 0);
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(-1), -1);
    }
    #[test]
    fn test_unpack_r_examples() {
        assert_eq!(unpack_r(0), 0);
        assert_eq!(unpack_r(1), 1);
        assert_eq!(unpack_r(-1), -1);
    }
    #[test]
    fn test_unpack_g_examples() {
        assert_eq!(unpack_g(0), 0);
        assert_eq!(unpack_g(1), 1);
        assert_eq!(unpack_g(-1), -1);
    }
    #[test]
    fn test_unpack_b_examples() {
        assert_eq!(unpack_b(0), 0);
        assert_eq!(unpack_b(1), 1);
        assert_eq!(unpack_b(-1), -1);
    }
    #[test]
    fn test_rle_encode_examples() {
        assert_eq!(rle_encode(vec![]), vec![]);
        assert_eq!(rle_encode(vec![1]), vec![1]);
    }
    #[test]
    fn test_rle_decode_examples() {
        assert_eq!(rle_decode(vec![]), vec![]);
        assert_eq!(rle_decode(vec![1]), vec![1]);
    }
    #[test]
    fn test_delta_encode_examples() {
        assert_eq!(delta_encode(vec![]), vec![]);
        assert_eq!(delta_encode(vec![1]), vec![1]);
    }
    #[test]
    fn test_delta_decode_examples() {
        assert_eq!(delta_decode(vec![]), vec![]);
        assert_eq!(delta_decode(vec![1]), vec![1]);
    }
    #[test]
    fn test_varint_decode_examples() {
        assert_eq!(varint_decode(&vec![]), 0);
        assert_eq!(varint_decode(&vec![1]), 1);
        assert_eq!(varint_decode(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_varint_encode_list_examples() {
        assert_eq!(varint_encode_list(vec![]), vec![]);
        assert_eq!(varint_encode_list(vec![1]), vec![1]);
    }
    #[test]
    fn test_to_gray_examples() {
        assert_eq!(to_gray(0), 0);
        assert_eq!(to_gray(1), 1);
        assert_eq!(to_gray(-1), -1);
    }
    #[test]
    fn test_from_gray_examples() {
        assert_eq!(from_gray(0), 0);
        assert_eq!(from_gray(1), 1);
        assert_eq!(from_gray(-1), -1);
    }
    #[test]
    fn test_hamming_distance_examples() {
        assert_eq!(hamming_distance(0, 0), 0);
        assert_eq!(hamming_distance(1, 2), 3);
        assert_eq!(hamming_distance(-1, 1), 0);
    }
    #[test]
    fn test_rotate_left_32_examples() {
        assert_eq!(rotate_left_32(0, 0), 0);
        assert_eq!(rotate_left_32(1, 2), 3);
        assert_eq!(rotate_left_32(-1, 1), 0);
    }
    #[test]
    fn test_rotate_right_32_examples() {
        assert_eq!(rotate_right_32(0, 0), 0);
        assert_eq!(rotate_right_32(1, 2), 3);
        assert_eq!(rotate_right_32(-1, 1), 0);
    }
    #[test]
    fn test_base64_like_encode_examples() {
        assert_eq!(base64_like_encode(vec![]), vec![]);
        assert_eq!(base64_like_encode(vec![1]), vec![1]);
    }
    #[test]
    fn test_zigzag_encode_examples() {
        assert_eq!(zigzag_encode(0), 0);
        assert_eq!(zigzag_encode(1), 1);
        assert_eq!(zigzag_encode(-1), -1);
    }
    #[test]
    fn test_zigzag_decode_examples() {
        assert_eq!(zigzag_decode(0), 0);
        assert_eq!(zigzag_decode(1), 1);
        assert_eq!(zigzag_decode(-1), -1);
    }
    #[test]
    fn test_interleave_bits_examples() {
        assert_eq!(interleave_bits(0, 0), 0);
        assert_eq!(interleave_bits(1, 2), 3);
        assert_eq!(interleave_bits(-1, 1), 0);
    }
    #[test]
    fn test_deinterleave_x_examples() {
        assert_eq!(deinterleave_x(0), 0);
        assert_eq!(deinterleave_x(1), 1);
        assert_eq!(deinterleave_x(-1), -1);
    }
    #[test]
    fn test_deinterleave_y_examples() {
        assert_eq!(deinterleave_y(0), 0);
        assert_eq!(deinterleave_y(1), 1);
        assert_eq!(deinterleave_y(-1), -1);
    }
    #[test]
    fn test_test_popcount_and_parity_examples() {
        let _ = test_popcount_and_parity();
    }
    #[test]
    fn test_test_reverse_bits_examples() {
        let _ = test_reverse_bits();
    }
    #[test]
    fn test_test_isolate_and_clear_examples() {
        let _ = test_isolate_and_clear();
    }
    #[test]
    fn test_test_bit_utilities_examples() {
        let _ = test_bit_utilities();
    }
    #[test]
    fn test_test_pack_unpack_rgb_examples() {
        let _ = test_pack_unpack_rgb();
    }
    #[test]
    fn test_test_bitfield_ops_examples() {
        let _ = test_bitfield_ops();
    }
    #[test]
    fn test_test_xor_cipher_examples() {
        let _ = test_xor_cipher();
    }
    #[test]
    fn test_test_rle_examples() {
        let _ = test_rle();
    }
    #[test]
    fn test_test_delta_encoding_examples() {
        let _ = test_delta_encoding();
    }
    #[test]
    fn test_test_varint_examples() {
        let _ = test_varint();
    }
    #[test]
    fn test_test_gray_code_examples() {
        let _ = test_gray_code();
    }
    #[test]
    fn test_test_hamming_and_rotation_examples() {
        let _ = test_hamming_and_rotation();
    }
    #[test]
    fn test_test_crc_examples() {
        let _ = test_crc();
    }
    #[test]
    fn test_test_base64_like_examples() {
        let _ = test_base64_like();
    }
    #[test]
    fn test_test_zigzag_examples() {
        let _ = test_zigzag();
    }
    #[test]
    fn test_test_prefix_free_examples() {
        let _ = test_prefix_free();
    }
    #[test]
    fn test_test_morton_codes_examples() {
        let _ = test_morton_codes();
    }
    #[test]
    fn test_test_huffman_like_examples() {
        let _ = test_huffman_like();
    }
    #[test]
    fn test_run_all_tests_examples() {
        let _ = run_all_tests();
    }
}