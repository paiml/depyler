#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
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
    _dv_dict.insert(DepylerValue::Str(key.to_string()), DepylerValue::Str(value.to_string()));
   
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
} #[doc = "Multiply two fixed-point numbers with proper scaling."] #[doc = " Depyler: proven to terminate"] pub fn fixed_multiply(mut a: i32, mut b: i32, scale: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut sign: i32 = Default::default();
    let _cse_temp_0 = scale == 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
sign = 1;
    let _cse_temp_1 = a<0;
    if _cse_temp_1 {
    sign = - sign;
    a = - a;
   
}
let _cse_temp_2 = b<0;
    if _cse_temp_2 {
    sign = - sign;
    b = - b;
   
}
let _cse_temp_3 = {
    let a = a;
    let b = scale;
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
    let high_a: i32 = _cse_temp_3;
    let _cse_temp_4  = ((a).py_mod(scale)) as i32;
    let low_a: i32 = _cse_temp_4;
    let _cse_temp_5 = {
    let a = b;
    let b = scale;
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
    let high_b: i32 = _cse_temp_5;
    let _cse_temp_6  = ((b).py_mod(scale)) as i32;
    let low_b: i32 = _cse_temp_6;
    let _cse_temp_7  = ((high_a).py_mul(high_b)) as i32;
    let _cse_temp_8  = ((_cse_temp_7).py_mul(scale)) as i32;
    let _cse_temp_9  = ((high_a).py_mul(low_b)) as i32;
    let _cse_temp_10  = ((low_a).py_mul(high_b)) as i32;
    let _cse_temp_11  = ((((_cse_temp_8).py_add(_cse_temp_9) as i32)).py_add(_cse_temp_10)) as i32;
    let _cse_temp_12  = ((low_a).py_mul(low_b)) as i32;
    let _cse_temp_13 = {
    let a = _cse_temp_12;
    let b = scale;
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
    let result: i32  = ((_cse_temp_11).py_add(_cse_temp_13)) as i32;
    Ok((sign).py_mul(result))
}
#[doc = "Divide two fixed-point numbers."] #[doc = " Depyler: proven to terminate"] pub fn fixed_divide(mut a: i32, mut b: i32, scale: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut sign: i32 = Default::default();
    let _cse_temp_0 = b == 0;
    let _cse_temp_1 = scale == 0;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return Ok(0);
   
}
sign = 1;
    let _cse_temp_3 = a<0;
    if _cse_temp_3 {
    sign = - sign;
    a = - a;
   
}
let _cse_temp_4 = b<0;
    if _cse_temp_4 {
    sign = - sign;
    b = - b;
   
}
let _cse_temp_5  = ((a).py_mul(scale)) as i32;
    let _cse_temp_6 = {
    let a = _cse_temp_5;
    let b = b;
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
    let result: i32 = _cse_temp_6;
    Ok((sign).py_mul(result))
}
#[doc = "Integer square root of fixed-point number using Newton's method."] pub fn fixed_sqrt(x: i32, scale: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut guess: i32 = Default::default();
    let _cse_temp_0 = x <= 0;
    let _cse_temp_1 = scale <= 0;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return Ok(0);
   
}
guess = x;
    let mut prev: i32 = 0;
    let mut iterations: i32 = 0;
    while(guess != prev) &&(iterations<100) {
    prev = guess;
    if guess == 0 {
    return Ok(0);
   
}
guess = {
    let a  = (guess).py_add(fixed_divide(x, guess, scale) ?);
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
    iterations  = ((iterations).py_add(1i32)) as i32;
   
}
Ok(guess)
}
#[doc = "Approximate e^x using Taylor series in fixed-point."] pub fn fixed_exp_approx(x: i32, scale: i32, terms: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let _cse_temp_0 = scale == 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
result = scale;
    let mut term: i32 = scale.clone();
    let mut i: i32 = 1;
    while i <= terms {
    term = {
    let a = fixed_multiply(term, x, scale) ?;
    let b = i;
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
    if term == 0 {
    break;
   
}
result  = ((result).py_add(term)) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Test fixed-point arithmetic operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_fixed_point() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let scale: i32 = 1000;
    let r1: i32 = fixed_multiply(1500, 2000, scale) ?;
    let _cse_temp_0 = r1 == 3000;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r2: i32 = fixed_multiply(- 1500, 2000, scale) ?;
    let _cse_temp_1 = r2 == - 3000;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r3: i32 = fixed_divide(3000, 1500, scale) ?;
    let _cse_temp_2 = r3 == 2000;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r4: i32 = fixed_divide(0, 1500, scale) ?;
    let _cse_temp_3 = r4 == 0;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r5: i32 = fixed_divide(1000, 0, scale) ?;
    let _cse_temp_4 = r5 == 0;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r6: i32 = fixed_sqrt(4000, scale) ?;
    let _cse_temp_5 = r6>= 1990;
    let _cse_temp_6 = r6 <= 2010;
    let _cse_temp_7  = (_cse_temp_5) &&(_cse_temp_6);
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r7: i32 = fixed_sqrt(0, scale) ?;
    let _cse_temp_8 = r7 == 0;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r8: i32 = fixed_exp_approx(0, scale, 10) ?;
    let _cse_temp_9 = r8 == scale;
    if _cse_temp_9 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Count set bits in non-negative integer."] #[doc = " Depyler: verified panic-free"] pub fn popcount(mut n: i32) -> i32 {
    let mut count: i32 = Default::default();
    let _cse_temp_0 = n<0;
    if _cse_temp_0 {
    n = - n;
   
}
count = 0;
    while n>0 {
    count  = ((count).py_add(n & 1)) as i32;
    n = n>>1;
   
}
count
}
#[doc = "Return 0 if even number of set bits, 1 if odd."] #[doc = " Depyler: verified panic-free"] pub fn parity(mut n: i32) -> i32 {
    let mut p: i32 = Default::default();
    let _cse_temp_0 = n<0;
    if _cse_temp_0 {
    n = - n;
   
}
p = 0;
    while n>0 {
    p = p ^ n & 1;
    n = n>>1;
   
}
p
}
#[doc = "Count leading zeros in a 32-bit representation."] #[doc = " Depyler: verified panic-free"] pub fn leading_zeros_32(n: i32) -> i32 {
    let mut count: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return 32;
   
}
return 0;
   
}
count = 0;
    let _cse_temp_2 = 1 <<31;
    let mut mask: i32 = _cse_temp_2.clone();
    while(mask>0) &&(n & mask == 0) {
    count  = ((count).py_add(1i32)) as i32;
    mask = mask>>1;
   
}
count
}
#[doc = "Count trailing zeros."] #[doc = " Depyler: verified panic-free"] pub fn trailing_zeros(mut n: i32) -> i32 {
    let mut count: i32 = Default::default();
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
    return 32;
   
}
let _cse_temp_1 = n<0;
    if _cse_temp_1 {
    n = - n;
   
}
count = 0;
    while n & 1 == 0 {
    count  = ((count).py_add(1i32)) as i32;
    n = n>>1;
   
}
count
}
#[doc = "Reverse the bits of a 32-bit integer."] #[doc = " Depyler: verified panic-free"] pub fn reverse_bits_32(mut n: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n<0;
    if _cse_temp_0 {
    let _cse_temp_1 = n & 4294967295;
    n = _cse_temp_1;
   
}
result = 0;
    let mut i: i32 = 0;
    while i<32 {
    result = result <<1 | n & 1;
    n = n>>1;
    i  = ((i).py_add(1i32)) as i32;
   
}
result
}
#[doc = "Find the next power of two>= n."] #[doc = " Depyler: verified panic-free"] pub fn next_power_of_two(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
    return 1;
   
}
let _cse_temp_1 = n &(n) - (1i32);
    let _cse_temp_2 = _cse_temp_1 == 0;
    if _cse_temp_2 {
    return n;
   
}
result = 1;
    while result<n {
    result = result <<1;
   
}
result
}
#[doc = "Isolate the lowest set bit."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn isolate_lowest_set_bit(n: i32) -> i32 {
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
    return 0;
   
}
n &(- n)
}
#[doc = "Clear the lowest set bit."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn clear_lowest_set_bit(n: i32) -> i32 {
    n &(n) - (1i32)
}
#[doc = "Interleave bits of x and y(Morton code) for bottom 8 bits each."] #[doc = " Depyler: verified panic-free"] pub fn bit_interleave(mut x: i32, mut y: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = x & 255;
    x = _cse_temp_0;
    let _cse_temp_1 = y & 255;
    y = _cse_temp_1;
    result = 0;
    let mut i: i32 = 0;
    while i<8 {
    result = result |(x & 1 <<i) <<i |(y & 1 <<i) <<(i).py_add(1i32);
    i  = ((i).py_add(1i32)) as i32;
   
}
result
}
#[doc = "Test bit manipulation operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_bit_manipulation() -> i32 {
    let mut total: i32 = Default::default();
    total = 0;
    let _cse_temp_0 = popcount(0) == 0;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = popcount(7) == 3;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = popcount(255) == 8;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = parity(7) == 1;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = parity(3) == 0;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = leading_zeros_32(1) == 31;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = leading_zeros_32(0) == 32;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = trailing_zeros(8) == 3;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = trailing_zeros(0) == 32;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_9 = next_power_of_two(5) == 8;
    if _cse_temp_9 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_10 = next_power_of_two(8) == 8;
    if _cse_temp_10 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_11 = isolate_lowest_set_bit(12) == 4;
    if _cse_temp_11 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_12 = clear_lowest_set_bit(12) == 8;
    if _cse_temp_12 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r: i32 = reverse_bits_32(1);
    let _cse_temp_13 = 1 <<31;
    let _cse_temp_14 = r == _cse_temp_13;
    if _cse_temp_14 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_15 = bit_interleave(0, 0) == 0;
    if _cse_temp_15 {
    total  = ((total).py_add(1i32)) as i32;
   
}
total
}
#[doc = "Add with overflow detection, returns max_val on overflow."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn safe_add(a: i32, b: i32, max_val: i32) -> i32 {
    let _cse_temp_0 = b>0;
    let _cse_temp_1 = a>(max_val) - (b);
    let _cse_temp_2  = (_cse_temp_0) &&(_cse_temp_1);
    if _cse_temp_2 {
    return max_val;
   
}
let _cse_temp_3 = b<0;
    let _cse_temp_4  = ((- max_val) - (b)) as i32;
    let _cse_temp_5 = a<_cse_temp_4;
    let _cse_temp_6  = (_cse_temp_3) &&(_cse_temp_5);
    if _cse_temp_6 {
    return - max_val;
   
}
(a).py_add(b)
}
#[doc = "Multiply with overflow detection."] #[doc = " Depyler: proven to terminate"] pub fn safe_multiply(a: i32, b: i32, max_val: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut sign: i32 = Default::default();
    let mut abs_b: i32 = Default::default();
    let mut abs_a: i32 = Default::default();
    let _cse_temp_0 = a == 0;
    let _cse_temp_1 = b == 0;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return Ok(0);
   
}
let _cse_temp_3 = max_val <= 0;
    if _cse_temp_3 {
    return Ok(0);
   
}
sign = 1;
    abs_a = a;
    abs_b = b;
    let _cse_temp_4 = a<0;
    if _cse_temp_4 {
    sign = - sign;
    abs_a = - a;
   
}
let _cse_temp_5 = b<0;
    if _cse_temp_5 {
    sign = - sign;
    abs_b = - b;
   
}
let _cse_temp_6 = {
    let a = max_val;
    let b = abs_b;
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
    let _cse_temp_7 = abs_a>_cse_temp_6;
    if _cse_temp_7 {
    return Ok((sign).py_mul(max_val));
   
}
Ok((a).py_mul(b))
}
#[doc = "Compute base^exp with overflow detection."] #[doc = " Depyler: verified panic-free"] pub fn safe_power(base: i32, exp: i32, max_val: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let _cse_temp_0 = exp<0;
    if _cse_temp_0 {
    return Ok(0);
   
}
let _cse_temp_1 = exp == 0;
    if _cse_temp_1 {
    return Ok(1);
   
}
let _cse_temp_2 = max_val <= 0;
    if _cse_temp_2 {
    return Ok(0);
   
}
result = 1;
    let mut b: i32 = base.clone();
    let mut e: i32 = exp.clone();
    while e>0 {
    if e & 1 == 1 {
    result = safe_multiply(result, b, max_val) ?;
    if(result>= max_val) ||(result<= (- max_val)) {
    return Ok(result);
   
}
} b = safe_multiply(b, b, max_val) ?;
    e = e>>1;
   
}
Ok(result)
}
#[doc = "Test overflow detection patterns."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_overflow_detection() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let mx: i32 = 1000000;
    let _cse_temp_0 = safe_add(999999, 2, mx) == mx;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = safe_add(100, 200, mx) == 300;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = safe_add(- 999999, - 2, mx) == (- mx);
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = safe_multiply(0, 100, mx) ? == 0;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = safe_multiply(1001, 1001, mx) ? == mx;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = safe_multiply(100, 100, mx) ? == 10000;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = safe_power(2, 0, mx) ? == 1;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = safe_power(2, 10, mx) ? == 1024;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = safe_power(2, 30, mx) ? == mx;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Division that handles zero and negative correctly."] #[doc = " Depyler: proven to terminate"] pub fn safe_div(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
    let _cse_temp_1 = a>0;
    if _cse_temp_1 {
    return Ok(2147483647);
   
}
let _cse_temp_2 = a<0;
    if _cse_temp_2 {
    return Ok(- 2147483647);
   
}
return Ok(0);
   
}
let _cse_temp_3 = a == - 2147483648;
    let _cse_temp_4 = b == - 1;
    let _cse_temp_5  = (_cse_temp_3) &&(_cse_temp_4);
    if _cse_temp_5 {
    return Ok(2147483647);
   
}
Ok({ let a = a;
    let b = b;
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
})
}
#[doc = "Modulo that handles negative numbers consistently."] #[doc = " Depyler: proven to terminate"] pub fn safe_mod(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut r: i32 = Default::default();
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
let _cse_temp_1  = ((a).py_mod(b)) as i32;
    r = _cse_temp_1;
    let _cse_temp_2 = r<0;
    let _cse_temp_3 = b>0;
    let _cse_temp_4  = (_cse_temp_2) &&(_cse_temp_3);
    if _cse_temp_4 {
    r  = ((r).py_add(b)) as i32;
   
}
else {
    let _cse_temp_5 = r>0;
    let _cse_temp_6 = b<0;
    let _cse_temp_7  = (_cse_temp_5) &&(_cse_temp_6);
    if _cse_temp_7 {
    r  = ((r).py_add(b)) as i32;
   
}
} Ok(r)
}
#[doc = "Euclidean division(result always non-negative remainder)."] #[doc = " Depyler: proven to terminate"] pub fn euclidean_div(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut q: i32 = Default::default();
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
let _cse_temp_1 = {
    let a = a;
    let b = b;
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
    q = _cse_temp_1;
    let _cse_temp_2  = ((q).py_mul(b)) as i32;
    let r: i32  = ((a) - (_cse_temp_2)) as i32;
    let _cse_temp_3 = r<0;
    if _cse_temp_3 {
    let _cse_temp_4 = b>0;
    if _cse_temp_4 {
    q  = ((q) - (1i32)) as i32;
   
}
else {
    q  = ((q).py_add(1i32)) as i32;
   
}
} Ok(q)
}
#[doc = "Ceiling division."] #[doc = " Depyler: proven to terminate"] pub fn ceiling_div(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
let _cse_temp_1 = a>= 0;
    let _cse_temp_2 = b>0;
    let _cse_temp_3  = (_cse_temp_1) &&(_cse_temp_2);
    let _cse_temp_4 = a <= 0;
    let _cse_temp_5 = b<0;
    let _cse_temp_6  = (_cse_temp_4) &&(_cse_temp_5);
    let _cse_temp_7  = (_cse_temp_3) ||(_cse_temp_6);
    if _cse_temp_7 {
    return Ok(if b>0 {
    { let a  = (((a).py_add(b) as i32)) - (1i32);
    let b = b;
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
}
}
else {
    { let a  = (((a).py_add(b) as i32)).py_add(1i32);
    let b = b;
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
} });
   
}
Ok({ let a = a;
    let b = b;
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
})
}
#[doc = "Test division edge cases."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_division_edge_cases() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let _cse_temp_0 = safe_div(10, 3) ? == 3;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = safe_div(10, 0) ? == 2147483647;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = safe_div(- 10, 0) ? == - 2147483647;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = safe_mod(10, 3) ? == 1;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = safe_mod(0, 5) ? == 0;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = safe_mod(10, 0) ? == 0;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = euclidean_div(7, 3) ? == 2;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = euclidean_div(0, 5) ? == 0;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = ceiling_div(7, 3) ? == 3;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_9 = ceiling_div(6, 3) ? == 2;
    if _cse_temp_9 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_10 = ceiling_div(0, 1) ? == 0;
    if _cse_temp_10 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Add two big numbers represented as digit lists(LSB first)."] pub fn bignum_add<'a, 'b>(a: & 'a Vec<i32>, b: & 'b Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let max_len: i32 = if a.len() as i32>b.len() as i32 {
    a.len() as i32
}
else {
    b.len() as i32 };
    let mut result: Vec<i32>= vec! [];
    let mut carry: i32 = 0;
    let mut i: i32 = 0;
    while(i<max_len) ||(carry>0) {
    let digit_a: i32 = if i<a.len() as i32 {
    a.get(i as usize).cloned().expect("IndexError: list index out of range")
}
else {
    0 };
    let digit_b: i32 = if i<b.len() as i32 {
    b.get(i as usize).cloned().expect("IndexError: list index out of range")
}
else {
    0 };
    let s: i32  = ((((digit_a).py_add(digit_b) as i32)).py_add(carry)) as i32;
    result.push((s).py_mod(10i32));
    carry = {
    let a = s;
    let b = 10;
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
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Multiply a big number by a single digit."] pub fn bignum_multiply_scalar(a: & Vec<i32>, scalar: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut carry: i32 = Default::default();
    let _cse_temp_0 = scalar == 0;
    if _cse_temp_0 {
    return Ok(vec! [0]);
   
}
let mut result: Vec<i32>= vec! [];
    carry = 0;
    let mut i: i32 = 0;
    while i<a.len() as i32 {
    let prod: i32  = ((((a.get(i as usize).cloned().expect("IndexError: list index out of range")).py_mul(scalar) as i32)).py_add(carry)) as i32;
    result.push((prod).py_mod(10i32));
    carry = {
    let a = prod;
    let b = 10;
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
    i  = ((i).py_add(1i32)) as i32;
   
}
while carry>0 {
    result.push((carry).py_mod(10i32));
    carry = {
    let a = carry;
    let b = 10;
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
   
}
Ok(result)
}
#[doc = "Convert digit list(LSB first) to integer."] pub fn bignum_to_int(digits: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    result = 0;
    let mut power: i32 = 1;
    let mut i: i32 = 0;
    while i<digits.len() as i32 {
    result  = ((result).py_add((digits.get(i as usize).cloned().expect("IndexError: list index out of range")).py_mul(power))) as i32;
    power  = ((power).py_mul(10i32)) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Convert integer to digit list(LSB first)."] pub fn bignum_from_int(n: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut val: i32 = Default::default();
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
    return Ok(vec! [0]);
   
}
let mut result: Vec<i32>= vec! [];
    val = n;
    let _cse_temp_1 = val<0;
    if _cse_temp_1 {
    val = - val;
   
}
while val>0 {
    result.push((val).py_mod(10i32));
    val = {
    let a = val;
    let b = 10;
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
   
}
Ok(result)
}
#[doc = "Compare two bignums. Returns -1, 0, or 1."] pub fn bignum_compare<'a, 'b>(a: & 'a Vec<i32>, b: & 'b Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut la: i32 = Default::default();
    let mut lb: i32 = Default::default();
    let _cse_temp_0 = a.len() as i32;
    la = _cse_temp_0;
    let _cse_temp_1 = b.len() as i32;
    lb = _cse_temp_1;
    while(la>1) &&({ let base = & a;
    let idx: i32  = (la) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
== 0) {
    la  = ((la) - (1i32)) as i32;
   
}
while(lb>1) &&({ let base = & b;
    let idx: i32  = (lb) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
== 0) {
    lb  = ((lb) - (1i32)) as i32;
   
}
let _cse_temp_2 = la != lb;
    if _cse_temp_2 {
    let _cse_temp_3 = la<lb;
    if _cse_temp_3 {
    return Ok(- 1);
   
}
return Ok(1);
   
}
let mut i: i32  = ((la) - (1i32)) as i32;
    while i>= 0 {
    if a.get(i as usize).cloned().expect("IndexError: list index out of range")<b.get(i as usize).cloned().expect("IndexError: list index out of range") {
    return Ok(- 1);
   
}
if a.get(i as usize).cloned().expect("IndexError: list index out of range")>b.get(i as usize).cloned().expect("IndexError: list index out of range") {
    return Ok(1);
   
}
i  = ((i) - (1i32)) as i32;
   
}
Ok(0)
}
#[doc = "Test multi-precision arithmetic."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_bignum() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let a: Vec<i32>= vec! [9, 9, 9];
    let b: Vec<i32>= vec! [1];
    let s: Vec<i32>= bignum_add(& a, & b) ?;
    let _cse_temp_0 = bignum_to_int(& s) ? == 1000;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let c: Vec<i32>= bignum_multiply_scalar(& vec! [2, 1], 3) ?;
    let _cse_temp_1 = bignum_to_int(& c) ? == 36;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let d: Vec<i32>= bignum_from_int(12345) ?;
    let _cse_temp_2 = bignum_to_int(& d) ? == 12345;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = bignum_from_int(0) ? == vec! [0];
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = bignum_compare(& vec! [1, 2], & vec! [1, 2]) ? == 0;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = bignum_compare(& vec! [1, 2], & vec! [1, 3]) ? == - 1;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = bignum_compare(& vec! [1, 3], & vec! [1, 2]) ? == 1;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Convert n to given base, returns digit list(LSB first)."] pub fn to_base(n: i32, base: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut val: i32 = Default::default();
    let _cse_temp_0 = base<2;
    if _cse_temp_0 {
    return Ok(vec! [0]);
   
}
let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(vec! [0]);
   
}
val = n;
    let _cse_temp_2 = val<0;
    if _cse_temp_2 {
    val = - val;
   
}
let mut result: Vec<i32>= vec! [];
    while val>0 {
    result.push((val).py_mod(base));
    val = {
    let a = val;
    let b = base;
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
   
}
Ok(result)
}
#[doc = "Convert digit list(LSB first) from given base to decimal."] pub fn from_base(digits: & Vec<i32>, base: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let _cse_temp_0 = base<2;
    if _cse_temp_0 {
    return Ok(0);
   
}
result = 0;
    let mut power: i32 = 1;
    let mut i: i32 = 0;
    while i<digits.len() as i32 {
    result  = ((result).py_add((digits.get(i as usize).cloned().expect("IndexError: list index out of range")).py_mul(power))) as i32;
    power  = ((power).py_mul(base)) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Return the decimal number formed by the binary digits of n.\n    E.g. 5 -> 101(reading binary digits as decimal)."] #[doc = " Depyler: verified panic-free"] pub fn to_binary_str_value(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    let mut val: i32 = Default::default();
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
    return 0;
   
}
val = n;
    let _cse_temp_1 = val<0;
    if _cse_temp_1 {
    val = - val;
   
}
result = 0;
    let mut power: i32 = 1;
    while val>0 {
    let bit: i32 = val & 1;
    result  = ((result).py_add((bit).py_mul(power))) as i32;
    power  = ((power).py_mul(10i32)) as i32;
    val = val>>1;
   
}
result
}
#[doc = "Count how many digits n has in given base."] pub fn count_digits_in_base(n: i32, base: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut count: i32 = Default::default();
    let mut val: i32 = Default::default();
    let _cse_temp_0 = base<2;
    if _cse_temp_0 {
    return Ok(0);
   
}
let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(1);
   
}
val = n;
    let _cse_temp_2 = val<0;
    if _cse_temp_2 {
    val = - val;
   
}
count = 0;
    while val>0 {
    val = {
    let a = val;
    let b = base;
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
    count  = ((count).py_add(1i32)) as i32;
   
}
Ok(count)
}
#[doc = "Test base conversion operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_base_conversion() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let b: Vec<i32>= to_base(10, 2) ?;
    let _cse_temp_0 = from_base(& b, 2) ? == 10;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let h: Vec<i32>= to_base(255, 16) ?;
    let _cse_temp_1 = from_base(& h, 16) ? == 255;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let o: Vec<i32>= to_base(8, 8) ?;
    let _cse_temp_2 = from_base(& o, 8) ? == 8;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = to_base(0, 2) ? == vec! [0];
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = to_binary_str_value(5) == 101;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = to_binary_str_value(0) == 0;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = count_digits_in_base(255, 16) ? == 2;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = count_digits_in_base(0, 10) ? == 1;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = count_digits_in_base(999, 10) ? == 3;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Parse MSB-first digit list to integer."] pub fn parse_digits(digits: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    result = 0;
    let mut i: i32 = 0;
    while i<digits.len() as i32 {
    let d: i32 = digits.get(i as usize).cloned().expect("IndexError: list index out of range");
    if(d<0) ||(d>9) {
    return Ok(- 1);
   
}
result  = ((((result).py_mul(10i32) as i32)).py_add(d)) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Split integer into MSB-first digit list."] pub fn split_into_digits(n: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut val: i32 = Default::default();
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
    return Ok(vec! [0]);
   
}
val = n;
    let _cse_temp_1 = val<0;
    if _cse_temp_1 {
    val = - val;
   
}
let mut digits: Vec<i32>= vec! [];
    while val>0 {
    digits.push((val).py_mod(10i32));
    val = {
    let a = val;
    let b = 10;
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
   
}
let mut result: Vec<i32>= vec! [];
    let _cse_temp_2 = digits.len() as i32;
    let mut i: i32  = ((_cse_temp_2) - (1i32)) as i32;
    while i>= 0 {
    result.push(digits.get(i as usize).cloned().expect("IndexError: list index out of range"));
    i  = ((i) - (1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Check if all digits are valid for given base. 1=valid, 0=invalid."] pub fn is_valid_digit_list(digits: & Vec<i32>, base: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let _cse_temp_0 = base<2;
    if _cse_temp_0 {
    return Ok(0);
   
}
let mut i: i32 = 0;
    while i<digits.len() as i32 {
    if(digits.get(i as usize).cloned().expect("IndexError: list index out of range")<0) ||(digits.get(i as usize).cloned().expect("IndexError: list index out of range")>= base) {
    return Ok(0);
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(1)
}
#[doc = "Recursively sum digits until single digit."] pub fn digit_sum_recursive(mut n: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let _cse_temp_0 = n<0;
    if _cse_temp_0 {
    n = - n;
   
}
while n>= 10 {
    let mut s: i32 = 0;
    while n>0 {
    s  = ((s).py_add((n).py_mod(10i32))) as i32;
    n = {
    let a = n;
    let b = 10;
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
   
}
n = s;
   
}
Ok(n)
}
#[doc = "Test digit parsing operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_digit_parsing() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let _cse_temp_0 = parse_digits(& vec! [1, 2, 3]) ? == 123;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = parse_digits(& vec! [0]) ? == 0;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = parse_digits(& vec! [1, 11, 3]) ? == - 1;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let d: Vec<i32>= split_into_digits(4567) ?;
    let _cse_temp_3 = d == vec! [4, 5, 6, 7];
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = split_into_digits(0) ? == vec! [0];
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = is_valid_digit_list(& vec! [0, 1], 2) ? == 1;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = is_valid_digit_list(& vec! [0, 2], 2) ? == 0;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = digit_sum_recursive(9999) ? == 9;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = digit_sum_recursive(0) ? == 0;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Compute continued fraction expansion of sqrt(n) up to 'terms' terms."] pub fn continued_fraction_sqrt(n: i32, terms: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut a0: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
    return Ok(vec! [0]);
   
}
a0 = 0;
    while(((a0).py_add(1i32) as i32)).py_mul((a0).py_add(1i32)) <= n {
    a0  = ((a0).py_add(1i32)) as i32;
   
}
let _cse_temp_1  = ((a0).py_mul(a0)) as i32;
    let _cse_temp_2 = _cse_temp_1 == n;
    if _cse_temp_2 {
    return Ok(vec! [a0]);
   
}
let mut result: Vec<i32>= vec! [a0];
    let mut m: i32 = 0;
    let mut d: i32 = 1;
    let mut a: i32 = a0.clone();
    let mut count: i32 = 0;
    while count<terms {
    m  = ((((d).py_mul(a) as i32)) - (m)) as i32;
    if(m == 0) &&(d == 0) {
    break;
   
}
d = {
    let a  = (n) - ((m).py_mul(m));
    let b = d;
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
    if d == 0 {
    break;
   
}
a = {
    let a  = (a0).py_add(m);
    let b = d;
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
    result.push(a);
    count  = ((count).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Evaluate continued fraction, returns [numerator, denominator]."] pub fn evaluate_continued_fraction(cf: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut den: i32 = Default::default();
    let mut num: i32 = Default::default();
    let _cse_temp_0 = cf.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(vec! [0, 1]);
   
}
let mut i: i32  = ((_cse_temp_0) - (1i32)) as i32;
    num = cf.get(i as usize).cloned().expect("IndexError: list index out of range");
    den = 1;
    i  = ((i) - (1i32)) as i32;
    while i>= 0 {
    let old_num: i32 = num;
    num  = ((((cf.get(i as usize).cloned().expect("IndexError: list index out of range")).py_mul(num) as i32)).py_add(den)) as i32;
    den = old_num;
    i  = ((i) - (1i32)) as i32;
   
}
Ok(vec! [num, den])
}
#[doc = "Get rational approximation of sqrt(n) as [num, den]."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn convergent_of_sqrt(n: i32, depth: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let cf: Vec<i32>= continued_fraction_sqrt(n, depth) ?;
    evaluate_continued_fraction(& cf)
}
#[doc = "Test continued fraction computations."] #[doc = " Depyler: proven to terminate"] pub fn test_continued_fractions() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    let mut diff: i32 = Default::default();
    total = 0;
    let cf4: Vec<i32>= continued_fraction_sqrt(4, 5) ?;
    let _cse_temp_0 = cf4 == vec! [2];
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let cf2: Vec<i32>= continued_fraction_sqrt(2, 4) ?;
    let _cse_temp_1 = cf2.len() as i32;
    let _cse_temp_2 = _cse_temp_1>= 2;
    let _cse_temp_3 = cf2.get(0usize).cloned().expect("IndexError: list index out of range") == 1;
    let _cse_temp_4  = (_cse_temp_2) &&(_cse_temp_3);
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let ev: Vec<i32>= evaluate_continued_fraction(& vec! [3, 7]) ?;
    let _cse_temp_5 = ev.get(0usize).cloned().expect("IndexError: list index out of range") == 22;
    let _cse_temp_6 = ev.get(1usize).cloned().expect("IndexError: list index out of range") == 7;
    let _cse_temp_7  = (_cse_temp_5) &&(_cse_temp_6);
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let ev2: Vec<i32>= evaluate_continued_fraction(& vec! [1]) ?;
    let _cse_temp_8  = (_cse_temp_3) &&(_cse_temp_3);
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let approx: Vec<i32>= convergent_of_sqrt(2, 5) ?;
    let _cse_temp_9  = ((approx.get(0usize).cloned().expect("IndexError: list index out of range")).py_mul(approx.get(0usize).cloned().expect("IndexError: list index out of range"))) as i32;
    let ratio: i32 = _cse_temp_9;
    let _cse_temp_10  = ((2i32).py_mul(approx.get(1usize).cloned().expect("IndexError: list index out of range"))) as i32;
    let _cse_temp_11  = ((_cse_temp_10).py_mul(approx.get(1usize).cloned().expect("IndexError: list index out of range"))) as i32;
    let target: i32 = _cse_temp_11;
    diff  = ((ratio) - (target)) as i32;
    let _cse_temp_12 = diff<0;
    if _cse_temp_12 {
    diff = - diff;
   
}
let _cse_temp_13 = diff <= approx.get(1usize).cloned().expect("IndexError: list index out of range");
    if _cse_temp_13 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_14 = continued_fraction_sqrt(0, 5) ? == vec! [0];
    if _cse_temp_14 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Integer square root using Newton's method."] pub fn isqrt(n: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut x: i32 = Default::default();
    let _cse_temp_0 = n<0;
    if _cse_temp_0 {
    return Ok(- 1);
   
}
let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(0);
   
}
x = n;
    let _cse_temp_2 = {
    let a  = (x).py_add(1i32);
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
    let mut y: i32 = _cse_temp_2.clone();
    while y<x {
    x = y;
    y = {
    let a  = (x).py_add({ let a = n;
    let b = x;
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
});
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
   
}
Ok(x)
}
#[doc = "Integer cube root using Newton's method."] pub fn icbrt(n: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut x: i32 = Default::default();
    let mut neg: i32 = Default::default();
    let mut val: i32 = Default::default();
    let _cse_temp_0 = n == 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
neg = 0;
    val = n;
    let _cse_temp_1 = n<0;
    if _cse_temp_1 {
    neg = 1;
    val = - n;
   
}
x = val;
    loop {
    if x == 0 {
    break;
   
}
let x2: i32 = {
    let a  = (((2i32).py_mul(x) as i32)).py_add({ let a = val;
    let b  = (x).py_mul(x);
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
});
    let b = 3;
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
    if x2>= x {
    break;
   
}
x = x2;
   
}
while(((x).py_mul(x) as i32)).py_mul(x)>val {
    x  = ((x) - (1i32)) as i32;
   
}
let _cse_temp_2 = neg == 1;
    if _cse_temp_2 {
    return Ok(- x);
   
}
Ok(x)
}
#[doc = "Compute floor(n^(1/k)) using Newton's method."] #[doc = " Depyler: verified panic-free"] pub fn integer_nth_root(n: i32, k: i32) -> i32 {
    let mut x: i32 = Default::default();
    let _cse_temp_0 = k <= 0;
    if _cse_temp_0 {
    return 0;
   
}
let _cse_temp_1 = n <= 0;
    if _cse_temp_1 {
    return 0;
   
}
let _cse_temp_2 = k == 1;
    if _cse_temp_2 {
    return n;
   
}
x = 1;
    loop {
    let mut xk: i32 = 1;
    let mut i: i32 = 0;
    while i<k {
    xk  = ((xk).py_mul((x).py_add(1i32))) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
if xk>n {
    break;
   
}
x  = ((x).py_add(1i32)) as i32;
   
}
x
}
#[doc = "Test Newton's method implementations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_newton_methods() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let _cse_temp_0  = (((0) as f64).sqrt().floor() as i32) ? == 0;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1  = (((1) as f64).sqrt().floor() as i32) ? == 1;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2  = (((4) as f64).sqrt().floor() as i32) ? == 2;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3  = (((8) as f64).sqrt().floor() as i32) ? == 2;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4  = (((9) as f64).sqrt().floor() as i32) ? == 3;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5  = (((100) as f64).sqrt().floor() as i32) ? == 10;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6  = (((- 1) as f64).sqrt().floor() as i32) ? == - 1;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = icbrt(0) ? == 0;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = icbrt(8) ? == 2;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_9 = icbrt(27) ? == 3;
    if _cse_temp_9 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_10 = icbrt(- 27) ? == - 3;
    if _cse_temp_10 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_11 = integer_nth_root(16, 2) == 4;
    if _cse_temp_11 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_12 = integer_nth_root(16, 4) == 2;
    if _cse_temp_12 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Multiply two matrices represented as 2D lists."] pub fn matrix_multiply<'b, 'a>(a: & 'a Vec<Vec<i32>>, b: & 'b Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = a.len() as i32;
    let rows_a: i32 = _cse_temp_0;
    let _cse_temp_1 = rows_a == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let _cse_temp_2 = a.get(0usize).cloned().expect("IndexError: list index out of range").len() as i32;
    let cols_a: i32 = _cse_temp_2;
    let _cse_temp_3 = b.len() as i32;
    let rows_b: i32 = _cse_temp_3;
    let _cse_temp_4 = rows_b == 0;
    let _cse_temp_5 = cols_a != rows_b;
    let _cse_temp_6  = (_cse_temp_4) ||(_cse_temp_5);
    if _cse_temp_6 {
    return Ok(vec! []);
   
}
let cols_b: i32 = _cse_temp_2;
    let mut result: Vec<Vec<i32>>= vec! [];
    let mut i: i32 = 0;
    while i<rows_a {
    let mut row: Vec<i32>= vec! [];
    let mut j: i32 = 0;
    while j<cols_b {
    let mut s: i32 = 0;
    let mut k: i32 = 0;
    while k<cols_a {
    s  = ((s).py_add((a.get(i as usize).cloned().expect("IndexError: list index out of range").get(k as usize).cloned().expect("IndexError: list index out of range")).py_mul(b.get(k as usize).cloned().expect("IndexError: list index out of range").get(j as usize).cloned().expect("IndexError: list index out of range")))) as i32;
    k  = ((k).py_add(1i32)) as i32;
   
}
row.push(s);
    j  = ((j).py_add(1i32)) as i32;
   
}
result.push(row);
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Transpose a matrix."] pub fn matrix_transpose(m: & Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = m.len() as i32;
    let rows: i32 = _cse_temp_0;
    let _cse_temp_1 = rows == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let _cse_temp_2 = m.get(0usize).cloned().expect("IndexError: list index out of range").len() as i32;
    let cols: i32 = _cse_temp_2;
    let mut result: Vec<Vec<i32>>= vec! [];
    let mut j: i32 = 0;
    while j<cols {
    let mut row: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i<rows {
    row.push(m.get(i as usize).cloned().expect("IndexError: list index out of range").get(j as usize).cloned().expect("IndexError: list index out of range"));
    i  = ((i).py_add(1i32)) as i32;
   
}
result.push(row);
    j  = ((j).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Compute determinant of a 2x2 matrix."] #[doc = " Depyler: proven to terminate"] pub fn matrix_determinant_2x2(m: & Vec<Vec<i32>>) -> Result<i32, Box<dyn std::error::Error>>{
    let _cse_temp_0 = m.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 2;
    let _cse_temp_2 = m.get(0usize).cloned().expect("IndexError: list index out of range").len() as i32;
    let _cse_temp_3 = _cse_temp_2 != 2;
    let _cse_temp_4  = (_cse_temp_1) ||(_cse_temp_3);
    let _cse_temp_5  = (_cse_temp_4) ||(_cse_temp_3);
    if _cse_temp_5 {
    return Ok(0);
   
}
Ok({ let _r: i32  = (((m.get(0usize).cloned().expect("IndexError: list index out of range").get(0usize).cloned().expect("IndexError: list index out of range")).py_mul(m.get(1usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range")) as i32)) - ((m.get(0usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range")).py_mul(m.get(1usize).cloned().expect("IndexError: list index out of range").get(0usize).cloned().expect("IndexError: list index out of range")));
    _r })
}
#[doc = "Compute determinant of a 3x3 matrix using cofactor expansion."] pub fn matrix_determinant_3x3(m: & Vec<Vec<i32>>) -> Result<i32, Box<dyn std::error::Error>>{
    let _cse_temp_0 = m.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 3;
    if _cse_temp_1 {
    return Ok(0);
   
}
let mut i: i32 = 0;
    while i<3 {
    if m.get(i as usize).cloned().expect("IndexError: list index out of range").len() as i32 != 3 {
    return Ok(0);
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
let a: i32 = m.get(0usize).cloned().expect("IndexError: list index out of range").get(0usize).cloned().expect("IndexError: list index out of range");
    let b: i32 = m.get(0usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range");
    let c: i32 = m.get(0usize).cloned().expect("IndexError: list index out of range").get(2usize).cloned().expect("IndexError: list index out of range");
    let d: i32 = m.get(1usize).cloned().expect("IndexError: list index out of range").get(0usize).cloned().expect("IndexError: list index out of range");
    let e: i32 = m.get(1usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range");
    let f: i32 = m.get(1usize).cloned().expect("IndexError: list index out of range").get(2usize).cloned().expect("IndexError: list index out of range");
    let g: i32 = m.get(2usize).cloned().expect("IndexError: list index out of range").get(0usize).cloned().expect("IndexError: list index out of range");
    let h: i32 = m.get(2usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range");
    let k: i32 = m.get(2usize).cloned().expect("IndexError: list index out of range").get(2usize).cloned().expect("IndexError: list index out of range");
    Ok({ let _r: i32  = (((((a).py_mul((((e).py_mul(k) as i32)) - ((f).py_mul(h))) as i32)) - ((b).py_mul((((d).py_mul(k) as i32)) - ((f).py_mul(g)))) as i32)).py_add((c).py_mul((((d).py_mul(h) as i32)) - ((e).py_mul(g))));
    _r })
}
#[doc = "Compute trace(sum of diagonal) of a square matrix."] pub fn matrix_trace(m: & Vec<Vec<i32>>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut s: i32 = Default::default();
    let _cse_temp_0 = m.len() as i32;
    let n: i32 = _cse_temp_0;
    s = 0;
    let mut i: i32 = 0;
    while i<n {
    if i<m.get(i as usize).cloned().expect("IndexError: list index out of range").len() as i32 {
    s  = ((s).py_add(m.get(i as usize).cloned().expect("IndexError: list index out of range").get(i as usize).cloned().expect("IndexError: list index out of range"))) as i32;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(s)
}
#[doc = "Create n x n identity matrix."] #[doc = " Depyler: verified panic-free"] pub fn matrix_identity(n: i32) -> Vec<Vec<i32>>{
    let mut result: Vec<Vec<i32>>= vec! [];
    let mut i: i32 = 0;
    while i<n {
    let mut row: Vec<i32>= vec! [];
    let mut j: i32 = 0;
    while j<n {
    if i == j {
    row.push(1);
   
}
else {
    row.push(0);
   
}
j  = ((j).py_add(1i32)) as i32;
   
}
result.push(row);
    i  = ((i).py_add(1i32)) as i32;
   
}
result
}
#[doc = "Test matrix operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_matrix_ops() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let a: Vec<Vec<i32>>= vec! [vec! [1, 2], vec! [3, 4]];
    let b: Vec<Vec<i32>>= vec! [vec! [5, 6], vec! [7, 8]];
    let c: Vec<Vec<i32>>= matrix_multiply(& a, & b) ?;
    let _cse_temp_0 = c == vec! [vec! [19, 22], vec! [43, 50]];
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let t: Vec<Vec<i32>>= matrix_transpose(& a) ?;
    let _cse_temp_1 = t == vec! [vec! [1, 3], vec! [2, 4]];
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let det: i32 = matrix_determinant_2x2(& a) ?;
    let _cse_temp_2 = det == - 2;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let m3: Vec<Vec<i32>>= vec! [vec! [1, 0, 0], vec! [0, 1, 0], vec! [0, 0, 1]];
    let det3: i32 = matrix_determinant_3x3(& m3) ?;
    let _cse_temp_3 = det3 == 1;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let tr: i32 = matrix_trace(& a) ?;
    let _cse_temp_4 = tr == 5;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let ident: Vec<Vec<i32>>= matrix_identity(3);
    let _cse_temp_5 = matrix_trace(& ident) ? == 3;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = matrix_determinant_3x3(& ident) ? == 1;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let empty: Vec<Vec<i32>>= matrix_multiply(& vec! [], & b) ?;
    let _cse_temp_7 = empty.is_empty();
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Evaluate polynomial using Horner's method.\n    coeffs[0] is highest degree coefficient."] pub fn horner_eval(coeffs: & Vec<i32>, x: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let _cse_temp_0 = coeffs.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(0);
   
}
result = coeffs.get(0usize).cloned().expect("IndexError: list index out of range");
    let mut i: i32 = 1;
    while i<coeffs.len() as i32 {
    result  = ((((result).py_mul(x) as i32)).py_add(coeffs.get(i as usize).cloned().expect("IndexError: list index out of range"))) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Add two polynomials(index = degree)."] #[doc = " Depyler: verified panic-free"] pub fn poly_add<'a, 'b>(a: & 'a Vec<i32>, b: & 'b Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let max_len: i32 = if a.len() as i32>b.len() as i32 {
    a.len() as i32
}
else {
    b.len() as i32 };
    let mut result: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i<max_len {
    let va: i32 = if i<a.len() as i32 {
    a.get(i as usize).cloned().expect("IndexError: list index out of range")
}
else {
    0 };
    let vb: i32 = if i<b.len() as i32 {
    b.get(i as usize).cloned().expect("IndexError: list index out of range")
}
else {
    0 };
    result.push((va).py_add(vb));
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Multiply two polynomials(index = degree)."] pub fn poly_multiply<'b, 'a>(a: & 'a Vec<i32>, b: & 'b Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut i: i32 = Default::default();
    let _cse_temp_0 = a.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let _cse_temp_2 = b.len() as i32;
    let _cse_temp_3 = _cse_temp_2 == 0;
    let _cse_temp_4  = (_cse_temp_1) ||(_cse_temp_3);
    if _cse_temp_4 {
    return Ok(vec! []);
   
}
let _cse_temp_5  = ((((_cse_temp_0).py_add(_cse_temp_2) as i32)) - (1i32)) as i32;
    let result_len: i32 = _cse_temp_5;
    let mut result: Vec<i32>= vec! [];
    i = 0;
    while i<result_len {
    result.push(0);
    i  = ((i).py_add(1i32)) as i32;
   
}
i = 0;
    while i<a.len() as i32 {
    let mut j: i32 = 0;
    while j<b.len() as i32 {
    result [((i).py_add(j)) as usize]  = ({ let base = & result;
    let idx: i32  = (i).py_add(j);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") }).py_add((a.get(i as usize).cloned().expect("IndexError: list index out of range")).py_mul(b.get(j as usize).cloned().expect("IndexError: list index out of range")));
    j  = ((j).py_add(1i32)) as i32;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Derivative of polynomial(index = degree)."] #[doc = " Depyler: verified panic-free"] pub fn poly_derivative(coeffs: & Vec<i32>) -> Vec<i32>{
    let _cse_temp_0 = coeffs.len() as i32;
    let _cse_temp_1 = _cse_temp_0 <= 1;
    if _cse_temp_1 {
    return vec! [0];
   
}
let mut result: Vec<i32>= vec! [];
    let mut i: i32 = 1;
    while i<coeffs.len() as i32 {
    result.push((i).py_mul(coeffs.get(i as usize).cloned().expect("IndexError: list index out of range")));
    i  = ((i).py_add(1i32)) as i32;
   
}
result
}
#[doc = "Test polynomial operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_polynomial() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let val: i32 = horner_eval(& vec! [1, - 3, 2], 3) ?;
    let _cse_temp_0 = val == 2;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let val2: i32 = horner_eval(& vec! [1, 0, 0], 5) ?;
    let _cse_temp_1 = val2 == 25;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = horner_eval(& vec! [], 5) ? == 0;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let s: Vec<i32>= poly_add(& vec! [1, 2, 3], & vec! [4, 5]) ?;
    let _cse_temp_3 = s == vec! [5, 7, 3];
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let p: Vec<i32>= poly_multiply(& vec! [1, 1], & vec! [1, 1]) ?;
    let _cse_temp_4 = p == vec! [1, 2, 1];
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let d: Vec<i32>= poly_derivative(& vec! [5, 3, 6]);
    let _cse_temp_5 = d == vec! [3, 12];
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = poly_derivative(& vec! [7]) == vec! [0];
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Greatest common divisor."] pub fn gcd(mut a: i32, mut b: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let _cse_temp_0 = a<0;
    if _cse_temp_0 {
    a = - a;
   
}
let _cse_temp_1 = b<0;
    if _cse_temp_1 {
    b = - b;
   
}
while b != 0 {
    let t: i32 = b;
    b  = ((a).py_mod(b)) as i32;
    a = t;
   
}
Ok(a)
}
#[doc = "Extended GCD returning [gcd, x, y] where a*x + b*y = gcd."] pub fn extended_gcd(a: i32, b: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut old_r: i32 = Default::default();
    let mut old_s: i32 = Default::default();
    let mut old_t: i32 = Default::default();
    let _cse_temp_0 = a == 0;
    if _cse_temp_0 {
    return Ok(vec! [b, 0, 1]);
   
}
old_r = a;
    let mut r: i32 = b.clone();
    old_s = 1;
    let mut s: i32 = 0;
    old_t = 0;
    let mut t: i32 = 1;
    while r != 0 {
    let q: i32 = {
    let a = old_r;
    let b = r;
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
    let tmp_r: i32 = r;
    r  = ((old_r) - ((q).py_mul(r))) as i32;
    old_r = tmp_r;
    let tmp_s: i32 = s;
    s  = ((old_s) - ((q).py_mul(s))) as i32;
    old_s = tmp_s;
    let tmp_t: i32 = t;
    t  = ((old_t) - ((q).py_mul(t))) as i32;
    old_t = tmp_t;
   
}
let _cse_temp_1 = old_r<0;
    if _cse_temp_1 {
    old_r = - old_r;
    old_s = - old_s;
    old_t = - old_t;
   
}
Ok(vec! [old_r, old_s, old_t])
}
#[doc = "Least common multiple."] #[doc = " Depyler: proven to terminate"] pub fn lcm(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut vb: i32 = Default::default();
    let mut va: i32 = Default::default();
    let _cse_temp_0 = a == 0;
    let _cse_temp_1 = b == 0;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return Ok(0);
   
}
let g: i32 = gcd(a, b) ?;
    let _cse_temp_3 = g == 0;
    if _cse_temp_3 {
    return Ok(0);
   
}
va = a;
    vb = b;
    let _cse_temp_4 = va<0;
    if _cse_temp_4 {
    va = - va;
   
}
let _cse_temp_5 = vb<0;
    if _cse_temp_5 {
    vb = - vb;
   
}
Ok(({ let a = va;
    let b = g;
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
}).py_mul(vb))
}
#[doc = "GCD of a list of numbers."] pub fn gcd_of_list(nums: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let _cse_temp_0 = nums.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(0);
   
}
result = nums.get(0usize).cloned().expect("IndexError: list index out of range");
    let mut i: i32 = 1;
    while i<nums.len() as i32 {
    result = gcd(result, nums.get(i as usize).cloned().expect("IndexError: list index out of range")) ?;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Modular multiplicative inverse using extended GCD.\n    Returns -1 if no inverse exists."] #[doc = " Depyler: proven to terminate"] pub fn mod_inverse(a: i32, m: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut inv: i32 = Default::default();
    let _cse_temp_0 = m <= 0;
    if _cse_temp_0 {
    return Ok(- 1);
   
}
let res: Vec<i32>= extended_gcd((a).py_mod(m), m) ?;
    let _cse_temp_1 = res.get(0usize).cloned().expect("IndexError: list index out of range") != 1;
    if _cse_temp_1 {
    return Ok(- 1);
   
}
let _cse_temp_2  = ((res.get(1usize).cloned().expect("IndexError: list index out of range")).py_mod(m)) as i32;
    inv = _cse_temp_2;
    let _cse_temp_3 = inv<0;
    if _cse_temp_3 {
    inv  = ((inv).py_add(m)) as i32;
   
}
Ok(inv)
}
#[doc = "Test GCD and extended GCD operations."] #[doc = " Depyler: proven to terminate"] pub fn test_gcd_extended() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let _cse_temp_0 = gcd(12, 8) ? == 4;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = gcd(0, 5) ? == 5;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = gcd(7, 0) ? == 7;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = gcd(- 12, 8) ? == 4;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let eg: Vec<i32>= extended_gcd(12, 8) ?;
    let _cse_temp_4 = eg.get(0usize).cloned().expect("IndexError: list index out of range") == 4;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5  = ((12i32).py_mul(eg.get(1usize).cloned().expect("IndexError: list index out of range"))) as i32;
    let _cse_temp_6  = ((8i32).py_mul(eg.get(2usize).cloned().expect("IndexError: list index out of range"))) as i32;
    let verify: i32  = ((_cse_temp_5).py_add(_cse_temp_6)) as i32;
    let _cse_temp_7 = verify == 4;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = lcm(4, 6) ? == 12;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_9 = lcm(0, 5) ? == 0;
    if _cse_temp_9 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_10 = gcd_of_list(& vec! [12, 8, 4]) ? == 4;
    if _cse_temp_10 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let inv: i32 = mod_inverse(3, 7) ?;
    let _cse_temp_11  = ((3i32).py_mul(inv)) as i32;
    let _cse_temp_12  = ((_cse_temp_11).py_mod(7i32)) as i32;
    let _cse_temp_13 = _cse_temp_12 == 1;
    if _cse_temp_13 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_14 = mod_inverse(2, 4) ? == - 1;
    if _cse_temp_14 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Chinese Remainder Theorem for two congruences.\n    Returns [solution, combined_modulus] or [-1, 0] if no solution."] #[doc = " Depyler: proven to terminate"] pub fn crt_two(r1: i32, m1: i32, r2: i32, m2: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut x: i32 = Default::default();
    let eg: Vec<i32>= extended_gcd(m1, m2) ?;
    let g: i32 = eg.get(0usize).cloned().expect("IndexError: list index out of range");
    let _cse_temp_0 = g == 0;
    if _cse_temp_0 {
    return Ok(vec! [- 1, 0]);
   
}
let _cse_temp_1  = ((((r2) - (r1) as i32)).py_mod(g)) as i32;
    let _cse_temp_2 = _cse_temp_1 != 0;
    if _cse_temp_2 {
    return Ok(vec! [- 1, 0]);
   
}
let combined: i32 = lcm(m1, m2) ?;
    let _cse_temp_3 = combined == 0;
    if _cse_temp_3 {
    return Ok(vec! [- 1, 0]);
   
}
let _cse_temp_4 = {
    let a  = (r2) - (r1);
    let b = g;
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
    let diff: i32 = _cse_temp_4;
    let _cse_temp_5  = ((diff).py_mul(eg.get(1usize).cloned().expect("IndexError: list index out of range"))) as i32;
    let _cse_temp_6 = {
    let a = m2;
    let b = g;
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
    let _cse_temp_7  = ((_cse_temp_5).py_mod(_cse_temp_6)) as i32;
    let _cse_temp_8  = ((m1).py_mul(_cse_temp_7)) as i32;
    x  = ((r1).py_add(_cse_temp_8)) as i32;
    let _cse_temp_9  = ((x).py_mod(combined)) as i32;
    x = _cse_temp_9;
    let _cse_temp_10 = x<0;
    if _cse_temp_10 {
    x  = ((x).py_add(combined)) as i32;
   
}
Ok(vec! [x, combined])
}
#[doc = "CRT for a list of congruences."] pub fn crt_list<'a, 'b>(remainders: & 'a Vec<i32>, moduli: & 'b Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut cur_m: i32 = Default::default();
    let mut cur_r: i32 = Default::default();
    let _cse_temp_0 = remainders.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let _cse_temp_2 = moduli.len() as i32;
    let _cse_temp_3 = _cse_temp_0 != _cse_temp_2;
    let _cse_temp_4  = (_cse_temp_1) ||(_cse_temp_3);
    if _cse_temp_4 {
    return Ok(vec! [- 1, 0]);
   
}
cur_r = remainders.get(0usize).cloned().expect("IndexError: list index out of range");
    cur_m = moduli.get(0usize).cloned().expect("IndexError: list index out of range");
    let mut i: i32 = 1;
    while i<remainders.len() as i32 {
    let result: Vec<i32>= crt_two(cur_r, cur_m, remainders.get(i as usize).cloned().expect("IndexError: list index out of range"), moduli.get(i as usize).cloned().expect("IndexError: list index out of range")) ?;
    if result.get(1usize).cloned().expect("IndexError: list index out of range") == 0 {
    return Ok(vec! [- 1, 0]);
   
}
cur_r = result.get(0usize).cloned().expect("IndexError: list index out of range");
    cur_m = result.get(1usize).cloned().expect("IndexError: list index out of range");
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(vec! [cur_r, cur_m])
}
#[doc = "Test Chinese Remainder Theorem."] #[doc = " Depyler: proven to terminate"] pub fn test_crt() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let r: Vec<i32>= crt_two(2, 3, 3, 5) ?;
    let _cse_temp_0 = r.get(0usize).cloned().expect("IndexError: list index out of range") == 8;
    let _cse_temp_1 = r.get(1usize).cloned().expect("IndexError: list index out of range") == 15;
    let _cse_temp_2  = (_cse_temp_0) &&(_cse_temp_1);
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= crt_two(0, 2, 0, 3) ?;
    let _cse_temp_3 = r2.get(0usize).cloned().expect("IndexError: list index out of range") == 0;
    let _cse_temp_4 = r2.get(1usize).cloned().expect("IndexError: list index out of range") == 6;
    let _cse_temp_5  = (_cse_temp_3) &&(_cse_temp_4);
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= crt_list(& vec! [2, 3, 2], & vec! [3, 5, 7]) ?;
    let _cse_temp_6 = r3.get(1usize).cloned().expect("IndexError: list index out of range") == 105;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7  = ((r3.get(0usize).cloned().expect("IndexError: list index out of range")).py_mod(3i32)) as i32;
    let check: i32 = _cse_temp_7;
    let _cse_temp_8 = check == 2;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_9  = ((r3.get(0usize).cloned().expect("IndexError: list index out of range")).py_mod(5i32)) as i32;
    let check2: i32 = _cse_temp_9;
    let _cse_temp_10 = check2 == 3;
    if _cse_temp_10 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_11  = ((r3.get(0usize).cloned().expect("IndexError: list index out of range")).py_mod(7i32)) as i32;
    let check3: i32 = _cse_temp_11;
    let _cse_temp_12 = check3 == 2;
    if _cse_temp_12 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let bad: Vec<i32>= crt_two(0, 2, 1, 2) ?;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Floor of log base 2."] #[doc = " Depyler: verified panic-free"] pub fn floor_log2(n: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
    return - 1;
   
}
result = 0;
    let mut val: i32 = n.clone();
    while val>1 {
    val = val>>1;
    result  = ((result).py_add(1i32)) as i32;
   
}
result
}
#[doc = "Floor of log base 10."] pub fn floor_log10(n: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
    return Ok(- 1);
   
}
result = 0;
    let mut val: i32 = n.clone();
    while val>= 10 {
    val = {
    let a = val;
    let b = 10;
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
    result  = ((result).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Floor of log base 'base'."] pub fn floor_log_base(n: i32, base: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    let _cse_temp_1 = base <= 1;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return Ok(- 1);
   
}
result = 0;
    let mut val: i32 = n.clone();
    while val>= base {
    val = {
    let a = val;
    let b = base;
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
    result  = ((result).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Check if n is a perfect power(n = a^b for b>= 2). 1=yes, 0=no."] #[doc = " Depyler: verified panic-free"] pub fn is_perfect_power(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return 0;
   
}
let mut b: i32 = 2;
    while b <= 40 {
    let a: i32 = integer_nth_root(n, b);
    let mut power: i32 = 1;
    let mut i: i32 = 0;
    while i<b {
    power  = ((power).py_mul(a)) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
if power == n {
    return 1;
   
}
b  = ((b).py_add(1i32)) as i32;
   
}
0
}
#[doc = "Test integer logarithm operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_integer_log() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let _cse_temp_0 = floor_log2(1) == 0;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = floor_log2(2) == 1;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = floor_log2(7) == 2;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = floor_log2(8) == 3;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = floor_log2(0) == - 1;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = floor_log10(1) ? == 0;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = floor_log10(99) ? == 1;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = floor_log10(100) ? == 2;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = floor_log_base(8, 2) ? == 3;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_9 = floor_log_base(27, 3) ? == 3;
    if _cse_temp_9 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_10 = is_perfect_power(8) == 1;
    if _cse_temp_10 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_11 = is_perfect_power(10) == 0;
    if _cse_temp_11 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_12 = is_perfect_power(1) == 0;
    if _cse_temp_12 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Modular exponentiation:(base^exp) % mod."] pub fn power_mod(base: i32, exp: i32, r#mod: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let mut b: i32 = Default::default();
    let _cse_temp_0 = r#mod <= 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
let _cse_temp_1 = r#mod == 1;
    if _cse_temp_1 {
    return Ok(0);
   
}
let _cse_temp_2 = exp<0;
    if _cse_temp_2 {
    return Ok(0);
   
}
result = 1;
    let _cse_temp_3  = ((base).py_mod(r#mod)) as i32;
    b = _cse_temp_3;
    let _cse_temp_4 = b<0;
    if _cse_temp_4 {
    b  = ((b).py_add(r#mod)) as i32;
   
}
let mut e: i32 = exp.clone();
    while e>0 {
    if e & 1 == 1 {
    result  = ((((result).py_mul(b) as i32)).py_mod(r#mod)) as i32;
   
}
e = e>>1;
    b  = ((((b).py_mul(b) as i32)).py_mod(r#mod)) as i32;
   
}
Ok(result)
}
#[doc = "Fermat primality test. 1=probably prime, 0=composite."] pub fn is_probable_prime_fermat(n: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut i: i32 = Default::default();
    let _cse_temp_0 = n<2;
    if _cse_temp_0 {
    return Ok(0);
   
}
let _cse_temp_1 = n == 2;
    let _cse_temp_2 = n == 3;
    let _cse_temp_3  = (_cse_temp_1) ||(_cse_temp_2);
    if _cse_temp_3 {
    return Ok(1);
   
}
let _cse_temp_4  = ((n).py_mod(2i32)) as i32;
    let _cse_temp_5 = _cse_temp_4 == 0;
    if _cse_temp_5 {
    return Ok(0);
   
}
let bases: Vec<i32>= vec! [2, 3, 5, 7, 11, 13];
    i = 0;
    while i<bases.len() as i32 {
    let a: i32 = bases.get(i as usize).cloned().expect("IndexError: list index out of range");
    if a>= n {
    i  = ((i).py_add(1i32)) as i32;
    continue;
   
}
if power_mod(a ,(n) - (1i32), n) ? != 1 {
    return Ok(0);
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(1)
}
#[doc = "Discrete log by brute force: find x such that base^x = target(mod mod).\n    Returns -1 if not found within mod steps."] pub fn discrete_log_brute(base: i32, target: i32, r#mod: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let _cse_temp_0 = r#mod <= 0;
    if _cse_temp_0 {
    return Ok(- 1);
   
}
let mut val: i32 = 1;
    let mut x: i32 = 0;
    while x<r#mod {
    if(val).py_mod(r#mod) == (target).py_mod(r#mod) {
    return Ok(x);
   
}
val  = ((((val).py_mul(base) as i32)).py_mod(r#mod)) as i32;
    x  = ((x).py_add(1i32)) as i32;
   
}
Ok(- 1)
}
#[doc = "Euler's totient function."] pub fn euler_totient(n: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut temp: i32 = Default::default();
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
let _cse_temp_1 = n == 1;
    if _cse_temp_1 {
    return Ok(1);
   
}
result = n;
    let mut p: i32 = 2;
    temp = n;
    while(p).py_mul(p) <= temp {
    if(temp).py_mod(p) == 0 {
    while(temp).py_mod(p) == 0 {
    temp = {
    let a = temp;
    let b = p;
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
   
}
result  = ((result) - ({ let a = result;
    let b = p;
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
})) as i32;
   
}
p  = ((p).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = temp>1;
    if _cse_temp_2 {
    let _cse_temp_3 = {
    let a = result;
    let b = temp;
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
    result  = ((result) - (_cse_temp_3)) as i32;
   
}
Ok(result)
}
#[doc = "Compute(1^k + 2^k +...+ n^k) % mod."] pub fn sum_of_powers_mod(n: i32, k: i32, r#mod: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    let _cse_temp_1 = r#mod <= 0;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return Ok(0);
   
}
result = 0;
    let mut i: i32 = 1;
    while i <= n {
    result  = ((((result).py_add(power_mod(i, k, r#mod) ?) as i32)).py_mod(r#mod)) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Test modular exponentiation operations."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_power_mod() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let _cse_temp_0 = power_mod(2, 10, 1000) ? == 24;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = power_mod(2, 10, 1024) ? == 0;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = power_mod(3, 0, 7) ? == 1;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = power_mod(5, 3, 13) ? == 8;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = power_mod(2, 10, 1) ? == 0;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = is_probable_prime_fermat(2) ? == 1;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = is_probable_prime_fermat(17) ? == 1;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = is_probable_prime_fermat(4) ? == 0;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = is_probable_prime_fermat(1) ? == 0;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let dl: i32 = discrete_log_brute(2, 8, 13) ?;
    let _cse_temp_9 = power_mod(2, dl, 13) ? == 8;
    if _cse_temp_9 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_10 = euler_totient(1) ? == 1;
    if _cse_temp_10 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_11 = euler_totient(12) ? == 4;
    if _cse_temp_11 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let sp: i32 = sum_of_powers_mod(5, 2, 1000) ?;
    let _cse_temp_12 = sp == 55;
    if _cse_temp_12 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Simplified Karatsuba multiplication for pedagogical purposes."] pub fn karatsuba_multiply(x: i32, y: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut power: i32 = Default::default();
    let _cse_temp_0 = x<10;
    let _cse_temp_1 = y<10;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return Ok((x).py_mul(y));
   
}
let n: i32 = count_digits_in_base(x, 10) ?;
    let m: i32 = count_digits_in_base(y, 10) ?;
    let _cse_temp_3 = {
    let a = if n<m {
    n
}
else {
    m };
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
    let half: i32 = _cse_temp_3;
    let _cse_temp_4 = half == 0;
    if _cse_temp_4 {
    return Ok((x).py_mul(y));
   
}
power = 1;
    let mut i: i32 = 0;
    while i<half {
    power  = ((power).py_mul(10i32)) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = {
    let a = x;
    let b = power;
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
    let high_x: i32 = _cse_temp_5;
    let _cse_temp_6  = ((x).py_mod(power)) as i32;
    let low_x: i32 = _cse_temp_6;
    let _cse_temp_7 = {
    let a = y;
    let b = power;
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
    let high_y: i32 = _cse_temp_7;
    let _cse_temp_8  = ((y).py_mod(power)) as i32;
    let low_y: i32 = _cse_temp_8;
    let _cse_temp_9  = ((low_x).py_mul(low_y)) as i32;
    let z0: i32 = _cse_temp_9;
    let _cse_temp_10  = ((high_x).py_mul(high_y)) as i32;
    let z2: i32 = _cse_temp_10;
    let _cse_temp_11  = ((((low_x).py_add(high_x) as i32)).py_mul((low_y).py_add(high_y))) as i32;
    let _cse_temp_12  = ((((_cse_temp_11) - (z2) as i32)) - (z0)) as i32;
    let z1: i32 = _cse_temp_12;
    Ok({ let _r: i32  = (((((((z2).py_mul(power) as i32)).py_mul(power) as i32)).py_add((z1).py_mul(power)) as i32)).py_add(z0);
    _r })
}
#[doc = "Count primes up to limit using a sieve."] pub fn sieve_count_primes(limit: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut count: i32 = Default::default();
    let _cse_temp_0 = limit<2;
    if _cse_temp_0 {
    return Ok(0);
   
}
let mut is_prime: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i <= limit {
    is_prime.push(1);
    i  = ((i).py_add(1i32)) as i32;
   
}
is_prime [(0) as usize] = 0;
    is_prime [(1) as usize] = 0;
    let mut p: i32 = 2;
    while(p).py_mul(p) <= limit {
    if is_prime.get(p as usize).cloned().expect("IndexError: list index out of range") == 1 {
    let mut j: i32  = ((p).py_mul(p)) as i32;
    while j <= limit {
    is_prime [(j) as usize] = 0;
    j  = ((j).py_add(p)) as i32;
   
}
} p  = ((p).py_add(1i32)) as i32;
   
}
count = 0;
    let mut k: i32 = 0;
    while k <= limit {
    count  = ((count).py_add(is_prime.get(k as usize).cloned().expect("IndexError: list index out of range"))) as i32;
    k  = ((k).py_add(1i32)) as i32;
   
}
Ok(count)
}
#[doc = "Length of Collatz sequence starting from n."] pub fn collatz_length(n: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut steps: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
    return Ok(0);
   
}
steps = 0;
    let mut val: i32 = n.clone();
    while val != 1 {
    if(val).py_mod(2i32) == 0 {
    val = {
    let a = val;
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
   
}
else {
    val  = ((((3i32).py_mul(val) as i32)).py_add(1i32)) as i32;
   
}
steps  = ((steps).py_add(1i32)) as i32;
    if steps>10000f64 {
    return Ok(- 1);
   
}
} Ok(steps)
}
#[doc = "Bounded Ackermann function to prevent stack overflow."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn ackermann_bounded(m: i32, n: i32, limit: i32) -> i32 {
    let _cse_temp_0 = limit <= 0;
    if _cse_temp_0 {
    return - 1;
   
}
let _cse_temp_1 = m == 0;
    if _cse_temp_1 {
    return(n).py_add(1i32);
   
}
let _cse_temp_2 = n == 0;
    if _cse_temp_2 {
    return ackermann_bounded((m) - (1i32), 1 ,(limit) - (1i32));
   
}
let inner: i32 = ackermann_bounded(m ,(n) - (1i32) ,(limit) - (1i32));
    let _cse_temp_3 = inner == - 1;
    if _cse_temp_3 {
    return - 1;
   
}
ackermann_bounded((m) - (1i32), inner ,(limit) - (1i32))
}
#[doc = "Compute n-th Fibonacci number modulo mod."] pub fn fibonacci_mod(n: i32, r#mod: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut b: i32 = Default::default();
    let _cse_temp_0 = n <= 0;
    let _cse_temp_1 = r#mod <= 0;
    let _cse_temp_2  = (_cse_temp_0) ||(_cse_temp_1);
    if _cse_temp_2 {
    return Ok(0);
   
}
let _cse_temp_3 = n == 1;
    let _cse_temp_4 = n == 2;
    let _cse_temp_5  = (_cse_temp_3) ||(_cse_temp_4);
    if _cse_temp_5 {
    return Ok((1i32).py_mod(r#mod));
   
}
let mut a: i32 = 1;
    b = 1;
    let mut i: i32 = 3;
    while i <= n {
    let c: i32  = ((((a).py_add(b) as i32)).py_mod(r#mod)) as i32;
    a = b;
    b = c;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(b)
}
#[doc = "Test bonus pathological patterns."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_bonus_patterns() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    total = 0;
    let _cse_temp_0 = karatsuba_multiply(12, 34) ? == 408;
    if _cse_temp_0 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = karatsuba_multiply(0, 100) ? == 0;
    if _cse_temp_1 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = karatsuba_multiply(999, 999) ? == 998001;
    if _cse_temp_2 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = sieve_count_primes(10) ? == 4;
    if _cse_temp_3 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = sieve_count_primes(1) ? == 0;
    if _cse_temp_4 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_5 = sieve_count_primes(100) ? == 25;
    if _cse_temp_5 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_6 = collatz_length(1) ? == 0;
    if _cse_temp_6 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_7 = collatz_length(6) ? == 8;
    if _cse_temp_7 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_8 = collatz_length(0) ? == 0;
    if _cse_temp_8 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_9 = ackermann_bounded(0, 0, 100) == 1;
    if _cse_temp_9 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_10 = ackermann_bounded(1, 1, 100) == 3;
    if _cse_temp_10 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_11 = ackermann_bounded(2, 2, 200) == 7;
    if _cse_temp_11 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_12 = fibonacci_mod(10, 1000) ? == 55;
    if _cse_temp_12 {
    total  = ((total).py_add(1i32)) as i32;
   
}
let _cse_temp_13 = fibonacci_mod(1, 100) ? == 1;
    if _cse_temp_13 {
    total  = ((total).py_add(1i32)) as i32;
   
}
Ok(total)
}
#[doc = "Run all tests and return sum of passing tests."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn run_all_tests() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = 0;
    let _cse_temp_0  = ((total).py_add(test_fixed_point() ?)) as i32;
    total = _cse_temp_0;
    let _cse_temp_1  = ((total).py_add(test_bit_manipulation())) as i32;
    total = _cse_temp_1;
    let _cse_temp_2  = ((total).py_add(test_overflow_detection() ?)) as i32;
    total = _cse_temp_2;
    let _cse_temp_3  = ((total).py_add(test_division_edge_cases() ?)) as i32;
    total = _cse_temp_3;
    let _cse_temp_4  = ((total).py_add(test_bignum() ?)) as i32;
    total = _cse_temp_4;
    let _cse_temp_5  = ((total).py_add(test_base_conversion() ?)) as i32;
    total = _cse_temp_5;
    let _cse_temp_6  = ((total).py_add(test_digit_parsing() ?)) as i32;
    total = _cse_temp_6;
    let _cse_temp_7  = ((total).py_add(test_continued_fractions() ?)) as i32;
    total = _cse_temp_7;
    let _cse_temp_8  = ((total).py_add(test_newton_methods() ?)) as i32;
    total = _cse_temp_8;
    let _cse_temp_9  = ((total).py_add(test_matrix_ops() ?)) as i32;
    total = _cse_temp_9;
    let _cse_temp_10  = ((total).py_add(test_polynomial() ?)) as i32;
    total = _cse_temp_10;
    let _cse_temp_11  = ((total).py_add(test_gcd_extended() ?)) as i32;
    total = _cse_temp_11;
    let _cse_temp_12  = ((total).py_add(test_crt() ?)) as i32;
    total = _cse_temp_12;
    let _cse_temp_13  = ((total).py_add(test_power_mod() ?)) as i32;
    total = _cse_temp_13;
    let _cse_temp_14  = ((total).py_add(test_bonus_patterns() ?)) as i32;
    total = _cse_temp_14;
    Ok(total)
}
pub fn main () -> Result <(), Box<dyn std::error::Error>>{
    let result: i32 = run_all_tests() ?;
    assert!(result>0);
    Ok(())
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_fixed_sqrt_examples() {
    assert_eq!(fixed_sqrt(0, 0), 0);
    assert_eq!(fixed_sqrt(1, 2), 3);
    assert_eq!(fixed_sqrt(- 1, 1), 0);
   
}
#[test] fn test_test_fixed_point_examples() {
    let _ = test_fixed_point();
   
}
#[test] fn test_popcount_examples() {
    assert_eq!(popcount(0), 0);
    assert_eq!(popcount(1), 1);
    assert_eq!(popcount(- 1), - 1);
   
}
#[test] fn test_parity_examples() {
    assert_eq!(parity(0), 0);
    assert_eq!(parity(1), 1);
    assert_eq!(parity(- 1), - 1);
   
}
#[test] fn test_leading_zeros_32_examples() {
    assert_eq!(leading_zeros_32(0), 0);
    assert_eq!(leading_zeros_32(1), 1);
    assert_eq!(leading_zeros_32(- 1), - 1);
   
}
#[test] fn test_trailing_zeros_examples() {
    assert_eq!(trailing_zeros(0), 0);
    assert_eq!(trailing_zeros(1), 1);
    assert_eq!(trailing_zeros(- 1), - 1);
   
}
#[test] fn test_reverse_bits_32_examples() {
    assert_eq!(reverse_bits_32(0), 0);
    assert_eq!(reverse_bits_32(1), 1);
    assert_eq!(reverse_bits_32(- 1), - 1);
   
}
#[test] fn test_next_power_of_two_examples() {
    assert_eq!(next_power_of_two(0), 0);
    assert_eq!(next_power_of_two(1), 1);
    assert_eq!(next_power_of_two(- 1), - 1);
   
}
#[test] fn test_isolate_lowest_set_bit_examples() {
    assert_eq!(isolate_lowest_set_bit(0), 0);
    assert_eq!(isolate_lowest_set_bit(1), 1);
    assert_eq!(isolate_lowest_set_bit(- 1), - 1);
   
}
#[test] fn test_clear_lowest_set_bit_examples() {
    assert_eq!(clear_lowest_set_bit(0), 0);
    assert_eq!(clear_lowest_set_bit(1), 1);
    assert_eq!(clear_lowest_set_bit(- 1), - 1);
   
}
#[test] fn test_bit_interleave_examples() {
    assert_eq!(bit_interleave(0, 0), 0);
    assert_eq!(bit_interleave(1, 2), 3);
    assert_eq!(bit_interleave(- 1, 1), 0);
   
}
#[test] fn test_test_bit_manipulation_examples() {
    let _ = test_bit_manipulation();
   
}
#[test] fn test_test_overflow_detection_examples() {
    let _ = test_overflow_detection();
   
}
#[test] fn test_safe_div_examples() {
    assert_eq!(safe_div(0, 0), 0);
    assert_eq!(safe_div(1, 2), 3);
    assert_eq!(safe_div(- 1, 1), 0);
   
}
#[test] fn test_safe_mod_examples() {
    assert_eq!(safe_mod(0, 0), 0);
    assert_eq!(safe_mod(1, 2), 3);
    assert_eq!(safe_mod(- 1, 1), 0);
   
}
#[test] fn test_euclidean_div_examples() {
    assert_eq!(euclidean_div(0, 0), 0);
    assert_eq!(euclidean_div(1, 2), 3);
    assert_eq!(euclidean_div(- 1, 1), 0);
   
}
#[test] fn test_ceiling_div_examples() {
    assert_eq!(ceiling_div(0, 0), 0);
    assert_eq!(ceiling_div(1, 2), 3);
    assert_eq!(ceiling_div(- 1, 1), 0);
   
}
#[test] fn test_test_division_edge_cases_examples() {
    let _ = test_division_edge_cases();
   
}
#[test] fn test_bignum_to_int_examples() {
    assert_eq!(bignum_to_int(& vec! []), 0);
    assert_eq!(bignum_to_int(& vec! [1]), 1);
    assert_eq!(bignum_to_int(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_bignum_examples() {
    let _ = test_bignum();
   
}
#[test] fn test_to_binary_str_value_examples() {
    assert_eq!(to_binary_str_value(0), 0);
    assert_eq!(to_binary_str_value(1), 1);
    assert_eq!(to_binary_str_value(- 1), - 1);
   
}
#[test] fn test_count_digits_in_base_examples() {
    assert_eq!(count_digits_in_base(0, 0), 0);
    assert_eq!(count_digits_in_base(1, 2), 3);
    assert_eq!(count_digits_in_base(- 1, 1), 0);
   
}
#[test] fn test_test_base_conversion_examples() {
    let _ = test_base_conversion();
   
}
#[test] fn test_parse_digits_examples() {
    assert_eq!(parse_digits(& vec! []), 0);
    assert_eq!(parse_digits(& vec! [1]), 1);
    assert_eq!(parse_digits(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_digit_sum_recursive_examples() {
    assert_eq!(digit_sum_recursive(0), 0);
    assert_eq!(digit_sum_recursive(1), 1);
    assert_eq!(digit_sum_recursive(- 1), - 1);
   
}
#[test] fn test_test_digit_parsing_examples() {
    let _ = test_digit_parsing();
   
}
#[test] fn test_evaluate_continued_fraction_examples() {
    assert_eq!(evaluate_continued_fraction(vec! []), vec! []);
    assert_eq!(evaluate_continued_fraction(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_continued_fractions_examples() {
    let _ = test_continued_fractions();
   
}
#[test] fn test_isqrt_examples() {
    assert_eq!(isqrt(0), 0);
    assert_eq!(isqrt(1), 1);
    assert_eq!(isqrt(- 1), - 1);
   
}
#[test] fn test_icbrt_examples() {
    assert_eq!(icbrt(0), 0);
    assert_eq!(icbrt(1), 1);
    assert_eq!(icbrt(- 1), - 1);
   
}
#[test] fn test_integer_nth_root_examples() {
    assert_eq!(integer_nth_root(0, 0), 0);
    assert_eq!(integer_nth_root(1, 2), 3);
    assert_eq!(integer_nth_root(- 1, 1), 0);
   
}
#[test] fn test_test_newton_methods_examples() {
    let _ = test_newton_methods();
   
}
#[test] fn test_matrix_transpose_examples() {
    assert_eq!(matrix_transpose(vec! []), vec! []);
    assert_eq!(matrix_transpose(vec! [1]), vec! [1]);
   
}
#[test] fn test_matrix_determinant_2x2_examples() {
    assert_eq!(matrix_determinant_2x2(& vec! []), 0);
    assert_eq!(matrix_determinant_2x2(& vec! [1]), 1);
    assert_eq!(matrix_determinant_2x2(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_matrix_determinant_3x3_examples() {
    assert_eq!(matrix_determinant_3x3(& vec! []), 0);
    assert_eq!(matrix_determinant_3x3(& vec! [1]), 1);
    assert_eq!(matrix_determinant_3x3(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_matrix_trace_examples() {
    assert_eq!(matrix_trace(& vec! []), 0);
    assert_eq!(matrix_trace(& vec! [1]), 1);
    assert_eq!(matrix_trace(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_matrix_ops_examples() {
    let _ = test_matrix_ops();
   
}
#[test] fn test_poly_derivative_examples() {
    assert_eq!(poly_derivative(vec! []), vec! []);
    assert_eq!(poly_derivative(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_polynomial_examples() {
    let _ = test_polynomial();
   
}
#[test] fn test_gcd_examples() {
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(1, 2), 3);
    assert_eq!(gcd(- 1, 1), 0);
   
}
#[test] fn test_lcm_examples() {
    assert_eq!(lcm(0, 0), 0);
    assert_eq!(lcm(1, 2), 3);
    assert_eq!(lcm(- 1, 1), 0);
   
}
#[test] fn test_gcd_of_list_examples() {
    assert_eq!(gcd_of_list(& vec! []), 0);
    assert_eq!(gcd_of_list(& vec! [1]), 1);
    assert_eq!(gcd_of_list(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_mod_inverse_examples() {
    assert_eq!(mod_inverse(0, 0), 0);
    assert_eq!(mod_inverse(1, 2), 3);
    assert_eq!(mod_inverse(- 1, 1), 0);
   
}
#[test] fn test_test_gcd_extended_examples() {
    let _ = test_gcd_extended();
   
}
#[test] fn test_test_crt_examples() {
    let _ = test_crt();
   
}
#[test] fn test_floor_log2_examples() {
    assert_eq!(floor_log2(0), 0);
    assert_eq!(floor_log2(1), 1);
    assert_eq!(floor_log2(- 1), - 1);
   
}
#[test] fn test_floor_log10_examples() {
    assert_eq!(floor_log10(0), 0);
    assert_eq!(floor_log10(1), 1);
    assert_eq!(floor_log10(- 1), - 1);
   
}
#[test] fn test_floor_log_base_examples() {
    assert_eq!(floor_log_base(0, 0), 0);
    assert_eq!(floor_log_base(1, 2), 3);
    assert_eq!(floor_log_base(- 1, 1), 0);
   
}
#[test] fn test_is_perfect_power_examples() {
    assert_eq!(is_perfect_power(0), 0);
    assert_eq!(is_perfect_power(1), 1);
    assert_eq!(is_perfect_power(- 1), - 1);
   
}
#[test] fn test_test_integer_log_examples() {
    let _ = test_integer_log();
   
}
#[test] fn test_is_probable_prime_fermat_examples() {
    assert_eq!(is_probable_prime_fermat(0), 0);
    assert_eq!(is_probable_prime_fermat(1), 1);
    assert_eq!(is_probable_prime_fermat(- 1), - 1);
   
}
#[test] fn test_euler_totient_examples() {
    assert_eq!(euler_totient(0), 0);
    assert_eq!(euler_totient(1), 1);
    assert_eq!(euler_totient(- 1), - 1);
   
}
#[test] fn test_test_power_mod_examples() {
    let _ = test_power_mod();
   
}
#[test] fn test_karatsuba_multiply_examples() {
    assert_eq!(karatsuba_multiply(0, 0), 0);
    assert_eq!(karatsuba_multiply(1, 2), 3);
    assert_eq!(karatsuba_multiply(- 1, 1), 0);
   
}
#[test] fn test_sieve_count_primes_examples() {
    assert_eq!(sieve_count_primes(0), 0);
    assert_eq!(sieve_count_primes(1), 1);
    assert_eq!(sieve_count_primes(- 1), - 1);
   
}
#[test] fn test_collatz_length_examples() {
    assert_eq!(collatz_length(0), 0);
    assert_eq!(collatz_length(1), 1);
    assert_eq!(collatz_length(- 1), - 1);
   
}
#[test] fn test_fibonacci_mod_examples() {
    assert_eq!(fibonacci_mod(0, 0), 0);
    assert_eq!(fibonacci_mod(1, 2), 3);
    assert_eq!(fibonacci_mod(- 1, 1), 0);
   
}
#[test] fn test_test_bonus_patterns_examples() {
    let _ = test_bonus_patterns();
   
}
#[test] fn test_run_all_tests_examples() {
    let _ = run_all_tests();
   
}
}