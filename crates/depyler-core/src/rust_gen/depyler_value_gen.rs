//! DepylerValue enum generation - heterogeneous Python dict support
//! DEPYLER-DECOMPOSE: Extracted from rust_gen.rs
//!
//! Contains the DepylerValue sum type and all its trait implementations:
//! - Display, Hash, Eq, PartialEq, PartialOrd, Ord
//! - From<T> for various types
//! - Index<usize>, Index<i64>, IndexMut
//! - Iterator support
//! - Arithmetic operators (Add, Sub, Mul, Div, Rem, Neg)
//! - String methods (len, chars, contains, starts_with, etc.)

use quote::quote;

/// Generate the DepylerValue enum and all its trait implementations.
///
/// Returns a TokenStream containing the complete DepylerValue type definition
/// with all necessary trait implementations for Python-compatible heterogeneous
/// dictionary values.
pub(super) fn generate_depyler_value_tokens() -> proc_macro2::TokenStream {
    quote! {
            /// Sum type for heterogeneous dictionary values (Python fidelity)
            /// DEPYLER-1040b: Now implements Hash + Eq to support non-string dict keys
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
                /// DEPYLER-1050: Tuple variant for Python tuple support
                Tuple(Vec<DepylerValue>),
            }

            // DEPYLER-1040b: Implement PartialEq manually (f64 doesn't derive Eq)
            // DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables
            impl PartialEq for DepylerValue {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => _dv_a == _dv_b,
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => _dv_a.to_bits() == _dv_b.to_bits(),
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

            // DEPYLER-1040b: Implement Eq (required for HashMap keys)
            impl Eq for DepylerValue {}

            // DEPYLER-1040b: Implement Hash (required for HashMap keys)
            // Uses to_bits() for f64 to ensure consistent hashing
            // DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables
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
                            // Dicts are not hashable in Python either
                            // We hash the length as a fallback (matches Python's TypeError)
                            0u8.hash(state);
                        }
                        DepylerValue::Tuple(_dv_tuple) => _dv_tuple.hash(state),
                    }
                }
            }

            // DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables
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
                /// Get length of string, list, or dict
                /// DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables
                pub fn len(&self) -> usize {
                    match self {
                        DepylerValue::Str(_dv_str) => _dv_str.len(),
                        DepylerValue::List(_dv_list) => _dv_list.len(),
                        DepylerValue::Dict(_dv_dict) => _dv_dict.len(),
                        DepylerValue::Tuple(_dv_tuple) => _dv_tuple.len(),
                        _ => 0,
                    }
                }

                /// Check if empty
                pub fn is_empty(&self) -> bool {
                    self.len() == 0
                }

                /// Get chars iterator for string values
                pub fn chars(&self) -> std::str::Chars<'_> {
                    match self {
                        DepylerValue::Str(_dv_str) => _dv_str.chars(),
                        _ => "".chars(),
                    }
                }

                /// Insert into dict (mutates self if Dict variant)
                /// DEPYLER-1040b: Now accepts DepylerValue keys for non-string dict keys
                pub fn insert(&mut self, key: impl Into<DepylerValue>, value: impl Into<DepylerValue>) {
                    if let DepylerValue::Dict(_dv_dict) = self {
                        _dv_dict.insert(key.into(), value.into());
                    }
                }

                /// Get value from dict by key
                /// DEPYLER-1040b: Now accepts DepylerValue keys
                pub fn get(&self, key: &DepylerValue) -> Option<&DepylerValue> {
                    if let DepylerValue::Dict(_dv_dict) = self {
                        _dv_dict.get(key)
                    } else {
                        Option::None
                    }
                }

                /// Get value from dict by string key (convenience method)
                pub fn get_str(&self, key: &str) -> Option<&DepylerValue> {
                    self.get(&DepylerValue::Str(key.to_string()))
                }

                /// Check if dict contains key
                /// DEPYLER-1040b: Now accepts DepylerValue keys
                pub fn contains_key(&self, key: &DepylerValue) -> bool {
                    if let DepylerValue::Dict(_dv_dict) = self {
                        _dv_dict.contains_key(key)
                    } else {
                        false
                    }
                }

                /// Check if dict contains string key (convenience method)
                pub fn contains_key_str(&self, key: &str) -> bool {
                    self.contains_key(&DepylerValue::Str(key.to_string()))
                }

                /// DEPYLER-1051: Get iterator over list values
                /// Returns an empty iterator for non-list types
                pub fn iter(&self) -> std::slice::Iter<'_, DepylerValue> {
                    match self {
                        DepylerValue::List(_dv_list) => _dv_list.iter(),
                        _ => [].iter(),
                    }
                }

                /// DEPYLER-1051: Get mutable iterator over list values
                pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, DepylerValue> {
                    match self {
                        DepylerValue::List(_dv_list) => _dv_list.iter_mut(),
                        _ => [].iter_mut(),
                    }
                }

                /// DEPYLER-1051: Get iterator over dict key-value pairs
                /// DEPYLER-1040b: Now uses DepylerValue keys
                pub fn items(&self) -> std::collections::hash_map::Iter<'_, DepylerValue, DepylerValue> {
                    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>> = std::sync::LazyLock::new(|| std::collections::HashMap::new());
                    match self {
                        DepylerValue::Dict(_dv_dict) => _dv_dict.iter(),
                        _ => EMPTY_MAP.iter(),
                    }
                }

                /// DEPYLER-1051: Get iterator over dict keys
                /// DEPYLER-1040b: Now returns DepylerValue keys
                pub fn keys(&self) -> std::collections::hash_map::Keys<'_, DepylerValue, DepylerValue> {
                    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>> = std::sync::LazyLock::new(|| std::collections::HashMap::new());
                    match self {
                        DepylerValue::Dict(_dv_dict) => _dv_dict.keys(),
                        _ => EMPTY_MAP.keys(),
                    }
                }

                /// DEPYLER-1051: Get iterator over dict values
                /// DEPYLER-1040b: Now uses DepylerValue keys internally
                pub fn values(&self) -> std::collections::hash_map::Values<'_, DepylerValue, DepylerValue> {
                    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>> = std::sync::LazyLock::new(|| std::collections::HashMap::new());
                    match self {
                        DepylerValue::Dict(_dv_dict) => _dv_dict.values(),
                        _ => EMPTY_MAP.values(),
                    }
                }

                /// Convert to String (renamed to avoid shadowing Display::to_string)
                /// DEPYLER-1121: Renamed from to_string to as_string to fix clippy::inherent_to_string_shadow_display
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

                /// DEPYLER-1215: Get as str reference (for string values only)
                pub fn as_str(&self) -> Option<&str> {
                    match self {
                        DepylerValue::Str(_dv_str) => Some(_dv_str.as_str()),
                        _ => None,
                    }
                }

                /// DEPYLER-1215: Get as i64 (for integer values)
                pub fn as_i64(&self) -> Option<i64> {
                    match self {
                        DepylerValue::Int(_dv_int) => Some(*_dv_int),
                        _ => None,
                    }
                }

                /// DEPYLER-1215: Get as f64 (for float values)
                pub fn as_f64(&self) -> Option<f64> {
                    match self {
                        DepylerValue::Float(_dv_float) => Some(*_dv_float),
                        DepylerValue::Int(_dv_int) => Some(*_dv_int as f64),
                        _ => None,
                    }
                }

                /// DEPYLER-1215: Get as bool (for boolean values)
                pub fn as_bool(&self) -> Option<bool> {
                    match self {
                        DepylerValue::Bool(_dv_bool) => Some(*_dv_bool),
                        _ => None,
                    }
                }

                /// Convert to i64
                pub fn to_i64(&self) -> i64 {
                    match self {
                        DepylerValue::Int(_dv_int) => *_dv_int,
                        DepylerValue::Float(_dv_float) => *_dv_float as i64,
                        DepylerValue::Bool(_dv_bool) => if *_dv_bool { 1 } else { 0 },
                        DepylerValue::Str(_dv_str) => _dv_str.parse().unwrap_or(0),
                        _ => 0,
                    }
                }

                /// Convert to f64
                pub fn to_f64(&self) -> f64 {
                    match self {
                        DepylerValue::Float(_dv_float) => *_dv_float,
                        DepylerValue::Int(_dv_int) => *_dv_int as f64,
                        DepylerValue::Bool(_dv_bool) => if *_dv_bool { 1.0 } else { 0.0 },
                        DepylerValue::Str(_dv_str) => _dv_str.parse().unwrap_or(0.0),
                        _ => 0.0,
                    }
                }

                /// Convert to bool
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

                /// DEPYLER-1064: Get tuple element by index for tuple unpacking
                /// Returns the element at the given index, or panics with a readable error
                /// Works on both Tuple and List variants (Python treats them similarly for unpacking)
                pub fn get_tuple_elem(&self, _dv_idx: usize) -> DepylerValue {
                    match self {
                        DepylerValue::Tuple(_dv_tuple) => {
                            if _dv_idx < _dv_tuple.len() {
                                _dv_tuple[_dv_idx].clone()
                            } else {
                                panic!("Tuple index {} out of bounds (length {})", _dv_idx, _dv_tuple.len())
                            }
                        }
                        DepylerValue::List(_dv_list) => {
                            if _dv_idx < _dv_list.len() {
                                _dv_list[_dv_idx].clone()
                            } else {
                                panic!("List index {} out of bounds (length {})", _dv_idx, _dv_list.len())
                            }
                        }
                        _dv_other => panic!("Expected tuple or list for unpacking, found {:?}", _dv_other),
                    }
                }

                /// DEPYLER-1064: Extract tuple as Vec for multiple assignment
                /// Validates that the value is a tuple/list with the expected number of elements
                pub fn extract_tuple(&self, _dv_expected_len: usize) -> Vec<DepylerValue> {
                    match self {
                        DepylerValue::Tuple(_dv_tuple) => {
                            if _dv_tuple.len() != _dv_expected_len {
                                panic!("Expected tuple of length {}, got length {}", _dv_expected_len, _dv_tuple.len())
                            }
                            _dv_tuple.clone()
                        }
                        DepylerValue::List(_dv_list) => {
                            if _dv_list.len() != _dv_expected_len {
                                panic!("Expected list of length {}, got length {}", _dv_expected_len, _dv_list.len())
                            }
                            _dv_list.clone()
                        }
                        _dv_other => panic!("Expected tuple or list for unpacking, found {:?}", _dv_other),
                    }
                }

                // DEPYLER-1137: XML Element-compatible proxy methods
                // These allow DepylerValue to be used as a drop-in replacement for XML elements

                /// DEPYLER-1137: Get tag name (XML element proxy)
                /// Returns empty string for non-element types
                pub fn tag(&self) -> String {
                    match self {
                        DepylerValue::Str(_dv_s) => _dv_s.clone(),
                        _ => String::new(),
                    }
                }

                /// DEPYLER-1137: Get text content (XML element proxy)
                /// Returns None for non-string types
                pub fn text(&self) -> Option<String> {
                    match self {
                        DepylerValue::Str(_dv_s) => Some(_dv_s.clone()),
                        DepylerValue::None => Option::None,
                        _ => Option::None,
                    }
                }

                /// DEPYLER-1137: Find child element by tag (XML element proxy)
                /// Returns DepylerValue::None for non-matching/non-container types
                pub fn find(&self, _tag: &str) -> DepylerValue {
                    match self {
                        DepylerValue::List(_dv_list) => {
                            _dv_list.first().cloned().unwrap_or(DepylerValue::None)
                        }
                        DepylerValue::Dict(_dv_dict) => {
                            _dv_dict.get(&DepylerValue::Str(_tag.to_string()))
                                .cloned()
                                .unwrap_or(DepylerValue::None)
                        }
                        _ => DepylerValue::None,
                    }
                }

                /// DEPYLER-1137: Find all child elements by tag (XML element proxy)
                /// Returns empty Vec for non-container types
                pub fn findall(&self, _tag: &str) -> Vec<DepylerValue> {
                    match self {
                        DepylerValue::List(_dv_list) => _dv_list.clone(),
                        _ => Vec::new(),
                    }
                }

                /// DEPYLER-1137: Set attribute (XML element proxy)
                /// No-op for non-dict types
                pub fn set(&mut self, key: &str, value: &str) {
                    if let DepylerValue::Dict(_dv_dict) = self {
                        _dv_dict.insert(
                            DepylerValue::Str(String::from(key)),
                            DepylerValue::Str(String::from(value))
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
                    // DEPYLER-1040b: Convert &str to DepylerValue for lookup
                    match self {
                        DepylerValue::Dict(_dv_dict) => _dv_dict.get(&DepylerValue::Str(_dv_key.to_string())).unwrap_or(&DepylerValue::None),
                        _ => panic!("Cannot index non-dict DepylerValue with string key"),
                    }
                }
            }

            // DEPYLER-1040b: Index by DepylerValue key (for non-string keys like integers)
            impl std::ops::Index<DepylerValue> for DepylerValue {
                type Output = DepylerValue;
                fn index(&self, _dv_key: DepylerValue) -> &Self::Output {
                    match self {
                        DepylerValue::Dict(_dv_dict) => _dv_dict.get(&_dv_key).unwrap_or(&DepylerValue::None),
                        _ => panic!("Cannot index non-dict DepylerValue"),
                    }
                }
            }

            // DEPYLER-1040b: Index by integer key (common Python pattern: d[1])
            // DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables
            impl std::ops::Index<i64> for DepylerValue {
                type Output = DepylerValue;
                fn index(&self, _dv_key: i64) -> &Self::Output {
                    match self {
                        DepylerValue::Dict(_dv_dict) => _dv_dict.get(&DepylerValue::Int(_dv_key)).unwrap_or(&DepylerValue::None),
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

            // DEPYLER-1051: From<T> implementations for seamless value creation
            // Enables: let x: DepylerValue = 42.into();
            impl From<i64> for DepylerValue {
                fn from(v: i64) -> Self { DepylerValue::Int(v) }
            }
            impl From<i32> for DepylerValue {
                fn from(v: i32) -> Self { DepylerValue::Int(v as i64) }
            }
            impl From<f64> for DepylerValue {
                fn from(v: f64) -> Self { DepylerValue::Float(v) }
            }
            impl From<String> for DepylerValue {
                fn from(v: String) -> Self { DepylerValue::Str(v) }
            }
            impl From<&str> for DepylerValue {
                fn from(v: &str) -> Self { DepylerValue::Str(String::from(v)) }
            }
            impl From<bool> for DepylerValue {
                fn from(v: bool) -> Self { DepylerValue::Bool(v) }
            }
            impl From<Vec<DepylerValue>> for DepylerValue {
                fn from(v: Vec<DepylerValue>) -> Self { DepylerValue::List(v) }
            }
            // DEPYLER-1140: From<Vec<T>> implementations for typed vectors
            // Enables seamless conversion of typed vectors to DepylerValue::List
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
                    DepylerValue::List(v.into_iter().map(|s| DepylerValue::Str(s.to_string())).collect())
                }
            }
            // DEPYLER-1040b: Updated to use DepylerValue keys
            impl From<std::collections::HashMap<DepylerValue, DepylerValue>> for DepylerValue {
                fn from(v: std::collections::HashMap<DepylerValue, DepylerValue>) -> Self { DepylerValue::Dict(v) }
            }
            // DEPYLER-1040b: Backward compatibility for String-keyed HashMaps
            impl From<std::collections::HashMap<String, DepylerValue>> for DepylerValue {
                fn from(v: std::collections::HashMap<String, DepylerValue>) -> Self {
                    let converted: std::collections::HashMap<DepylerValue, DepylerValue> = v
                        .into_iter()
                        .map(|(k, v)| (DepylerValue::Str(k), v))
                        .collect();
                    DepylerValue::Dict(converted)
                }
            }

            // DEPYLER-1160: From<HashSet<T>> and From<Arc<HashSet<T>>> for set/frozenset support
            // Python sets become DepylerValue::List (as both are unordered collections of unique values)
            // frozenset uses Arc for immutability semantics
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

            // Typed HashSet conversions
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

            // DEPYLER-1123: From<DepylerValue> for basic types - enables type extraction from dict values
            // Used when accessing bare dict (HashMap<DepylerValue, DepylerValue>) and need typed value
            impl From<DepylerValue> for i64 {
                fn from(v: DepylerValue) -> Self { v.to_i64() }
            }
            impl From<DepylerValue> for i32 {
                fn from(v: DepylerValue) -> Self { v.to_i64() as i32 }
            }
            impl From<DepylerValue> for f64 {
                fn from(v: DepylerValue) -> Self { v.to_f64() }
            }
            impl From<DepylerValue> for f32 {
                fn from(v: DepylerValue) -> Self { v.to_f64() as f32 }
            }
            impl From<DepylerValue> for String {
                fn from(v: DepylerValue) -> Self { v.as_string() }
            }
            impl From<DepylerValue> for bool {
                fn from(v: DepylerValue) -> Self { v.to_bool() }
            }

            // DEPYLER-1051: Arithmetic operations for DepylerValue
            // Enables: let result = x + y; where x, y are DepylerValue
            // DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables
            impl std::ops::Add for DepylerValue {
                type Output = DepylerValue;
                fn add(self, rhs: Self) -> Self::Output {
                    match (self, rhs) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Int(_dv_a + _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a + _dv_b),
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a as f64 + _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Float(_dv_a + _dv_b as f64),
                        (DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) => DepylerValue::Str(_dv_a + &_dv_b),
                        _ => DepylerValue::None, // Incompatible types
                    }
                }
            }

            impl std::ops::Sub for DepylerValue {
                type Output = DepylerValue;
                fn sub(self, rhs: Self) -> Self::Output {
                    match (self, rhs) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Int(_dv_a - _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a - _dv_b),
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a as f64 - _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Float(_dv_a - _dv_b as f64),
                        _ => DepylerValue::None,
                    }
                }
            }

            impl std::ops::Mul for DepylerValue {
                type Output = DepylerValue;
                fn mul(self, rhs: Self) -> Self::Output {
                    match (self, rhs) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Int(_dv_a * _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a * _dv_b),
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a as f64 * _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Float(_dv_a * _dv_b as f64),
                        _ => DepylerValue::None,
                    }
                }
            }

            impl std::ops::Div for DepylerValue {
                type Output = DepylerValue;
                fn div(self, rhs: Self) -> Self::Output {
                    match (self, rhs) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => DepylerValue::Int(_dv_a / _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 => DepylerValue::Float(_dv_a / _dv_b),
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 => DepylerValue::Float(_dv_a as f64 / _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => DepylerValue::Float(_dv_a / _dv_b as f64),
                        _ => DepylerValue::None,
                    }
                }
            }

            // DEPYLER-1051: Add with concrete types (for mixed operations)
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

            // DEPYLER-1041/1043: Reverse Add implementations (primitive + DepylerValue)
            // Returns the LHS primitive type so `total = total + item` compiles
            // where total is i32 and item is DepylerValue
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

            // DEPYLER-1040b: Sub with concrete types
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

            // DEPYLER-1041/1043: Reverse Sub implementations (primitive - DepylerValue)
            // Returns the LHS primitive type for assignment compatibility
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

            // DEPYLER-1040b: Mul with concrete types
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

            // DEPYLER-1041/1043: Reverse Mul implementations (primitive * DepylerValue)
            // Returns the LHS primitive type for assignment compatibility
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

            // DEPYLER-1040b: Div with concrete types
            impl std::ops::Div<i64> for DepylerValue {
                type Output = DepylerValue;
                fn div(self, rhs: i64) -> Self::Output {
                    if rhs == 0 { return DepylerValue::None; }
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
                    if rhs == 0.0 { return DepylerValue::None; }
                    match self {
                        DepylerValue::Int(_dv_int) => DepylerValue::Float(_dv_int as f64 / rhs),
                        DepylerValue::Float(_dv_float) => DepylerValue::Float(_dv_float / rhs),
                        _ => DepylerValue::None,
                    }
                }
            }

            // DEPYLER-1041/1043: Reverse Div implementations (primitive / DepylerValue)
            // Returns the LHS primitive type for assignment compatibility
            // Division by zero returns 0 (safe default)
            impl std::ops::Div<DepylerValue> for i32 {
                type Output = i32;
                fn div(self, rhs: DepylerValue) -> Self::Output {
                    let divisor = rhs.to_i64() as i32;
                    if divisor == 0 { 0 } else { self / divisor }
                }
            }
            impl std::ops::Div<DepylerValue> for i64 {
                type Output = i64;
                fn div(self, rhs: DepylerValue) -> Self::Output {
                    let divisor = rhs.to_i64();
                    if divisor == 0 { 0 } else { self / divisor }
                }
            }
            impl std::ops::Div<DepylerValue> for f64 {
                type Output = f64;
                fn div(self, rhs: DepylerValue) -> Self::Output {
                    let divisor = rhs.to_f64();
                    if divisor == 0.0 { 0.0 } else { self / divisor }
                }
            }

            // DEPYLER-1040b: Add f64 for completeness
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

            // DEPYLER-1040b: Neg (unary minus) for DepylerValue
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

            // DEPYLER-1040b: Not (logical not) for DepylerValue
            impl std::ops::Not for DepylerValue {
                type Output = bool;
                fn not(self) -> Self::Output {
                    !self.to_bool()
                }
            }

            // DEPYLER-1040b: BitNot (bitwise not) for DepylerValue
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

            // DEPYLER-1046: IntoIterator for DepylerValue to allow `for x in value` syntax
            // Python behavior:
            // - list: iterate over elements
            // - dict: iterate over keys
            // - str: iterate over characters (collected to avoid borrow issues)
            // - other: empty iterator
            impl IntoIterator for DepylerValue {
                type Item = DepylerValue;
                type IntoIter = std::vec::IntoIter<DepylerValue>;

                fn into_iter(self) -> Self::IntoIter {
                    match self {
                        DepylerValue::List(_dv_list) => _dv_list.into_iter(),
                        DepylerValue::Tuple(_dv_tuple) => _dv_tuple.into_iter(),
                        DepylerValue::Dict(_dv_dict) => _dv_dict.into_keys().collect::<Vec<_>>().into_iter(),
                        DepylerValue::Str(_dv_str) => {
                            _dv_str.chars().map(|_dv_c| DepylerValue::Str(_dv_c.to_string())).collect::<Vec<_>>().into_iter()
                        }
                        _ => Vec::new().into_iter(),
                    }
                }
            }

            // DEPYLER-1046: IntoIterator for &DepylerValue (by reference)
            impl<'_dv_a> IntoIterator for &'_dv_a DepylerValue {
                type Item = DepylerValue;
                type IntoIter = std::vec::IntoIter<DepylerValue>;

                fn into_iter(self) -> Self::IntoIter {
                    match self {
                        DepylerValue::List(_dv_list) => _dv_list.iter().cloned().collect::<Vec<_>>().into_iter(),
                        DepylerValue::Tuple(_dv_tuple) => _dv_tuple.iter().cloned().collect::<Vec<_>>().into_iter(),
                        DepylerValue::Dict(_dv_dict) => _dv_dict.keys().cloned().collect::<Vec<_>>().into_iter(),
                        DepylerValue::Str(_dv_str) => {
                            _dv_str.chars().map(|_dv_c| DepylerValue::Str(_dv_c.to_string())).collect::<Vec<_>>().into_iter()
                        }
                        _ => Vec::new().into_iter(),
                    }
                }
            }

            // DEPYLER-1062: PartialOrd for DepylerValue to support min/max builtins
            // Uses total ordering for f64 (NaN sorts as greater than all other values)
            impl std::cmp::PartialOrd for DepylerValue {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    match (self, other) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => Some(_dv_a.cmp(_dv_b)),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => Some(_dv_a.total_cmp(_dv_b)),
                        (DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) => Some(_dv_a.cmp(_dv_b)),
                        (DepylerValue::Bool(_dv_a), DepylerValue::Bool(_dv_b)) => Some(_dv_a.cmp(_dv_b)),
                        // Cross-type comparisons: convert to f64 for numeric, string for others
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => Some((*_dv_a as f64).total_cmp(_dv_b)),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => Some(_dv_a.total_cmp(&(*_dv_b as f64))),
                        // None compares less than everything except None
                        (DepylerValue::None, DepylerValue::None) => Some(std::cmp::Ordering::Equal),
                        (DepylerValue::None, _) => Some(std::cmp::Ordering::Less),
                        (_, DepylerValue::None) => Some(std::cmp::Ordering::Greater),
                        // Collections compare by length then element-wise
                        (DepylerValue::List(_dv_a), DepylerValue::List(_dv_b)) => _dv_a.partial_cmp(_dv_b),
                        (DepylerValue::Tuple(_dv_a), DepylerValue::Tuple(_dv_b)) => _dv_a.partial_cmp(_dv_b),
                        // Incompatible types: return None (not comparable in Python either)
                        _ => Option::None,
                    }
                }
            }

            // DEPYLER-1062: Ord for DepylerValue (required for .min()/.max() on iterators)
            impl std::cmp::Ord for DepylerValue {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
                }
            }

            // DEPYLER-99MODE-E0308-P2: Cross-type comparisons for DepylerValue
            // Enables: if depyler_val > 5 (without explicit conversion)
            // This fixes ~25% of E0308 errors from NASA mode type coercion mismatches
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
            // Reverse direction: allow i32 > depyler_val
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

            // DEPYLER-99MODE-E0308-P2: Cross-type equality for DepylerValue
            impl std::cmp::PartialEq<i32> for DepylerValue {
                fn eq(&self, other: &i32) -> bool { self == &DepylerValue::Int(*other as i64) }
            }
            impl std::cmp::PartialEq<i64> for DepylerValue {
                fn eq(&self, other: &i64) -> bool { self == &DepylerValue::Int(*other) }
            }
            impl std::cmp::PartialEq<f64> for DepylerValue {
                fn eq(&self, other: &f64) -> bool { self == &DepylerValue::Float(*other) }
            }
            impl std::cmp::PartialEq<DepylerValue> for i32 {
                fn eq(&self, other: &DepylerValue) -> bool { &DepylerValue::Int(*self as i64) == other }
            }
            impl std::cmp::PartialEq<DepylerValue> for i64 {
                fn eq(&self, other: &DepylerValue) -> bool { &DepylerValue::Int(*self) == other }
            }
            impl std::cmp::PartialEq<DepylerValue> for f64 {
                fn eq(&self, other: &DepylerValue) -> bool { &DepylerValue::Float(*self) == other }
            }

            // DEPYLER-1062: Safe min helper that handles f64 NaN correctly
            // Python: min(1.0, float('nan')) returns 1.0 (NaN is "ignored")
            pub fn depyler_min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
                if a.partial_cmp(&b).map_or(true, |c| c == std::cmp::Ordering::Less || c == std::cmp::Ordering::Equal) {
                    a
                } else {
                    b
                }
            }

            // DEPYLER-1062: Safe max helper that handles f64 NaN correctly
            // Python: max(1.0, float('nan')) returns 1.0 (NaN is "ignored")
            pub fn depyler_max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
                if a.partial_cmp(&b).map_or(true, |c| c == std::cmp::Ordering::Greater || c == std::cmp::Ordering::Equal) {
                    a
                } else {
                    b
                }
            }

            // DEPYLER-1103: PyTruthy trait for Python truthiness semantics
            // In Python: 0, 0.0, "", [], {}, None, False are falsy, everything else is truthy
            // This trait provides a unified interface for boolean coercion across all types.
            pub trait PyTruthy {
                /// Returns true if the value is "truthy" in Python semantics.
                fn is_true(&self) -> bool;
            }

            impl PyTruthy for bool {
                #[inline]
                fn is_true(&self) -> bool { *self }
            }

            impl PyTruthy for i32 {
                #[inline]
                fn is_true(&self) -> bool { *self != 0 }
            }

            impl PyTruthy for i64 {
                #[inline]
                fn is_true(&self) -> bool { *self != 0 }
            }

            impl PyTruthy for f32 {
                #[inline]
                fn is_true(&self) -> bool { *self != 0.0 }
            }

            impl PyTruthy for f64 {
                #[inline]
                fn is_true(&self) -> bool { *self != 0.0 }
            }

            impl PyTruthy for String {
                #[inline]
                fn is_true(&self) -> bool { !self.is_empty() }
            }

            impl PyTruthy for &str {
                #[inline]
                fn is_true(&self) -> bool { !self.is_empty() }
            }

            impl<T> PyTruthy for Vec<T> {
                #[inline]
                fn is_true(&self) -> bool { !self.is_empty() }
            }

            impl<T> PyTruthy for Option<T> {
                #[inline]
                fn is_true(&self) -> bool { self.is_some() }
            }

            impl<K, V> PyTruthy for std::collections::HashMap<K, V> {
                #[inline]
                fn is_true(&self) -> bool { !self.is_empty() }
            }

            impl<K, V> PyTruthy for std::collections::BTreeMap<K, V> {
                #[inline]
                fn is_true(&self) -> bool { !self.is_empty() }
            }

            impl<T> PyTruthy for std::collections::HashSet<T> {
                #[inline]
                fn is_true(&self) -> bool { !self.is_empty() }
            }

            impl<T> PyTruthy for std::collections::BTreeSet<T> {
                #[inline]
                fn is_true(&self) -> bool { !self.is_empty() }
            }

            impl<T> PyTruthy for std::collections::VecDeque<T> {
                #[inline]
                fn is_true(&self) -> bool { !self.is_empty() }
            }

            impl PyTruthy for DepylerValue {
                /// Python truthiness for DepylerValue:
                /// - Int(0), Float(0.0), Str(""), Bool(false), None -> false
                /// - List([]), Dict({}), Tuple([]) -> false
                /// - Everything else -> true
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

            // DEPYLER-1104: PyAdd trait for Python addition semantics
            // Handles cross-type promotion (int + float = float, str + str = str concat)
            pub trait PyAdd<Rhs = Self> {
                type Output;
                fn py_add(self, rhs: Rhs) -> Self::Output;
            }

            // DEPYLER-1104: PySub trait for Python subtraction semantics
            pub trait PySub<Rhs = Self> {
                type Output;
                fn py_sub(self, rhs: Rhs) -> Self::Output;
            }

            // DEPYLER-1104: PyMul trait for Python multiplication semantics
            // Includes str * int for string repetition
            pub trait PyMul<Rhs = Self> {
                type Output;
                fn py_mul(self, rhs: Rhs) -> Self::Output;
            }

            // DEPYLER-1104: PyDiv trait for Python division semantics
            // Python 3 division always returns float
            pub trait PyDiv<Rhs = Self> {
                type Output;
                fn py_div(self, rhs: Rhs) -> Self::Output;
            }

            // DEPYLER-1109: PyMod trait for Python modulo semantics
            // Handles cross-type modulo (int % float, etc.)
            pub trait PyMod<Rhs = Self> {
                type Output;
                fn py_mod(self, rhs: Rhs) -> Self::Output;
            }

            // DEPYLER-1104: PyIndex trait for Python indexing semantics
            // Handles negative indices (list[-1] = last element)
            pub trait PyIndex<Idx> {
                type Output;
                fn py_index(&self, index: Idx) -> Self::Output;
            }

            // === PyAdd implementations ===

            impl PyAdd for i32 {
                type Output = i32;
                #[inline]
                fn py_add(self, rhs: i32) -> i32 { self + rhs }
            }

            impl PyAdd<i64> for i32 {
                type Output = i64;
                #[inline]
                fn py_add(self, rhs: i64) -> i64 { self as i64 + rhs }
            }

            impl PyAdd<f64> for i32 {
                type Output = f64;
                #[inline]
                fn py_add(self, rhs: f64) -> f64 { self as f64 + rhs }
            }

            impl PyAdd for i64 {
                type Output = i64;
                #[inline]
                fn py_add(self, rhs: i64) -> i64 { self + rhs }
            }

            impl PyAdd<i32> for i64 {
                type Output = i64;
                #[inline]
                fn py_add(self, rhs: i32) -> i64 { self + rhs as i64 }
            }

            impl PyAdd<f64> for i64 {
                type Output = f64;
                #[inline]
                fn py_add(self, rhs: f64) -> f64 { self as f64 + rhs }
            }

            impl PyAdd for f64 {
                type Output = f64;
                #[inline]
                fn py_add(self, rhs: f64) -> f64 { self + rhs }
            }

            impl PyAdd<i32> for f64 {
                type Output = f64;
                #[inline]
                fn py_add(self, rhs: i32) -> f64 { self + rhs as f64 }
            }

            impl PyAdd<i64> for f64 {
                type Output = f64;
                #[inline]
                fn py_add(self, rhs: i64) -> f64 { self + rhs as f64 }
            }

            impl PyAdd for String {
                type Output = String;
                #[inline]
                fn py_add(self, rhs: String) -> String { self + &rhs }
            }

            impl PyAdd<&str> for String {
                type Output = String;
                #[inline]
                fn py_add(self, rhs: &str) -> String { self + rhs }
            }

            // DEPYLER-1118: PyAdd for &str - string concatenation
            impl PyAdd<&str> for &str {
                type Output = String;
                #[inline]
                fn py_add(self, rhs: &str) -> String { format!("{}{}", self, rhs) }
            }

            impl PyAdd<String> for &str {
                type Output = String;
                #[inline]
                fn py_add(self, rhs: String) -> String { format!("{}{}", self, rhs) }
            }

            // DEPYLER-1129: PyAdd<char> for String - appending single characters
            impl PyAdd<char> for String {
                type Output = String;
                #[inline]
                fn py_add(mut self, rhs: char) -> String { self.push(rhs); self }
            }

            impl PyAdd<char> for &str {
                type Output = String;
                #[inline]
                fn py_add(self, rhs: char) -> String { format!("{}{}", self, rhs) }
            }

            impl PyAdd for DepylerValue {
                type Output = DepylerValue;
                fn py_add(self, rhs: DepylerValue) -> DepylerValue {
                    match (self, rhs) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Int(_dv_a + _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a + _dv_b),
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a as f64 + _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Float(_dv_a + _dv_b as f64),
                        (DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) => DepylerValue::Str(_dv_a + &_dv_b),
                        _ => DepylerValue::None,
                    }
                }
            }

            // DEPYLER-1160: PyAdd<DepylerValue> for primitives - universal arithmetic symmetry
            // Enables: let result = count + item; where count is i32/i64/f64 and item is DepylerValue
            impl PyAdd<DepylerValue> for i32 {
                type Output = i64;
                #[inline]
                fn py_add(self, rhs: DepylerValue) -> i64 { self as i64 + rhs.to_i64() }
            }

            impl PyAdd<DepylerValue> for i64 {
                type Output = i64;
                #[inline]
                fn py_add(self, rhs: DepylerValue) -> i64 { self + rhs.to_i64() }
            }

            impl PyAdd<DepylerValue> for f64 {
                type Output = f64;
                #[inline]
                fn py_add(self, rhs: DepylerValue) -> f64 { self + rhs.to_f64() }
            }

            // === PySub implementations ===

            impl PySub for i32 {
                type Output = i32;
                #[inline]
                fn py_sub(self, rhs: i32) -> i32 { self - rhs }
            }

            impl PySub<f64> for i32 {
                type Output = f64;
                #[inline]
                fn py_sub(self, rhs: f64) -> f64 { self as f64 - rhs }
            }

            impl PySub for i64 {
                type Output = i64;
                #[inline]
                fn py_sub(self, rhs: i64) -> i64 { self - rhs }
            }

            impl PySub<f64> for i64 {
                type Output = f64;
                #[inline]
                fn py_sub(self, rhs: f64) -> f64 { self as f64 - rhs }
            }

            impl PySub for f64 {
                type Output = f64;
                #[inline]
                fn py_sub(self, rhs: f64) -> f64 { self - rhs }
            }

            impl PySub<i32> for f64 {
                type Output = f64;
                #[inline]
                fn py_sub(self, rhs: i32) -> f64 { self - rhs as f64 }
            }

            impl PySub<i64> for f64 {
                type Output = f64;
                #[inline]
                fn py_sub(self, rhs: i64) -> f64 { self - rhs as f64 }
            }

            impl PySub for DepylerValue {
                type Output = DepylerValue;
                fn py_sub(self, rhs: DepylerValue) -> DepylerValue {
                    match (self, rhs) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Int(_dv_a - _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a - _dv_b),
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a as f64 - _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Float(_dv_a - _dv_b as f64),
                        _ => DepylerValue::None,
                    }
                }
            }

            // DEPYLER-1160: PySub<DepylerValue> for primitives - universal arithmetic symmetry
            impl PySub<DepylerValue> for i32 {
                type Output = i64;
                #[inline]
                fn py_sub(self, rhs: DepylerValue) -> i64 { self as i64 - rhs.to_i64() }
            }

            impl PySub<DepylerValue> for i64 {
                type Output = i64;
                #[inline]
                fn py_sub(self, rhs: DepylerValue) -> i64 { self - rhs.to_i64() }
            }

            impl PySub<DepylerValue> for f64 {
                type Output = f64;
                #[inline]
                fn py_sub(self, rhs: DepylerValue) -> f64 { self - rhs.to_f64() }
            }

            // DEPYLER-HASHSET-PYSUB: PySub for HashSet - Python set difference (s1 - s2)
            impl<T: Eq + std::hash::Hash + Clone> PySub for std::collections::HashSet<T> {
                type Output = std::collections::HashSet<T>;
                fn py_sub(self, rhs: std::collections::HashSet<T>) -> Self::Output {
                    self.difference(&rhs).cloned().collect()
                }
            }

            impl<T: Eq + std::hash::Hash + Clone> PySub<&std::collections::HashSet<T>> for std::collections::HashSet<T> {
                type Output = std::collections::HashSet<T>;
                fn py_sub(self, rhs: &std::collections::HashSet<T>) -> Self::Output {
                    self.difference(rhs).cloned().collect()
                }
            }

            // === PyMul implementations ===

            impl PyMul for i32 {
                type Output = i32;
                #[inline]
                fn py_mul(self, rhs: i32) -> i32 { self * rhs }
            }

            impl PyMul<f64> for i32 {
                type Output = f64;
                #[inline]
                fn py_mul(self, rhs: f64) -> f64 { self as f64 * rhs }
            }

            // DEPYLER-1160: Cross-type integer multiplication
            impl PyMul<i64> for i32 {
                type Output = i64;
                #[inline]
                fn py_mul(self, rhs: i64) -> i64 { self as i64 * rhs }
            }

            impl PyMul for i64 {
                type Output = i64;
                #[inline]
                fn py_mul(self, rhs: i64) -> i64 { self * rhs }
            }

            impl PyMul<f64> for i64 {
                type Output = f64;
                #[inline]
                fn py_mul(self, rhs: f64) -> f64 { self as f64 * rhs }
            }

            // DEPYLER-1160: Cross-type integer multiplication
            impl PyMul<i32> for i64 {
                type Output = i64;
                #[inline]
                fn py_mul(self, rhs: i32) -> i64 { self * rhs as i64 }
            }

            impl PyMul for f64 {
                type Output = f64;
                #[inline]
                fn py_mul(self, rhs: f64) -> f64 { self * rhs }
            }

            impl PyMul<i32> for f64 {
                type Output = f64;
                #[inline]
                fn py_mul(self, rhs: i32) -> f64 { self * rhs as f64 }
            }

            impl PyMul<i64> for f64 {
                type Output = f64;
                #[inline]
                fn py_mul(self, rhs: i64) -> f64 { self * rhs as f64 }
            }

            // Python str * int = string repetition
            impl PyMul<i32> for String {
                type Output = String;
                fn py_mul(self, rhs: i32) -> String {
                    if rhs <= 0 { String::new() } else { self.repeat(rhs as usize) }
                }
            }

            impl PyMul<i64> for String {
                type Output = String;
                fn py_mul(self, rhs: i64) -> String {
                    if rhs <= 0 { String::new() } else { self.repeat(rhs as usize) }
                }
            }

            // DEPYLER-1118: PyMul for &str - string repetition
            impl PyMul<i32> for &str {
                type Output = String;
                fn py_mul(self, rhs: i32) -> String {
                    if rhs <= 0 { String::new() } else { self.repeat(rhs as usize) }
                }
            }

            impl PyMul<i64> for &str {
                type Output = String;
                fn py_mul(self, rhs: i64) -> String {
                    if rhs <= 0 { String::new() } else { self.repeat(rhs as usize) }
                }
            }

            impl PyMul for DepylerValue {
                type Output = DepylerValue;
                fn py_mul(self, rhs: DepylerValue) -> DepylerValue {
                    match (self, rhs) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Int(_dv_a * _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a * _dv_b),
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) => DepylerValue::Float(_dv_a as f64 * _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) => DepylerValue::Float(_dv_a * _dv_b as f64),
                        (DepylerValue::Str(_dv_s), DepylerValue::Int(_dv_n)) => {
                            if _dv_n <= 0 { DepylerValue::Str(String::new()) } else { DepylerValue::Str(_dv_s.repeat(_dv_n as usize)) }
                        }
                        _ => DepylerValue::None,
                    }
                }
            }

            // DEPYLER-1160: PyMul<DepylerValue> for primitives - universal arithmetic symmetry
            impl PyMul<DepylerValue> for i32 {
                type Output = i64;
                #[inline]
                fn py_mul(self, rhs: DepylerValue) -> i64 { self as i64 * rhs.to_i64() }
            }

            impl PyMul<DepylerValue> for i64 {
                type Output = i64;
                #[inline]
                fn py_mul(self, rhs: DepylerValue) -> i64 { self * rhs.to_i64() }
            }

            impl PyMul<DepylerValue> for f64 {
                type Output = f64;
                #[inline]
                fn py_mul(self, rhs: DepylerValue) -> f64 { self * rhs.to_f64() }
            }

            // DEPYLER-1131: Vec list concatenation - [1,2] + [3,4] = [1,2,3,4]
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

            // DEPYLER-1129: Vec list repetition - [0] * 10 creates vec of 10 zeros
            impl<T: Clone> PyMul<i32> for Vec<T> {
                type Output = Vec<T>;
                fn py_mul(self, rhs: i32) -> Vec<T> {
                    if rhs <= 0 {
                        Vec::new()
                    } else {
                        self.iter().cloned().cycle().take(self.len() * rhs as usize).collect()
                    }
                }
            }

            impl<T: Clone> PyMul<i64> for Vec<T> {
                type Output = Vec<T>;
                fn py_mul(self, rhs: i64) -> Vec<T> {
                    if rhs <= 0 {
                        Vec::new()
                    } else {
                        self.iter().cloned().cycle().take(self.len() * rhs as usize).collect()
                    }
                }
            }

            impl<T: Clone> PyMul<usize> for Vec<T> {
                type Output = Vec<T>;
                fn py_mul(self, rhs: usize) -> Vec<T> {
                    self.iter().cloned().cycle().take(self.len() * rhs).collect()
                }
            }

            // Reverse: 10 * [0] also works in Python
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

            // DEPYLER-1307: Vec element-wise operations for NumPy semantics
            // vec_a - vec_b, vec_a * vec_b, vec_a / vec_b (element-wise)

            // Element-wise subtraction: [1.0, 2.0] - [0.5, 0.5] = [0.5, 1.5]
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

            // Element-wise multiplication: [2.0, 3.0] * [4.0, 5.0] = [8.0, 15.0]
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

            // Element-wise division: [8.0, 15.0] / [2.0, 3.0] = [4.0, 5.0]
            impl PyDiv<Vec<f64>> for Vec<f64> {
                type Output = Vec<f64>;
                fn py_div(self, rhs: Vec<f64>) -> Vec<f64> {
                    self.iter().zip(rhs.iter()).map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b }).collect()
                }
            }

            impl PyDiv<&Vec<f64>> for Vec<f64> {
                type Output = Vec<f64>;
                fn py_div(self, rhs: &Vec<f64>) -> Vec<f64> {
                    self.iter().zip(rhs.iter()).map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b }).collect()
                }
            }

            impl PyDiv<Vec<f64>> for &Vec<f64> {
                type Output = Vec<f64>;
                fn py_div(self, rhs: Vec<f64>) -> Vec<f64> {
                    self.iter().zip(rhs.iter()).map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b }).collect()
                }
            }

            impl PyDiv<&Vec<f64>> for &Vec<f64> {
                type Output = Vec<f64>;
                fn py_div(self, rhs: &Vec<f64>) -> Vec<f64> {
                    self.iter().zip(rhs.iter()).map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b }).collect()
                }
            }

            impl PyDiv<Vec<f32>> for Vec<f32> {
                type Output = Vec<f32>;
                fn py_div(self, rhs: Vec<f32>) -> Vec<f32> {
                    self.iter().zip(rhs.iter()).map(|(a, b)| if *b == 0.0 { f32::NAN } else { a / b }).collect()
                }
            }

            // Vec<i64>/Vec<i32> division returns Vec<f64> (Python 3 semantics)
            impl PyDiv<Vec<i64>> for Vec<i64> {
                type Output = Vec<f64>;
                fn py_div(self, rhs: Vec<i64>) -> Vec<f64> {
                    self.iter().zip(rhs.iter()).map(|(a, b)| if *b == 0 { f64::NAN } else { *a as f64 / *b as f64 }).collect()
                }
            }

            impl PyDiv<Vec<i32>> for Vec<i32> {
                type Output = Vec<f64>;
                fn py_div(self, rhs: Vec<i32>) -> Vec<f64> {
                    self.iter().zip(rhs.iter()).map(|(a, b)| if *b == 0 { f64::NAN } else { *a as f64 / *b as f64 }).collect()
                }
            }

            // Scalar-vector operations for broadcasting: vec * scalar, scalar * vec
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

            // === PyDiv implementations ===
            // Python 3: division always returns float

            impl PyDiv for i32 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: i32) -> f64 {
                    if rhs == 0 { f64::NAN } else { self as f64 / rhs as f64 }
                }
            }

            impl PyDiv<f64> for i32 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: f64) -> f64 {
                    if rhs == 0.0 { f64::NAN } else { self as f64 / rhs }
                }
            }

            impl PyDiv for i64 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: i64) -> f64 {
                    if rhs == 0 { f64::NAN } else { self as f64 / rhs as f64 }
                }
            }

            impl PyDiv<f64> for i64 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: f64) -> f64 {
                    if rhs == 0.0 { f64::NAN } else { self as f64 / rhs }
                }
            }

            impl PyDiv for f64 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: f64) -> f64 {
                    if rhs == 0.0 { f64::NAN } else { self / rhs }
                }
            }

            impl PyDiv<i32> for f64 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: i32) -> f64 {
                    if rhs == 0 { f64::NAN } else { self / rhs as f64 }
                }
            }

            impl PyDiv<i64> for f64 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: i64) -> f64 {
                    if rhs == 0 { f64::NAN } else { self / rhs as f64 }
                }
            }

            impl PyDiv for DepylerValue {
                type Output = DepylerValue;
                fn py_div(self, rhs: DepylerValue) -> DepylerValue {
                    match (self, rhs) {
                        (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => DepylerValue::Float(_dv_a as f64 / _dv_b as f64),
                        (DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 => DepylerValue::Float(_dv_a / _dv_b),
                        (DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 => DepylerValue::Float(_dv_a as f64 / _dv_b),
                        (DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 => DepylerValue::Float(_dv_a / _dv_b as f64),
                        _ => DepylerValue::None,
                    }
                }
            }

            // DEPYLER-1160: PyDiv<DepylerValue> for primitives - universal arithmetic symmetry
            impl PyDiv<DepylerValue> for i32 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: DepylerValue) -> f64 {
                    let divisor = rhs.to_f64();
                    if divisor == 0.0 { f64::NAN } else { self as f64 / divisor }
                }
            }

            impl PyDiv<DepylerValue> for i64 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: DepylerValue) -> f64 {
                    let divisor = rhs.to_f64();
                    if divisor == 0.0 { f64::NAN } else { self as f64 / divisor }
                }
            }

            impl PyDiv<DepylerValue> for f64 {
                type Output = f64;
                #[inline]
                fn py_div(self, rhs: DepylerValue) -> f64 {
                    let divisor = rhs.to_f64();
                    if divisor == 0.0 { f64::NAN } else { self / divisor }
                }
            }

            // === PyMod implementations ===
            // Python modulo uses floored division semantics (result has same sign as divisor)

            impl PyMod for i32 {
                type Output = i32;
                #[inline]
                fn py_mod(self, rhs: i32) -> i32 {
                    if rhs == 0 { 0 } else { ((self % rhs) + rhs) % rhs }
                }
            }

            impl PyMod<f64> for i32 {
                type Output = f64;
                #[inline]
                fn py_mod(self, rhs: f64) -> f64 {
                    if rhs == 0.0 { f64::NAN } else { ((self as f64 % rhs) + rhs) % rhs }
                }
            }

            impl PyMod for i64 {
                type Output = i64;
                #[inline]
                fn py_mod(self, rhs: i64) -> i64 {
                    if rhs == 0 { 0 } else { ((self % rhs) + rhs) % rhs }
                }
            }

            impl PyMod<f64> for i64 {
                type Output = f64;
                #[inline]
                fn py_mod(self, rhs: f64) -> f64 {
                    if rhs == 0.0 { f64::NAN } else { ((self as f64 % rhs) + rhs) % rhs }
                }
            }

            impl PyMod for f64 {
                type Output = f64;
                #[inline]
                fn py_mod(self, rhs: f64) -> f64 {
                    if rhs == 0.0 { f64::NAN } else { ((self % rhs) + rhs) % rhs }
                }
            }

            impl PyMod<i32> for f64 {
                type Output = f64;
                #[inline]
                fn py_mod(self, rhs: i32) -> f64 {
                    if rhs == 0 { f64::NAN } else { ((self % rhs as f64) + rhs as f64) % rhs as f64 }
                }
            }

            impl PyMod<i64> for f64 {
                type Output = f64;
                #[inline]
                fn py_mod(self, rhs: i64) -> f64 {
                    if rhs == 0 { f64::NAN } else { ((self % rhs as f64) + rhs as f64) % rhs as f64 }
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

            // === PyIndex implementations ===
            // Handles negative indices: list[-1] = last element

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
                                _dv_str.chars().nth(_dv_idx as usize)
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
                        DepylerValue::Dict(_dv_dict) => {
                            _dv_dict.get(&DepylerValue::Str(key.to_string())).cloned().unwrap_or(DepylerValue::None)
                        }
                        _ => DepylerValue::None,
                    }
                }
            }

            // DEPYLER-1118: PyStringMethods trait for Python string method parity
            // Maps Python string methods to their Rust equivalents:
            // - str.lower() -> to_lowercase()
            // - str.upper() -> to_uppercase()
            // - str.strip() -> trim()
            // - str.lstrip() -> trim_start()
            // - str.rstrip() -> trim_end()
            // - str.split(sep) -> split(sep)
            // - str.replace(old, new) -> replace(old, new)
            // - str.startswith(prefix) -> starts_with(prefix)
            // - str.endswith(suffix) -> ends_with(suffix)
            // - str.find(sub) -> find(sub) returning Option<usize> or -1
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
                fn lower(&self) -> String { self.to_lowercase() }
                #[inline]
                fn upper(&self) -> String { self.to_uppercase() }
                #[inline]
                fn strip(&self) -> String { self.trim().to_string() }
                #[inline]
                fn lstrip(&self) -> String { self.trim_start().to_string() }
                #[inline]
                fn rstrip(&self) -> String { self.trim_end().to_string() }
                #[inline]
                fn py_split(&self, sep: &str) -> Vec<String> {
                    self.split(sep).map(|s| s.to_string()).collect()
                }
                #[inline]
                fn py_replace(&self, old: &str, new: &str) -> String {
                    self.replace(old, new)
                }
                #[inline]
                fn startswith(&self, prefix: &str) -> bool { self.starts_with(prefix) }
                #[inline]
                fn endswith(&self, suffix: &str) -> bool { self.ends_with(suffix) }
                #[inline]
                fn py_find(&self, sub: &str) -> i64 {
                    self.find(sub).map(|i| i as i64).unwrap_or(-1)
                }
                #[inline]
                fn capitalize(&self) -> String {
                    let mut chars = self.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().chain(chars.flat_map(|c| c.to_lowercase())).collect(),
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
                    self.chars().map(|c| {
                        if c.is_uppercase() { c.to_lowercase().collect::<String>() }
                        else { c.to_uppercase().collect::<String>() }
                    }).collect()
                }
                #[inline]
                fn isalpha(&self) -> bool { !self.is_empty() && self.chars().all(|c| c.is_alphabetic()) }
                #[inline]
                fn isdigit(&self) -> bool { !self.is_empty() && self.chars().all(|c| c.is_ascii_digit()) }
                #[inline]
                fn isalnum(&self) -> bool { !self.is_empty() && self.chars().all(|c| c.is_alphanumeric()) }
                #[inline]
                fn isspace(&self) -> bool { !self.is_empty() && self.chars().all(|c| c.is_whitespace()) }
                #[inline]
                fn islower(&self) -> bool { self.chars().any(|c| c.is_lowercase()) && !self.chars().any(|c| c.is_uppercase()) }
                #[inline]
                fn isupper(&self) -> bool { self.chars().any(|c| c.is_uppercase()) && !self.chars().any(|c| c.is_lowercase()) }
                #[inline]
                fn center(&self, width: usize) -> String {
                    if self.len() >= width { return self.to_string(); }
                    let padding = width - self.len();
                    let left = padding / 2;
                    let right = padding - left;
                    format!("{}{}{}", " ".repeat(left), self, " ".repeat(right))
                }
                #[inline]
                fn ljust(&self, width: usize) -> String {
                    if self.len() >= width { return self.to_string(); }
                    format!("{}{}", self, " ".repeat(width - self.len()))
                }
                #[inline]
                fn rjust(&self, width: usize) -> String {
                    if self.len() >= width { return self.to_string(); }
                    format!("{}{}", " ".repeat(width - self.len()), self)
                }
                #[inline]
                fn zfill(&self, width: usize) -> String {
                    if self.len() >= width { return self.to_string(); }
                    format!("{}{}", "0".repeat(width - self.len()), self)
                }
                #[inline]
                fn count(&self, sub: &str) -> usize { self.matches(sub).count() }
            }

            impl PyStringMethods for String {
                #[inline]
                fn lower(&self) -> String { self.as_str().lower() }
                #[inline]
                fn upper(&self) -> String { self.as_str().upper() }
                #[inline]
                fn strip(&self) -> String { self.as_str().strip() }
                #[inline]
                fn lstrip(&self) -> String { self.as_str().lstrip() }
                #[inline]
                fn rstrip(&self) -> String { self.as_str().rstrip() }
                #[inline]
                fn py_split(&self, sep: &str) -> Vec<String> { self.as_str().py_split(sep) }
                #[inline]
                fn py_replace(&self, old: &str, new: &str) -> String { self.as_str().py_replace(old, new) }
                #[inline]
                fn startswith(&self, prefix: &str) -> bool { self.as_str().startswith(prefix) }
                #[inline]
                fn endswith(&self, suffix: &str) -> bool { self.as_str().endswith(suffix) }
                #[inline]
                fn py_find(&self, sub: &str) -> i64 { self.as_str().py_find(sub) }
                #[inline]
                fn capitalize(&self) -> String { self.as_str().capitalize() }
                #[inline]
                fn title(&self) -> String { self.as_str().title() }
                #[inline]
                fn swapcase(&self) -> String { self.as_str().swapcase() }
                #[inline]
                fn isalpha(&self) -> bool { self.as_str().isalpha() }
                #[inline]
                fn isdigit(&self) -> bool { self.as_str().isdigit() }
                #[inline]
                fn isalnum(&self) -> bool { self.as_str().isalnum() }
                #[inline]
                fn isspace(&self) -> bool { self.as_str().isspace() }
                #[inline]
                fn islower(&self) -> bool { self.as_str().islower() }
                #[inline]
                fn isupper(&self) -> bool { self.as_str().isupper() }
                #[inline]
                fn center(&self, width: usize) -> String { self.as_str().center(width) }
                #[inline]
                fn ljust(&self, width: usize) -> String { self.as_str().ljust(width) }
                #[inline]
                fn rjust(&self, width: usize) -> String { self.as_str().rjust(width) }
                #[inline]
                fn zfill(&self, width: usize) -> String { self.as_str().zfill(width) }
                #[inline]
                fn count(&self, sub: &str) -> usize { self.as_str().count(sub) }
            }

            // DEPYLER-1118: PyStringMethods for DepylerValue
            // Delegates to the inner string when the value is Str, otherwise returns default
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

            // DEPYLER-1118: Additional string-like methods for DepylerValue
            impl DepylerValue {
                /// Check if string contains substring (Python's `in` operator for strings)
                #[inline]
                pub fn contains(&self, sub: &str) -> bool {
                    match self {
                        DepylerValue::Str(_dv_s) => _dv_s.contains(sub),
                        DepylerValue::List(_dv_l) => _dv_l.iter().any(|v| {
                            if let DepylerValue::Str(s) = v { s == sub } else { false }
                        }),
                        _ => false,
                    }
                }
            }
    }

}
