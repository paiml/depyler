#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
pub const PI: f64 = 3.141592653589793;
    pub const EPSILON: f64 = 0.000000001;
    #[derive(Debug, Clone)] pub struct ZeroDivisionError {
    message: String ,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "division by zero: {}", self.message)
}
} impl std::error::Error for ZeroDivisionError {
   
}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
    Self {
    message: message.into()
}
}
}
#[derive(Debug, Clone)] pub struct IndexError {
    message: String ,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "index out of range: {}", self.message)
}
} impl std::error::Error for IndexError {
   
}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"] #[doc = r" DEPYLER-1040b: Now implements Hash + Eq to support non-string dict keys"] #[derive(Debug, Clone, Default)] pub enum DepylerValue {
    Int(i64), Float(f64), Str(String), Bool(bool), #[default] None, List(Vec<DepylerValue>), Dict(std::collections::HashMap<DepylerValue, DepylerValue>), #[doc = r" DEPYLER-1050: Tuple variant for Python tuple support"] Tuple(Vec<DepylerValue>) ,
}
impl PartialEq for DepylerValue {
    fn eq(&self, other: & Self) -> bool {
    match(self, other) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>_dv_a.to_bits() == _dv_b.to_bits() ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::Bool(_dv_a), DepylerValue::Bool(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::None, DepylerValue::None) =>true ,(DepylerValue::List(_dv_a), DepylerValue::List(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::Dict(_dv_a), DepylerValue::Dict(_dv_b)) =>_dv_a == _dv_b ,(DepylerValue::Tuple(_dv_a), DepylerValue::Tuple(_dv_b)) =>_dv_a == _dv_b, _ =>false ,
}
}
}
impl Eq for DepylerValue {
   
}
impl std::hash::Hash for DepylerValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::mem::discriminant(self).hash(state);
    match self {
    DepylerValue::Int(_dv_int) =>_dv_int.hash(state), DepylerValue::Float(_dv_float) =>_dv_float.to_bits().hash(state), DepylerValue::Str(_dv_str) =>_dv_str.hash(state), DepylerValue::Bool(_dv_bool) =>_dv_bool.hash(state), DepylerValue::None =>{
   
}
DepylerValue::List(_dv_list) =>_dv_list.hash(state), DepylerValue::Dict(_) =>{
    0u8.hash(state);
   
}
DepylerValue::Tuple(_dv_tuple) =>_dv_tuple.hash(state) ,
}
}
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, _dv_fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
    DepylerValue::Int(_dv_int) =>write!(_dv_fmt, "{}", _dv_int), DepylerValue::Float(_dv_float) =>write!(_dv_fmt, "{}", _dv_float), DepylerValue::Str(_dv_str) =>write!(_dv_fmt, "{}", _dv_str), DepylerValue::Bool(_dv_bool) =>write!(_dv_fmt, "{}", _dv_bool), DepylerValue::None =>write!(_dv_fmt, "None"), DepylerValue::List(_dv_list) =>write!(_dv_fmt, "{:?}", _dv_list), DepylerValue::Dict(_dv_dict) =>write!(_dv_fmt, "{:?}", _dv_dict), DepylerValue::Tuple(_dv_tuple) =>write!(_dv_fmt, "{:?}", _dv_tuple) ,
}
}
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"] #[doc = r" DEPYLER-1060: Use _dv_ prefix to avoid shadowing user variables"] pub fn len(&self) -> usize {
    match self {
    DepylerValue::Str(_dv_str) =>_dv_str.len(), DepylerValue::List(_dv_list) =>_dv_list.len(), DepylerValue::Dict(_dv_dict) =>_dv_dict.len(), DepylerValue::Tuple(_dv_tuple) =>_dv_tuple.len(), _ =>0 ,
}
} #[doc = r" Check if empty"] pub fn is_empty(&self) -> bool {
    self.len() == 0
}
#[doc = r" Get chars iterator for string values"] pub fn chars(&self) -> std::str::Chars<'_>{
    match self {
    DepylerValue::Str(_dv_str) =>_dv_str.chars(), _ =>"".chars() ,
}
} #[doc = r" Insert into dict(mutates self if Dict variant)"] #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys for non-string dict keys"] pub fn insert(&mut self, key: impl Into<DepylerValue>, value: impl Into<DepylerValue>) {
    if let DepylerValue::Dict(_dv_dict) = self {
    _dv_dict.insert(key.into(), value.into());
   
}
} #[doc = r" Get value from dict by key"] #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys"] pub fn get(&self, key: & DepylerValue) -> Option<& DepylerValue>{
    if let DepylerValue::Dict(_dv_dict) = self {
    _dv_dict.get(key)
}
else {
    Option::None
}
} #[doc = r" Get value from dict by string key(convenience method)"] pub fn get_str(&self, key: & str) -> Option<& DepylerValue>{
    self.get(& DepylerValue::Str(key.to_string()))
}
#[doc = r" Check if dict contains key"] #[doc = r" DEPYLER-1040b: Now accepts DepylerValue keys"] pub fn contains_key(&self, key: & DepylerValue) -> bool {
    if let DepylerValue::Dict(_dv_dict) = self {
    _dv_dict.contains_key(key)
}
else {
    false
}
} #[doc = r" Check if dict contains string key(convenience method)"] pub fn contains_key_str(&self, key: & str) -> bool {
    self.contains_key(& DepylerValue::Str(key.to_string()))
}
#[doc = r" DEPYLER-1051: Get iterator over list values"] #[doc = r" Returns an empty iterator for non-list types"] pub fn iter(&self) -> std::slice::Iter<'_, DepylerValue>{
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.iter(), _ =>[].iter() ,
}
} #[doc = r" DEPYLER-1051: Get mutable iterator over list values"] pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, DepylerValue>{
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.iter_mut(), _ =>[].iter_mut() ,
}
} #[doc = r" DEPYLER-1051: Get iterator over dict key-value pairs"] #[doc = r" DEPYLER-1040b: Now uses DepylerValue keys"] pub fn items(&self) -> std::collections::hash_map::Iter<'_, DepylerValue, DepylerValue>{
    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>>= std::sync::LazyLock::new(|| std::collections::HashMap::new());
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.iter(), _ =>EMPTY_MAP.iter() ,
}
} #[doc = r" DEPYLER-1051: Get iterator over dict keys"] #[doc = r" DEPYLER-1040b: Now returns DepylerValue keys"] pub fn keys(&self) -> std::collections::hash_map::Keys<'_, DepylerValue, DepylerValue>{
    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>>= std::sync::LazyLock::new(|| std::collections::HashMap::new());
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.keys(), _ =>EMPTY_MAP.keys() ,
}
} #[doc = r" DEPYLER-1051: Get iterator over dict values"] #[doc = r" DEPYLER-1040b: Now uses DepylerValue keys internally"] pub fn values(&self) -> std::collections::hash_map::Values<'_, DepylerValue, DepylerValue>{
    static EMPTY_MAP: std::sync::LazyLock<std::collections::HashMap<DepylerValue, DepylerValue>>= std::sync::LazyLock::new(|| std::collections::HashMap::new());
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.values(), _ =>EMPTY_MAP.values() ,
}
} #[doc = r" Convert to String(renamed to avoid shadowing Display::to_string)"] #[doc = r" DEPYLER-1121: Renamed from to_string to as_string to fix clippy::inherent_to_string_shadow_display"] pub fn as_string(&self) -> String {
    match self {
    DepylerValue::Str(_dv_str) =>_dv_str.clone(), DepylerValue::Int(_dv_int) =>_dv_int.to_string(), DepylerValue::Float(_dv_float) =>_dv_float.to_string(), DepylerValue::Bool(_dv_bool) =>_dv_bool.to_string(), DepylerValue::None =>"None".to_string(), DepylerValue::List(_dv_list) =>format!("{:?}", _dv_list), DepylerValue::Dict(_dv_dict) =>format!("{:?}", _dv_dict), DepylerValue::Tuple(_dv_tuple) =>format!("{:?}", _dv_tuple) ,
}
} #[doc = r" DEPYLER-1215: Get as str reference(for string values only)"] pub fn as_str(&self) -> Option<& str>{
    match self {
    DepylerValue::Str(_dv_str) =>Some(_dv_str.as_str()), _ =>None ,
}
} #[doc = r" DEPYLER-1215: Get as i64(for integer values)"] pub fn as_i64(&self) -> Option<i64>{
    match self {
    DepylerValue::Int(_dv_int) =>Some(*_dv_int), _ =>None ,
}
} #[doc = r" DEPYLER-1215: Get as f64(for float values)"] pub fn as_f64(&self) -> Option<f64>{
    match self {
    DepylerValue::Float(_dv_float) =>Some(*_dv_float), DepylerValue::Int(_dv_int) =>Some(*_dv_int as f64), _ =>None ,
}
} #[doc = r" DEPYLER-1215: Get as bool(for boolean values)"] pub fn as_bool(&self) -> Option<bool>{
    match self {
    DepylerValue::Bool(_dv_bool) =>Some(*_dv_bool), _ =>None ,
}
} #[doc = r" Convert to i64"] pub fn to_i64(&self) -> i64 {
    match self {
    DepylerValue::Int(_dv_int) =>* _dv_int, DepylerValue::Float(_dv_float) =>* _dv_float as i64, DepylerValue::Bool(_dv_bool) =>if * _dv_bool {
    1
}
else {
    0
}
, DepylerValue::Str(_dv_str) =>_dv_str.parse().unwrap_or(0), _ =>0 ,
}
} #[doc = r" Convert to f64"] pub fn to_f64(&self) -> f64 {
    match self {
    DepylerValue::Float(_dv_float) =>* _dv_float, DepylerValue::Int(_dv_int) =>* _dv_int as f64, DepylerValue::Bool(_dv_bool) =>if * _dv_bool {
    1.0
}
else {
    0.0
}
, DepylerValue::Str(_dv_str) =>_dv_str.parse().unwrap_or(0.0), _ =>0.0 ,
}
} #[doc = r" Convert to bool"] pub fn to_bool(&self) -> bool {
    match self {
    DepylerValue::Bool(_dv_bool) =>* _dv_bool, DepylerValue::Int(_dv_int) =>* _dv_int != 0, DepylerValue::Float(_dv_float) =>* _dv_float != 0.0, DepylerValue::Str(_dv_str) =>! _dv_str.is_empty(), DepylerValue::List(_dv_list) =>! _dv_list.is_empty(), DepylerValue::Dict(_dv_dict) =>! _dv_dict.is_empty(), DepylerValue::Tuple(_dv_tuple) =>! _dv_tuple.is_empty(), DepylerValue::None =>false ,
}
} #[doc = r" DEPYLER-1064: Get tuple element by index for tuple unpacking"] #[doc = r" Returns the element at the given index, or panics with a readable error"] #[doc = r" Works on both Tuple and List variants(Python treats them similarly for unpacking)"] pub fn get_tuple_elem(&self, _dv_idx: usize) -> DepylerValue {
    match self {
    DepylerValue::Tuple(_dv_tuple) =>{
    if _dv_idx<_dv_tuple.len() {
    _dv_tuple [_dv_idx].clone()
}
else {
    panic!("Tuple index {} out of bounds(length {})", _dv_idx, _dv_tuple.len())
}
} DepylerValue::List(_dv_list) =>{
    if _dv_idx<_dv_list.len() {
    _dv_list [_dv_idx].clone()
}
else {
    panic!("List index {} out of bounds(length {})", _dv_idx, _dv_list.len())
}
} _dv_other =>panic!("Expected tuple or list for unpacking, found {:?}", _dv_other) ,
}
} #[doc = r" DEPYLER-1064: Extract tuple as Vec for multiple assignment"] #[doc = r" Validates that the value is a tuple/list with the expected number of elements"] pub fn extract_tuple(&self, _dv_expected_len: usize) -> Vec<DepylerValue>{
    match self {
    DepylerValue::Tuple(_dv_tuple) =>{
    if _dv_tuple.len() != _dv_expected_len {
    panic!("Expected tuple of length {}, got length {}", _dv_expected_len, _dv_tuple.len())
}
_dv_tuple.clone()
}
DepylerValue::List(_dv_list) =>{
    if _dv_list.len() != _dv_expected_len {
    panic!("Expected list of length {}, got length {}", _dv_expected_len, _dv_list.len())
}
_dv_list.clone()
}
_dv_other =>panic!("Expected tuple or list for unpacking, found {:?}", _dv_other) ,
}
} #[doc = r" DEPYLER-1137: Get tag name(XML element proxy)"] #[doc = r" Returns empty string for non-element types"] pub fn tag(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.clone(), _ =>String::new() ,
}
} #[doc = r" DEPYLER-1137: Get text content(XML element proxy)"] #[doc = r" Returns None for non-string types"] pub fn text(&self) -> Option<String>{
    match self {
    DepylerValue::Str(_dv_s) =>Some(_dv_s.clone()), DepylerValue::None =>Option::None, _ =>Option::None ,
}
} #[doc = r" DEPYLER-1137: Find child element by tag(XML element proxy)"] #[doc = r" Returns DepylerValue::None for non-matching/non-container types"] pub fn find(&self, _tag: & str) -> DepylerValue {
    match self {
    DepylerValue::List(_dv_list) =>{
    _dv_list.first().cloned().unwrap_or(DepylerValue::None)
}
DepylerValue::Dict(_dv_dict) =>{
    _dv_dict.get(& DepylerValue::Str(_tag.to_string())).cloned().unwrap_or(DepylerValue::None)
}
_ =>DepylerValue::None ,
}
} #[doc = r" DEPYLER-1137: Find all child elements by tag(XML element proxy)"] #[doc = r" Returns empty Vec for non-container types"] pub fn findall(&self, _tag: & str) -> Vec<DepylerValue>{
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.clone(), _ =>Vec::new() ,
}
} #[doc = r" DEPYLER-1137: Set attribute(XML element proxy)"] #[doc = r" No-op for non-dict types"] pub fn set(&mut self, key: & str, value: & str) {
    if let DepylerValue::Dict(_dv_dict) = self {
    _dv_dict.insert(DepylerValue::Str(String::from(key)), DepylerValue::Str(String::from(value)));
   
}
}
}
impl std::ops::Index<usize>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_idx: usize) -> & Self::Output {
    match self {
    DepylerValue::List(_dv_list) =>& _dv_list [_dv_idx], DepylerValue::Tuple(_dv_tuple) =>& _dv_tuple [_dv_idx], _ =>panic!("Cannot index non-list/tuple DepylerValue") ,
}
}
}
impl std::ops::Index<& str>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: & str) -> & Self::Output {
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.get(& DepylerValue::Str(_dv_key.to_string())).unwrap_or(& DepylerValue::None), _ =>panic!("Cannot index non-dict DepylerValue with string key") ,
}
}
}
impl std::ops::Index<DepylerValue>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: DepylerValue) -> & Self::Output {
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.get(& _dv_key).unwrap_or(& DepylerValue::None), _ =>panic!("Cannot index non-dict DepylerValue") ,
}
}
}
impl std::ops::Index<i64>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: i64) -> & Self::Output {
    match self {
    DepylerValue::Dict(_dv_dict) =>_dv_dict.get(& DepylerValue::Int(_dv_key)).unwrap_or(& DepylerValue::None), DepylerValue::List(_dv_list) =>& _dv_list [_dv_key as usize], DepylerValue::Tuple(_dv_tuple) =>& _dv_tuple [_dv_key as usize], _ =>panic!("Cannot index DepylerValue with integer") ,
}
}
}
impl std::ops::Index<i32>for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, _dv_key: i32) -> & Self::Output {
    &self [_dv_key as i64]
}
} impl From<i64>for DepylerValue {
    fn from(v: i64) -> Self {
    DepylerValue::Int(v)
}
} impl From<i32>for DepylerValue {
    fn from(v: i32) -> Self {
    DepylerValue::Int(v as i64)
}
} impl From<f64>for DepylerValue {
    fn from(v: f64) -> Self {
    DepylerValue::Float(v)
}
} impl From<String>for DepylerValue {
    fn from(v: String) -> Self {
    DepylerValue::Str(v)
}
} impl From<& str>for DepylerValue {
    fn from(v: & str) -> Self {
    DepylerValue::Str(String::from(v))
}
} impl From<bool>for DepylerValue {
    fn from(v: bool) -> Self {
    DepylerValue::Bool(v)
}
} impl From<Vec<DepylerValue>>for DepylerValue {
    fn from(v: Vec<DepylerValue>) -> Self {
    DepylerValue::List(v)
}
} impl From<Vec<String>>for DepylerValue {
    fn from(v: Vec<String>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Str).collect())
}
} impl From<Vec<i32>>for DepylerValue {
    fn from(v: Vec<i32>) -> Self {
    DepylerValue::List(v.into_iter().map(| x | DepylerValue::Int(x as i64)).collect())
}
} impl From<Vec<i64>>for DepylerValue {
    fn from(v: Vec<i64>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Int).collect())
}
} impl From<Vec<f64>>for DepylerValue {
    fn from(v: Vec<f64>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Float).collect())
}
} impl From<Vec<bool>>for DepylerValue {
    fn from(v: Vec<bool>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Bool).collect())
}
} impl From<Vec<& str>>for DepylerValue {
    fn from(v: Vec<& str>) -> Self {
    DepylerValue::List(v.into_iter().map(| s | DepylerValue::Str(s.to_string())).collect())
}
} impl From<std::collections::HashMap<DepylerValue, DepylerValue>>for DepylerValue {
    fn from(v: std::collections::HashMap<DepylerValue, DepylerValue>) -> Self {
    DepylerValue::Dict(v)
}
} impl From<std::collections::HashMap<String, DepylerValue>>for DepylerValue {
    fn from(v: std::collections::HashMap<String, DepylerValue>) -> Self {
    let converted: std::collections::HashMap<DepylerValue, DepylerValue>= v.into_iter().map(|(k, v) |(DepylerValue::Str(k), v)).collect();
    DepylerValue::Dict(converted)
}
} impl From<std::collections::HashSet<DepylerValue>>for DepylerValue {
    fn from(v: std::collections::HashSet<DepylerValue>) -> Self {
    DepylerValue::List(v.into_iter().collect())
}
} impl From<std::sync::Arc<std::collections::HashSet<DepylerValue>>>for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<DepylerValue>>) -> Self {
    DepylerValue::List(v.iter().cloned().collect())
}
} impl From<std::collections::HashSet<i32>>for DepylerValue {
    fn from(v: std::collections::HashSet<i32>) -> Self {
    DepylerValue::List(v.into_iter().map(| x | DepylerValue::Int(x as i64)).collect())
}
} impl From<std::collections::HashSet<i64>>for DepylerValue {
    fn from(v: std::collections::HashSet<i64>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Int).collect())
}
} impl From<std::collections::HashSet<String>>for DepylerValue {
    fn from(v: std::collections::HashSet<String>) -> Self {
    DepylerValue::List(v.into_iter().map(DepylerValue::Str).collect())
}
} impl From<std::sync::Arc<std::collections::HashSet<i32>>>for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<i32>>) -> Self {
    DepylerValue::List(v.iter().map(| x | DepylerValue::Int(*x as i64)).collect())
}
} impl From<std::sync::Arc<std::collections::HashSet<i64>>>for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<i64>>) -> Self {
    DepylerValue::List(v.iter().map(| x | DepylerValue::Int(*x)).collect())
}
} impl From<std::sync::Arc<std::collections::HashSet<String>>>for DepylerValue {
    fn from(v: std::sync::Arc<std::collections::HashSet<String>>) -> Self {
    DepylerValue::List(v.iter().map(| s | DepylerValue::Str(s.clone())).collect())
}
} impl From<DepylerValue>for i64 {
    fn from(v: DepylerValue) -> Self {
    v.to_i64()
}
} impl From<DepylerValue>for i32 {
    fn from(v: DepylerValue) -> Self {
    v.to_i64() as i32
}
} impl From<DepylerValue>for f64 {
    fn from(v: DepylerValue) -> Self {
    v.to_f64()
}
} impl From<DepylerValue>for f32 {
    fn from(v: DepylerValue) -> Self {
    v.to_f64() as f32
}
} impl From<DepylerValue>for String {
    fn from(v: DepylerValue) -> Self {
    v.as_string()
}
} impl From<DepylerValue>for bool {
    fn from(v: DepylerValue) -> Self {
    v.to_bool()
}
} impl std::ops::Add for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: Self) -> Self::Output {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a + _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a + _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 + _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a + _dv_b as f64) ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>DepylerValue::Str(_dv_a + & _dv_b), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Sub for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: Self) -> Self::Output {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a - _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a - _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 - _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a - _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Mul for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: Self) -> Self::Output {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a * _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a * _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 * _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a * _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Div for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: Self) -> Self::Output {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>DepylerValue::Int(_dv_a / _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>DepylerValue::Float(_dv_a / _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>DepylerValue::Float(_dv_a as f64 / _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>DepylerValue::Float(_dv_a / _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Add<i64>for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int + rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float + rhs as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Add<i32>for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: i32) -> Self::Output {
    self +(rhs as i64)
}
} impl std::ops::Add<DepylerValue>for i32 {
    type Output = i32;
    fn add(self, rhs: DepylerValue) -> Self::Output {
    self + rhs.to_i64() as i32
}
} impl std::ops::Add<DepylerValue>for i64 {
    type Output = i64;
    fn add(self, rhs: DepylerValue) -> Self::Output {
    self + rhs.to_i64()
}
} impl std::ops::Add<DepylerValue>for f64 {
    type Output = f64;
    fn add(self, rhs: DepylerValue) -> Self::Output {
    self + rhs.to_f64()
}
} impl std::ops::Sub<i64>for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int - rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float - rhs as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Sub<i32>for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: i32) -> Self::Output {
    self -(rhs as i64)
}
} impl std::ops::Sub<f64>for DepylerValue {
    type Output = DepylerValue;
    fn sub(self, rhs: f64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Float(_dv_int as f64 - rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float - rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Sub<DepylerValue>for i32 {
    type Output = i32;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
    self - rhs.to_i64() as i32
}
} impl std::ops::Sub<DepylerValue>for i64 {
    type Output = i64;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
    self - rhs.to_i64()
}
} impl std::ops::Sub<DepylerValue>for f64 {
    type Output = f64;
    fn sub(self, rhs: DepylerValue) -> Self::Output {
    self - rhs.to_f64()
}
} impl std::ops::Mul<i64>for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int * rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float * rhs as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Mul<i32>for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: i32) -> Self::Output {
    self *(rhs as i64)
}
} impl std::ops::Mul<f64>for DepylerValue {
    type Output = DepylerValue;
    fn mul(self, rhs: f64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Float(_dv_int as f64 * rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float * rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Mul<DepylerValue>for i32 {
    type Output = i32;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
    self * rhs.to_i64() as i32
}
} impl std::ops::Mul<DepylerValue>for i64 {
    type Output = i64;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
    self * rhs.to_i64()
}
} impl std::ops::Mul<DepylerValue>for f64 {
    type Output = f64;
    fn mul(self, rhs: DepylerValue) -> Self::Output {
    self * rhs.to_f64()
}
} impl std::ops::Div<i64>for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: i64) -> Self::Output {
    if rhs == 0 {
    return DepylerValue::None;
   
}
match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int / rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float / rhs as f64), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Div<i32>for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: i32) -> Self::Output {
    self /(rhs as i64)
}
} impl std::ops::Div<f64>for DepylerValue {
    type Output = DepylerValue;
    fn div(self, rhs: f64) -> Self::Output {
    if rhs == 0.0 {
    return DepylerValue::None;
   
}
match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Float(_dv_int as f64 / rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float / rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Div<DepylerValue>for i32 {
    type Output = i32;
    fn div(self, rhs: DepylerValue) -> Self::Output {
    let divisor = rhs.to_i64() as i32;
    if divisor == 0 {
    0
}
else {
    self / divisor
}
}
}
impl std::ops::Div<DepylerValue>for i64 {
    type Output = i64;
    fn div(self, rhs: DepylerValue) -> Self::Output {
    let divisor = rhs.to_i64();
    if divisor == 0 {
    0
}
else {
    self / divisor
}
}
}
impl std::ops::Div<DepylerValue>for f64 {
    type Output = f64;
    fn div(self, rhs: DepylerValue) -> Self::Output {
    let divisor = rhs.to_f64();
    if divisor == 0.0 {
    0.0
}
else {
    self / divisor
}
}
}
impl std::ops::Add<f64>for DepylerValue {
    type Output = DepylerValue;
    fn add(self, rhs: f64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Float(_dv_int as f64 + rhs), DepylerValue::Float(_dv_float) =>DepylerValue::Float(_dv_float + rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Neg for DepylerValue {
    type Output = DepylerValue;
    fn neg(self) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(- _dv_int), DepylerValue::Float(_dv_float) =>DepylerValue::Float(- _dv_float), _ =>DepylerValue::None ,
}
}
}
impl std::ops::Not for DepylerValue {
    type Output = bool;
    fn not(self) -> Self::Output {
    ! self.to_bool()
}
} impl std::ops::BitXor<i64>for DepylerValue {
    type Output = DepylerValue;
    fn bitxor(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int ^ rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::BitAnd<i64>for DepylerValue {
    type Output = DepylerValue;
    fn bitand(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int & rhs), _ =>DepylerValue::None ,
}
}
}
impl std::ops::BitOr<i64>for DepylerValue {
    type Output = DepylerValue;
    fn bitor(self, rhs: i64) -> Self::Output {
    match self {
    DepylerValue::Int(_dv_int) =>DepylerValue::Int(_dv_int | rhs), _ =>DepylerValue::None ,
}
}
}
impl IntoIterator for DepylerValue {
    type Item = DepylerValue;
    type IntoIter = std::vec::IntoIter<DepylerValue>;
    fn into_iter(self) -> Self::IntoIter {
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.into_iter(), DepylerValue::Tuple(_dv_tuple) =>_dv_tuple.into_iter(), DepylerValue::Dict(_dv_dict) =>_dv_dict.into_keys().collect::<Vec<_>>().into_iter(), DepylerValue::Str(_dv_str) =>{
    _dv_str.chars().map(| _dv_c | DepylerValue::Str(_dv_c.to_string())).collect::<Vec<_>>().into_iter()
}
_ =>Vec::new().into_iter() ,
}
}
}
impl<'_dv_a>IntoIterator for & '_dv_a DepylerValue {
    type Item = DepylerValue;
    type IntoIter = std::vec::IntoIter<DepylerValue>;
    fn into_iter(self) -> Self::IntoIter {
    match self {
    DepylerValue::List(_dv_list) =>_dv_list.iter().cloned().collect::<Vec<_>>().into_iter(), DepylerValue::Tuple(_dv_tuple) =>_dv_tuple.iter().cloned().collect::<Vec<_>>().into_iter(), DepylerValue::Dict(_dv_dict) =>_dv_dict.keys().cloned().collect::<Vec<_>>().into_iter(), DepylerValue::Str(_dv_str) =>{
    _dv_str.chars().map(| _dv_c | DepylerValue::Str(_dv_c.to_string())).collect::<Vec<_>>().into_iter()
}
_ =>Vec::new().into_iter() ,
}
}
}
impl std::cmp::PartialOrd for DepylerValue {
    fn partial_cmp(&self, other: & Self) -> Option<std::cmp::Ordering>{
    match(self, other) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>Some(_dv_a.total_cmp(_dv_b)) ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Bool(_dv_a), DepylerValue::Bool(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>Some((*_dv_a as f64).total_cmp(_dv_b)) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>Some(_dv_a.total_cmp(&(*_dv_b as f64))) ,(DepylerValue::None, DepylerValue::None) =>Some(std::cmp::Ordering::Equal) ,(DepylerValue::None, _) =>Some(std::cmp::Ordering::Less) ,(_, DepylerValue::None) =>Some(std::cmp::Ordering::Greater) ,(DepylerValue::List(_dv_a), DepylerValue::List(_dv_b)) =>_dv_a.partial_cmp(_dv_b) ,(DepylerValue::Tuple(_dv_a), DepylerValue::Tuple(_dv_b)) =>_dv_a.partial_cmp(_dv_b), _ =>Option::None ,
}
}
}
impl std::cmp::Ord for DepylerValue {
    fn cmp(&self, other: & Self) -> std::cmp::Ordering {
    self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
}
} impl std::cmp::PartialOrd<i32>for DepylerValue {
    fn partial_cmp(&self, other: & i32) -> Option<std::cmp::Ordering>{
    self.partial_cmp(& DepylerValue::Int(*other as i64))
}
} impl std::cmp::PartialOrd<i64>for DepylerValue {
    fn partial_cmp(&self, other: & i64) -> Option<std::cmp::Ordering>{
    self.partial_cmp(& DepylerValue::Int(*other))
}
} impl std::cmp::PartialOrd<f64>for DepylerValue {
    fn partial_cmp(&self, other: & f64) -> Option<std::cmp::Ordering>{
    self.partial_cmp(& DepylerValue::Float(*other))
}
} impl std::cmp::PartialOrd<DepylerValue>for i32 {
    fn partial_cmp(&self, other: & DepylerValue) -> Option<std::cmp::Ordering>{
    DepylerValue::Int(*self as i64).partial_cmp(other)
}
} impl std::cmp::PartialOrd<DepylerValue>for i64 {
    fn partial_cmp(&self, other: & DepylerValue) -> Option<std::cmp::Ordering>{
    DepylerValue::Int(*self).partial_cmp(other)
}
} impl std::cmp::PartialOrd<DepylerValue>for f64 {
    fn partial_cmp(&self, other: & DepylerValue) -> Option<std::cmp::Ordering>{
    DepylerValue::Float(*self).partial_cmp(other)
}
} impl std::cmp::PartialEq<i32>for DepylerValue {
    fn eq(&self, other: & i32) -> bool {
    self == & DepylerValue::Int(*other as i64)
}
} impl std::cmp::PartialEq<i64>for DepylerValue {
    fn eq(&self, other: & i64) -> bool {
    self == & DepylerValue::Int(*other)
}
} impl std::cmp::PartialEq<f64>for DepylerValue {
    fn eq(&self, other: & f64) -> bool {
    self == & DepylerValue::Float(*other)
}
} impl std::cmp::PartialEq<DepylerValue>for i32 {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Int(*self as i64) == other
}
} impl std::cmp::PartialEq<DepylerValue>for i64 {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Int(*self) == other
}
} impl std::cmp::PartialEq<DepylerValue>for f64 {
    fn eq(&self, other: & DepylerValue) -> bool {
    & DepylerValue::Float(*self) == other
}
} pub fn depyler_min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a.partial_cmp(& b).map_or(true, | c | c == std::cmp::Ordering::Less || c == std::cmp::Ordering::Equal) {
    a
}
else {
    b
}
} pub fn depyler_max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a.partial_cmp(& b).map_or(true, | c | c == std::cmp::Ordering::Greater || c == std::cmp::Ordering::Equal) {
    a
}
else {
    b
}
} pub trait PyTruthy {
    #[doc = r#" Returns true if the value is "truthy" in Python semantics."#] fn is_true(&self) -> bool;
   
}
impl PyTruthy for bool {
    #[inline] fn is_true(&self) -> bool {
    * self
}
} impl PyTruthy for i32 {
    #[inline] fn is_true(&self) -> bool {
    * self != 0
}
} impl PyTruthy for i64 {
    #[inline] fn is_true(&self) -> bool {
    * self != 0
}
} impl PyTruthy for f32 {
    #[inline] fn is_true(&self) -> bool {
    * self != 0.0
}
} impl PyTruthy for f64 {
    #[inline] fn is_true(&self) -> bool {
    * self != 0.0
}
} impl PyTruthy for String {
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl PyTruthy for & str {
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for Vec<T>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for Option<T>{
    #[inline] fn is_true(&self) -> bool {
    self.is_some()
}
} impl<K, V>PyTruthy for std::collections::HashMap<K, V>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<K, V>PyTruthy for std::collections::BTreeMap<K, V>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for std::collections::HashSet<T>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for std::collections::BTreeSet<T>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl<T>PyTruthy for std::collections::VecDeque<T>{
    #[inline] fn is_true(&self) -> bool {
    ! self.is_empty()
}
} impl PyTruthy for DepylerValue {
    #[doc = r" Python truthiness for DepylerValue:"] #[doc = r#" - Int(0), Float(0.0), Str(""), Bool(false), None -> false"#] #[doc = r" - List([]), Dict({}), Tuple([]) -> false"] #[doc = r" - Everything else -> true"] #[inline] fn is_true(&self) -> bool {
    match self {
    DepylerValue::Bool(_dv_b) =>* _dv_b, DepylerValue::Int(_dv_i) =>* _dv_i != 0, DepylerValue::Float(_dv_f) =>* _dv_f != 0.0, DepylerValue::Str(_dv_s) =>! _dv_s.is_empty(), DepylerValue::List(_dv_l) =>! _dv_l.is_empty(), DepylerValue::Dict(_dv_d) =>! _dv_d.is_empty(), DepylerValue::Tuple(_dv_t) =>! _dv_t.is_empty(), DepylerValue::None =>false ,
}
}
}
pub trait PyAdd<Rhs = Self>{
    type Output;
    fn py_add(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PySub<Rhs = Self>{
    type Output;
    fn py_sub(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PyMul<Rhs = Self>{
    type Output;
    fn py_mul(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PyDiv<Rhs = Self>{
    type Output;
    fn py_div(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PyMod<Rhs = Self>{
    type Output;
    fn py_mod(self, rhs: Rhs) -> Self::Output;
   
}
pub trait PyIndex<Idx>{
    type Output;
    fn py_index(&self, index: Idx) -> Self::Output;
   
}
impl PyAdd for i32 {
    type Output = i32;
    #[inline] fn py_add(self, rhs: i32) -> i32 {
    self + rhs
}
} impl PyAdd<i64>for i32 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: i64) -> i64 {
    self as i64 + rhs
}
} impl PyAdd<f64>for i32 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: f64) -> f64 {
    self as f64 + rhs
}
} impl PyAdd for i64 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: i64) -> i64 {
    self + rhs
}
} impl PyAdd<i32>for i64 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: i32) -> i64 {
    self + rhs as i64
}
} impl PyAdd<f64>for i64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: f64) -> f64 {
    self as f64 + rhs
}
} impl PyAdd for f64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: f64) -> f64 {
    self + rhs
}
} impl PyAdd<i32>for f64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: i32) -> f64 {
    self + rhs as f64
}
} impl PyAdd<i64>for f64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: i64) -> f64 {
    self + rhs as f64
}
} impl PyAdd for String {
    type Output = String;
    #[inline] fn py_add(self, rhs: String) -> String {
    self + & rhs
}
} impl PyAdd<& str>for String {
    type Output = String;
    #[inline] fn py_add(self, rhs: & str) -> String {
    self + rhs
}
} impl PyAdd<& str>for & str {
    type Output = String;
    #[inline] fn py_add(self, rhs: & str) -> String {
    format!("{}{}", self, rhs)
}
} impl PyAdd<String>for & str {
    type Output = String;
    #[inline] fn py_add(self, rhs: String) -> String {
    format!("{}{}", self, rhs)
}
} impl PyAdd<char>for String {
    type Output = String;
    #[inline] fn py_add(mut self, rhs: char) -> String {
    self.push(rhs);
    self
}
} impl PyAdd<char>for & str {
    type Output = String;
    #[inline] fn py_add(self, rhs: char) -> String {
    format!("{}{}", self, rhs)
}
} impl PyAdd for DepylerValue {
    type Output = DepylerValue;
    fn py_add(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a + _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a + _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 + _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a + _dv_b as f64) ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>DepylerValue::Str(_dv_a + & _dv_b), _ =>DepylerValue::None ,
}
}
}
impl PyAdd<DepylerValue>for i32 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: DepylerValue) -> i64 {
    self as i64 + rhs.to_i64()
}
} impl PyAdd<DepylerValue>for i64 {
    type Output = i64;
    #[inline] fn py_add(self, rhs: DepylerValue) -> i64 {
    self + rhs.to_i64()
}
} impl PyAdd<DepylerValue>for f64 {
    type Output = f64;
    #[inline] fn py_add(self, rhs: DepylerValue) -> f64 {
    self + rhs.to_f64()
}
} impl PySub for i32 {
    type Output = i32;
    #[inline] fn py_sub(self, rhs: i32) -> i32 {
    self - rhs
}
} impl PySub<f64>for i32 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: f64) -> f64 {
    self as f64 - rhs
}
} impl PySub for i64 {
    type Output = i64;
    #[inline] fn py_sub(self, rhs: i64) -> i64 {
    self - rhs
}
} impl PySub<f64>for i64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: f64) -> f64 {
    self as f64 - rhs
}
} impl PySub for f64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: f64) -> f64 {
    self - rhs
}
} impl PySub<i32>for f64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: i32) -> f64 {
    self - rhs as f64
}
} impl PySub<i64>for f64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: i64) -> f64 {
    self - rhs as f64
}
} impl PySub for DepylerValue {
    type Output = DepylerValue;
    fn py_sub(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a - _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a - _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 - _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a - _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl PySub<DepylerValue>for i32 {
    type Output = i64;
    #[inline] fn py_sub(self, rhs: DepylerValue) -> i64 {
    self as i64 - rhs.to_i64()
}
} impl PySub<DepylerValue>for i64 {
    type Output = i64;
    #[inline] fn py_sub(self, rhs: DepylerValue) -> i64 {
    self - rhs.to_i64()
}
} impl PySub<DepylerValue>for f64 {
    type Output = f64;
    #[inline] fn py_sub(self, rhs: DepylerValue) -> f64 {
    self - rhs.to_f64()
}
} impl<T: Eq + std::hash::Hash + Clone>PySub for std::collections::HashSet<T>{
    type Output = std::collections::HashSet<T>;
    fn py_sub(self, rhs: std::collections::HashSet<T>) -> Self::Output {
    self.difference(& rhs).cloned().collect()
}
} impl<T: Eq + std::hash::Hash + Clone>PySub<& std::collections::HashSet<T>>for std::collections::HashSet<T>{
    type Output = std::collections::HashSet<T>;
    fn py_sub(self, rhs: & std::collections::HashSet<T>) -> Self::Output {
    self.difference(rhs).cloned().collect()
}
} impl PyMul for i32 {
    type Output = i32;
    #[inline] fn py_mul(self, rhs: i32) -> i32 {
    self * rhs
}
} impl PyMul<f64>for i32 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: f64) -> f64 {
    self as f64 * rhs
}
} impl PyMul<i64>for i32 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: i64) -> i64 {
    self as i64 * rhs
}
} impl PyMul for i64 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: i64) -> i64 {
    self * rhs
}
} impl PyMul<f64>for i64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: f64) -> f64 {
    self as f64 * rhs
}
} impl PyMul<i32>for i64 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: i32) -> i64 {
    self * rhs as i64
}
} impl PyMul for f64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: f64) -> f64 {
    self * rhs
}
} impl PyMul<i32>for f64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: i32) -> f64 {
    self * rhs as f64
}
} impl PyMul<i64>for f64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: i64) -> f64 {
    self * rhs as f64
}
} impl PyMul<i32>for String {
    type Output = String;
    fn py_mul(self, rhs: i32) -> String {
    if rhs <= 0 {
    String::new()
}
else {
    self.repeat(rhs as usize)
}
}
}
impl PyMul<i64>for String {
    type Output = String;
    fn py_mul(self, rhs: i64) -> String {
    if rhs <= 0 {
    String::new()
}
else {
    self.repeat(rhs as usize)
}
}
}
impl PyMul<i32>for & str {
    type Output = String;
    fn py_mul(self, rhs: i32) -> String {
    if rhs <= 0 {
    String::new()
}
else {
    self.repeat(rhs as usize)
}
}
}
impl PyMul<i64>for & str {
    type Output = String;
    fn py_mul(self, rhs: i64) -> String {
    if rhs <= 0 {
    String::new()
}
else {
    self.repeat(rhs as usize)
}
}
}
impl PyMul for DepylerValue {
    type Output = DepylerValue;
    fn py_mul(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Int(_dv_a * _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a * _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>DepylerValue::Float(_dv_a as f64 * _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>DepylerValue::Float(_dv_a * _dv_b as f64) ,(DepylerValue::Str(_dv_s), DepylerValue::Int(_dv_n)) =>{
    if _dv_n <= 0 {
    DepylerValue::Str(String::new())
}
else {
    DepylerValue::Str(_dv_s.repeat(_dv_n as usize))
}
} _ =>DepylerValue::None ,
}
}
}
impl PyMul<DepylerValue>for i32 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: DepylerValue) -> i64 {
    self as i64 * rhs.to_i64()
}
} impl PyMul<DepylerValue>for i64 {
    type Output = i64;
    #[inline] fn py_mul(self, rhs: DepylerValue) -> i64 {
    self * rhs.to_i64()
}
} impl PyMul<DepylerValue>for f64 {
    type Output = f64;
    #[inline] fn py_mul(self, rhs: DepylerValue) -> f64 {
    self * rhs.to_f64()
}
} impl<T: Clone>PyAdd<Vec<T>>for Vec<T>{
    type Output = Vec<T>;
    fn py_add(mut self, rhs: Vec<T>) -> Vec<T>{
    self.extend(rhs);
    self
}
} impl<T: Clone>PyAdd<& Vec<T>>for Vec<T>{
    type Output = Vec<T>;
    fn py_add(mut self, rhs: & Vec<T>) -> Vec<T>{
    self.extend(rhs.iter().cloned());
    self
}
} impl<T: Clone>PyAdd<Vec<T>>for & Vec<T>{
    type Output = Vec<T>;
    fn py_add(self, rhs: Vec<T>) -> Vec<T>{
    let mut result = self.clone();
    result.extend(rhs);
    result
}
} impl<T: Clone>PyMul<i32>for Vec<T>{
    type Output = Vec<T>;
    fn py_mul(self, rhs: i32) -> Vec<T>{
    if rhs <= 0 {
    Vec::new()
}
else {
    self.iter().cloned().cycle().take(self.len() * rhs as usize).collect()
}
}
}
impl<T: Clone>PyMul<i64>for Vec<T>{
    type Output = Vec<T>;
    fn py_mul(self, rhs: i64) -> Vec<T>{
    if rhs <= 0 {
    Vec::new()
}
else {
    self.iter().cloned().cycle().take(self.len() * rhs as usize).collect()
}
}
}
impl<T: Clone>PyMul<usize>for Vec<T>{
    type Output = Vec<T>;
    fn py_mul(self, rhs: usize) -> Vec<T>{
    self.iter().cloned().cycle().take(self.len() * rhs).collect()
}
} impl<T: Clone>PyMul<Vec<T>>for i32 {
    type Output = Vec<T>;
    fn py_mul(self, rhs: Vec<T>) -> Vec<T>{
    rhs.py_mul(self)
}
} impl<T: Clone>PyMul<Vec<T>>for i64 {
    type Output = Vec<T>;
    fn py_mul(self, rhs: Vec<T>) -> Vec<T>{
    rhs.py_mul(self)
}
} impl PySub<Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<& Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<& Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<Vec<f32>>for Vec<f32>{
    type Output = Vec<f32>;
    fn py_sub(self, rhs: Vec<f32>) -> Vec<f32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<Vec<i64>>for Vec<i64>{
    type Output = Vec<i64>;
    fn py_sub(self, rhs: Vec<i64>) -> Vec<i64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PySub<Vec<i32>>for Vec<i32>{
    type Output = Vec<i32>;
    fn py_sub(self, rhs: Vec<i32>) -> Vec<i32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a - b).collect()
}
} impl PyMul<Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<& Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<& Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<Vec<f32>>for Vec<f32>{
    type Output = Vec<f32>;
    fn py_mul(self, rhs: Vec<f32>) -> Vec<f32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<Vec<i64>>for Vec<i64>{
    type Output = Vec<i64>;
    fn py_mul(self, rhs: Vec<i64>) -> Vec<i64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyMul<Vec<i32>>for Vec<i32>{
    type Output = Vec<i32>;
    fn py_mul(self, rhs: Vec<i32>) -> Vec<i32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | a * b).collect()
}
} impl PyDiv<Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f64::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<& Vec<f64>>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f64::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f64::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<& Vec<f64>>for & Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: & Vec<f64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f64::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<Vec<f32>>for Vec<f32>{
    type Output = Vec<f32>;
    fn py_div(self, rhs: Vec<f32>) -> Vec<f32>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0.0 {
    f32::NAN
}
else {
    a / b }).collect()
}
} impl PyDiv<Vec<i64>>for Vec<i64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<i64>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0 {
    f64::NAN
}
else {
    * a as f64 / * b as f64 }).collect()
}
} impl PyDiv<Vec<i32>>for Vec<i32>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: Vec<i32>) -> Vec<f64>{
    self.iter().zip(rhs.iter()).map(|(a, b) | if * b == 0 {
    f64::NAN
}
else {
    * a as f64 / * b as f64 }).collect()
}
} impl PyMul<f64>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_mul(self, rhs: f64) -> Vec<f64>{
    self.iter().map(| a | a * rhs).collect()
}
} impl PyMul<Vec<f64>>for f64 {
    type Output = Vec<f64>;
    fn py_mul(self, rhs: Vec<f64>) -> Vec<f64>{
    rhs.iter().map(| a | a * self).collect()
}
} impl PyDiv<f64>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_div(self, rhs: f64) -> Vec<f64>{
    if rhs == 0.0 {
    self.iter().map(| _ | f64::NAN).collect()
}
else {
    self.iter().map(| a | a / rhs).collect()
}
}
}
impl PySub<f64>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_sub(self, rhs: f64) -> Vec<f64>{
    self.iter().map(| a | a - rhs).collect()
}
} impl PyAdd<f64>for Vec<f64>{
    type Output = Vec<f64>;
    fn py_add(self, rhs: f64) -> Vec<f64>{
    self.iter().map(| a | a + rhs).collect()
}
} impl PyDiv for i32 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: i32) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
    self as f64 / rhs as f64
}
}
}
impl PyDiv<f64>for i32 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
    self as f64 / rhs
}
}
}
impl PyDiv for i64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: i64) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
    self as f64 / rhs as f64
}
}
}
impl PyDiv<f64>for i64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
    self as f64 / rhs
}
}
}
impl PyDiv for f64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
    self / rhs
}
}
}
impl PyDiv<i32>for f64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: i32) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
    self / rhs as f64
}
}
}
impl PyDiv<i64>for f64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: i64) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
    self / rhs as f64
}
}
}
impl PyDiv for DepylerValue {
    type Output = DepylerValue;
    fn py_div(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>DepylerValue::Float(_dv_a as f64 / _dv_b as f64) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>DepylerValue::Float(_dv_a / _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>DepylerValue::Float(_dv_a as f64 / _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>DepylerValue::Float(_dv_a / _dv_b as f64), _ =>DepylerValue::None ,
}
}
}
impl PyDiv<DepylerValue>for i32 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: DepylerValue) -> f64 {
    let divisor = rhs.to_f64();
    if divisor == 0.0 {
    f64::NAN
}
else {
    self as f64 / divisor
}
}
}
impl PyDiv<DepylerValue>for i64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: DepylerValue) -> f64 {
    let divisor = rhs.to_f64();
    if divisor == 0.0 {
    f64::NAN
}
else {
    self as f64 / divisor
}
}
}
impl PyDiv<DepylerValue>for f64 {
    type Output = f64;
    #[inline] fn py_div(self, rhs: DepylerValue) -> f64 {
    let divisor = rhs.to_f64();
    if divisor == 0.0 {
    f64::NAN
}
else {
    self / divisor
}
}
}
impl PyMod for i32 {
    type Output = i32;
    #[inline] fn py_mod(self, rhs: i32) -> i32 {
    if rhs == 0 {
    0
}
else {
   ((self % rhs) + rhs) % rhs
}
}
}
impl PyMod<f64>for i32 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
   ((self as f64 % rhs) + rhs) % rhs
}
}
}
impl PyMod for i64 {
    type Output = i64;
    #[inline] fn py_mod(self, rhs: i64) -> i64 {
    if rhs == 0 {
    0
}
else {
   ((self % rhs) + rhs) % rhs
}
}
}
impl PyMod<f64>for i64 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
   ((self as f64 % rhs) + rhs) % rhs
}
}
}
impl PyMod for f64 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: f64) -> f64 {
    if rhs == 0.0 {
    f64::NAN
}
else {
   ((self % rhs) + rhs) % rhs
}
}
}
impl PyMod<i32>for f64 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: i32) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
   ((self % rhs as f64) + rhs as f64) % rhs as f64
}
}
}
impl PyMod<i64>for f64 {
    type Output = f64;
    #[inline] fn py_mod(self, rhs: i64) -> f64 {
    if rhs == 0 {
    f64::NAN
}
else {
   ((self % rhs as f64) + rhs as f64) % rhs as f64
}
}
}
impl PyMod for DepylerValue {
    type Output = DepylerValue;
    fn py_mod(self, rhs: DepylerValue) -> DepylerValue {
    match(self, rhs) {
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>{
    DepylerValue::Int(((_dv_a % _dv_b) + _dv_b) % _dv_b)
}
(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>{
    DepylerValue::Float(((_dv_a % _dv_b) + _dv_b) % _dv_b)
}
(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b != 0.0 =>{
    let a = _dv_a as f64;
    DepylerValue::Float(((a % _dv_b) + _dv_b) % _dv_b)
}
(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b != 0 =>{
    let b = _dv_b as f64;
    DepylerValue::Float(((_dv_a % b) + b) % b)
}
_ =>DepylerValue::None ,
}
}
}
impl<T: Clone>PyIndex<i32>for Vec<T>{
    type Output = Option<T>;
    fn py_index(&self, index: i32) -> Option<T>{
    let _dv_len = self.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 &&(_dv_idx as usize)< self.len() {
    Some(self [_dv_idx as usize].clone())
}
else {
    Option::None
}
}
}
impl<T: Clone>PyIndex<i64>for Vec<T>{
    type Output = Option<T>;
    fn py_index(&self, index: i64) -> Option<T>{
    let _dv_len = self.len() as i64;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 &&(_dv_idx as usize)< self.len() {
    Some(self [_dv_idx as usize].clone())
}
else {
    Option::None
}
}
}
impl PyIndex<& str>for std::collections::HashMap<String, DepylerValue>{
    type Output = Option<DepylerValue>;
    fn py_index(&self, key: & str) -> Option<DepylerValue>{
    self.get(key).cloned()
}
} impl PyIndex<i32>for String {
    type Output = Option<char>;
    fn py_index(&self, index: i32) -> Option<char>{
    let _dv_len = self.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 {
    self.chars().nth(_dv_idx as usize)
}
else {
    Option::None
}
}
}
impl PyIndex<i64>for String {
    type Output = Option<char>;
    fn py_index(&self, index: i64) -> Option<char>{
    let _dv_len = self.len() as i64;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 {
    self.chars().nth(_dv_idx as usize)
}
else {
    Option::None
}
}
}
impl PyIndex<i32>for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, index: i32) -> DepylerValue {
    match self {
    DepylerValue::List(_dv_list) =>{
    let _dv_len = _dv_list.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 &&(_dv_idx as usize)<_dv_list.len() {
    _dv_list [_dv_idx as usize].clone()
}
else {
    DepylerValue::None
}
} DepylerValue::Tuple(_dv_tuple) =>{
    let _dv_len = _dv_tuple.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 &&(_dv_idx as usize)<_dv_tuple.len() {
    _dv_tuple [_dv_idx as usize].clone()
}
else {
    DepylerValue::None
}
} DepylerValue::Str(_dv_str) =>{
    let _dv_len = _dv_str.len() as i32;
    let _dv_idx = if index<0 {
    _dv_len + index
}
else {
    index };
    if _dv_idx>= 0 {
    _dv_str.chars().nth(_dv_idx as usize).map(| _dv_c | DepylerValue::Str(_dv_c.to_string())).unwrap_or(DepylerValue::None)
}
else {
    DepylerValue::None
}
} _ =>DepylerValue::None ,
}
}
}
impl PyIndex<i64>for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, index: i64) -> DepylerValue {
    self.py_index(index as i32)
}
} impl PyIndex<& str>for DepylerValue {
    type Output = DepylerValue;
    fn py_index(&self, key: & str) -> DepylerValue {
    match self {
    DepylerValue::Dict(_dv_dict) =>{
    _dv_dict.get(& DepylerValue::Str(key.to_string())).cloned().unwrap_or(DepylerValue::None)
}
_ =>DepylerValue::None ,
}
}
}
pub trait PyStringMethods {
    fn lower(&self) -> String;
    fn upper(&self) -> String;
    fn strip(&self) -> String;
    fn lstrip(&self) -> String;
    fn rstrip(&self) -> String;
    fn py_split(&self, sep: & str) -> Vec<String>;
    fn py_replace(&self, old: & str, new: & str) -> String;
    fn startswith(&self, prefix: & str) -> bool;
    fn endswith(&self, suffix: & str) -> bool;
    fn py_find(&self, sub: & str) -> i64;
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
    fn count(&self, sub: & str) -> usize;
   
}
impl PyStringMethods for str {
    #[inline] fn lower(&self) -> String {
    self.to_lowercase()
}
#[inline] fn upper(&self) -> String {
    self.to_uppercase()
}
#[inline] fn strip(&self) -> String {
    self.trim().to_string()
}
#[inline] fn lstrip(&self) -> String {
    self.trim_start().to_string()
}
#[inline] fn rstrip(&self) -> String {
    self.trim_end().to_string()
}
#[inline] fn py_split(&self, sep: & str) -> Vec<String>{
    self.split(sep).map(| s | s.to_string()).collect()
}
#[inline] fn py_replace(&self, old: & str, new: & str) -> String {
    self.replace(old, new)
}
#[inline] fn startswith(&self, prefix: & str) -> bool {
    self.starts_with(prefix)
}
#[inline] fn endswith(&self, suffix: & str) -> bool {
    self.ends_with(suffix)
}
#[inline] fn py_find(&self, sub: & str) -> i64 {
    self.find(sub).map(| i | i as i64).unwrap_or(- 1)
}
#[inline] fn capitalize(&self) -> String {
    let mut chars = self.chars();
    match chars.next() {
    None =>String::new(), Some(c) =>c.to_uppercase().chain (chars.flat_map(| c | c.to_lowercase())).collect() ,
}
} #[inline] fn title(&self) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in self.chars() {
    if c.is_whitespace() {
    result.push(c);
    capitalize_next = true;
   
}
else if capitalize_next {
    result.extend(c.to_uppercase());
    capitalize_next = false;
   
}
else {
    result.extend(c.to_lowercase());
   
}
} result
}
#[inline] fn swapcase(&self) -> String {
    self.chars().map(| c | {
    if c.is_uppercase() {
    c.to_lowercase().collect::<String>()
}
else {
    c.to_uppercase().collect::<String>()
}
}).collect()
}
#[inline] fn isalpha(&self) -> bool {
    ! self.is_empty() &&self.chars().all(| c | c.is_alphabetic())
}
#[inline] fn isdigit(&self) -> bool {
    ! self.is_empty() &&self.chars().all(| c | c.is_ascii_digit())
}
#[inline] fn isalnum(&self) -> bool {
    ! self.is_empty() &&self.chars().all(| c | c.is_alphanumeric())
}
#[inline] fn isspace(&self) -> bool {
    ! self.is_empty() &&self.chars().all(| c | c.is_whitespace())
}
#[inline] fn islower(&self) -> bool {
    self.chars().any(| c | c.is_lowercase()) && ! self.chars().any(| c | c.is_uppercase())
}
#[inline] fn isupper(&self) -> bool {
    self.chars().any(| c | c.is_uppercase()) && ! self.chars().any(| c | c.is_lowercase())
}
#[inline] fn center(&self, width: usize) -> String {
    if self.len()>= width {
    return self.to_string();
   
}
let padding = width - self.len();
    let left = padding / 2;
    let right = padding - left;
    format!("{}{}{}", " ".repeat(left), self, " ".repeat(right))
}
#[inline] fn ljust(&self, width: usize) -> String {
    if self.len()>= width {
    return self.to_string();
   
}
format!("{}{}", self, " ".repeat(width - self.len()))
}
#[inline] fn rjust(&self, width: usize) -> String {
    if self.len()>= width {
    return self.to_string();
   
}
format!("{}{}", " ".repeat(width - self.len()), self)
}
#[inline] fn zfill(&self, width: usize) -> String {
    if self.len()>= width {
    return self.to_string();
   
}
format!("{}{}", "0".repeat(width - self.len()), self)
}
#[inline] fn count(&self, sub: & str) -> usize {
    self.matches(sub).count()
}
} impl PyStringMethods for String {
    #[inline] fn lower(&self) -> String {
    self.as_str().lower()
}
#[inline] fn upper(&self) -> String {
    self.as_str().upper()
}
#[inline] fn strip(&self) -> String {
    self.as_str().strip()
}
#[inline] fn lstrip(&self) -> String {
    self.as_str().lstrip()
}
#[inline] fn rstrip(&self) -> String {
    self.as_str().rstrip()
}
#[inline] fn py_split(&self, sep: & str) -> Vec<String>{
    self.as_str().py_split(sep)
}
#[inline] fn py_replace(&self, old: & str, new: & str) -> String {
    self.as_str().py_replace(old, new)
}
#[inline] fn startswith(&self, prefix: & str) -> bool {
    self.as_str().startswith(prefix)
}
#[inline] fn endswith(&self, suffix: & str) -> bool {
    self.as_str().endswith(suffix)
}
#[inline] fn py_find(&self, sub: & str) -> i64 {
    self.as_str().py_find(sub)
}
#[inline] fn capitalize(&self) -> String {
    self.as_str().capitalize()
}
#[inline] fn title(&self) -> String {
    self.as_str().title()
}
#[inline] fn swapcase(&self) -> String {
    self.as_str().swapcase()
}
#[inline] fn isalpha(&self) -> bool {
    self.as_str().isalpha()
}
#[inline] fn isdigit(&self) -> bool {
    self.as_str().isdigit()
}
#[inline] fn isalnum(&self) -> bool {
    self.as_str().isalnum()
}
#[inline] fn isspace(&self) -> bool {
    self.as_str().isspace()
}
#[inline] fn islower(&self) -> bool {
    self.as_str().islower()
}
#[inline] fn isupper(&self) -> bool {
    self.as_str().isupper()
}
#[inline] fn center(&self, width: usize) -> String {
    self.as_str().center(width)
}
#[inline] fn ljust(&self, width: usize) -> String {
    self.as_str().ljust(width)
}
#[inline] fn rjust(&self, width: usize) -> String {
    self.as_str().rjust(width)
}
#[inline] fn zfill(&self, width: usize) -> String {
    self.as_str().zfill(width)
}
#[inline] fn count(&self, sub: & str) -> usize {
    self.as_str().count(sub)
}
} impl PyStringMethods for DepylerValue {
    #[inline] fn lower(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.lower(), _ =>String::new() ,
}
} #[inline] fn upper(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.upper(), _ =>String::new() ,
}
} #[inline] fn strip(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.strip(), _ =>String::new() ,
}
} #[inline] fn lstrip(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.lstrip(), _ =>String::new() ,
}
} #[inline] fn rstrip(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.rstrip(), _ =>String::new() ,
}
} #[inline] fn py_split(&self, sep: & str) -> Vec<String>{
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.py_split(sep), _ =>Vec::new() ,
}
} #[inline] fn py_replace(&self, old: & str, new: & str) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.py_replace(old, new), _ =>String::new() ,
}
} #[inline] fn startswith(&self, prefix: & str) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.startswith(prefix), _ =>false ,
}
} #[inline] fn endswith(&self, suffix: & str) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.endswith(suffix), _ =>false ,
}
} #[inline] fn py_find(&self, sub: & str) -> i64 {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.py_find(sub), _ =>- 1 ,
}
} #[inline] fn capitalize(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.capitalize(), _ =>String::new() ,
}
} #[inline] fn title(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.title(), _ =>String::new() ,
}
} #[inline] fn swapcase(&self) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.swapcase(), _ =>String::new() ,
}
} #[inline] fn isalpha(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isalpha(), _ =>false ,
}
} #[inline] fn isdigit(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isdigit(), _ =>false ,
}
} #[inline] fn isalnum(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isalnum(), _ =>false ,
}
} #[inline] fn isspace(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isspace(), _ =>false ,
}
} #[inline] fn islower(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.islower(), _ =>false ,
}
} #[inline] fn isupper(&self) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.isupper(), _ =>false ,
}
} #[inline] fn center(&self, width: usize) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.center(width), _ =>String::new() ,
}
} #[inline] fn ljust(&self, width: usize) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.ljust(width), _ =>String::new() ,
}
} #[inline] fn rjust(&self, width: usize) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.rjust(width), _ =>String::new() ,
}
} #[inline] fn zfill(&self, width: usize) -> String {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.zfill(width), _ =>String::new() ,
}
} #[inline] fn count(&self, sub: & str) -> usize {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.count(sub), _ =>0 ,
}
}
}
impl DepylerValue {
    #[doc = r" Check if string contains substring(Python's `in` operator for strings)"] #[inline] pub fn contains(&self, sub: & str) -> bool {
    match self {
    DepylerValue::Str(_dv_s) =>_dv_s.contains(sub), DepylerValue::List(_dv_l) =>_dv_l.iter().any(| v | {
    if let DepylerValue::Str(s) = v {
    s == sub
}
else {
    false
}
}), _ =>false ,
}
}
}
#[doc = r" DEPYLER-1202: Python integer operations for Rust integer types."] pub trait PythonIntOps {
    fn bit_length(&self) -> u32;
    fn bit_count(&self) -> u32;
   
}
impl PythonIntOps for i32 {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<i32>() as u32 * 8) - self.unsigned_abs().leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.unsigned_abs().count_ones()
}
} impl PythonIntOps for i64 {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<i64>() as u32 * 8) - self.unsigned_abs().leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.unsigned_abs().count_ones()
}
} impl PythonIntOps for u32 {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<u32>() as u32 * 8) - self.leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.count_ones()
}
} impl PythonIntOps for u64 {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<u64>() as u32 * 8) - self.leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.count_ones()
}
} impl PythonIntOps for usize {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<usize>() as u32 * 8) - self.leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.count_ones()
}
} impl PythonIntOps for isize {
    fn bit_length(&self) -> u32 {
    if * self == 0 {
    0
}
else {
   (std::mem::size_of::<isize>() as u32 * 8) - self.unsigned_abs().leading_zeros()
}
} fn bit_count(&self) -> u32 {
    self.unsigned_abs().count_ones()
}
} #[doc = r" DEPYLER-1066: Wrapper for Python datetime.date"] #[doc = r" Provides .day(), .month(), .year() methods matching Python's API"] #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)] pub struct DepylerDate(pub u32, pub u32, pub u32);
    impl DepylerDate {
    #[doc = r" Create a new date from year, month, day"] pub fn new(year: u32, month: u32, day: u32) -> Self {
    DepylerDate(year, month, day)
}
#[doc = r" Get today's date(NASA mode: computed from SystemTime)"] pub fn today() -> Self {
    use std::time::{
    SystemTime, UNIX_EPOCH };
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(| d | d.as_secs()).unwrap_or(0);
    let days  = (secs / 86400) as i64;
    let z = days + 719468;
    let era = if z>= 0 {
    z
}
else {
    z - 146096
}
/ 146097;
    let doe  = (z - era * 146097) as u32;
    let yoe  = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe -(365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d = doy -(153 * mp + 2) / 5 + 1;
    let m = if mp<10 {
    mp + 3
}
else {
    mp - 9 };
    let y = if m <= 2 {
    y + 1
}
else {
    y };
    DepylerDate(y as u32, m, d)
}
#[doc = r" Get the year component"] pub fn year(&self) -> u32 {
    self.0
}
#[doc = r" Get the month component(1-12)"] pub fn month(&self) -> u32 {
    self.1
}
#[doc = r" Get the day component(1-31)"] pub fn day(&self) -> u32 {
    self.2
}
#[doc = r" Convert to tuple(year, month, day) for interop"] pub fn to_tuple(&self) -> (u32, u32, u32) {
   (self.0, self.1, self.2)
}
#[doc = r" Get weekday(0 = Monday, 6 = Sunday) - Python datetime.date.weekday()"] pub fn weekday(&self) -> u32 {
    let(mut y, mut m, d)  = (self.0 as i32, self.1 as i32, self.2 as i32);
    if m<3 {
    m += 12;
    y -= 1;
   
}
let q = d;
    let k = y % 100;
    let j = y / 100;
    let h  = (q +(13 *(m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
   ((h + 5) % 7) as u32
}
#[doc = r" Get ISO weekday(1 = Monday, 7 = Sunday) - Python datetime.date.isoweekday()"] pub fn isoweekday(&self) -> u32 {
    self.weekday() + 1
}
#[doc = r" Create date from ordinal(days since year 1, January 1 = ordinal 1)"] #[doc = r" Python: date.fromordinal(730120) -> date(2000, 1, 1)"] pub fn from_ordinal(ordinal: i64) -> Self {
    let days = ordinal - 719163 - 1;
    let z = days + 719468;
    let era = if z>= 0 {
    z
}
else {
    z - 146096
}
/ 146097;
    let doe  = (z - era * 146097) as u32;
    let yoe  = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe -(365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d = doy -(153 * mp + 2) / 5 + 1;
    let m = if mp<10 {
    mp + 3
}
else {
    mp - 9 };
    let y = if m <= 2 {
    y + 1
}
else {
    y };
    DepylerDate(y as u32, m, d)
}
} impl std::fmt::Display for DepylerDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:04}-{:02}-{:02}", self.0, self.1, self.2)
}
} #[doc = r" DEPYLER-1067: Wrapper for Python datetime.datetime"] #[doc = r" Provides .year(), .month(), .day(), .hour(), .minute(), .second(), .microsecond() methods"] #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)] pub struct DepylerDateTime {
    pub year: u32, pub month: u32, pub day: u32, pub hour: u32, pub minute: u32, pub second: u32, pub microsecond: u32 ,
}
impl DepylerDateTime {
    #[doc = r" Create a new datetime from components"] pub fn new(year: u32, month: u32, day: u32, hour: u32, minute: u32, second: u32, microsecond: u32) -> Self {
    DepylerDateTime {
    year, month, day, hour, minute, second, microsecond
}
} #[doc = r" Get current datetime(NASA mode: computed from SystemTime)"] pub fn now() -> Self {
    use std::time::{
    SystemTime, UNIX_EPOCH };
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(| d | d.as_secs()).unwrap_or(0);
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(| d | d.subsec_nanos()).unwrap_or(0);
    let days  = (secs / 86400) as i64;
    let day_secs  = (secs % 86400) as u32;
    let z = days + 719468;
    let era = if z>= 0 {
    z
}
else {
    z - 146096
}
/ 146097;
    let doe  = (z - era * 146097) as u32;
    let yoe  = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe -(365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d = doy -(153 * mp + 2) / 5 + 1;
    let m = if mp<10 {
    mp + 3
}
else {
    mp - 9 };
    let y = if m <= 2 {
    y + 1
}
else {
    y };
    let hour = day_secs / 3600;
    let minute  = (day_secs % 3600) / 60;
    let second = day_secs % 60;
    let microsecond = nanos / 1000;
    DepylerDateTime {
    year: y as u32, month: m, day: d, hour, minute, second, microsecond
}
} #[doc = r" Alias for now() - Python datetime.datetime.today()"] pub fn today() -> Self {
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
#[doc = r" Get weekday(0 = Monday, 6 = Sunday)"] pub fn weekday(&self) -> u32 {
    DepylerDate::new(self.year, self.month, self.day).weekday()
}
#[doc = r" Get ISO weekday(1 = Monday, 7 = Sunday)"] pub fn isoweekday(&self) -> u32 {
    self.weekday() + 1
}
#[doc = r" Extract date component"] pub fn date(&self) -> DepylerDate {
    DepylerDate::new(self.year, self.month, self.day)
}
#[doc = r" Get Unix timestamp"] pub fn timestamp(&self) -> f64 {
    let days = self.days_since_epoch();
    let secs = days as f64 * 86400.0 + self.hour as f64 * 3600.0 + self.minute as f64 * 60.0 + self.second as f64 + self.microsecond as f64 / 1_000_000.0;
    secs
}
fn days_since_epoch(&self) -> i64 {
    let(mut y, mut m)  = (self.year as i64, self.month as i64);
    if m <= 2 {
    y -= 1;
    m += 12;
   
}
let era = if y>= 0 {
    y
}
else {
    y - 399
}
/ 400;
    let yoe  = (y - era * 400) as u32;
    let doy  = (153 *(m as u32 - 3) + 2) / 5 + self.day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i64 - 719468
}
#[doc = r" Create from Unix timestamp"] pub fn fromtimestamp(ts: f64) -> Self {
    let secs = ts as u64;
    let microsecond  = ((ts - secs as f64) * 1_000_000.0) as u32;
    let days  = (secs / 86400) as i64;
    let day_secs  = (secs % 86400) as u32;
    let z = days + 719468;
    let era = if z>= 0 {
    z
}
else {
    z - 146096
}
/ 146097;
    let doe  = (z - era * 146097) as u32;
    let yoe  = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe -(365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d = doy -(153 * mp + 2) / 5 + 1;
    let m = if mp<10 {
    mp + 3
}
else {
    mp - 9 };
    let y = if m <= 2 {
    y + 1
}
else {
    y };
    let hour = day_secs / 3600;
    let minute  = (day_secs % 3600) / 60;
    let second = day_secs % 60;
    DepylerDateTime {
    year: y as u32, month: m, day: d, hour, minute, second, microsecond
}
} #[doc = r" ISO format string"] pub fn isoformat(&self) -> String {
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
}
} impl std::fmt::Display for DepylerDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
}
} #[doc = r" DEPYLER-1068: Wrapper for Python datetime.timedelta"] #[doc = r" Provides .days, .seconds, .microseconds, .total_seconds() methods"] #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)] pub struct DepylerTimeDelta {
    pub days: i64, pub seconds: i64, pub microseconds: i64 ,
}
impl DepylerTimeDelta {
    #[doc = r" Create a new timedelta from components"] pub fn new(days: i64, seconds: i64, microseconds: i64) -> Self {
    let total_us = days * 86400 * 1_000_000 + seconds * 1_000_000 + microseconds;
    let total_secs = total_us / 1_000_000;
    let us = total_us % 1_000_000;
    let d = total_secs / 86400;
    let s = total_secs % 86400;
    DepylerTimeDelta {
    days: d, seconds: s, microseconds: us
}
} #[doc = r" Create from keyword-style arguments(hours, minutes, etc.)"] pub fn from_components(days: i64, seconds: i64, microseconds: i64, milliseconds: i64, minutes: i64, hours: i64, weeks: i64 ,) -> Self {
    let total_days = days + weeks * 7;
    let total_secs = seconds + minutes * 60 + hours * 3600;
    let total_us = microseconds + milliseconds * 1000;
    Self::new(total_days, total_secs, total_us)
}
#[doc = r" Get total seconds as f64"] pub fn total_seconds(&self) -> f64 {
    self.days as f64 * 86400.0 + self.seconds as f64 + self.microseconds as f64 / 1_000_000.0
}
#[doc = r" Get days component"] pub fn days(&self) -> i64 {
    self.days
}
#[doc = r" Get seconds component(0-86399)"] pub fn seconds(&self) -> i64 {
    self.seconds
}
#[doc = r" Get microseconds component(0-999999)"] pub fn microseconds(&self) -> i64 {
    self.microseconds
}
} impl std::ops::Add for DepylerTimeDelta {
    type Output = Self;
    fn add(self, other: Self) -> Self {
    Self::new(self.days + other.days, self.seconds + other.seconds, self.microseconds + other.microseconds ,)
}
} impl std::ops::Sub for DepylerTimeDelta {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
    Self::new(self.days - other.days, self.seconds - other.seconds, self.microseconds - other.microseconds ,)
}
} impl std::fmt::Display for DepylerTimeDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let hours = self.seconds / 3600;
    let mins  = (self.seconds % 3600) / 60;
    let secs = self.seconds % 60;
    if self.days != 0 {
    write!(f, "{} day{}, {:02}:{:02}:{:02}", self.days, if self.days == 1 {
    ""
}
else {
    "s"
}
, hours, mins, secs)
}
else {
    write!(f, "{:02}:{:02}:{:02}", hours, mins, secs)
}
}
}
#[doc = r" DEPYLER-1070: Wrapper for Python re.Match object"] #[doc = r" Provides .group(), .groups(), .start(), .end(), .span() methods"] #[derive(Debug, Clone, PartialEq, Eq, Default)] pub struct DepylerRegexMatch {
    pub matched: String, pub start: usize, pub end: usize, pub groups: Vec<String>,
}
impl DepylerRegexMatch {
    #[doc = r" Create a new match from a string slice match"] pub fn new(text: & str, start: usize, end: usize) -> Self {
    DepylerRegexMatch {
    matched: text [start..end].to_string(), start, end, groups: vec! [text [start..end].to_string()] ,
}
} #[doc = r" Create a match with capture groups"] pub fn with_groups(text: & str, start: usize, end: usize, groups: Vec<String>) -> Self {
    DepylerRegexMatch {
    matched: text [start..end].to_string(), start, end, groups ,
}
} #[doc = r" Get the matched string(group 0)"] pub fn group(&self, n: usize) -> String {
    self.groups.get(n).cloned().unwrap_or_default()
}
#[doc = r" Get all capture groups as a tuple-like Vec"] pub fn groups(&self) -> Vec<String>{
    if self.groups.len()>1 {
    self.groups [1..].to_vec()
}
else {
    vec! []
}
} #[doc = r" Get the start position"] pub fn start(&self) -> usize {
    self.start
}
#[doc = r" Get the end position"] pub fn end(&self) -> usize {
    self.end
}
#[doc = r" Get(start, end) tuple"] pub fn span(&self) -> (usize, usize) {
   (self.start, self.end)
}
#[doc = r" Get the matched string(equivalent to group(0))"] pub fn as_str(&self) -> & str {
    &self.matched
}
#[doc = r" Simple pattern search(NASA mode alternative to regex)"] #[doc = r" Searches for literal string pattern in text"] pub fn search(pattern: & str, text: & str) -> Option<Self>{
    text.find(pattern).map(| start | {
    let end = start + pattern.len();
    DepylerRegexMatch::new(text, start, end) })
}
#[doc = r" Simple pattern match at start(NASA mode alternative to regex)"] pub fn match_start(pattern: & str, text: & str) -> Option<Self>{
    if text.starts_with(pattern) {
    Some(DepylerRegexMatch::new(text, 0, pattern.len()))
}
else {
    None
}
} #[doc = r" Find all occurrences(NASA mode alternative to regex findall)"] pub fn findall(pattern: & str, text: & str) -> Vec<String>{
    let mut results = Vec::new();
    let mut start = 0;
    while let Some(pos) = text [start..].find(pattern) {
    results.push(pattern.to_string());
    start += pos + pattern.len();
   
}
results
}
#[doc = r" Simple string replacement(NASA mode alternative to regex sub)"] pub fn sub(pattern: & str, repl: & str, text: & str) -> String {
    text.replace(pattern, repl)
}
#[doc = r" Simple string split(NASA mode alternative to regex split)"] pub fn split(pattern: & str, text: & str) -> Vec<String>{
    text.split(pattern).map(| s | s.to_string()).collect()
}
} #[doc = "Absolute value for float."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn float_abs(x: f64) -> f64 {
    let _cse_temp_0 = x<0.0;
    if _cse_temp_0 {
    return - x;
   
}
x
}
#[doc = "Check approximate float equality."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn float_eq(a: f64, b: f64) -> bool {
    float_abs((a) - (b)) <(EPSILON as f64)
}
#[doc = "Square root using Newton's method(pattern 10)."] #[doc = " Depyler: proven to terminate"] pub fn newton_sqrt(x: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let mut guess: f64 = Default::default();
    let _cse_temp_0 = x <= 0.0;
    if _cse_temp_0 {
    return Ok(0.0);
   
}
let _cse_temp_1  = (x).py_div(2.0);
    guess = _cse_temp_1;
    let _cse_temp_2 = guess == 0.0;
    if _cse_temp_2 {
    guess = 1.0;
   
}
for __sanitized in 0..(100) {
    let next_guess: f64  = (((guess).py_add((x).py_div(guess)) as f64)).py_div(2.0);
    if float_abs((next_guess) - (guess)) <(EPSILON as f64) {
    return Ok(next_guess);
   
}
guess = next_guess;
   
}
Ok(guess)
}
#[doc = "Approximate sine using Taylor series(trig helper)."] pub fn sin_approx(mut x: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let _cse_temp_0  = (2.0).py_mul(PI);
    let tau: f64 = _cse_temp_0;
    while x>(PI as f64) {
    x  = (x) - (tau);
   
}
while x <((- PI as f64)) {
    x  = (x).py_add(tau);
   
}
let _cse_temp_1  = (x).py_mul(x);
    let x2: f64 = _cse_temp_1;
    let _cse_temp_2  = (x2).py_mul(x);
    let x3: f64 = _cse_temp_2;
    let _cse_temp_3  = (x3).py_mul(x2);
    let x5: f64 = _cse_temp_3;
    let _cse_temp_4  = (x5).py_mul(x2);
    let x7: f64 = _cse_temp_4;
    let _cse_temp_5  = (x7).py_mul(x2);
    let x9: f64 = _cse_temp_5;
    Ok({ let _r: f64  = (((((((x) - ((x3).py_div(6.0)) as f64)).py_add((x5).py_div(120.0)) as f64)) - ((x7).py_div(5040.0)) as f64)).py_add((x9).py_div(362880.0));
    _r })
}
#[doc = "Approximate cosine using Taylor series(trig helper)."] pub fn cos_approx(mut x: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let _cse_temp_0  = (2.0).py_mul(PI);
    let tau: f64 = _cse_temp_0;
    while x>(PI as f64) {
    x  = (x) - (tau);
   
}
while x <((- PI as f64)) {
    x  = (x).py_add(tau);
   
}
let _cse_temp_1  = (x).py_mul(x);
    let x2: f64 = _cse_temp_1;
    let _cse_temp_2  = (x2).py_mul(x2);
    let x4: f64 = _cse_temp_2;
    let _cse_temp_3  = (x4).py_mul(x2);
    let x6: f64 = _cse_temp_3;
    let _cse_temp_4  = (x6).py_mul(x2);
    let x8: f64 = _cse_temp_4;
    Ok({ let _r: f64  = (((((((1.0) - ((x2).py_div(2.0)) as f64)).py_add((x4).py_div(24.0)) as f64)) - ((x6).py_div(720.0)) as f64)).py_add((x8).py_div(40320.0));
    _r })
}
#[doc = "Approximate acos(Abramowitz & Stegun polynomial)."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn acos_approx(mut x: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let mut negate: f64 = Default::default();
    let _cse_temp_0 = x< - 1.0;
    if _cse_temp_0 {
    x = - 1.0;
   
}
let _cse_temp_1 = x>1.0;
    if _cse_temp_1 {
    x = 1.0;
   
}
negate = 0.0;
    let _cse_temp_2 = x<0.0;
    if _cse_temp_2 {
    negate = PI;
    x = - x;
   
}
let _cse_temp_3  = (- 0.0187293).py_mul(x);
    let mut ret: f64 = _cse_temp_3.clone();
    ret  = (ret).py_add(0.074261);
    let _cse_temp_4  = (ret).py_mul(x);
    ret = _cse_temp_4;
    ret  = (ret) - (0.2121144);
    ret = _cse_temp_4;
    ret  = (ret).py_add(1.5707288);
    let sq: f64 = newton_sqrt((1.0) - (x)) ?;
    let _cse_temp_5  = (ret).py_mul(sq);
    ret = _cse_temp_5;
    let _cse_temp_6 = negate>0.0;
    if _cse_temp_6 {
    return Ok((negate).py_add(ret));
   
}
Ok(ret)
}
#[doc = "Euclidean distance between two 2D points."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let dx: f64  = (x2) - (x1);
    let dy: f64  = (y2) - (y1);
    newton_sqrt((((dx).py_mul(dx) as f64)).py_add((dy).py_mul(dy)))
}
#[doc = "Euclidean distance between two 3D points."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn distance_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let dx: f64  = (x2) - (x1);
    let dy: f64  = (y2) - (y1);
    let dz: f64  = (z2) - (z1);
    newton_sqrt((((((dx).py_mul(dx) as f64)).py_add((dy).py_mul(dy)) as f64)).py_add((dz).py_mul(dz)))
}
#[doc = "Manhattan(L1) distance between two 2D points."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn manhattan_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
   (float_abs((x2) - (x1))).py_add(float_abs((y2) - (y1)))
}
#[doc = "Chebyshev(L-infinity) distance between two 2D points."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn chebyshev_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let adx: f64 = float_abs((x2) - (x1));
    let ady: f64 = float_abs((y2) - (y1));
    let _cse_temp_0 = adx>= ady;
    if _cse_temp_0 {
    return adx;
   
}
ady
}
#[doc = "Find intersection point of two line segments, or None."] #[doc = " Depyler: proven to terminate"] pub fn line_intersection(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, x4: f64, y4: f64) -> Result<Option <(f64, f64)>, Box<dyn std::error::Error>>{
    let _cse_temp_0  = (((x1) - (x2) as f64)).py_mul((y3) - (y4));
    let _cse_temp_1  = (((y1) - (y2) as f64)).py_mul((x3) - (x4));
    let denom: f64  = (_cse_temp_0) - (_cse_temp_1);
    let _cse_temp_2 = float_abs(denom) <(EPSILON as f64);
    if _cse_temp_2 {
    return Ok(None);
   
}
let _cse_temp_3  = (((x1) - (x3) as f64)).py_mul((y3) - (y4));
    let _cse_temp_4  = (((y1) - (y3) as f64)).py_mul((x3) - (x4));
    let t_num: f64  = (_cse_temp_3) - (_cse_temp_4);
    let u_num: f64 = -(((((x1) - (x2) as f64)).py_mul((y1) - (y3)) as f64)) - ((((y1) - (y2) as f64)).py_mul((x1) - (x3)));
    let _cse_temp_5  = (t_num).py_div(denom);
    let t: f64 = _cse_temp_5;
    let _cse_temp_6  = (u_num).py_div(denom);
    let u: f64 = _cse_temp_6;
    let _cse_temp_7 = t<0.0;
    let _cse_temp_8 = t>1.0;
    let _cse_temp_9  = (_cse_temp_7) ||(_cse_temp_8);
    let _cse_temp_10 = u<0.0;
    let _cse_temp_11  = (_cse_temp_9) ||(_cse_temp_10);
    let _cse_temp_12 = u>1.0;
    let _cse_temp_13  = (_cse_temp_11) ||(_cse_temp_12);
    if _cse_temp_13 {
    return Ok(None);
   
}
let _cse_temp_14  = (t).py_mul((x2) - (x1));
    let ix: f64  = (x1).py_add(_cse_temp_14);
    let _cse_temp_15  = (t).py_mul((y2) - (y1));
    let iy: f64  = (y1).py_add(_cse_temp_15);
    Ok(Some((ix, iy)))
}
#[doc = "Compute area of a polygon using the shoelace formula."] #[doc = " Depyler: proven to terminate"] pub fn polygon_area<'a, 'b>(xs: & 'a Vec<f64>, ys: & 'b Vec<f64>) -> Result<f64, Box<dyn std::error::Error>>{
    let mut area: f64 = Default::default();
    let _cse_temp_0 = xs.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n<3;
    if _cse_temp_1 {
    return Ok(0.0);
   
}
area = 0.0;
    let mut j: i32  = ((n) - (1i32)) as i32;
    for i in 0..(n) {
    area  = (area).py_add((((xs.get(j as usize).cloned().expect("IndexError: list index out of range")).py_add(xs.get(i as usize).cloned().expect("IndexError: list index out of range")) as f64)).py_mul((ys.get(j as usize).cloned().expect("IndexError: list index out of range")) - (ys.get(i as usize).cloned().expect("IndexError: list index out of range"))));
    j = i;
   
}
Ok((float_abs(area)).py_div(2.0))
}
#[doc = "Compute area of the convex hull via gift wrapping + shoelace."] pub fn convex_hull_area<'a, 'b>(xs: & 'a Vec<f64>, ys: & 'b Vec<f64>) -> Result<f64, Box<dyn std::error::Error>>{
    let mut start: i32 = Default::default();
    let mut candidate: i32 = Default::default();
    let _cse_temp_0 = xs.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n<3;
    if _cse_temp_1 {
    return Ok(0.0);
   
}
start = 0;
    for i in (1)..(n) {
    if xs.get(i as usize).cloned().expect("IndexError: list index out of range")<xs.get(start as usize).cloned().expect("IndexError: list index out of range") {
    start = i;
   
}
else {
    if(xs.get(i as usize).cloned().expect("IndexError: list index out of range") == xs.get(start as usize).cloned().expect("IndexError: list index out of range")) &&(ys.get(i as usize).cloned().expect("IndexError: list index out of range")<ys.get(start as usize).cloned().expect("IndexError: list index out of range")) {
    start = i;
   
}
}
}
let mut hull_xs: Vec<f64>= vec! [];
    let mut hull_ys: Vec<f64>= vec! [];
    let mut current: i32 = start.clone();
    loop {
    hull_xs.push(xs.get(current as usize).cloned().expect("IndexError: list index out of range"));
    hull_ys.push(ys.get(current as usize).cloned().expect("IndexError: list index out of range"));
    candidate = 0;
    for i in (1)..(n) {
    if candidate == current {
    candidate = i;
    continue;
   
}
let cross: f64  = (((((xs.get(candidate as usize).cloned().expect("IndexError: list index out of range")) - (xs.get(current as usize).cloned().expect("IndexError: list index out of range")) as f64)).py_mul((ys.get(i as usize).cloned().expect("IndexError: list index out of range")) - (ys.get(current as usize).cloned().expect("IndexError: list index out of range"))) as f64)) - ((((ys.get(candidate as usize).cloned().expect("IndexError: list index out of range")) - (ys.get(current as usize).cloned().expect("IndexError: list index out of range")) as f64)).py_mul((xs.get(i as usize).cloned().expect("IndexError: list index out of range")) - (xs.get(current as usize).cloned().expect("IndexError: list index out of range"))));
    if cross<0.0 {
    candidate = i;
   
}
} current = candidate;
    if current == start {
    break;
   
}
} polygon_area(& hull_xs, & hull_ys)
}
#[doc = "Test if point(px, py) is inside polygon using ray casting."] #[doc = " Depyler: proven to terminate"] pub fn point_in_polygon<'a, 'b>(px: f64, py: f64, xs: & 'a Vec<f64>, ys: & 'b Vec<f64>) -> Result<bool, Box<dyn std::error::Error>>{
    let mut inside: bool = Default::default();
    let _cse_temp_0 = xs.len() as i32;
    let n: i32 = _cse_temp_0;
    inside = false;
    let mut j: i32  = ((n) - (1i32)) as i32;
    for i in 0..(n) {
    if((ys.get(i as usize).cloned().expect("IndexError: list index out of range") as f64)>py) != ((ys.get(j as usize).cloned().expect("IndexError: list index out of range") as f64)>py) {
    let slope: f64  = (((((((xs.get(j as usize).cloned().expect("IndexError: list index out of range")) - (xs.get(i as usize).cloned().expect("IndexError: list index out of range")) as f64)).py_mul((py) - (ys.get(i as usize).cloned().expect("IndexError: list index out of range"))) as f64)).py_div((ys.get(j as usize).cloned().expect("IndexError: list index out of range")) - (ys.get(i as usize).cloned().expect("IndexError: list index out of range"))) as f64)).py_add(xs.get(i as usize).cloned().expect("IndexError: list index out of range"));
    if px<slope {
    inside = ! inside;
   
}
} j = i;
   
}
Ok(inside)
}
#[doc = "Area of triangle from coordinates."] #[doc = " Depyler: proven to terminate"] pub fn triangle_area_coords(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let _cse_temp_0  = (x1).py_mul((y2) - (y3));
    let _cse_temp_1  = (x2).py_mul((y3) - (y1));
    let _cse_temp_2  = (x3).py_mul((y1) - (y2));
    let _cse_temp_3  = (((_cse_temp_0).py_add(_cse_temp_1) as f64)).py_add(_cse_temp_2);
    let area: f64 = _cse_temp_3;
    Ok((float_abs(area)).py_div(2.0))
}
#[doc = "Classify triangle by sides and angles combined."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn triangle_classify(a: f64, b: f64, c: f64) -> String {
    let mut s3: f64 = Default::default();
    let mut s1: f64 = Default::default();
    let mut s2: f64 = Default::default();
    let _cse_temp_0 = a <= 0.0;
    let _cse_temp_1 = b <= 0.0;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    let _cse_temp_3 = c <= 0.0;
    let _cse_temp_4  = (_cse_temp_2) ||(_cse_temp_3);
    if _cse_temp_4 {
    return "invalid".to_string().to_string();
   
}
let _cse_temp_5  = (a).py_add(b) <= c;
    let _cse_temp_6  = (b).py_add(c) <= a;
    let _cse_temp_7  = (_cse_temp_5) ||(_cse_temp_6);
    let _cse_temp_8  = (a).py_add(c) <= b;
    let _cse_temp_9  = (_cse_temp_7) ||(_cse_temp_8);
    if _cse_temp_9 {
    return "invalid".to_string().to_string();
   
}
let _cse_temp_10  = (float_eq(a, b)) &&(float_eq(b, c));
    if _cse_temp_10 {
    return "equilateral".to_string().to_string();
   
}
s1 = a;
    s2 = b;
    s3 = c;
    let _cse_temp_11 = s1>s2;
    if _cse_temp_11 {
    let tmp: f64 = s1;
    s1 = s2;
    s2 = tmp;
   
}
let _cse_temp_12 = s2>s3;
    if _cse_temp_12 {
    let tmp2: f64 = s2;
    s2 = s3;
    s3 = tmp2;
   
}
if _cse_temp_11 {
    let tmp3: f64 = s1;
    s1 = s2;
    s2 = tmp3;
   
}
let _cse_temp_13  = (s1).py_mul(s1);
    let _cse_temp_14  = (s2).py_mul(s2);
    let sq_sum: f64  = (_cse_temp_13).py_add(_cse_temp_14);
    let _cse_temp_15  = (s3).py_mul(s3);
    let sq_big: f64 = _cse_temp_15;
    if float_eq(sq_big, sq_sum) {
    return "right".to_string().to_string();
   
}
let _cse_temp_16 = sq_big>sq_sum;
    if _cse_temp_16 {
    return "obtuse".to_string().to_string();
   
}
"acute".to_string().to_string()
}
#[doc = "Triangle area using Heron's formula."] #[doc = " Depyler: proven to terminate"] pub fn heron_area(a: f64, b: f64, c: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let _cse_temp_0  = (((a).py_add(b) as f64)).py_add(c);
    let _cse_temp_1  = (_cse_temp_0).py_div(2.0);
    let s: f64 = _cse_temp_1;
    let _cse_temp_2  = (s).py_mul((s) - (a));
    let _cse_temp_3  = (_cse_temp_2).py_mul((s) - (b));
    let _cse_temp_4  = (_cse_temp_3).py_mul((s) - (c));
    let val: f64 = _cse_temp_4;
    let _cse_temp_5 = val<0.0;
    if _cse_temp_5 {
    return Ok(0.0);
   
}
newton_sqrt(val)
}
#[doc = "Area of a circle."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn circle_area(radius: f64) -> f64 {
    { let _r: f64  = (((PI).py_mul(radius) as f64)).py_mul(radius);
    _r
}
} #[doc = "Check if two circles intersect or touch."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn circles_intersect(x1: f64, y1: f64, r1: f64, x2: f64, y2: f64, r2: f64) -> Result<bool, Box<dyn std::error::Error>>{
    let d: f64 = distance_2d(x1, y1, x2, y2) ?;
    Ok((d<= (r1).py_add(r2)) &&(d>= float_abs((r1) - (r2))))
}
#[doc = "Determinant of a 2x2 matrix [[a,b],[c,d]]."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn det_2x2(a: f64, b: f64, c: f64, d: f64) -> f64 {
    { let _r: f64  = (((a).py_mul(d) as f64)) - ((b).py_mul(c));
    _r
}
} #[doc = "Determinant of a 3x3 matrix [[a,b,c],[d,e,f],[g,h,k]]."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn det_3x3(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, k: f64) -> f64 {
    { let _r: f64  = (((((a).py_mul((((e).py_mul(k) as f64)) - ((f).py_mul(h))) as f64)) - ((b).py_mul((((d).py_mul(k) as f64)) - ((f).py_mul(g)))) as f64)).py_add((c).py_mul((((d).py_mul(h) as f64)) - ((e).py_mul(g))));
    _r
}
} #[doc = "Dot product of two vectors."] #[doc = " Depyler: proven to terminate"] pub fn dot_product<'a, 'b>(xs: & 'a Vec<f64>, ys: & 'b Vec<f64>) -> Result<f64, Box<dyn std::error::Error>>{
    let mut result: f64 = Default::default();
    result = 0.0;
    let _cse_temp_0 = xs.len() as i32;
    let n: i32 = _cse_temp_0;
    for i in 0..(n) {
    result  = (result).py_add((xs.get(i as usize).cloned().expect("IndexError: list index out of range")).py_mul(ys.get(i as usize).cloned().expect("IndexError: list index out of range")));
   
}
Ok(result)
}
#[doc = "Cross product of two 3D vectors."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn cross_product_3d(ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64) -> (f64, f64, f64) {
    let _cse_temp_0  = (ay).py_mul(bz);
    let _cse_temp_1  = (az).py_mul(by);
    let cx: f64  = (_cse_temp_0) - (_cse_temp_1);
    let _cse_temp_2  = (az).py_mul(bx);
    let _cse_temp_3  = (ax).py_mul(bz);
    let cy: f64  = (_cse_temp_2) - (_cse_temp_3);
    let _cse_temp_4  = (ax).py_mul(by);
    let _cse_temp_5  = (ay).py_mul(bx);
    let cz: f64  = (_cse_temp_4) - (_cse_temp_5);
   (cx, cy, cz)
}
#[doc = "Normalize a vector to unit length."] #[doc = " Depyler: verified panic-free"] pub fn vector_normalize(xs: Vec<f64>) -> Result<Vec<f64>, Box<dyn std::error::Error>>{
    let mut sq_sum: f64 = Default::default();
    sq_sum = 0.0;
    for x in xs.iter().cloned() {
    sq_sum  = (sq_sum).py_add((x).py_mul(x));
   
}
let mag: f64 = newton_sqrt(sq_sum) ?;
    let _cse_temp_0 = mag <(EPSILON as f64);
    if _cse_temp_0 {
    return Ok(xs);
   
}
let mut result: Vec<f64>= vec! [];
    for x in xs.iter().cloned() {
    result.push((x).py_div(mag));
   
}
Ok(result)
}
#[doc = "Evaluate polynomial using Horner's method.\n\n    coeffs[0] is the highest degree coefficient.\n    "] #[doc = " Depyler: verified panic-free"] pub fn horner_eval(coeffs: & Vec<f64>, x: f64) -> f64 {
    let mut result: f64 = Default::default();
    result = 0.0;
    for c in coeffs.iter().cloned() {
    result  = (((result).py_mul(x) as f64)).py_add(c);
   
}
result
}
#[doc = "Approximate integral of polynomial using the trapezoidal rule."] #[doc = " Depyler: proven to terminate"] pub fn trapezoidal_rule(coeffs: & Vec<f64>, a: f64, b: f64, n: i32) -> Result<f64, Box<dyn std::error::Error>>{
    let mut result: f64 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
    return Ok(0.0);
   
}
let _cse_temp_1  = (n) as f64;
    let _cse_temp_2  = (((b) - (a) as f64)).py_div(_cse_temp_1);
    let h: f64 = _cse_temp_2;
    let _cse_temp_3  = (horner_eval(& coeffs, a)).py_add(horner_eval(& coeffs, b));
    let _cse_temp_4  = (_cse_temp_3).py_div(2.0);
    result = _cse_temp_4;
    for i in (1)..(n) {
    let x: f64  = (a).py_add(((i) as f64).py_mul(h));
    result  = (result).py_add(horner_eval(& coeffs, x));
   
}
Ok((result).py_mul(h))
}
#[doc = "Approximate integral of polynomial using Simpson's 1/3 rule."] #[doc = " Depyler: proven to terminate"] pub fn simpson_rule(coeffs: & Vec<f64>, a: f64, b: f64, n: i32) -> Result<f64, Box<dyn std::error::Error>>{
    let mut result: f64 = Default::default();
    let _cse_temp_0 = n <= 0;
    let _cse_temp_1  = ((n).py_mod(2i32)) as i32;
    let _cse_temp_2 = _cse_temp_1 != 0;
    let _cse_temp_3  = (_cse_temp_0) ||(_cse_temp_2);
    if _cse_temp_3 {
    return Ok(0.0);
   
}
let _cse_temp_4  = (n) as f64;
    let _cse_temp_5  = (((b) - (a) as f64)).py_div(_cse_temp_4);
    let h: f64 = _cse_temp_5;
    let _cse_temp_6  = (horner_eval(& coeffs, a)).py_add(horner_eval(& coeffs, b));
    result = _cse_temp_6;
    for i in (1)..(n) {
    let x: f64  = (a).py_add(((i) as f64).py_mul(h));
    if(i).py_mod(2i32) == 0 {
    result  = (result).py_add((2.0).py_mul(horner_eval(& coeffs, x)));
   
}
else {
    result  = (result).py_add((4.0).py_mul(horner_eval(& coeffs, x)));
   
}
} Ok({ let _r: f64  = (((result).py_mul(h) as f64)).py_div(3.0);
    _r })
}
#[doc = "Linear interpolation between a and b."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    { let _r: f64  = (a).py_add((((b) - (a) as f64)).py_mul(t));
    _r
}
} #[doc = "Quadratic Bezier curve point at parameter t."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn bezier_quadratic(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> (f64, f64) {
    let u: f64  = (1.0) - (t);
    let _cse_temp_0  = (u).py_mul(u);
    let _cse_temp_1  = (_cse_temp_0).py_mul(x0);
    let _cse_temp_2  = (2.0).py_mul(u);
    let _cse_temp_3  = (_cse_temp_2).py_mul(t);
    let _cse_temp_4  = (_cse_temp_3).py_mul(x1);
    let _cse_temp_5  = (t).py_mul(t);
    let _cse_temp_6  = (_cse_temp_5).py_mul(x2);
    let _cse_temp_7  = (((_cse_temp_1).py_add(_cse_temp_4) as f64)).py_add(_cse_temp_6);
    let bx: f64 = _cse_temp_7;
    let _cse_temp_8  = (_cse_temp_0).py_mul(y0);
    let _cse_temp_9  = (_cse_temp_3).py_mul(y1);
    let _cse_temp_10  = (_cse_temp_5).py_mul(y2);
    let _cse_temp_11  = (((_cse_temp_8).py_add(_cse_temp_9) as f64)).py_add(_cse_temp_10);
    let by: f64 = _cse_temp_11;
   (bx, by)
}
#[doc = "Cubic Bezier curve point at parameter t."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn bezier_cubic(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, t: f64) -> (f64, f64) {
    let u: f64  = (1.0) - (t);
    let _cse_temp_0  = (u).py_mul(u);
    let u2: f64 = _cse_temp_0;
    let _cse_temp_1  = (u2).py_mul(u);
    let u3: f64 = _cse_temp_1;
    let _cse_temp_2  = (t).py_mul(t);
    let t2: f64 = _cse_temp_2;
    let _cse_temp_3  = (t2).py_mul(t);
    let t3: f64 = _cse_temp_3;
    let _cse_temp_4  = (u3).py_mul(x0);
    let _cse_temp_5  = (3.0).py_mul(u2);
    let _cse_temp_6  = (_cse_temp_5).py_mul(t);
    let _cse_temp_7  = (_cse_temp_6).py_mul(x1);
    let _cse_temp_8  = (3.0).py_mul(u);
    let _cse_temp_9  = (_cse_temp_8).py_mul(t2);
    let _cse_temp_10  = (_cse_temp_9).py_mul(x2);
    let _cse_temp_11  = (((_cse_temp_4).py_add(_cse_temp_7) as f64)).py_add(_cse_temp_10);
    let _cse_temp_12  = (t3).py_mul(x3);
    let bx: f64  = (_cse_temp_11).py_add(_cse_temp_12);
    let _cse_temp_13  = (u3).py_mul(y0);
    let _cse_temp_14  = (_cse_temp_6).py_mul(y1);
    let _cse_temp_15  = (_cse_temp_9).py_mul(y2);
    let _cse_temp_16  = (((_cse_temp_13).py_add(_cse_temp_14) as f64)).py_add(_cse_temp_15);
    let _cse_temp_17  = (t3).py_mul(y3);
    let by: f64  = (_cse_temp_16).py_add(_cse_temp_17);
   (bx, by)
}
#[doc = "Convert degrees to radians."] #[doc = " Depyler: proven to terminate"] pub fn degrees_to_radians(deg: f64) -> Result<f64, Box<dyn std::error::Error>>{
    Ok({ let _r: f64  = (((deg).py_mul(PI) as f64)).py_div(180.0);
    _r })
}
#[doc = "Convert radians to degrees."] #[doc = " Depyler: proven to terminate"] pub fn radians_to_degrees(rad: f64) -> Result<f64, Box<dyn std::error::Error>>{
    Ok({ let _r: f64  = (((rad).py_mul(180.0) as f64)).py_div(PI);
    _r })
}
#[doc = "Angle in radians between two 2D vectors."] #[doc = " Depyler: proven to terminate"] pub fn angle_between_vectors(ax: f64, ay: f64, bx: f64, by: f64) -> Result<f64, Box<dyn std::error::Error>>{
    let _cse_temp_0  = (ax).py_mul(bx);
    let _cse_temp_1  = (ay).py_mul(by);
    let dot: f64  = (_cse_temp_0).py_add(_cse_temp_1);
    let mag_a: f64 = newton_sqrt((((ax).py_mul(ax) as f64)).py_add((ay).py_mul(ay))) ?;
    let mag_b: f64 = newton_sqrt((((bx).py_mul(bx) as f64)).py_add((by).py_mul(by))) ?;
    let _cse_temp_2 = mag_a <(EPSILON as f64);
    let _cse_temp_3 = mag_b <(EPSILON as f64);
    let _cse_temp_4  = (_cse_temp_2) ||(_cse_temp_3);
    if _cse_temp_4 {
    return Ok(0.0);
   
}
let _cse_temp_5  = (mag_a).py_mul(mag_b);
    let _cse_temp_6  = (dot).py_div(_cse_temp_5);
    let cos_val: f64 = _cse_temp_6;
    acos_approx(cos_val)
}
#[doc = "Convert polar coordinates to cartesian."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn polar_to_cartesian(r: f64, theta: f64) -> Result <(f64, f64), Box<dyn std::error::Error>>{
    let _cse_temp_0  = (r).py_mul(cos_approx(theta) ?);
    let x: f64 = _cse_temp_0;
    let _cse_temp_1  = (r).py_mul(sin_approx(theta) ?);
    let y: f64 = _cse_temp_1;
    Ok((x, y))
}
#[doc = "Rotate a 2D point around the origin by angle(radians)."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn rotate_point(x: f64, y: f64, angle: f64) -> Result <(f64, f64), Box<dyn std::error::Error>>{
    let cos_a: f64 = cos_approx(angle) ?;
    let sin_a: f64 = sin_approx(angle) ?;
    let _cse_temp_0  = (x).py_mul(cos_a);
    let _cse_temp_1  = (y).py_mul(sin_a);
    let rx: f64  = (_cse_temp_0) - (_cse_temp_1);
    let _cse_temp_2  = (x).py_mul(sin_a);
    let _cse_temp_3  = (y).py_mul(cos_a);
    let ry: f64  = (_cse_temp_2).py_add(_cse_temp_3);
    Ok((rx, ry))
}
#[doc = "Weighted average with int weights and float values."] #[doc = " Depyler: proven to terminate"] pub fn weighted_average<'b, 'a>(values: & 'a Vec<f64>, weights: & 'b Vec<i32>) -> Result<f64, Box<dyn std::error::Error>>{
    let mut total: f64 = Default::default();
    let mut weight_sum: i32 = Default::default();
    total = 0.0;
    weight_sum = 0;
    let _cse_temp_0 = values.len() as i32;
    let n: i32 = _cse_temp_0;
    for i in 0..(n) {
    total  = (total).py_add((values.get(i as usize).cloned().expect("IndexError: list index out of range")).py_mul((weights.get(i as usize).cloned().expect("IndexError: list index out of range")) as f64));
    weight_sum  = ((weight_sum).py_add(weights.get(i as usize).cloned().expect("IndexError: list index out of range"))) as i32;
   
}
let _cse_temp_1 = weight_sum == 0;
    if _cse_temp_1 {
    return Ok(0.0);
   
}
Ok((total).py_div((weight_sum) as f64))
}
#[doc = "Distance between integer-coordinate points returning float."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn int_to_float_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> Result<f64, Box<dyn std::error::Error>>{
    let _cse_temp_0  = ((x2) - (x1)) as f64;
    let dx: f64 = _cse_temp_0;
    let _cse_temp_1  = ((y2) - (y1)) as f64;
    let dy: f64 = _cse_temp_1;
    newton_sqrt((((dx).py_mul(dx) as f64)).py_add((dy).py_mul(dy)))
}
#[doc = "Count integer lattice points within a circle of given radius."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn grid_point_count(radius: i32) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    for x in (- radius)..((radius).py_add(1i32)) {
    for y in (- radius)..((radius).py_add(1i32)) {
    let dist_sq: i32  = ((((x).py_mul(x) as i32)).py_add((y).py_mul(y))) as i32;
    if dist_sq<= (radius).py_mul(radius) {
    count  = ((count).py_add(1i32)) as i32;
   
}
}
}
count
}
#[doc = "Normalize an integer vector to unit length(returns floats)."] #[doc = " Depyler: verified panic-free"] pub fn normalize_int_vector(xs: & Vec<i32>) -> Result<Vec<f64>, Box<dyn std::error::Error>>{
    let mut sq_sum: f64 = Default::default();
    sq_sum = 0.0;
    for x in xs.iter().cloned() {
    sq_sum  = (sq_sum).py_add(((x) as f64).py_mul((x) as f64));
   
}
let mag: f64 = newton_sqrt(sq_sum) ?;
    let _cse_temp_0 = mag <(EPSILON as f64);
    if _cse_temp_0 {
    let mut result: Vec<f64>= vec! [];
    for x in xs.iter().cloned() {
    result.push((x) as f64);
   
}
return Ok(result);
   
}
let mut result2: Vec<f64>= vec! [];
    for x in xs.iter().cloned() {
    result2.push(((x) as f64).py_div(mag));
   
}
Ok(result2)
}
#[doc = "Generate points on a line using Bresenham's algorithm."] #[doc = " Depyler: proven to terminate"] pub fn discrete_line_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Result<Vec <(i32, i32)>, Box<dyn std::error::Error>>{
    let mut dx: i32 = Default::default();
    let mut dy: i32 = Default::default();
    let mut points: Vec <(i32, i32)>= vec! [];
    dx  = ((x2) - (x1)) as i32;
    dy  = ((y2) - (y1)) as i32;
    let sx: i32 = if dx>0 {
    1
}
else {
    - 1 };
    let sy: i32 = if dy>0 {
    1
}
else {
    - 1 };
    let _cse_temp_0 = dx<0;
    if _cse_temp_0 {
    dx = - dx;
   
}
let _cse_temp_1 = dy<0;
    if _cse_temp_1 {
    dy = - dy;
   
}
let mut cx: i32 = x1.clone();
    let mut cy: i32 = y1.clone();
    let _cse_temp_2 = dx>= dy;
    if _cse_temp_2 {
    let _cse_temp_3 = {
    let a = dx;
    let b = 2;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
    if needs_adjustment {
    q - 1
}
else {
    q
}
};
    let mut err: i32 = _cse_temp_3.clone();
    for __sanitized in 0..((dx).py_add(1i32)) {
    points.push((cx, cy));
    err  = ((err) - (dy)) as i32;
    if err<0 {
    cy  = ((cy).py_add(sy)) as i32;
    err  = ((err).py_add(dx)) as i32;
   
}
cx  = ((cx).py_add(sx)) as i32;
   
}
} else {
    let _cse_temp_4 = {
    let a = dy;
    let b = 2;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
    if needs_adjustment {
    q - 1
}
else {
    q
}
};
    let mut err2: i32 = _cse_temp_4.clone();
    for __sanitized in 0..((dy).py_add(1i32)) {
    points.push((cx, cy));
    err2  = ((err2) - (dx)) as i32;
    if err2<0 {
    cx  = ((cx).py_add(sx)) as i32;
    err2  = ((err2).py_add(dy)) as i32;
   
}
cy  = ((cy).py_add(sy)) as i32;
   
}
} Ok(points)
}
#[doc = "Test every function in this module for correctness."] #[doc = " Depyler: proven to terminate"] pub fn test_all() -> Result<bool, Box<dyn std::error::Error>>{
    let mut ok: bool = Default::default();
    ok = true;
    if ! float_eq(newton_sqrt(4.0) ?, 2.0) {
    ok = false;
   
}
if ! float_eq(newton_sqrt(9.0) ?, 3.0) {
    ok = false;
   
}
if ! float_eq(newton_sqrt(0.0) ?, 0.0) {
    ok = false;
   
}
if ! float_eq(distance_2d(0.0, 0.0, 3.0, 4.0) ?, 5.0) {
    ok = false;
   
}
if ! float_eq(distance_3d(0.0, 0.0, 0.0, 1.0, 2.0, 2.0) ?, 3.0) {
    ok = false;
   
}
if ! float_eq(manhattan_distance(0.0, 0.0, 3.0, 4.0), 7.0) {
    ok = false;
   
}
if ! float_eq(chebyshev_distance(0.0, 0.0, 3.0, 4.0), 4.0) {
    ok = false;
   
}
let isect: Option <(f64, f64)>= line_intersection(0.0, 0.0, 2.0, 2.0, 0.0, 2.0, 2.0, 0.0) ?;
    if false {
    ok = false;
   
}
else {
    let _cse_temp_0  = (! float_eq(isect.unwrap().0, 1.0)) ||(! float_eq(isect.unwrap().1, 1.0));
    if _cse_temp_0 {
    ok = false;
   
}
} if line_intersection(0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 2.0) ?.is_some() {
    ok = false;
   
}
let sq_xs: Vec<f64>= vec! [0.0, 1.0, 1.0, 0.0];
    let sq_ys: Vec<f64>= vec! [0.0, 0.0, 1.0, 1.0];
    if ! float_eq(polygon_area(& sq_xs, & sq_ys) ?, 1.0) {
    ok = false;
   
}
let hull_xs: Vec<f64>= vec! [0.0, 1.0, 2.0, 1.0, 0.5];
    let hull_ys: Vec<f64>= vec! [0.0, 0.0, 1.0, 2.0, 1.0];
    let hull_a: f64 = convex_hull_area(& hull_xs, & hull_ys) ?;
    let _cse_temp_1 = hull_a<1.5;
    let _cse_temp_2 = hull_a>4.5;
    let _cse_temp_3  = (_cse_temp_1) ||(_cse_temp_2);
    if _cse_temp_3 {
    ok = false;
   
}
if ! point_in_polygon(0.5, 0.5, & sq_xs, & sq_ys) ? {
    ok = false;
   
}
if point_in_polygon(5.0, 5.0, & sq_xs, & sq_ys).unwrap_or(false) {
    ok = false;
   
}
if ! float_eq(triangle_area_coords(0.0, 0.0, 4.0, 0.0, 0.0, 3.0) ?, 6.0) {
    ok = false;
   
}
let _cse_temp_4 = triangle_classify(1.0, 1.0, 1.0) != "equilateral".to_string();
    if _cse_temp_4 {
    ok = false;
   
}
let _cse_temp_5 = triangle_classify(3.0, 4.0, 5.0) != "right".to_string();
    if _cse_temp_5 {
    ok = false;
   
}
let _cse_temp_6 = triangle_classify(2.0, 2.0, 3.5) != "obtuse".to_string();
    if _cse_temp_6 {
    ok = false;
   
}
if ! float_eq(heron_area(3.0, 4.0, 5.0) ?, 6.0) {
    ok = false;
   
}
if ! float_eq(circle_area(1.0), PI) {
    ok = false;
   
}
if ! circles_intersect(0.0, 0.0, 1.0, 1.5, 0.0, 1.0) ? {
    ok = false;
   
}
if circles_intersect(0.0, 0.0, 1.0, 10.0, 0.0, 1.0).unwrap_or(false) {
    ok = false;
   
}
if ! float_eq(det_2x2(1.0, 2.0, 3.0, 4.0), - 2.0) {
    ok = false;
   
}
if ! float_eq(det_3x3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0), 1.0) {
    ok = false;
   
}
if ! float_eq(dot_product(& vec! [1.0, 2.0, 3.0], & vec! [4.0, 5.0, 6.0]) ?, 32.0) {
    ok = false;
   
}
let cp3 :(f64, f64, f64) = cross_product_3d(1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    if ! float_eq(cp3.2, 1.0) {
    ok = false;
   
}
let vn: Vec<f64>= vector_normalize(vec! [3.0, 4.0]) ?;
    let _cse_temp_7  = (! float_eq(vn.get(0usize).cloned().expect("IndexError: list index out of range"), 0.6)) ||(! float_eq(vn.get(1usize).cloned().expect("IndexError: list index out of range"), 0.8));
    if _cse_temp_7 {
    ok = false;
   
}
if ! float_eq(horner_eval(& vec! [2.0, 3.0, 1.0], 2.0), 15.0) {
    ok = false;
   
}
let trap: f64 = trapezoidal_rule(& vec! [1.0, 0.0], 0.0, 1.0, 1000) ?;
    let _cse_temp_8 = float_abs((trap) - (0.5))>0.001;
    if _cse_temp_8 {
    ok = false;
   
}
let simp: f64 = simpson_rule(& vec! [1.0, 0.0, 0.0], 0.0, 1.0, 100) ?;
    let _cse_temp_9 = float_abs((simp) - (0.3333333333333333))>0.001;
    if _cse_temp_9 {
    ok = false;
   
}
if ! float_eq(lerp(0.0, 10.0, 0.5), 5.0) {
    ok = false;
   
}
let bq0 :(f64, f64) = bezier_quadratic(0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0);
    if _cse_temp_7 {
    ok = false;
   
}
let bq1 :(f64, f64) = bezier_quadratic(0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0);
    if ! float_eq(bq1.0, 3.0) {
    ok = false;
   
}
let bc0 :(f64, f64) = bezier_cubic(0.0, 0.0, 1.0, 1.0, 2.0, 1.0, 3.0, 0.0, 0.0);
    if ! float_eq(bc0.0, 0.0) {
    ok = false;
   
}
if ! float_eq(degrees_to_radians(180.0) ?, PI) {
    ok = false;
   
}
if ! float_eq(radians_to_degrees(PI) ?, 180.0) {
    ok = false;
   
}
let abv: f64 = angle_between_vectors(1.0, 0.0, 0.0, 1.0) ?;
    let _cse_temp_10 = float_abs((abv) - ((PI).py_div(2.0)))>0.01;
    if _cse_temp_10 {
    ok = false;
   
}
let ptc :(f64, f64) = polar_to_cartesian(1.0, 0.0) ?;
    let _cse_temp_11 = float_abs((ptc.0) - (1.0))>0.01;
    let _cse_temp_12 = float_abs(ptc.1)>0.01;
    let _cse_temp_13  = (_cse_temp_11) ||(_cse_temp_12);
    if _cse_temp_13 {
    ok = false;
   
}
let rp :(f64, f64) = rotate_point(1.0, 0.0 ,(PI).py_div(2.0)) ?;
    let _cse_temp_14  = (_cse_temp_12) ||(_cse_temp_11);
    if _cse_temp_14 {
    ok = false;
   
}
let _cse_temp_15 = float_abs(sin_approx(0.0) ?)>0.001;
    if _cse_temp_15 {
    ok = false;
   
}
let _cse_temp_16 = float_abs((sin_approx((PI).py_div(2.0)) ?) - (1.0))>0.001;
    if _cse_temp_16 {
    ok = false;
   
}
let _cse_temp_17 = float_abs((cos_approx(0.0) ?) - (1.0))>0.001;
    if _cse_temp_17 {
    ok = false;
   
}
if ! float_eq(weighted_average(& vec! [1.0, 2.0, 3.0], & vec! [1, 1, 1]) ?, 2.0) {
    ok = false;
   
}
if ! float_eq(weighted_average(& vec! [10.0, 20.0], & vec! [1, 3]) ?, 17.5) {
    ok = false;
   
}
if ! float_eq(int_to_float_distance(0, 0, 3, 4) ?, 5.0) {
    ok = false;
   
}
let _cse_temp_18 = grid_point_count(1) != 5;
    if _cse_temp_18 {
    ok = false;
   
}
let niv: Vec<f64>= normalize_int_vector(& vec! [3, 4]) ?;
    let _cse_temp_19 = float_abs((niv.get(0usize).cloned().expect("IndexError: list index out of range")) - (0.6))>0.01;
    let _cse_temp_20 = float_abs((niv.get(1usize).cloned().expect("IndexError: list index out of range")) - (0.8))>0.01;
    let _cse_temp_21  = (_cse_temp_19) ||(_cse_temp_20);
    if _cse_temp_21 {
    ok = false;
   
}
let dlp: Vec <(i32, i32)>= discrete_line_points(0, 0, 3, 0) ?;
    let _cse_temp_22 = dlp.len() as i32;
    let _cse_temp_23 = _cse_temp_22 != 4;
    if _cse_temp_23 {
    ok = false;
   
}
Ok(ok)
}
#[doc = r" DEPYLER-1216: Auto-generated entry point wrapping top-level script statements"] #[doc = r" This file was transpiled from a Python script with executable top-level code."] pub fn main () -> Result <(), Box<dyn std::error::Error>>{
    Ok(())
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn quickcheck_float_abs() {
    fn prop(x: f64) -> TestResult {
    let result = float_abs(x.clone());
    if result<0 {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(f64) -> TestResult);
   
}
#[test] fn quickcheck_vector_normalize() {
    fn prop(xs: Vec<f64>) -> TestResult {
    let once = vector_normalize(& xs);
    let twice = vector_normalize(once.clone());
    if once != twice {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<f64>) -> TestResult);
   
}
#[test] fn test_vector_normalize_examples() {
    assert_eq!(vector_normalize(vec! []), vec! []);
    assert_eq!(vector_normalize(vec! [1]), vec! [1]);
   
}
#[test] fn test_grid_point_count_examples() {
    assert_eq!(grid_point_count(0), 0);
    assert_eq!(grid_point_count(1), 1);
    assert_eq!(grid_point_count(- 1), - 1);
   
}
#[test] fn quickcheck_normalize_int_vector() {
    fn prop(xs: Vec<i32>) -> TestResult {
    let once = normalize_int_vector(& xs);
    let twice = normalize_int_vector(once.clone());
    if once != twice {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_normalize_int_vector_examples() {
    assert_eq!(normalize_int_vector(vec! []), vec! []);
    assert_eq!(normalize_int_vector(vec! [1]), vec! [1]);
   
}
}