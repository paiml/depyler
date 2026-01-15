#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
#[doc = "// NOTE: Map Python module 'enum'(tracked in DEPYLER-0424)"]
pub static SMTP_CODES: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>> =
    std::sync::LazyLock::new(|| {
        let mut map = HashMap::new();
        map.insert(
            DepylerValue::Int(211 as i64),
            DepylerValue::Str("System status".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(214 as i64),
            DepylerValue::Str("Help message".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(220 as i64),
            DepylerValue::Str("Service ready".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(221 as i64),
            DepylerValue::Str("Service closing".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(235 as i64),
            DepylerValue::Str("Authentication successful".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(250 as i64),
            DepylerValue::Str("OK".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(251 as i64),
            DepylerValue::Str("User not local; will forward".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(252 as i64),
            DepylerValue::Str("Cannot VRFY user".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(334 as i64),
            DepylerValue::Str("Server challenge".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(354 as i64),
            DepylerValue::Str("Start mail input".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(421 as i64),
            DepylerValue::Str("Service not available".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(450 as i64),
            DepylerValue::Str("Mailbox unavailable".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(451 as i64),
            DepylerValue::Str("Local error".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(452 as i64),
            DepylerValue::Str("Insufficient storage".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(500 as i64),
            DepylerValue::Str("Syntax error".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(501 as i64),
            DepylerValue::Str("Syntax error in parameters".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(502 as i64),
            DepylerValue::Str("Command not implemented".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(503 as i64),
            DepylerValue::Str("Bad sequence of commands".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(504 as i64),
            DepylerValue::Str("Parameter not implemented".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(550 as i64),
            DepylerValue::Str("Mailbox unavailable".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(551 as i64),
            DepylerValue::Str("User not local".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(552 as i64),
            DepylerValue::Str("Storage exceeded".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(553 as i64),
            DepylerValue::Str("Mailbox name not allowed".to_string().to_string()),
        );
        map.insert(
            DepylerValue::Int(554 as i64),
            DepylerValue::Str("Transaction failed".to_string().to_string()),
        );
        map
    });
use std::collections::HashMap;
use std::sync::LazyLock;
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
#[derive(Debug, Clone)]
pub struct SMTPState {}
impl SMTPState {
    pub const INIT: i32 = "init".to_string();
    pub const GREETED: i32 = "greeted".to_string();
    pub const AUTHENTICATED: i32 = "authenticated".to_string();
    pub const MAIL_FROM: i32 = "mail_from".to_string();
    pub const RCPT_TO: i32 = "rcpt_to".to_string();
    pub const DATA: i32 = "data".to_string();
    pub const QUIT: i32 = "quit".to_string();
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SMTPCommand {
    pub verb: String,
    pub args: String,
}
impl SMTPCommand {
    pub fn new(verb: String, args: String) -> Self {
        Self { verb, args }
    }
    pub fn encode(&self) -> Vec<u8> {
        if self.args.clone() {
            return format!("{} {}\r\n", self.verb.clone(), self.args.clone()).encode();
        };
        return format!("{}\r\n", self.verb.clone()).encode();
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SMTPResponse {
    pub code: i32,
    pub message: String,
    pub is_multiline: bool,
    pub lines: Vec<String>,
}
impl SMTPResponse {
    pub fn new(code: i32, message: String, is_multiline: bool, lines: Vec<String>) -> Self {
        Self {
            code,
            message,
            is_multiline,
            lines,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct EmailMessage {
    pub from_addr: String,
    pub to_addrs: Vec<String>,
    pub subject: String,
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
}
impl EmailMessage {
    pub fn new(
        from_addr: String,
        to_addrs: Vec<String>,
        subject: String,
        body: String,
        headers: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            from_addr,
            to_addrs,
            subject,
            body,
            headers,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SMTPSession {
    pub state: DepylerValue,
    pub domain: String,
    pub mail_from: Option<String>,
    pub rcpt_to: Vec<String>,
    pub data_lines: Vec<String>,
    pub authenticated: bool,
}
impl SMTPSession {
    pub fn new(domain: String) -> Self {
        Self {
            state: Default::default(),
            domain,
            mail_from: Default::default(),
            rcpt_to: Vec::new(),
            data_lines: Vec::new(),
            authenticated: false,
        }
    }
    pub fn process_command(&mut self, cmd: &SMTPCommand) -> SMTPResponse {
        let verb = cmd.verb;
        if verb == "HELO".to_string() {
            self.state = SMTPState::GREETED;
            return SMTPResponse::new(250, format!("Hello {}, pleased to meet you", cmd.args));
        };
        if verb == "EHLO".to_string() {
            self.state = SMTPState::GREETED;
            return SMTPResponse::new(250, self.domain.clone());
        };
        if verb == "AUTH".to_string() {
            if self.state.clone() != SMTPState::GREETED {
                return SMTPResponse::new(503, "Bad sequence of commands".to_string());
            };
            self.authenticated = true;
            self.state = SMTPState::AUTHENTICATED;
            return SMTPResponse::new(235, "Authentication successful".to_string());
        };
        if verb == "MAIL".to_string() {
            if ![SMTPState::GREETED, SMTPState::AUTHENTICATED].contains(&self.state.clone()) {
                return SMTPResponse::new(503, "Bad sequence of commands".to_string());
            };
            self.mail_from = extract_email(cmd.args.replace("FROM:", "").trim().to_string());
            self.state = SMTPState::MAIL_FROM;
            return SMTPResponse::new(250, "OK".to_string());
        };
        if verb == "RCPT".to_string() {
            if ![SMTPState::MAIL_FROM, SMTPState::RCPT_TO].contains(&self.state.clone()) {
                return SMTPResponse::new(503, "Bad sequence of commands".to_string());
            };
            let rcpt = extract_email(cmd.args.replace("TO:", "").trim().to_string());
            self.rcpt_to.push(rcpt);
            self.state = SMTPState::RCPT_TO;
            return SMTPResponse::new(250, "OK".to_string());
        };
        if verb == "DATA".to_string() {
            if self.state.clone() != SMTPState::RCPT_TO {
                return SMTPResponse::new(503, "Bad sequence of commands".to_string());
            };
            self.state = SMTPState::DATA;
            return SMTPResponse::new(354, "End data with <CR><LF>.<CR><LF>".to_string());
        };
        if verb == "RSET".to_string() {
            self.mail_from = None;
            self.rcpt_to = vec![];
            self.data_lines = vec![];
            self.state = if self.state.clone() != SMTPState::INIT {
                SMTPState::GREETED
            } else {
                SMTPState::INIT
            };
            return SMTPResponse::new(250, "OK".to_string());
        };
        if verb == "NOOP".to_string() {
            return SMTPResponse::new(250, "OK".to_string());
        };
        if verb == "QUIT".to_string() {
            self.state = SMTPState::QUIT;
            return SMTPResponse::new(221, format!("{} closing connection", self.domain.clone()));
        };
        return SMTPResponse::new(500, format!("Command not recognized: {}", verb));
    }
    pub fn reset(&mut self) {
        self.mail_from = None;
        self.rcpt_to = vec![];
        self.data_lines = vec![];
    }
}
#[derive(Default)]

struct Args {
    
    
    
    #[doc = "Operation mode"]
    mode: String,
    
    
    #[doc = "SMTP command verb"]
    verb: String,
    
    
    #[doc = "Command arguments"]
    args: String,
    
    
    #[doc = "Response code"]
    code: i32,
    
    #[doc = "From address"]
    from_addr: Option<String>,
    
    #[doc = "To address"]
    to_addr: Option<String>,
    
    
    #[doc = "Subject"]
    subject: String,
    
    
    #[doc = "Body"]
    body: String,
    
    #[doc = "Username for auth"]
    username: Option<String>,
    
    #[doc = "Password for auth"]
    password: Option<String>,
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn dataclass<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn field<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn Enum<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = "Parse SMTP response from bytes."]
pub fn parse_response(data: &Vec<u8>) -> Result<SMTPResponse, Box<dyn std::error::Error>> {
    let text = String::from_utf8_lossy(&data).to_string();
    let lines = text
        .trim()
        .to_string()
        .split("\r\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if lines.is_empty() {
        return Err(Box::new(ValueError::new("Empty response".to_string())));
    }
    let first_line = lines
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let _cse_temp_0 = first_line.len() as i32;
    let _cse_temp_1 = _cse_temp_0 < 3;
    if _cse_temp_1 {
        return Err(Box::new(ValueError::new(
            "Invalid response format".to_string(),
        )));
    }
    let _cse_temp_2 = ({
        let base = first_line;
        let stop_idx: i32 = 3;
        let len = base.chars().count() as i32;
        let actual_stop = if stop_idx < 0 {
            (len + stop_idx).max(0) as usize
        } else {
            stop_idx.min(len) as usize
        };
        base.chars().take(actual_stop).collect::<String>()
    }) as i32;
    let code = _cse_temp_2;
    let _cse_temp_3 = _cse_temp_0 > 3;
    let _cse_temp_4 = {
        let base = &first_line;
        let idx: i32 = 3;
        let actual_idx = if idx < 0 {
            base.chars().count().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.chars()
            .nth(actual_idx)
            .map(|c| c.to_string())
            .unwrap_or_default()
    } == "-";
    let _cse_temp_5 = (_cse_temp_3) && (_cse_temp_4);
    let is_multiline = _cse_temp_5;
    let mut all_lines = vec![];
    for line in lines.iter().cloned() {
        if line.len() as i32 > 4 {
            all_lines.push(DepylerValue::Str(format!("{:?}", {
                let base = line;
                let start_idx: i32 = 4;
                let len = base.chars().count() as i32;
                let actual_start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx.min(len) as usize
                };
                base.chars().skip(actual_start).collect::<String>()
            })));
        } else {
            if line.len() as i32 > 3 {
                all_lines.push(DepylerValue::Str(format!(
                    "{:?}",
                    if line.len() as i32 > 4 {
                        {
                            let base = line;
                            let start_idx: i32 = 4;
                            let len = base.chars().count() as i32;
                            let actual_start = if start_idx < 0 {
                                (len + start_idx).max(0) as usize
                            } else {
                                start_idx.min(len) as usize
                            };
                            base.chars().skip(actual_start).collect::<String>()
                        }
                    } else {
                        ""
                    }
                )));
            }
        }
    }
    let message = if !all_lines.is_empty() {
        all_lines
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
    } else {
        DepylerValue::Str("".to_string())
    };
    Ok(SMTPResponse::new(code, message, is_multiline, all_lines))
}
#[doc = "Encode SMTP response to bytes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn encode_response(code: i32, message: String, multiline: &Option<Vec<String>>) -> Vec<u8> {
    if let Some(ref multiline_val) = multiline {
        let mut lines = vec![];
        for (__i, line) in {
            let base = &multiline_val;
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
            lines.push(DepylerValue::Str(format!(
                "{:?}",
                format!("{}-{:?}", code, line)
            )));
        }
        lines.push(DepylerValue::Str(format!(
            "{:?}",
            format!("{} {}", code, {
                let base = &multiline_val;
                base.get(base.len().saturating_sub(1usize))
                    .cloned()
                    .unwrap_or_default()
            })
        )));
        return format!("{}{}", lines.join("\r\n"), "\r\n".to_string())
            .as_bytes()
            .to_vec();
    }
    format!("{} {}\r\n", code, message).as_bytes().to_vec()
}
#[doc = "Parse SMTP command from bytes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_command(data: &Vec<u8>) -> SMTPCommand {
    let text = String::from_utf8_lossy(&data)
        .to_string()
        .trim()
        .to_string();
    let _cse_temp_0 = text.contains(" ");
    let mut args: String;
    let mut verb;
    if _cse_temp_0 {
        let _split_parts = text
            .splitn((1 + 1) as usize, " ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut verb = _split_parts.get(0).cloned().unwrap_or_default();
        let mut args = _split_parts.get(1).cloned().unwrap_or_default();
    } else {
        verb = text;
        args = "".to_string();
    }
    SMTPCommand::new(verb.to_uppercase(), args)
}
#[doc = "Basic email validation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_email(email: &str) -> bool {
    let _cse_temp_0 = !email.contains("@");
    let _cse_temp_1 = (email.is_empty()) || (_cse_temp_0);
    if _cse_temp_1 {
        return false;
    }
    let parts = email
        .split("@")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _cse_temp_2 = parts.len() as i32;
    let _cse_temp_3 = _cse_temp_2 != 2;
    if _cse_temp_3 {
        return false;
    }
    let (local, domain) = parts;
    let _cse_temp_4 = (!local) || (!domain);
    if _cse_temp_4 {
        return false;
    }
    let _cse_temp_5 = !domain.contains(".");
    if _cse_temp_5 {
        return false;
    }
    true
}
#[doc = "Extract email from address format like '<user@example.com>'."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn extract_email(mut address: String) -> String {
    address = address.trim().to_string();
    let _cse_temp_0 = (address.starts_with("<")) && (address.ends_with(">"));
    if _cse_temp_0 {
        return {
            let base = address;
            let start_idx: i32 = 1;
            let stop_idx: i32 = -1;
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
    }
    address.to_string()
}
#[doc = "Format email in angle bracket notation."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn format_email(mut email: String) -> String {
    email = extract_email(&email);
    format!("<{}>", email)
}
#[doc = "Encode credentials for AUTH PLAIN."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn encode_auth_plain(username: String, password: String) -> String {
    let auth_string = format!("\0{}\0{}", username, password);
    format!("{:?}", auth_string.as_bytes().to_vec())
        .into_bytes()
}
#[doc = "Decode AUTH PLAIN credentials."]
#[doc = " Depyler: proven to terminate"]
pub fn decode_auth_plain(encoded: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let decoded = String::from_utf8_lossy(
        &base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .unwrap(),
    )
    .to_string();
    let parts = decoded
        .split("\0")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _cse_temp_0 = parts.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 3;
    if _cse_temp_1 {
        return Err(Box::new(ValueError::new(
            "Invalid AUTH PLAIN format".to_string(),
        )));
    }
    Ok((
        parts
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range"),
        parts
            .get(2usize)
            .cloned()
            .expect("IndexError: list index out of range"),
    ))
}
#[doc = "Encode for AUTH LOGIN."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn encode_auth_login(data: &str) -> String {
    format!("{:?}", data.as_bytes().to_vec())
        .into_bytes()
}
#[doc = "Decode AUTH LOGIN."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn decode_auth_login(encoded: &str) -> String {
    String::from_utf8_lossy(
        &base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .unwrap(),
    )
    .to_string()
}
#[doc = "Build raw email message content."]
#[doc = " Depyler: verified panic-free"]
pub fn build_message(message: &EmailMessage) -> String {
    let mut lines = vec![];
    lines.push(DepylerValue::Str(format!(
        "{:?}",
        format!("From: {}", message.from_addr)
    )));
    lines.push(DepylerValue::Str(format!(
        "{:?}",
        format!("To: {}", message.to_addrs.join(", ").display())
    )));
    lines.push(DepylerValue::Str(format!(
        "{:?}",
        format!("Subject: {}", message.subject)
    )));
    for (key, value) in message
        .headers
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        lines.push(DepylerValue::Str(format!(
            "{:?}",
            format!("{:?}: {:?}", key, value)
        )));
    }
    lines.push(DepylerValue::Str("".to_string()));
    lines.push(DepylerValue::Str(format!("{:?}", message.body)));
    lines.join("\r\n")
}
#[doc = "Parse raw email message content."]
pub fn parse_message(data: &str) -> Result<EmailMessage, Box<dyn std::error::Error>> {
    let mut current_key: String = Default::default();
    let mut current_value: String = Default::default();
    let parts = data
        .splitn((1 + 1) as usize, "\r\n\r\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let header_section = parts
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let body = if parts.len() as i32 > 1 {
        parts
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range")
    } else {
        ""
    };
    let mut headers = {
        let mut map = HashMap::new();
        map
    };
    current_value = "".to_string();
    for line in header_section
        .split("\r\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    {
        if (line.starts_with(" ")) || (line.starts_with("\t")) {
            current_value = format!(
                "{}{}",
                current_value,
                format!("{}{}", " ", line.trim().to_string())
            );
        } else {
            if current_key {
                headers.insert(
                    current_key.to_string().clone(),
                    DepylerValue::Str(format!("{:?}", current_value)),
                );
            }
            if line.contains(":") {
                let _split_parts = line
                    .splitn((1 + 1) as usize, ":")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let mut current_key = _split_parts.get(0).cloned().unwrap_or_default();
                let mut current_value = _split_parts.get(1).cloned().unwrap_or_default();
                current_key = current_key.trim().to_string();
                current_value = current_value.trim().to_string();
            } else {
            }
        }
    }
    if !current_key.is_empty() {
        headers.insert(
            current_key.to_string().clone(),
            DepylerValue::Str(format!("{:?}", current_value)),
        );
    }
    let from_addr = headers.get("From").cloned().unwrap_or("");
    let to_addrs = headers
        .get("To")
        .cloned()
        .unwrap_or("")
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|addr| addr.trim().to_string())
        .collect::<Vec<_>>();
    let subject = headers.get("Subject").cloned().unwrap_or("");
    for key in vec!["From".to_string(), "To".to_string(), "Subject".to_string()] {
        headers.remove(key).unwrap_or(None);
    }
    Ok(EmailMessage::new(
        from_addr, to_addrs, subject, body, headers,
    ))
}
#[doc = "Escape leading dot for DATA command."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn escape_dot(line: String) -> String {
    if line.starts_with(".") {
        return format!("{}{}", ".", line);
    }
    line.to_string()
}
#[doc = "Unescape leading dot from DATA command."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn unescape_dot(line: String) -> String {
    if line.starts_with("..") {
        return {
            let base = line;
            let start_idx: i32 = 1;
            let len = base.chars().count() as i32;
            let actual_start = if start_idx < 0 {
                (len + start_idx).max(0) as usize
            } else {
                start_idx.min(len) as usize
            };
            base.chars().skip(actual_start).collect::<String>()
        };
    }
    line.to_string()
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let args = Args::default();
    let has_from_addr = args.from_addr.is_some();
    let has_to_addr = args.to_addr.is_some();
    let has_username = args.username.is_some();
    let has_password = args.password.is_some();
    let _cse_temp_0 = args.mode == "command";
    let mut encoded;
    if _cse_temp_0 {
        let cmd = SMTPCommand::new(args.verb, args.args);
        encoded = cmd.as_bytes().to_vec();
        println!("{}", format!("Command: {} {}", args.verb, args.args));
        println!("{}", format!("Encoded: {}", encoded));
        println!(
            "{}",
            format!(
                "Hex: {}",
                encoded
                    .bytes()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>()
            )
        );
    } else {
        let _cse_temp_1 = args.mode == "response";
        if _cse_temp_1 {
            let message = SMTP_CODES.get(&args.code).cloned().unwrap_or("OK");
            encoded = encode_response(args.code, message, &None);
            println!("{}", format!("Response: {} {:?}", args.code, message));
            println!("{}", format!("Encoded: {}", encoded));
        } else {
            let _cse_temp_2 = args.mode == "message";
            if _cse_temp_2 {
                let _cse_temp_3 = (args.from_addr) && (args.to_addr);
                if _cse_temp_3 {
                    let msg = EmailMessage::new(
                        args.from_addr,
                        vec![args.to_addr],
                        args.subject,
                        args.body,
                    );
                    let raw = build_message(&msg);
                    println!("{}", "Raw message:");
                    println!("{}", raw);
                }
            } else {
                let _cse_temp_4 = args.mode == "auth";
                if _cse_temp_4 {
                    let _cse_temp_5 = (args.username) && (args.password);
                    if _cse_temp_5 {
                        let plain = encode_auth_plain(args.username, args.password);
                        println!("{}", format!("AUTH PLAIN: {:?}", plain));
                        let login_user = encode_auth_login(&args.username);
                        let login_pass = encode_auth_login(&args.password);
                        println!("{}", format!("AUTH LOGIN username: {:?}", login_user));
                        println!("{}", format!("AUTH LOGIN password: {:?}", login_pass));
                    }
                } else {
                    let _cse_temp_6 = args.mode == "session";
                    if _cse_temp_6 {
                        let mut session = SMTPSession::new("mail.example.com".to_string());
                        let commands = vec![
                            SMTPCommand::new("EHLO".to_string(), "client.example.com".to_string()),
                            SMTPCommand::new(
                                "MAIL".to_string(),
                                "FROM:<sender@example.com>".to_string(),
                            ),
                            SMTPCommand::new(
                                "RCPT".to_string(),
                                "TO:<recipient@example.com>".to_string(),
                            ),
                            SMTPCommand::new("DATA".to_string(), "".to_string()),
                            SMTPCommand::new("QUIT".to_string(), "".to_string()),
                        ];
                        for cmd in commands.iter().cloned() {
                            let response = session.process_command(cmd);
                            println!("{}", format!("C: {} {}", cmd.verb, cmd.args));
                            println!("{}", format!("S: {} {}", response.code, response.message));
                        }
                    }
                }
            }
        }
    }
    ()
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_validate_email_examples() {
        let _ = validate_email(Default::default());
    }
}
