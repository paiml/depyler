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
} #[doc = "In-place style insertion sort on a copy."] pub fn insertion_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut i: i32 = 1;
    while i<n {
    let key: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    let mut j: i32  = ((i) - (1i32)) as i32;
    while(j>= 0) &&(result.get(j as usize).cloned().expect("IndexError: list index out of range")>key) {
    result [((j).py_add(1i32)) as usize] = result.get(j as usize).cloned().expect("IndexError: list index out of range");
    j  = ((j) - (1i32)) as i32;
   
}
result [((j).py_add(1i32)) as usize] = key;
    i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_insertion_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    let r1: Vec<i32>= insertion_sort(& vec! [5, 3, 8, 1, 2]) ?;
    let r2: Vec<i32>= insertion_sort(& vec! []) ?;
    let r3: Vec<i32>= insertion_sort(& vec! [1]) ?;
    let r4: Vec<i32>= insertion_sort(& vec! [3, 3, 3]) ?;
    let r5: Vec<i32>= insertion_sort(& vec! [5, 4, 3, 2, 1]) ?;
    ok = 0;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 5, 8];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = r3 == vec! [1];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = r4 == vec! [3, 3, 3];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = r5 == vec! [1, 2, 3, 4, 5];
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Selection sort with explicit min-finding and swap."] pub fn selection_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut i: i32 = 0;
    while i <(n) - (1i32) {
    let mut min_idx: i32 = i.clone();
    let mut j: i32  = ((i).py_add(1i32)) as i32;
    while j<n {
    if result.get(j as usize).cloned().expect("IndexError: list index out of range")<result.get(min_idx as usize).cloned().expect("IndexError: list index out of range") {
    min_idx = j;
   
}
j  = ((j).py_add(1i32)) as i32;
   
}
if min_idx != i {
    let tmp: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    result [(i) as usize] = result.get(min_idx as usize).cloned().expect("IndexError: list index out of range");
    result [(min_idx) as usize] = tmp;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_selection_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    let r1: Vec<i32>= selection_sort(& vec! [9, 1, 4, 7, 2]) ?;
    let r2: Vec<i32>= selection_sort(& vec! [1, 2, 3]) ?;
    let r3: Vec<i32>= selection_sort(& vec! []) ?;
    let r4: Vec<i32>= selection_sort(& vec! [100, - 5, 0, 42]) ?;
    ok = 0;
    let _cse_temp_0 = r1 == vec! [1, 2, 4, 7, 9];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = r2 == vec! [1, 2, 3];
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = r3.is_empty();
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = r4 == vec! [- 5, 0, 42, 100];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Merge two sorted sub-arrays within arr, return new array."] pub fn merge_two(arr: & Vec<i32>, left: i32, mid: i32, right: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut i: i32 = Default::default();
    let mut j: i32 = Default::default();
    let mut merged: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    merged.push(val);
   
}
let mut temp: Vec<i32>= vec! [];
    i = left;
    j  = ((mid).py_add(1i32)) as i32;
    while(i <= mid) &&(j <= right) {
    if arr.get(i as usize).cloned().expect("IndexError: list index out of range") <= arr.get(j as usize).cloned().expect("IndexError: list index out of range") {
    temp.push(arr.get(i as usize).cloned().expect("IndexError: list index out of range"));
    i  = ((i).py_add(1i32)) as i32;
   
}
else {
    temp.push(arr.get(j as usize).cloned().expect("IndexError: list index out of range"));
    j  = ((j).py_add(1i32)) as i32;
   
}
} while i <= mid {
    temp.push(arr.get(i as usize).cloned().expect("IndexError: list index out of range"));
    i  = ((i).py_add(1i32)) as i32;
   
}
while j <= right {
    temp.push(arr.get(j as usize).cloned().expect("IndexError: list index out of range"));
    j  = ((j).py_add(1i32)) as i32;
   
}
let mut k: i32 = 0;
    while k<temp.len() as i32 {
    merged [((left).py_add(k)) as usize] = temp.get(k as usize).cloned().expect("IndexError: list index out of range");
    k  = ((k).py_add(1i32)) as i32;
   
}
Ok(merged)
}
#[doc = "Bottom-up merge sort without recursion."] #[doc = " Depyler: verified panic-free"] pub fn iterative_merge_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= Default::default();
    let mut mid: i32 = Default::default();
    let mut right: i32 = Default::default();
    result = vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
    return Ok(result);
   
}
let mut width: i32 = 1;
    while width<n {
    let mut left: i32 = 0;
    while left<n {
    mid  = ((((left).py_add(width) as i32)) - (1i32)) as i32;
    right  = ((((left).py_add((2i32).py_mul(width)) as i32)) - (1i32)) as i32;
    if mid>= n {
    mid  = ((n) - (1i32)) as i32;
   
}
if right>= n {
    right  = ((n) - (1i32)) as i32;
   
}
if mid<right {
    result = merge_two(& result, left, mid, right) ?;
   
}
left  = ((left).py_add((2i32).py_mul(width))) as i32;
   
}
width  = ((width).py_mul(2i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_iterative_merge_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    let r1: Vec<i32>= iterative_merge_sort(& vec! [8, 3, 1, 5, 2, 7, 4, 6]) ?;
    let r2: Vec<i32>= iterative_merge_sort(& vec! []) ?;
    let r3: Vec<i32>= iterative_merge_sort(& vec! [42]) ?;
    let r4: Vec<i32>= iterative_merge_sort(& vec! [2, 1]) ?;
    let r5: Vec<i32>= iterative_merge_sort(& vec! [5, 5, 5, 5]) ?;
    ok = 0;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5, 6, 7, 8];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = r3 == vec! [42];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = r4 == vec! [1, 2];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = r5 == vec! [5, 5, 5, 5];
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Lomuto partition scheme, modifies arr in place."] pub fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut i: i32 = Default::default();
    let pivot: i32 = arr.get(high as usize).cloned().expect("IndexError: list index out of range");
    i  = ((low) - (1i32)) as i32;
    let mut j: i32 = low.clone();
    while j<high {
    if arr.get(j as usize).cloned().expect("IndexError: list index out of range") <= pivot {
    i  = ((i).py_add(1i32)) as i32;
    let tmp: i32 = arr.get(i as usize).cloned().expect("IndexError: list index out of range");
    arr [(i) as usize] = arr.get(j as usize).cloned().expect("IndexError: list index out of range");
    arr [(j) as usize] = tmp;
   
}
j  = ((j).py_add(1i32)) as i32;
   
}
let tmp2: i32 = {
    let base = & arr;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    arr [((i).py_add(1i32)) as usize] = arr.get(high as usize).cloned().expect("IndexError: list index out of range");
    arr [(high) as usize] = tmp2;
    Ok((i).py_add(1i32))
}
#[doc = "Quicksort using explicit stack instead of recursion."] #[doc = " Depyler: verified panic-free"] pub fn iterative_quicksort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
    return Ok(result);
   
}
let mut stack: Vec<i32>= vec! [];
    stack.push(0);
    stack.push((n) - (1i32));
    while stack.len() as i32>0 {
    let high: i32 = stack.pop().unwrap_or_default();
    let low: i32 = stack.pop().unwrap_or_default();
    if low<high {
    let p: i32 = partition(&mut result, low, high) ?;
    if(p) - (1i32)>low {
    stack.push(low);
    stack.push((p) - (1i32));
   
}
if(p).py_add(1i32)<high {
    stack.push((p).py_add(1i32));
    stack.push(high);
   
}
}
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_iterative_quicksort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    let r1: Vec<i32>= iterative_quicksort(& vec! [10, 7, 8, 9, 1, 5]) ?;
    let r2: Vec<i32>= iterative_quicksort(& vec! []) ?;
    let r3: Vec<i32>= iterative_quicksort(& vec! [1]) ?;
    let r4: Vec<i32>= iterative_quicksort(& vec! [3, 2, 1]) ?;
    ok = 0;
    let _cse_temp_0 = r1 == vec! [1, 5, 7, 8, 9, 10];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = r3 == vec! [1];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = r4 == vec! [1, 2, 3];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Partition into <pivot, ==pivot,>pivot regions."] pub fn dutch_national_flag(arr: & Vec<i32>, pivot: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let mut low: i32 = 0;
    let mut mid: i32 = 0;
    let _cse_temp_0 = result.len() as i32;
    let mut high: i32  = ((_cse_temp_0) - (1i32)) as i32;
    while mid <= high {
    if result.get(mid as usize).cloned().expect("IndexError: list index out of range")<pivot {
    let tmp: i32 = result.get(low as usize).cloned().expect("IndexError: list index out of range");
    result [(low) as usize] = result.get(mid as usize).cloned().expect("IndexError: list index out of range");
    result [(mid) as usize] = tmp;
    low  = ((low).py_add(1i32)) as i32;
    mid  = ((mid).py_add(1i32)) as i32;
   
}
else {
    if result.get(mid as usize).cloned().expect("IndexError: list index out of range") == pivot {
    mid  = ((mid).py_add(1i32)) as i32;
   
}
else {
    let tmp2: i32 = result.get(mid as usize).cloned().expect("IndexError: list index out of range");
    result [(mid) as usize] = result.get(high as usize).cloned().expect("IndexError: list index out of range");
    result [(high) as usize] = tmp2;
    high  = ((high) - (1i32)) as i32;
   
}
}
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] pub fn test_dutch_national_flag() -> Result<i32, Box<dyn std::error::Error>>{
    let mut good: bool = Default::default();
    let mut ok: i32 = Default::default();
    let r1: Vec<i32>= dutch_national_flag(& vec! [2, 0, 1, 2, 0, 1, 1], 1) ?;
    ok = 0;
    let mut phase: i32 = 0;
    good = true;
    for val in r1.iter().cloned() {
    if phase == 0 {
    if val == 1 {
    phase = 1;
   
}
else {
    if val>1 {
    phase = 2;
   
}
}
}
else {
    if phase == 1 {
    if val<1 {
    good = false;
   
}
else {
    if val>1 {
    phase = 2;
   
}
}
}
else {
    if phase == 2 {
    if val <= 1 {
    good = false;
   
}
}
}
}
}
let _cse_temp_0 = r1.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 7;
    let _cse_temp_2  = (good) &&(_cse_temp_1);
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= dutch_national_flag(& vec! [], 5) ?;
    let _cse_temp_3 = r2.is_empty();
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= dutch_national_flag(& vec! [1, 1, 1], 1) ?;
    let _cse_temp_4 = r3 == vec! [1, 1, 1];
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Partition into <lo_val, [lo_val..hi_val],>hi_val."] pub fn three_way_partition(arr: & Vec<i32>, lo_val: i32, hi_val: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(result);
   
}
let mut start: i32 = 0;
    let mut end: i32  = ((n) - (1i32)) as i32;
    let mut i: i32 = 0;
    while i <= end {
    if result.get(i as usize).cloned().expect("IndexError: list index out of range")<lo_val {
    let tmp: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    result [(i) as usize] = result.get(start as usize).cloned().expect("IndexError: list index out of range");
    result [(start) as usize] = tmp;
    start  = ((start).py_add(1i32)) as i32;
    i  = ((i).py_add(1i32)) as i32;
   
}
else {
    if result.get(i as usize).cloned().expect("IndexError: list index out of range")>hi_val {
    let tmp2: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    result [(i) as usize] = result.get(end as usize).cloned().expect("IndexError: list index out of range");
    result [(end) as usize] = tmp2;
    end  = ((end) - (1i32)) as i32;
   
}
else {
    i  = ((i).py_add(1i32)) as i32;
   
}
}
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] pub fn test_three_way_partition() -> Result<i32, Box<dyn std::error::Error>>{
    let mut valid: bool = Default::default();
    let mut ok: i32 = Default::default();
    let r: Vec<i32>= three_way_partition(& vec! [1, 14, 5, 20, 4, 2, 54, 20, 87, 98, 3, 1, 32], 10, 20) ?;
    ok = 0;
    let mut phase: i32 = 0;
    valid = true;
    for val in r.iter().cloned() {
    if phase == 0 {
    if(10 <= val) &&(val <= 20) {
    phase = 1;
   
}
else {
    if val>20 {
    phase = 2;
   
}
}
}
else {
    if phase == 1 {
    if val<10 {
    valid = false;
   
}
else {
    if val>20 {
    phase = 2;
   
}
}
}
else {
    if phase == 2 {
    if val <= 20 {
    valid = false;
   
}
}
}
}
}
if valid {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= three_way_partition(& vec! [], 1, 5) ?;
    let _cse_temp_0 = r2.is_empty();
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "First position where target could be inserted(leftmost)."] pub fn lower_bound(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut lo: i32 = Default::default();
    lo = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut hi: i32 = _cse_temp_0.clone();
    while lo<hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range")<target {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    hi = mid;
   
}
} Ok(lo)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_lower_bound() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = lower_bound(& vec! [1, 2, 4, 4, 4, 7, 9], 4) ? == 2;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = lower_bound(& vec! [1, 2, 4, 4, 4, 7, 9], 5) ? == 5;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = lower_bound(& vec! [1, 2, 3], 0) ? == 0;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = lower_bound(& vec! [1, 2, 3], 10) ? == 3;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = lower_bound(& vec! [], 5) ? == 0;
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "First position strictly greater than target."] pub fn upper_bound(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut lo: i32 = Default::default();
    lo = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut hi: i32 = _cse_temp_0.clone();
    while lo<hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range") <= target {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    hi = mid;
   
}
} Ok(lo)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_upper_bound() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = upper_bound(& vec! [1, 2, 4, 4, 4, 7, 9], 4) ? == 5;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = upper_bound(& vec! [1, 2, 4, 4, 4, 7, 9], 0) ? == 0;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = upper_bound(& vec! [1, 2, 3], 3) ? == 3;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = upper_bound(& vec! [], 1) ? == 0;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Index where target is found or would be inserted."] pub fn search_insert_position(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut lo: i32 = Default::default();
    lo = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut hi: i32 = _cse_temp_0.clone();
    while lo<hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range")<target {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    hi = mid;
   
}
} Ok(lo)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_search_insert_position() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = search_insert_position(& vec! [1, 3, 5, 6], 5) ? == 2;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = search_insert_position(& vec! [1, 3, 5, 6], 2) ? == 1;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = search_insert_position(& vec! [1, 3, 5, 6], 7) ? == 4;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = search_insert_position(& vec! [1, 3, 5, 6], 0) ? == 0;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Interpolation search for uniformly distributed sorted arrays."] pub fn interpolation_search(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut lo: i32 = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut hi: i32  = ((_cse_temp_0) - (1i32)) as i32;
    while(lo <= hi) &&(arr.len() as i32>0) {
    if arr.get(hi as usize).cloned().expect("IndexError: list index out of range") == arr.get(lo as usize).cloned().expect("IndexError: list index out of range") {
    if arr.get(lo as usize).cloned().expect("IndexError: list index out of range") == target {
    return Ok(lo);
   
}
else {
    return Ok(- 1);
   
}
} let pos: i32  = ((lo).py_add({ let a  = (((target) - (arr.get(lo as usize).cloned().expect("IndexError: list index out of range")) as i32)).py_mul((hi) - (lo));
    let b  = (arr.get(hi as usize).cloned().expect("IndexError: list index out of range")) - (arr.get(lo as usize).cloned().expect("IndexError: list index out of range"));
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
    if(pos<lo) ||(pos>hi) {
    return Ok(- 1);
   
}
if arr.get(pos as usize).cloned().expect("IndexError: list index out of range") == target {
    return Ok(pos);
   
}
else {
    if arr.get(pos as usize).cloned().expect("IndexError: list index out of range")<target {
    lo  = ((pos).py_add(1i32)) as i32;
   
}
else {
    hi  = ((pos) - (1i32)) as i32;
   
}
}
}
Ok(- 1)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_interpolation_search() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let data: Vec<i32>= vec! [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let _cse_temp_0 = interpolation_search(& data, 50) ? == 4;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = interpolation_search(& data, 10) ? == 0;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = interpolation_search(& data, 100) ? == 9;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = interpolation_search(& data, 55) ? == - 1;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = interpolation_search(& vec! [], 5) ? == - 1;
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Binary search within [lo, hi] range."] pub fn binary_search_range(arr: & Vec<i32>, target: i32, mut lo: i32, mut hi: i32) -> Result<i32, Box<dyn std::error::Error>>{
    while lo <= hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range") == target {
    return Ok(mid);
   
}
else {
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range")<target {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    hi  = ((mid) - (1i32)) as i32;
   
}
}
}
Ok(- 1)
}
#[doc = "Exponential search: find range then binary search."] pub fn exponential_search(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut bound: i32 = Default::default();
    let mut hi: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(- 1);
   
}
let _cse_temp_2 = arr.get(0usize).cloned().expect("IndexError: list index out of range") == target;
    if _cse_temp_2 {
    return Ok(0);
   
}
bound = 1;
    while(bound<n) &&(arr.get(bound as usize).cloned().expect("IndexError: list index out of range") <= target) {
    bound  = ((bound).py_mul(2i32)) as i32;
   
}
let _cse_temp_3 = {
    let a = bound;
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
    let lo: i32 = _cse_temp_3;
    hi = bound;
    let _cse_temp_4 = hi>= n;
    if _cse_temp_4 {
    hi  = ((n) - (1i32)) as i32;
   
}
binary_search_range(& arr, target, lo, hi)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_exponential_search() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let data: Vec<i32>= vec! [2, 3, 4, 10, 40, 50, 60, 70, 80, 90];
    let _cse_temp_0 = exponential_search(& data, 10) ? == 3;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = exponential_search(& data, 2) ? == 0;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = exponential_search(& data, 90) ? == 9;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = exponential_search(& data, 5) ? == - 1;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = exponential_search(& vec! [], 1) ? == - 1;
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Find index of maximum in a unimodal array using ternary search."] pub fn ternary_search_max(arr: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut lo: i32 = Default::default();
    let mut hi: i32 = Default::default();
    let mut best: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(- 1);
   
}
let _cse_temp_2 = n == 1;
    if _cse_temp_2 {
    return Ok(0);
   
}
lo = 0;
    hi  = ((n) - (1i32)) as i32;
    while(hi) - (lo)>2 {
    let m1: i32  = ((lo).py_add({ let a  = (hi) - (lo);
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
})) as i32;
    let m2: i32  = ((hi) - ({ let a  = (hi) - (lo);
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
})) as i32;
    if arr.get(m1 as usize).cloned().expect("IndexError: list index out of range")<arr.get(m2 as usize).cloned().expect("IndexError: list index out of range") {
    lo  = ((m1).py_add(1i32)) as i32;
   
}
else {
    hi  = ((m2) - (1i32)) as i32;
   
}
} best = lo;
    let mut i: i32  = ((lo).py_add(1i32)) as i32;
    while i <= hi {
    if arr.get(i as usize).cloned().expect("IndexError: list index out of range")>arr.get(best as usize).cloned().expect("IndexError: list index out of range") {
    best = i;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(best)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_ternary_search_max() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = ternary_search_max(& vec! [1, 3, 5, 7, 9, 8, 6, 4, 2]) ? == 4;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = ternary_search_max(& vec! [1, 10, 1]) ? == 1;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = ternary_search_max(& vec! [42]) ? == 0;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = ternary_search_max(& vec! []) ? == - 1;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Partition for quickselect, returns pivot index."] pub fn quickselect_partition(arr: &mut Vec<i32>, lo: i32, hi: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut i: i32 = Default::default();
    let pivot: i32 = arr.get(hi as usize).cloned().expect("IndexError: list index out of range");
    i = lo;
    let mut j: i32 = lo.clone();
    while j<hi {
    if arr.get(j as usize).cloned().expect("IndexError: list index out of range") <= pivot {
    let tmp: i32 = arr.get(i as usize).cloned().expect("IndexError: list index out of range");
    arr [(i) as usize] = arr.get(j as usize).cloned().expect("IndexError: list index out of range");
    arr [(j) as usize] = tmp;
    i  = ((i).py_add(1i32)) as i32;
   
}
j  = ((j).py_add(1i32)) as i32;
   
}
let tmp2: i32 = arr.get(i as usize).cloned().expect("IndexError: list index out of range");
    arr [(i) as usize] = arr.get(hi as usize).cloned().expect("IndexError: list index out of range");
    arr [(hi) as usize] = tmp2;
    Ok(i)
}
#[doc = "Find k-th smallest element(0-indexed). Modifies a copy."] pub fn quickselect(arr: & Vec<i32>, k: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut work: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    work.push(val);
   
}
let mut lo: i32 = 0;
    let _cse_temp_0 = work.len() as i32;
    let mut hi: i32  = ((_cse_temp_0) - (1i32)) as i32;
    while lo <= hi {
    let p: i32 = quickselect_partition(&mut work, lo, hi) ?;
    if p == k {
    return Ok(work.get(p as usize).cloned().expect("IndexError: list index out of range"));
   
}
else {
    if p<k {
    lo  = ((p).py_add(1i32)) as i32;
   
}
else {
    hi  = ((p) - (1i32)) as i32;
   
}
}
}
Ok(- 1)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_quickselect() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = quickselect(& vec! [7, 10, 4, 3, 20, 15], 0) ? == 3;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = quickselect(& vec! [7, 10, 4, 3, 20, 15], 3) ? == 10;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = quickselect(& vec! [7, 10, 4, 3, 20, 15], 5) ? == 20;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = quickselect(& vec! [1], 0) ? == 1;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Merge two sorted lists into one sorted list."] pub fn merge_two_sorted<'b, 'a>(a: & 'a Vec<i32>, b: & 'b Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut i: i32 = Default::default();
    let mut j: i32 = Default::default();
    let mut result: Vec<i32>= vec! [];
    i = 0;
    j = 0;
    while(i<a.len() as i32) &&(j<b.len() as i32) {
    if a.get(i as usize).cloned().expect("IndexError: list index out of range") <= b.get(j as usize).cloned().expect("IndexError: list index out of range") {
    result.push(a.get(i as usize).cloned().expect("IndexError: list index out of range"));
    i  = ((i).py_add(1i32)) as i32;
   
}
else {
    result.push(b.get(j as usize).cloned().expect("IndexError: list index out of range"));
    j  = ((j).py_add(1i32)) as i32;
   
}
} while i<a.len() as i32 {
    result.push(a.get(i as usize).cloned().expect("IndexError: list index out of range"));
    i  = ((i).py_add(1i32)) as i32;
   
}
while j<b.len() as i32 {
    result.push(b.get(j as usize).cloned().expect("IndexError: list index out of range"));
    j  = ((j).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Merge k sorted lists using pairwise merge(tournament style)."] pub fn merge_k_sorted(lists: & Vec<Vec<i32>>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut current: Vec<Vec<i32>>= Default::default();
    let _cse_temp_0 = lists.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
current = vec! [];
    for lst in lists.iter().cloned() {
    let mut cloned: Vec<i32>= vec! [];
    for val in lst.iter().cloned() {
    cloned.push(val);
   
}
current.push(cloned);
   
}
while current.len() as i32>1 {
    let mut next_round: Vec<Vec<i32>>= vec! [];
    let mut i: i32 = 0;
    while i<current.len() as i32 {
    if(i).py_add(1i32)<current.len() as i32 {
    let merged: Vec<i32>= merge_two_sorted(& current.get(i as usize).cloned().expect("IndexError: list index out of range"), & {
    let base = & current;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") }) ?;
    next_round.push(merged);
   
}
else {
    next_round.push(current.get(i as usize).cloned().expect("IndexError: list index out of range"));
   
}
i  = ((i).py_add(2i32)) as i32;
   
}
current = next_round.clone();
   
}
Ok(current.get(0usize).cloned().expect("IndexError: list index out of range"))
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_merge_k_sorted() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= merge_k_sorted(& vec! [vec! [1, 4, 7], vec! [2, 5, 8], vec! [3, 6, 9]]) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5, 6, 7, 8, 9];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= merge_k_sorted(& vec! [vec! [1], vec! [2], vec! [3], vec! [4]]) ?;
    let _cse_temp_1 = r2 == vec! [1, 2, 3, 4];
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= merge_k_sorted(& vec! []) ?;
    let _cse_temp_2 = r3.is_empty();
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r4: Vec<i32>= merge_k_sorted(& vec! [vec! [10, 20, 30]]) ?;
    let _cse_temp_3 = r4 == vec! [10, 20, 30];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Sort array in chunks then merge all chunks."] #[doc = " Depyler: verified panic-free"] pub fn chunk_sort(arr: & Vec<i32>, chunk_size: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut end: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let mut chunks: Vec<Vec<i32>>= vec! [];
    let mut i: i32 = 0;
    while i<n {
    end  = ((i).py_add(chunk_size)) as i32;
    if end>n {
    end = n;
   
}
let mut chunk: Vec<i32>= vec! [];
    let mut j: i32 = i.clone();
    while j<end {
    chunk.push(arr.get(j as usize).cloned().expect("IndexError: list index out of range"));
    j  = ((j).py_add(1i32)) as i32;
   
}
chunk = insertion_sort(& chunk) ?;
    chunks.push(chunk);
    i = end;
   
}
merge_k_sorted(& chunks)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_chunk_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= chunk_sort(& vec! [9, 3, 7, 1, 8, 2, 6, 4, 5], 3) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5, 6, 7, 8, 9];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= chunk_sort(& vec! [], 5) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= chunk_sort(& vec! [5, 1, 3], 10) ?;
    let _cse_temp_2 = r3 == vec! [1, 3, 5];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Counting sort for non-negative integers up to max_val."] pub fn counting_sort(arr: & Vec<i32>, max_val: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let mut count: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i <= max_val {
    count.push(0);
    i  = ((i).py_add(1i32)) as i32;
   
}
for val in arr.iter().cloned() {
    if(0 <= val) &&(val <= max_val) {
    count [(val) as usize]  = (count.get(val as usize).cloned().expect("IndexError: list index out of range")).py_add(1i32);
   
}
} let mut result: Vec<i32>= vec! [];
    let mut idx: i32 = 0;
    while idx <= max_val {
    let mut c: i32 = 0;
    while c<count.get(idx as usize).cloned().expect("IndexError: list index out of range") {
    result.push(idx);
    c  = ((c).py_add(1i32)) as i32;
   
}
idx  = ((idx).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_counting_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= counting_sort(& vec! [4, 2, 2, 8, 3, 3, 1], 9) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 2, 3, 3, 4, 8];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= counting_sort(& vec! [], 5) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= counting_sort(& vec! [0, 0, 0], 1) ?;
    let _cse_temp_2 = r3 == vec! [0, 0, 0];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r4: Vec<i32>= counting_sort(& vec! [5, 5, 5], 5) ?;
    let _cse_temp_3 = r4 == vec! [5, 5, 5];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Stable counting sort by a specific digit position."] pub fn counting_sort_by_digit(arr: & Vec<i32>, exp: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let mut output: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i<n {
    output.push(0);
    i  = ((i).py_add(1i32)) as i32;
   
}
let mut count: Vec<i32>= vec! [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for val in arr.iter().cloned() {
    let digit: i32  = (({ let a = val;
    let b = exp;
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
}).py_mod(10i32)) as i32;
    count [(digit) as usize]  = (count.get(digit as usize).cloned().expect("IndexError: list index out of range")).py_add(1i32);
   
}
let mut j: i32 = 1;
    while j<10 {
    count [(j) as usize]  = (count.get(j as usize).cloned().expect("IndexError: list index out of range")).py_add({ let base = & count;
    let idx: i32  = (j) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") });
    j  = ((j).py_add(1i32)) as i32;
   
}
let mut k: i32  = ((n) - (1i32)) as i32;
    while k>= 0 {
    let digit2: i32  = (({ let a = arr.get(k as usize).cloned().expect("IndexError: list index out of range");
    let b = exp;
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
}).py_mod(10i32)) as i32;
    count [(digit2) as usize]  = (count.get(digit2 as usize).cloned().expect("IndexError: list index out of range")) - (1i32);
    output [(count.get(digit2 as usize).cloned().expect("IndexError: list index out of range")) as usize] = arr.get(k as usize).cloned().expect("IndexError: list index out of range");
    k  = ((k) - (1i32)) as i32;
   
}
Ok(output)
}
#[doc = "LSD radix sort for non-negative integers."] pub fn radix_sort_lsd(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= Default::default();
    let mut max_val: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
max_val = arr.get(0usize).cloned().expect("IndexError: list index out of range");
    for val in arr.iter().cloned() {
    if val>max_val {
    max_val = val;
   
}
} result = vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let mut exp: i32 = 1;
    while {
    let a = max_val;
    let b = exp;
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
}>0 {
    result = counting_sort_by_digit(& result, exp) ?;
    exp  = ((exp).py_mul(10i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_radix_sort_lsd() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= radix_sort_lsd(& vec! [170, 45, 75, 90, 802, 24, 2, 66]) ?;
    let _cse_temp_0 = r1 == vec! [2, 24, 45, 66, 75, 90, 170, 802];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= radix_sort_lsd(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= radix_sort_lsd(& vec! [1]) ?;
    let _cse_temp_2 = r3 == vec! [1];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r4: Vec<i32>= radix_sort_lsd(& vec! [999, 1, 100, 10]) ?;
    let _cse_temp_3 = r4 == vec! [1, 10, 100, 999];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Max-heap sift down at index i, modifies copy."] pub fn sift_down(arr: & Vec<i32>, n: i32, i: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut largest: i32 = Default::default();
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let mut idx: i32 = i.clone();
    loop {
    largest = idx;
    let left: i32  = ((((2i32).py_mul(idx) as i32)).py_add(1i32)) as i32;
    let right: i32  = ((((2i32).py_mul(idx) as i32)).py_add(2i32)) as i32;
    if(left<n) &&(result.get(left as usize).cloned().expect("IndexError: list index out of range")>result.get(largest as usize).cloned().expect("IndexError: list index out of range")) {
    largest = left;
   
}
if(right<n) &&(result.get(right as usize).cloned().expect("IndexError: list index out of range")>result.get(largest as usize).cloned().expect("IndexError: list index out of range")) {
    largest = right;
   
}
if largest == idx {
    break;
   
}
let tmp: i32 = result.get(idx as usize).cloned().expect("IndexError: list index out of range");
    result [(idx) as usize] = result.get(largest as usize).cloned().expect("IndexError: list index out of range");
    result [(largest) as usize] = tmp;
    idx = largest;
   
}
Ok(result)
}
#[doc = " Depyler: proven to terminate"] pub fn test_sift_down() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r: Vec<i32>= sift_down(& vec! [1, 5, 3, 4, 2], 5, 0) ?;
    let _cse_temp_0 = r.get(0usize).cloned().expect("IndexError: list index out of range") == 5;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= sift_down(& vec! [10, 5, 3], 3, 0) ?;
    let _cse_temp_1 = r2.get(0usize).cloned().expect("IndexError: list index out of range") == 10;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Max-heap sift up from index i, modifies copy."] pub fn sift_up(arr: & Vec<i32>, i: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let mut idx: i32 = i.clone();
    while idx>0 {
    let parent: i32 = {
    let a  = (idx) - (1i32);
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
    if result.get(idx as usize).cloned().expect("IndexError: list index out of range")>result.get(parent as usize).cloned().expect("IndexError: list index out of range") {
    let tmp: i32 = result.get(idx as usize).cloned().expect("IndexError: list index out of range");
    result [(idx) as usize] = result.get(parent as usize).cloned().expect("IndexError: list index out of range");
    result [(parent) as usize] = tmp;
    idx = parent;
   
}
else {
    break;
   
}
} Ok(result)
}
#[doc = " Depyler: proven to terminate"] pub fn test_sift_up() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r: Vec<i32>= sift_up(& vec! [5, 3, 4, 1, 2, 10], 5) ?;
    let _cse_temp_0 = r.get(0usize).cloned().expect("IndexError: list index out of range") == 10;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= sift_up(& vec! [10, 5, 3], 2) ?;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Build a max-heap from array using bottom-up sift-down."] pub fn heapify(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut largest: i32 = Default::default();
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = {
    let a = n;
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
    let mut i: i32  = ((_cse_temp_1) - (1i32)) as i32;
    while i>= 0 {
    let mut idx: i32 = i.clone();
    let mut done: bool = false;
    while ! done {
    largest = idx;
    let left: i32  = ((((2i32).py_mul(idx) as i32)).py_add(1i32)) as i32;
    let right: i32  = ((((2i32).py_mul(idx) as i32)).py_add(2i32)) as i32;
    if(left<n) &&(result.get(left as usize).cloned().expect("IndexError: list index out of range")>result.get(largest as usize).cloned().expect("IndexError: list index out of range")) {
    largest = left;
   
}
if(right<n) &&(result.get(right as usize).cloned().expect("IndexError: list index out of range")>result.get(largest as usize).cloned().expect("IndexError: list index out of range")) {
    largest = right;
   
}
if largest == idx {
    done = true;
   
}
else {
    let tmp: i32 = result.get(idx as usize).cloned().expect("IndexError: list index out of range");
    result [(idx) as usize] = result.get(largest as usize).cloned().expect("IndexError: list index out of range");
    result [(largest) as usize] = tmp;
    idx = largest;
   
}
} i  = ((i) - (1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: proven to terminate"] pub fn test_heapify() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r: Vec<i32>= heapify(& vec! [3, 1, 6, 5, 2, 4]) ?;
    let _cse_temp_0 = r.get(0usize).cloned().expect("IndexError: list index out of range") == 6;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= heapify(& vec! [1]) ?;
    let _cse_temp_1 = r2.get(0usize).cloned().expect("IndexError: list index out of range") == 1;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= heapify(& vec! [1, 2, 3, 4, 5]) ?;
    let _cse_temp_2 = r3.get(0usize).cloned().expect("IndexError: list index out of range") == 5;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Heap sort using in-place heapify then extract."] pub fn heap_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut largest: i32 = Default::default();
    let mut largest2: i32 = Default::default();
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
    return Ok(result);
   
}
let _cse_temp_2 = {
    let a = n;
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
    let mut i: i32  = ((_cse_temp_2) - (1i32)) as i32;
    while i>= 0 {
    let mut idx: i32 = i.clone();
    let size: i32 = n;
    let mut cont: bool = true;
    while cont {
    largest = idx;
    let left: i32  = ((((2i32).py_mul(idx) as i32)).py_add(1i32)) as i32;
    let right: i32  = ((((2i32).py_mul(idx) as i32)).py_add(2i32)) as i32;
    if(left<size) &&(result.get(left as usize).cloned().expect("IndexError: list index out of range")>result.get(largest as usize).cloned().expect("IndexError: list index out of range")) {
    largest = left;
   
}
if(right<size) &&(result.get(right as usize).cloned().expect("IndexError: list index out of range")>result.get(largest as usize).cloned().expect("IndexError: list index out of range")) {
    largest = right;
   
}
if largest == idx {
    cont = false;
   
}
else {
    let tmp: i32 = result.get(idx as usize).cloned().expect("IndexError: list index out of range");
    result [(idx) as usize] = result.get(largest as usize).cloned().expect("IndexError: list index out of range");
    result [(largest) as usize] = tmp;
    idx = largest;
   
}
} i  = ((i) - (1i32)) as i32;
   
}
let mut end: i32  = ((n) - (1i32)) as i32;
    while end>0 {
    let tmp2: i32 = result.get(0usize).cloned().expect("IndexError: list index out of range");
    result [(0) as usize] = result.get(end as usize).cloned().expect("IndexError: list index out of range");
    result [(end) as usize] = tmp2;
    let mut idx2: i32 = 0;
    let mut cont2: bool = true;
    while cont2 {
    largest2 = idx2;
    let left2: i32  = ((((2i32).py_mul(idx2) as i32)).py_add(1i32)) as i32;
    let right2: i32  = ((((2i32).py_mul(idx2) as i32)).py_add(2i32)) as i32;
    if(left2<end) &&(result.get(left2 as usize).cloned().expect("IndexError: list index out of range")>result.get(largest2 as usize).cloned().expect("IndexError: list index out of range")) {
    largest2 = left2;
   
}
if(right2<end) &&(result.get(right2 as usize).cloned().expect("IndexError: list index out of range")>result.get(largest2 as usize).cloned().expect("IndexError: list index out of range")) {
    largest2 = right2;
   
}
if largest2 == idx2 {
    cont2 = false;
   
}
else {
    let tmp3: i32 = result.get(idx2 as usize).cloned().expect("IndexError: list index out of range");
    result [(idx2) as usize] = result.get(largest2 as usize).cloned().expect("IndexError: list index out of range");
    result [(largest2) as usize] = tmp3;
    idx2 = largest2;
   
}
} end  = ((end) - (1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_heap_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= heap_sort(& vec! [12, 11, 13, 5, 6, 7]) ?;
    let _cse_temp_0 = r1 == vec! [5, 6, 7, 11, 12, 13];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= heap_sort(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= heap_sort(& vec! [1]) ?;
    let _cse_temp_2 = r3 == vec! [1];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r4: Vec<i32>= heap_sort(& vec! [3, 1, 2]) ?;
    let _cse_temp_3 = r4 == vec! [1, 2, 3];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Kahn's algorithm. edges[i] = [from, to]. Returns order or empty if cycle."] pub fn topological_sort(num_nodes: i32, edges: & Vec<Vec<i32>>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut in_degree: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i<num_nodes {
    in_degree.push(0);
    i  = ((i).py_add(1i32)) as i32;
   
}
let mut adj: Vec<Vec<i32>>= vec! [];
    let mut k: i32 = 0;
    while k<num_nodes {
    adj.push(vec! []);
    k  = ((k).py_add(1i32)) as i32;
   
}
for edge in edges.iter().cloned() {
    let src: i32 = edge.get(0usize).cloned().expect("IndexError: list index out of range");
    let dst: i32 = edge.get(1usize).cloned().expect("IndexError: list index out of range");
    adj.get(src as usize).cloned().expect("IndexError: list index out of range").push(dst);
    in_degree [(dst) as usize]  = (in_degree.get(dst as usize).cloned().expect("IndexError: list index out of range")).py_add(1i32);
   
}
let mut queue: Vec<i32>= vec! [];
    let mut q: i32 = 0;
    while q<num_nodes {
    if in_degree.get(q as usize).cloned().expect("IndexError: list index out of range") == 0 {
    queue.push(q);
   
}
q  = ((q).py_add(1i32)) as i32;
   
}
let mut result: Vec<i32>= vec! [];
    while queue.len() as i32>0 {
    let node: i32 = queue.get(0usize).cloned().expect("IndexError: list index out of range");
    let mut new_queue: Vec<i32>= vec! [];
    let mut qi: i32 = 1;
    while qi<queue.len() as i32 {
    new_queue.push(queue.get(qi as usize).cloned().expect("IndexError: list index out of range"));
    qi  = ((qi).py_add(1i32)) as i32;
   
}
queue = new_queue.clone();
    result.push(node);
    for neighbor in adj.get(node as usize).cloned().expect("IndexError: list index out of range") {
    in_degree [(neighbor) as usize]  = (in_degree.get(neighbor as usize).cloned().expect("IndexError: list index out of range")) - (1i32);
    if in_degree.get(neighbor as usize).cloned().expect("IndexError: list index out of range") == 0 {
    queue.push(neighbor);
   
}
}
}
let _cse_temp_0 = result.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != num_nodes;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
Ok(result)
}
#[doc = " Depyler: proven to terminate"] pub fn test_topological_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut valid: bool = Default::default();
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= topological_sort(6, & vec! [vec! [5, 2], vec! [5, 0], vec! [4, 0], vec! [4, 1], vec! [2, 3], vec! [3, 1]]) ?;
    let _cse_temp_0 = r1.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 6;
    if _cse_temp_1 {
    let mut pos: Vec<i32>= vec! [0, 0, 0, 0, 0, 0];
    let mut idx: i32 = 0;
    while idx<6 {
    pos [(r1.get(idx as usize).cloned().expect("IndexError: list index out of range")) as usize] = idx;
    idx  = ((idx).py_add(1i32)) as i32;
   
}
valid = true;
    let _cse_temp_2 = pos.get(5usize).cloned().expect("IndexError: list index out of range")>pos.get(2usize).cloned().expect("IndexError: list index out of range");
    if _cse_temp_2 {
    valid = false;
   
}
if _cse_temp_2 {
    valid = false;
   
}
if _cse_temp_2 {
    valid = false;
   
}
if _cse_temp_2 {
    valid = false;
   
}
if _cse_temp_2 {
    valid = false;
   
}
if _cse_temp_2 {
    valid = false;
   
}
if valid {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
} let r2: Vec<i32>= topological_sort(3, & vec! [vec! [0, 1], vec! [1, 2], vec! [2, 0]]) ?;
    let _cse_temp_3 = r2.is_empty();
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= topological_sort(1, & vec! []) ?;
    let _cse_temp_4 = r3 == vec! [0];
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Merge and count inversions across halves."] pub fn merge_count<'a, 'b>(arr: & 'a mut Vec<i32>, temp: & 'b mut Vec<i32>, left: i32, mid: i32, right: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut j: i32 = Default::default();
    let mut inv_count: i32 = Default::default();
    let mut k: i32 = Default::default();
    let mut i: i32 = Default::default();
    i = left;
    j  = ((mid).py_add(1i32)) as i32;
    k = left;
    inv_count = 0;
    while(i <= mid) &&(j <= right) {
    if arr.get(i as usize).cloned().expect("IndexError: list index out of range") <= arr.get(j as usize).cloned().expect("IndexError: list index out of range") {
    temp [(k) as usize] = arr.get(i as usize).cloned().expect("IndexError: list index out of range");
    i  = ((i).py_add(1i32)) as i32;
   
}
else {
    temp [(k) as usize] = arr.get(j as usize).cloned().expect("IndexError: list index out of range");
    inv_count  = ((inv_count).py_add((((mid) - (i) as i32)).py_add(1i32))) as i32;
    j  = ((j).py_add(1i32)) as i32;
   
}
k  = ((k).py_add(1i32)) as i32;
   
}
while i <= mid {
    temp [(k) as usize] = arr.get(i as usize).cloned().expect("IndexError: list index out of range");
    i  = ((i).py_add(1i32)) as i32;
    k  = ((k).py_add(1i32)) as i32;
   
}
while j <= right {
    temp [(k) as usize] = arr.get(j as usize).cloned().expect("IndexError: list index out of range");
    j  = ((j).py_add(1i32)) as i32;
    k  = ((k).py_add(1i32)) as i32;
   
}
let mut copy_idx: i32 = left.clone();
    while copy_idx <= right {
    arr [(copy_idx) as usize] = temp.get(copy_idx as usize).cloned().expect("IndexError: list index out of range");
    copy_idx  = ((copy_idx).py_add(1i32)) as i32;
   
}
Ok(inv_count)
}
#[doc = "Count inversions using iterative merge sort approach."] #[doc = " Depyler: verified panic-free"] pub fn count_inversions(arr: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = Default::default();
    let mut mid: i32 = Default::default();
    let mut right: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
    return Ok(0);
   
}
let mut work: Vec<i32>= vec! [];
    let mut temp: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    work.push(val);
    temp.push(val);
   
}
total = 0;
    let mut width: i32 = 1;
    while width<n {
    let mut left: i32 = 0;
    while left<n {
    mid  = ((((left).py_add(width) as i32)) - (1i32)) as i32;
    right  = ((((left).py_add((2i32).py_mul(width)) as i32)) - (1i32)) as i32;
    if mid>= n {
    mid  = ((n) - (1i32)) as i32;
   
}
if right>= n {
    right  = ((n) - (1i32)) as i32;
   
}
if mid<right {
    total  = ((total).py_add(merge_count(&mut work, &mut temp, left, mid, right) ?)) as i32;
   
}
left  = ((left).py_add((2i32).py_mul(width))) as i32;
   
}
width  = ((width).py_mul(2i32)) as i32;
   
}
Ok(total)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_count_inversions() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = count_inversions(& vec! [1, 20, 6, 4, 5]) ? == 5;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = count_inversions(& vec! [1, 2, 3, 4, 5]) ? == 0;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = count_inversions(& vec! [5, 4, 3, 2, 1]) ? == 10;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Encode runs as [[value, count],...]."] pub fn run_length_encode(arr: & Vec<i32>) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>>{
    let mut count: i32 = Default::default();
    let mut current: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let mut result: Vec<Vec<i32>>= vec! [];
    current = arr.get(0usize).cloned().expect("IndexError: list index out of range");
    count = 1;
    let mut i: i32 = 1;
    while i<arr.len() as i32 {
    if arr.get(i as usize).cloned().expect("IndexError: list index out of range") == current {
    count  = ((count).py_add(1i32)) as i32;
   
}
else {
    result.push(vec! [current, count]);
    current = arr.get(i as usize).cloned().expect("IndexError: list index out of range");
    count = 1;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
result.push(vec! [current, count]);
    Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_run_length_encode() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<Vec<i32>>= run_length_encode(& vec! [1, 1, 2, 2, 2, 3, 1, 1]) ?;
    let _cse_temp_0 = r1 == vec! [vec! [1, 2], vec! [2, 3], vec! [3, 1], vec! [1, 2]];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<Vec<i32>>= run_length_encode(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<Vec<i32>>= run_length_encode(& vec! [5]) ?;
    let _cse_temp_2 = r3 == vec! [vec! [5, 1]];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Decode [[value, count],...] back to flat list."] pub fn run_length_decode(encoded: & Vec<Vec<i32>>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for pair in encoded.iter().cloned() {
    let val: i32 = pair.get(0usize).cloned().expect("IndexError: list index out of range");
    let count: i32 = pair.get(1usize).cloned().expect("IndexError: list index out of range");
    let mut i: i32 = 0;
    while i<count {
    result.push(val);
    i  = ((i).py_add(1i32)) as i32;
   
}
} Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_run_length_decode() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= run_length_decode(& vec! [vec! [1, 2], vec! [2, 3], vec! [3, 1], vec! [1, 2]]) ?;
    let _cse_temp_0 = r1 == vec! [1, 1, 2, 2, 2, 3, 1, 1];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= run_length_decode(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= run_length_decode(& vec! [vec! [7, 4]]) ?;
    let _cse_temp_2 = r3 == vec! [7, 7, 7, 7];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_rle_roundtrip() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let original: Vec<i32>= vec! [4, 4, 4, 1, 1, 2, 2, 2, 2, 3];
    let encoded: Vec<Vec<i32>>= run_length_encode(& original) ?;
    let decoded: Vec<i32>= run_length_decode(& encoded) ?;
    let _cse_temp_0 = decoded == original;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let empty_rt: Vec<i32>= run_length_decode(& run_length_encode(& vec! []) ?) ?;
    let _cse_temp_1 = empty_rt.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Find all starting indices where pattern appears in text."] pub fn naive_pattern_match<'a, 'b>(text: & 'a Vec<i32>, pattern: & 'b Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    let _cse_temp_0 = text.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = pattern.len() as i32;
    let m: i32 = _cse_temp_1;
    let _cse_temp_2 = m == 0;
    let _cse_temp_3 = m>n;
    let _cse_temp_4  = (_cse_temp_2) ||(_cse_temp_3);
    if _cse_temp_4 {
    return Ok(result);
   
}
let mut i: i32 = 0;
    while i<= (n) - (m) {
    let mut r#match: bool = true;
    let mut j: i32 = 0;
    while j<m {
    if {
    let base = & text;
    let idx: i32  = (i).py_add(j);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
!= pattern.get(j as usize).cloned().expect("IndexError: list index out of range") {
    r#match = false;
    break;
   
}
j  = ((j).py_add(1i32)) as i32;
   
}
if r#match {
    result.push(i);
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_naive_pattern_match() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= naive_pattern_match(& vec! [1, 2, 3, 1, 2, 3, 1], & vec! [1, 2, 3]) ?;
    let _cse_temp_0 = r1 == vec! [0, 3];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= naive_pattern_match(& vec! [1, 2, 3], & vec! [4, 5]) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= naive_pattern_match(& vec! [1, 1, 1, 1], & vec! [1, 1]) ?;
    let _cse_temp_2 = r3 == vec! [0, 1, 2];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r4: Vec<i32>= naive_pattern_match(& vec! [], & vec! [1]) ?;
    let _cse_temp_3 = r4.is_empty();
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Compute KMP failure function / prefix table."] pub fn kmp_failure(pattern: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut k: i32 = Default::default();
    let _cse_temp_0 = pattern.len() as i32;
    let m: i32 = _cse_temp_0;
    let _cse_temp_1 = m == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let mut fail: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i<m {
    fail.push(0);
    i  = ((i).py_add(1i32)) as i32;
   
}
k = 0;
    let mut j: i32 = 1;
    while j<m {
    while(k>0) &&(pattern.get(k as usize).cloned().expect("IndexError: list index out of range") != pattern.get(j as usize).cloned().expect("IndexError: list index out of range")) {
    k = {
    let base = & fail;
    let idx: i32  = (k) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
   
}
if pattern.get(k as usize).cloned().expect("IndexError: list index out of range") == pattern.get(j as usize).cloned().expect("IndexError: list index out of range") {
    k  = ((k).py_add(1i32)) as i32;
   
}
fail [(j) as usize] = k;
    j  = ((j).py_add(1i32)) as i32;
   
}
Ok(fail)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_kmp_failure() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= kmp_failure(& vec! [1, 2, 1, 2, 3]) ?;
    let _cse_temp_0 = r1 == vec! [0, 0, 1, 2, 0];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= kmp_failure(& vec! [1, 1, 1, 1]) ?;
    let _cse_temp_1 = r2 == vec! [0, 1, 2, 3];
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= kmp_failure(& vec! []) ?;
    let _cse_temp_2 = r3.is_empty();
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r4: Vec<i32>= kmp_failure(& vec! [5]) ?;
    let _cse_temp_3 = r4 == vec! [0];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "KMP pattern matching, returns list of match start indices."] pub fn kmp_search<'a, 'b>(text: & 'a Vec<i32>, pattern: & 'b Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut j: i32 = Default::default();
    let _cse_temp_0 = text.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = pattern.len() as i32;
    let m: i32 = _cse_temp_1;
    let _cse_temp_2 = m == 0;
    if _cse_temp_2 {
    return Ok(vec! []);
   
}
let fail: Vec<i32>= kmp_failure(& pattern) ?;
    let mut result: Vec<i32>= vec! [];
    j = 0;
    let mut i: i32 = 0;
    while i<n {
    while(j>0) &&(pattern.get(j as usize).cloned().expect("IndexError: list index out of range") != text.get(i as usize).cloned().expect("IndexError: list index out of range")) {
    j = {
    let base = & fail;
    let idx: i32  = (j) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
   
}
if pattern.get(j as usize).cloned().expect("IndexError: list index out of range") == text.get(i as usize).cloned().expect("IndexError: list index out of range") {
    j  = ((j).py_add(1i32)) as i32;
   
}
if j == m {
    result.push((((i) - (m) as i32)).py_add(1i32));
    j = {
    let base = & fail;
    let idx: i32  = (j) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_kmp_search() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= kmp_search(& vec! [1, 2, 1, 2, 1, 2, 3], & vec! [1, 2, 1]) ?;
    let _cse_temp_0 = r1 == vec! [0, 2];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= kmp_search(& vec! [1, 2, 3], & vec! [4]) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= kmp_search(& vec! [1, 1, 1, 1, 1], & vec! [1, 1]) ?;
    let _cse_temp_2 = r3 == vec! [0, 1, 2, 3];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Shell sort with gap sequence n/2, n/4,..., 1."] pub fn shell_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = {
    let a = n;
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
    let mut gap: i32 = _cse_temp_1.clone();
    while gap>0 {
    let mut i: i32 = gap.clone();
    while i<n {
    let temp: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    let mut j: i32 = i.clone();
    while(j>= gap) &&({ let base = & result;
    let idx: i32  = (j) - (gap);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
>temp) {
    result [(j) as usize] = {
    let base = & result;
    let idx: i32  = (j) - (gap);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    j  = ((j) - (gap)) as i32;
   
}
result [(j) as usize] = temp;
    i  = ((i).py_add(1i32)) as i32;
   
}
gap = {
    let a = gap;
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
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_shell_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= shell_sort(& vec! [23, 12, 1, 8, 34, 54, 2, 3]) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 8, 12, 23, 34, 54];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= shell_sort(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= shell_sort(& vec! [5, 4, 3, 2, 1]) ?;
    let _cse_temp_2 = r3 == vec! [1, 2, 3, 4, 5];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Comb sort with shrink factor 1.3 approximated as 10/13."] pub fn comb_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut gap: i32 = Default::default();
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    gap = n;
    let mut swapped: bool = true;
    while(gap>1) ||(swapped) {
    gap = {
    let a  = (gap).py_mul(10i32);
    let b = 13;
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
    if gap<1 {
    gap = 1;
   
}
swapped = false;
    let mut i: i32 = 0;
    while(i).py_add(gap)<n {
    if result.get(i as usize).cloned().expect("IndexError: list index out of range")>{
    let base = & result;
    let idx: i32  = (i).py_add(gap);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    let tmp: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    result [(i) as usize] = {
    let base = & result;
    let idx: i32  = (i).py_add(gap);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    result [((i).py_add(gap)) as usize] = tmp;
    swapped = true;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
} Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_comb_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= comb_sort(& vec! [8, 4, 1, 56, 3, 5, 7, 2]) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5, 7, 8, 56];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= comb_sort(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= comb_sort(& vec! [1]) ?;
    let _cse_temp_2 = r3 == vec! [1];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Bidirectional bubble sort."] pub fn cocktail_shaker_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
    return Ok(result);
   
}
let mut start: i32 = 0;
    let mut end: i32  = ((n) - (1i32)) as i32;
    let mut swapped: bool = true;
    while swapped {
    swapped = false;
    let mut i: i32 = start.clone();
    while i<end {
    if result.get(i as usize).cloned().expect("IndexError: list index out of range")>{
    let base = & result;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    let tmp: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    result [(i) as usize] = {
    let base = & result;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    result [((i).py_add(1i32)) as usize] = tmp;
    swapped = true;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
if ! swapped {
    break;
   
}
end  = ((end) - (1i32)) as i32;
    swapped = false;
    let mut j: i32 = end.clone();
    while j>start {
    if result.get(j as usize).cloned().expect("IndexError: list index out of range")<{
    let base = & result;
    let idx: i32  = (j) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    let tmp2: i32 = result.get(j as usize).cloned().expect("IndexError: list index out of range");
    result [(j) as usize] = {
    let base = & result;
    let idx: i32  = (j) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    result [((j) - (1i32)) as usize] = tmp2;
    swapped = true;
   
}
j  = ((j) - (1i32)) as i32;
   
}
start  = ((start).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_cocktail_shaker_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= cocktail_shaker_sort(& vec! [5, 1, 4, 2, 8, 0, 2]) ?;
    let _cse_temp_0 = r1 == vec! [0, 1, 2, 2, 4, 5, 8];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= cocktail_shaker_sort(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= cocktail_shaker_sort(& vec! [3, 2, 1]) ?;
    let _cse_temp_2 = r3 == vec! [1, 2, 3];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Gnome sort(stupid sort variant)."] pub fn gnome_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut pos: i32 = 0;
    while pos<n {
    if(pos == 0) ||(result.get(pos as usize).cloned().expect("IndexError: list index out of range")>= {
    let base = & result;
    let idx: i32  = (pos) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") }) {
    pos  = ((pos).py_add(1i32)) as i32;
   
}
else {
    let tmp: i32 = result.get(pos as usize).cloned().expect("IndexError: list index out of range");
    result [(pos) as usize] = {
    let base = & result;
    let idx: i32  = (pos) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    result [((pos) - (1i32)) as usize] = tmp;
    pos  = ((pos) - (1i32)) as i32;
   
}
} Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_gnome_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= gnome_sort(& vec! [34, 2, 10, - 9, 1]) ?;
    let _cse_temp_0 = r1 == vec! [- 9, 1, 2, 10, 34];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= gnome_sort(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= gnome_sort(& vec! [1, 2, 3]) ?;
    let _cse_temp_2 = r3 == vec! [1, 2, 3];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Reverse first k+1 elements of arr(copy)."] pub fn flip(arr: & Vec<i32>, k: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let mut lo: i32 = 0;
    let mut hi: i32 = k.clone();
    while lo<hi {
    let tmp: i32 = result.get(lo as usize).cloned().expect("IndexError: list index out of range");
    result [(lo) as usize] = result.get(hi as usize).cloned().expect("IndexError: list index out of range");
    result [(hi) as usize] = tmp;
    lo  = ((lo).py_add(1i32)) as i32;
    hi  = ((hi) - (1i32)) as i32;
   
}
Ok(result)
}
#[doc = "Pancake sort by finding max, flipping to front, flipping to position."] pub fn pancake_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= Default::default();
    result = vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut curr_size: i32 = n.clone();
    while curr_size>1 {
    let mut max_idx: i32 = 0;
    let mut i: i32 = 1;
    while i<curr_size {
    if result.get(i as usize).cloned().expect("IndexError: list index out of range")>result.get(max_idx as usize).cloned().expect("IndexError: list index out of range") {
    max_idx = i;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
if max_idx != (curr_size) - (1i32) {
    if max_idx != 0 {
    result = flip(& result, max_idx) ?;
   
}
result = flip(& result ,(curr_size) - (1i32)) ?;
   
}
curr_size  = ((curr_size) - (1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_pancake_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= pancake_sort(& vec! [3, 6, 2, 7, 4, 5, 1]) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5, 6, 7];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= pancake_sort(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= pancake_sort(& vec! [1]) ?;
    let _cse_temp_2 = r3 == vec! [1];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Cycle sort - minimizes writes, O(n^2)."] pub fn cycle_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut cycle_start: i32 = Default::default();
    let mut item: i32 = Default::default();
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    cycle_start = 0;
    while cycle_start <(n) - (1i32) {
    item = result.get(cycle_start as usize).cloned().expect("IndexError: list index out of range");
    let mut pos: i32 = cycle_start.clone();
    let mut i: i32  = ((cycle_start).py_add(1i32)) as i32;
    while i<n {
    if result.get(i as usize).cloned().expect("IndexError: list index out of range")<item {
    pos  = ((pos).py_add(1i32)) as i32;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
if pos == cycle_start {
    cycle_start  = ((cycle_start).py_add(1i32)) as i32;
    continue;
   
}
while item == result.get(pos as usize).cloned().expect("IndexError: list index out of range") {
    pos  = ((pos).py_add(1i32)) as i32;
   
}
if pos != cycle_start {
    let tmp: i32 = result.get(pos as usize).cloned().expect("IndexError: list index out of range");
    result [(pos) as usize] = item;
    item = tmp;
   
}
while pos != cycle_start {
    pos = cycle_start;
    let mut j: i32  = ((cycle_start).py_add(1i32)) as i32;
    while j<n {
    if result.get(j as usize).cloned().expect("IndexError: list index out of range")<item {
    pos  = ((pos).py_add(1i32)) as i32;
   
}
j  = ((j).py_add(1i32)) as i32;
   
}
while item == result.get(pos as usize).cloned().expect("IndexError: list index out of range") {
    pos  = ((pos).py_add(1i32)) as i32;
   
}
if item != result.get(pos as usize).cloned().expect("IndexError: list index out of range") {
    let tmp2: i32 = result.get(pos as usize).cloned().expect("IndexError: list index out of range");
    result [(pos) as usize] = item;
    item = tmp2;
   
}
} cycle_start  = ((cycle_start).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_cycle_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= cycle_sort(& vec! [5, 2, 3, 1, 4]) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= cycle_sort(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= cycle_sort(& vec! [1, 2, 3]) ?;
    let _cse_temp_2 = r3 == vec! [1, 2, 3];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Find first occurrence of target, or -1."] pub fn first_occurrence(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let mut lo: i32 = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut hi: i32  = ((_cse_temp_0) - (1i32)) as i32;
    result = - 1;
    while lo <= hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range") == target {
    result = mid;
    hi  = ((mid) - (1i32)) as i32;
   
}
else {
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range")<target {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    hi  = ((mid) - (1i32)) as i32;
   
}
}
}
Ok(result)
}
#[doc = "Find last occurrence of target, or -1."] pub fn last_occurrence(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut result: i32 = Default::default();
    let mut lo: i32 = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut hi: i32  = ((_cse_temp_0) - (1i32)) as i32;
    result = - 1;
    while lo <= hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range") == target {
    result = mid;
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range")<target {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    hi  = ((mid) - (1i32)) as i32;
   
}
}
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_first_last_occurrence() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let data: Vec<i32>= vec! [1, 2, 2, 2, 3, 4, 4, 5];
    let _cse_temp_0 = first_occurrence(& data, 2) ? == 1;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = last_occurrence(& data, 2) ? == 3;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = first_occurrence(& data, 4) ? == 5;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = last_occurrence(& data, 4) ? == 6;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_4 = first_occurrence(& data, 6) ? == - 1;
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Count occurrences of target in sorted array using binary search."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn count_occurrences(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let first: i32 = first_occurrence(& arr, target) ?;
    let _cse_temp_0 = first == - 1;
    if _cse_temp_0 {
    return Ok(0);
   
}
let last: i32 = last_occurrence(& arr, target) ?;
    Ok({ let _r: i32  = (((last) - (first) as i32)).py_add(1i32);
    _r })
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_count_occurrences() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let data: Vec<i32>= vec! [1, 1, 2, 2, 2, 3, 3, 3, 3, 4];
    let _cse_temp_0 = count_occurrences(& data, 3) ? == 4;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = count_occurrences(& data, 2) ? == 3;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = count_occurrences(& data, 5) ? == 0;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = count_occurrences(& vec! [], 1) ? == 0;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Search in a rotated sorted array."] pub fn search_rotated(arr: & Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>>{
    let mut lo: i32 = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut hi: i32  = ((_cse_temp_0) - (1i32)) as i32;
    while lo <= hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range") == target {
    return Ok(mid);
   
}
if arr.get(lo as usize).cloned().expect("IndexError: list index out of range") <= arr.get(mid as usize).cloned().expect("IndexError: list index out of range") {
    if(arr.get(lo as usize).cloned().expect("IndexError: list index out of range") <= target) &&(target<arr.get(mid as usize).cloned().expect("IndexError: list index out of range")) {
    hi  = ((mid) - (1i32)) as i32;
   
}
else {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
} else {
    if(arr.get(mid as usize).cloned().expect("IndexError: list index out of range")<target) &&(target <= arr.get(hi as usize).cloned().expect("IndexError: list index out of range")) {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    hi  = ((mid) - (1i32)) as i32;
   
}
}
}
Ok(- 1)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_search_rotated() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = search_rotated(& vec! [4, 5, 6, 7, 0, 1, 2], 0) ? == 4;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = search_rotated(& vec! [4, 5, 6, 7, 0, 1, 2], 3) ? == - 1;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = search_rotated(& vec! [1], 0) ? == - 1;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_3 = search_rotated(& vec! [1], 1) ? == 0;
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Find minimum element in rotated sorted array(no duplicates)."] pub fn find_min_rotated(arr: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut lo: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(- 1);
   
}
lo = 0;
    let mut hi: i32  = ((_cse_temp_0) - (1i32)) as i32;
    while lo<hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if arr.get(mid as usize).cloned().expect("IndexError: list index out of range")>arr.get(hi as usize).cloned().expect("IndexError: list index out of range") {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
else {
    hi = mid;
   
}
} Ok(arr.get(lo as usize).cloned().expect("IndexError: list index out of range"))
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_find_min_rotated() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = find_min_rotated(& vec! [3, 4, 5, 1, 2]) ? == 1;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = find_min_rotated(& vec! [4, 5, 6, 7, 0, 1, 2]) ? == 0;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = find_min_rotated(& vec! []) ? == - 1;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Find index of any peak element using binary search."] pub fn find_peak_element(arr: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut lo: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(- 1);
   
}
let _cse_temp_2 = n == 1;
    if _cse_temp_2 {
    return Ok(0);
   
}
lo = 0;
    let mut hi: i32  = ((n) - (1i32)) as i32;
    while lo <= hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    let left_ok: bool  = (mid == 0) ||(arr.get(mid as usize).cloned().expect("IndexError: list index out of range")>= {
    let base = & arr;
    let idx: i32  = (mid) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") });
    let right_ok: bool  = (mid == (n) - (1i32)) ||(arr.get(mid as usize).cloned().expect("IndexError: list index out of range")>= {
    let base = & arr;
    let idx: i32  = (mid).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") });
    if(left_ok) &&(right_ok) {
    return Ok(mid);
   
}
else {
    if(mid>0) &&({ let base = & arr;
    let idx: i32  = (mid) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
>arr.get(mid as usize).cloned().expect("IndexError: list index out of range")) {
    hi  = ((mid) - (1i32)) as i32;
   
}
else {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
}
}
Ok(lo)
}
#[doc = " Depyler: proven to terminate"] pub fn test_find_peak_element() -> Result<i32, Box<dyn std::error::Error>>{
    let mut is_peak: bool = Default::default();
    let mut ok: i32 = Default::default();
    ok = 0;
    let p1: i32 = find_peak_element(& vec! [1, 2, 3, 1]) ?;
    let _cse_temp_0 = p1 == 2;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let p2: i32 = find_peak_element(& vec! [1, 2, 1, 3, 5, 6, 4]) ?;
    let arr2: Vec<i32>= vec! [1, 2, 1, 3, 5, 6, 4];
    is_peak = true;
    let _cse_temp_1 = p2>0;
    let _cse_temp_2 = arr2.get(p2 as usize).cloned().expect("IndexError: list index out of range")<{
    let base = & arr2;
    let idx: i32  = (p2) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    let _cse_temp_3  = (_cse_temp_1) &&(_cse_temp_2);
    if _cse_temp_3 {
    is_peak = false;
   
}
let _cse_temp_4 = arr2.len() as i32;
    let _cse_temp_5 = p2 <(_cse_temp_4) - (1i32);
    let _cse_temp_6  = (_cse_temp_5) &&(_cse_temp_2);
    if _cse_temp_6 {
    is_peak = false;
   
}
let _cse_temp_7 = p2>= 0;
    let _cse_temp_8  = (is_peak) &&(_cse_temp_7);
    if _cse_temp_8 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_9 = find_peak_element(& vec! [1]) ? == 0;
    if _cse_temp_9 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_10 = find_peak_element(& vec! []) ? == - 1;
    if _cse_temp_10 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Find two indices whose elements sum to target(sorted array)."] pub fn two_sum_sorted(arr: & Vec<i32>, target: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut lo: i32 = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut hi: i32  = ((_cse_temp_0) - (1i32)) as i32;
    while lo<hi {
    let s: i32  = ((arr.get(lo as usize).cloned().expect("IndexError: list index out of range")).py_add(arr.get(hi as usize).cloned().expect("IndexError: list index out of range"))) as i32;
    if s == target {
    return Ok(vec! [lo, hi]);
   
}
else {
    if s<target {
    lo  = ((lo).py_add(1i32)) as i32;
   
}
else {
    hi  = ((hi) - (1i32)) as i32;
   
}
}
}
Ok(vec! [- 1, - 1])
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_two_sum_sorted() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= two_sum_sorted(& vec! [2, 7, 11, 15], 9) ?;
    let _cse_temp_0 = r1 == vec! [0, 1];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= two_sum_sorted(& vec! [1, 2, 3, 4, 5], 8) ?;
    let _cse_temp_1 = r2 == vec! [2, 4];
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= two_sum_sorted(& vec! [1, 2], 10) ?;
    let _cse_temp_2 = r3 == vec! [- 1, - 1];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Merge two sorted arrays simulating in-place merge with gap method."] pub fn merge_in_place_sim<'b, 'a>(a: & 'a Vec<i32>, b: & 'b Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut combined: Vec<i32>= vec! [];
    for val in a.iter().cloned() {
    combined.push(val);
   
}
for val in b.iter().cloned() {
    combined.push(val);
   
}
let _cse_temp_0 = combined.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut gap: i32 = n.clone();
    while gap>0 {
    gap = {
    let a  = (gap).py_add(1i32);
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
    let mut i: i32 = 0;
    while(i).py_add(gap)<n {
    if combined.get(i as usize).cloned().expect("IndexError: list index out of range")>{
    let base = & combined;
    let idx: i32  = (i).py_add(gap);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    let tmp: i32 = combined.get(i as usize).cloned().expect("IndexError: list index out of range");
    combined [(i) as usize] = {
    let base = & combined;
    let idx: i32  = (i).py_add(gap);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    combined [((i).py_add(gap)) as usize] = tmp;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
if gap == 1 {
    let mut did_swap: bool = true;
    while did_swap {
    did_swap = false;
    let mut j: i32 = 0;
    while(j).py_add(1i32)<n {
    if combined.get(j as usize).cloned().expect("IndexError: list index out of range")>{
    let base = & combined;
    let idx: i32  = (j).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    let tmp2: i32 = combined.get(j as usize).cloned().expect("IndexError: list index out of range");
    combined [(j) as usize] = {
    let base = & combined;
    let idx: i32  = (j).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    combined [((j).py_add(1i32)) as usize] = tmp2;
    did_swap = true;
   
}
j  = ((j).py_add(1i32)) as i32;
   
}
} break;
   
}
} Ok(combined)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_merge_in_place_sim() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= merge_in_place_sim(& vec! [1, 3, 5, 7], & vec! [2, 4, 6, 8]) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5, 6, 7, 8];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= merge_in_place_sim(& vec! [], & vec! [1, 2, 3]) ?;
    let _cse_temp_1 = r2 == vec! [1, 2, 3];
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= merge_in_place_sim(& vec! [5, 10], & vec! [1, 2, 3]) ?;
    let _cse_temp_2 = r3 == vec! [1, 2, 3, 5, 10];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Compare and swap for bitonic sort."] #[doc = " Depyler: proven to terminate"] pub fn bitonic_compare_swap(mut arr: Vec<i32>, i: i32, j: i32, ascending: bool) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    if ascending {
    let _cse_temp_0 = arr.get(i as usize).cloned().expect("IndexError: list index out of range")>arr.get(j as usize).cloned().expect("IndexError: list index out of range");
    if _cse_temp_0 {
    let tmp: i32 = arr.get(i as usize).cloned().expect("IndexError: list index out of range");
    arr [(i) as usize] = arr.get(j as usize).cloned().expect("IndexError: list index out of range");
    arr [(j) as usize] = tmp;
   
}
} else {
    let _cse_temp_1 = arr.get(i as usize).cloned().expect("IndexError: list index out of range")<arr.get(j as usize).cloned().expect("IndexError: list index out of range");
    if _cse_temp_1 {
    let tmp2: i32 = arr.get(i as usize).cloned().expect("IndexError: list index out of range");
    arr [(i) as usize] = arr.get(j as usize).cloned().expect("IndexError: list index out of range");
    arr [(j) as usize] = tmp2;
   
}
} Ok(arr)
}
#[doc = "Bitonic sort for arrays with power-of-2 length."] pub fn bitonic_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= Default::default();
    result = vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
    return Ok(result);
   
}
let mut k: i32 = 2;
    while k <= n {
    let mut j: i32 = {
    let a = k;
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
    while j>0 {
    let mut i: i32 = 0;
    while i<n {
    let partner: i32 = i ^ j;
    if partner>i {
    if i & k == 0 {
    result = bitonic_compare_swap(result.clone(), i, partner, true) ?;
   
}
else {
    result = bitonic_compare_swap(result.clone(), i, partner, false) ?;
   
}
} i  = ((i).py_add(1i32)) as i32;
   
}
j = {
    let a = j;
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
k  = ((k).py_mul(2i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_bitonic_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= bitonic_sort(& vec! [3, 7, 4, 8, 6, 2, 1, 5]) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5, 6, 7, 8];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= bitonic_sort(& vec! [4, 3, 2, 1]) ?;
    let _cse_temp_1 = r2 == vec! [1, 2, 3, 4];
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= bitonic_sort(& vec! [1]) ?;
    let _cse_temp_2 = r3 == vec! [1];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Find length of longest increasing subsequence via patience sort piles."] pub fn patience_sort_lis_length(arr: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut piles: Vec<i32>= vec! [];
    for card in arr.iter().cloned() {
    let mut lo: i32 = 0;
    let mut hi: i32 = piles.len() as i32 as i32;
    while lo<hi {
    let mid: i32 = {
    let a  = (lo).py_add(hi);
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
    if piles.get(mid as usize).cloned().expect("IndexError: list index out of range")>= card {
    hi = mid;
   
}
else {
    lo  = ((mid).py_add(1i32)) as i32;
   
}
} if lo == piles.len() as i32 {
    piles.push(card);
   
}
else {
    piles [(lo) as usize] = card;
   
}
} Ok(piles.len() as i32 as i32)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_patience_sort_lis_length() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = patience_sort_lis_length(& vec! [10, 9, 2, 5, 3, 7, 101, 18]) ? == 4;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = patience_sort_lis_length(& vec! [7, 7, 7, 7]) ? == 1;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_2 = patience_sort_lis_length(& vec! []) ? == 0;
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Stable counting sort preserving relative order of equal elements."] pub fn counting_sort_stable(arr: & Vec<i32>, max_val: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let mut count: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i <= max_val {
    count.push(0);
    i  = ((i).py_add(1i32)) as i32;
   
}
for val in arr.iter().cloned() {
    count [(val) as usize]  = (count.get(val as usize).cloned().expect("IndexError: list index out of range")).py_add(1i32);
   
}
let mut j: i32 = 1;
    while j <= max_val {
    count [(j) as usize]  = (count.get(j as usize).cloned().expect("IndexError: list index out of range")).py_add({ let base = & count;
    let idx: i32  = (j) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") });
    j  = ((j).py_add(1i32)) as i32;
   
}
let mut output: Vec<i32>= vec! [];
    let mut k: i32 = 0;
    while k<n {
    output.push(0);
    k  = ((k).py_add(1i32)) as i32;
   
}
let mut m: i32  = ((n) - (1i32)) as i32;
    while m>= 0 {
    let val2: i32 = arr.get(m as usize).cloned().expect("IndexError: list index out of range");
    count [(val2) as usize]  = (count.get(val2 as usize).cloned().expect("IndexError: list index out of range")) - (1i32);
    output [(count.get(val2 as usize).cloned().expect("IndexError: list index out of range")) as usize] = val2;
    m  = ((m) - (1i32)) as i32;
   
}
Ok(output)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_counting_sort_stable() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= counting_sort_stable(& vec! [4, 2, 2, 8, 3, 3, 1], 9) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 2, 3, 3, 4, 8];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= counting_sort_stable(& vec! [], 5) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= counting_sort_stable(& vec! [0, 0, 0], 0) ?;
    let _cse_temp_2 = r3 == vec! [0, 0, 0];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "For each element, find the next greater element to its right."] pub fn next_greater_element(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut result: Vec<i32>= vec! [];
    let mut i: i32 = 0;
    while i<n {
    result.push(- 1);
    i  = ((i).py_add(1i32)) as i32;
   
}
let mut stack: Vec<i32>= vec! [];
    let mut j: i32 = 0;
    while j<n {
    while(stack.len() as i32>0) &&({ let base = & arr;
    let idx: i32 = {
    let base = & stack;
    let idx: i32  = (stack.len() as i32) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
<arr.get(j as usize).cloned().expect("IndexError: list index out of range")) {
    let idx: i32 = stack.pop().unwrap_or_default();
    result [(idx) as usize] = arr.get(j as usize).cloned().expect("IndexError: list index out of range");
   
}
stack.push(j);
    j  = ((j).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_next_greater_element() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= next_greater_element(& vec! [4, 5, 2, 25]) ?;
    let _cse_temp_0 = r1 == vec! [5, 25, 25, - 1];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= next_greater_element(& vec! [13, 7, 6, 12]) ?;
    let _cse_temp_1 = r2 == vec! [- 1, 12, 12, - 1];
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= next_greater_element(& vec! []) ?;
    let _cse_temp_2 = r3.is_empty();
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "O(n^2) brute force inversion count for validation."] pub fn count_inversions_brute(arr: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut count: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    count = 0;
    let mut i: i32 = 0;
    while i<n {
    let mut j: i32  = ((i).py_add(1i32)) as i32;
    while j<n {
    if arr.get(i as usize).cloned().expect("IndexError: list index out of range")>arr.get(j as usize).cloned().expect("IndexError: list index out of range") {
    count  = ((count).py_add(1i32)) as i32;
   
}
j  = ((j).py_add(1i32)) as i32;
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(count)
}
#[doc = "Cross-check merge-based and brute-force inversion counts."] #[doc = " Depyler: verified panic-free"] pub fn test_inversions_cross_check() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let test_cases: Vec<Vec<i32>>= vec! [vec! [3, 1, 2], vec! [1, 2, 3], vec! [5, 4, 3, 2, 1], vec! [1, 5, 2, 4, 3]];
    for tc in test_cases.iter().cloned() {
    let merge_count_val: i32 = count_inversions(& tc) ?;
    let brute_count_val: i32 = count_inversions_brute(& tc) ?;
    if merge_count_val == brute_count_val {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
} Ok(ok)
}
#[doc = "Minimum swaps to sort array using cycle detection."] pub fn min_swaps_to_sort(arr: & Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let mut swaps: i32 = Default::default();
    let mut m: i32 = Default::default();
    let _cse_temp_0 = arr.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
    return Ok(0);
   
}
let mut indexed: Vec<Vec<i32>>= vec! [];
    let mut i: i32 = 0;
    while i<n {
    indexed.push(vec! [arr.get(i as usize).cloned().expect("IndexError: list index out of range"), i]);
    i  = ((i).py_add(1i32)) as i32;
   
}
let mut j: i32 = 1;
    while j<n {
    let key_val: i32 = indexed.get(j as usize).cloned().expect("IndexError: list index out of range").get(0usize).cloned().expect("IndexError: list index out of range");
    let key_idx: i32 = indexed.get(j as usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range");
    let mut k: i32  = ((j) - (1i32)) as i32;
    while(k>= 0) &&(indexed.get(k as usize).cloned().expect("IndexError: list index out of range").get(0usize).cloned().expect("IndexError: list index out of range")>key_val) {
    indexed [(k).py_add(1i32) as usize] [(0) as usize] = indexed.get(k as usize).cloned().expect("IndexError: list index out of range").get(0usize).cloned().expect("IndexError: list index out of range");
    indexed [(k).py_add(1i32) as usize] [(1) as usize] = indexed.get(k as usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range");
    k  = ((k) - (1i32)) as i32;
   
}
indexed [(k).py_add(1i32) as usize] [(0) as usize] = key_val;
    indexed [(k).py_add(1i32) as usize] [(1) as usize] = key_idx;
    j  = ((j).py_add(1i32)) as i32;
   
}
let mut visited: Vec<bool>= vec! [];
    let mut vi: i32 = 0;
    while vi<n {
    visited.push(false);
    vi  = ((vi).py_add(1i32)) as i32;
   
}
swaps = 0;
    m = 0;
    while m<n {
    if(visited.get(m as usize).cloned().expect("IndexError: list index out of range")) ||(indexed.get(m as usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range") == m) {
    visited [(m) as usize] = true;
    m  = ((m).py_add(1i32)) as i32;
    continue;
   
}
let mut cycle_size: i32 = 0;
    let mut node: i32 = m.clone();
    while ! visited.get(node as usize).cloned().expect("IndexError: list index out of range") {
    visited [(node) as usize] = true;
    node = indexed.get(node as usize).cloned().expect("IndexError: list index out of range").get(1usize).cloned().expect("IndexError: list index out of range");
    cycle_size  = ((cycle_size).py_add(1i32)) as i32;
   
}
if cycle_size>1 {
    swaps  = ((swaps).py_add((cycle_size) - (1i32))) as i32;
   
}
m  = ((m).py_add(1i32)) as i32;
   
}
Ok(swaps)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_min_swaps_to_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = min_swaps_to_sort(& vec! [4, 3, 2, 1]) ? == 2;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = min_swaps_to_sort(& vec! [1, 2, 3]) ? == 0;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Move all even numbers before odd numbers, preserve relative order within."] pub fn sort_by_parity(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut evens: Vec<i32>= vec! [];
    let mut odds: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    if(val).py_mod(2i32) == 0 {
    evens.push(val);
   
}
else {
    odds.push(val);
   
}
} let mut result: Vec<i32>= vec! [];
    for val in evens.iter().cloned() {
    result.push(val);
   
}
for val in odds.iter().cloned() {
    result.push(val);
   
}
Ok(result)
}
#[doc = "In-place style parity sort using two pointers."] pub fn sort_by_parity_inplace(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut lo: i32 = 0;
    let mut hi: i32  = ((n) - (1i32)) as i32;
    while lo<hi {
    while(lo<hi) &&((result.get(lo as usize).cloned().expect("IndexError: list index out of range")).py_mod(2i32) == 0) {
    lo  = ((lo).py_add(1i32)) as i32;
   
}
while(lo<hi) &&((result.get(hi as usize).cloned().expect("IndexError: list index out of range")).py_mod(2i32) == 1) {
    hi  = ((hi) - (1i32)) as i32;
   
}
if lo<hi {
    let tmp: i32 = result.get(lo as usize).cloned().expect("IndexError: list index out of range");
    result [(lo) as usize] = result.get(hi as usize).cloned().expect("IndexError: list index out of range");
    result [(hi) as usize] = tmp;
    lo  = ((lo).py_add(1i32)) as i32;
    hi  = ((hi) - (1i32)) as i32;
   
}
} Ok(result)
}
pub fn test_sort_by_parity() -> Result<i32, Box<dyn std::error::Error>>{
    let mut valid: bool = Default::default();
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= sort_by_parity(& vec! [3, 1, 2, 4]) ?;
    let _cse_temp_0 = r1 == vec! [2, 4, 3, 1];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= sort_by_parity(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= sort_by_parity_inplace(& vec! [3, 1, 2, 4]) ?;
    let mut phase: i32 = 0;
    valid = true;
    for val in r3.iter().cloned() {
    if phase == 0 {
    if(val).py_mod(2i32) == 1 {
    phase = 1;
   
}
} else {
    if phase == 1 {
    if(val).py_mod(2i32) == 0 {
    valid = false;
   
}
}
}
} if valid {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Produce arr[0]<=arr[1]>=arr[2]<=arr[3]...pattern."] pub fn wiggle_sort(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut i: i32 = 0;
    while i <(n) - (1i32) {
    if(i).py_mod(2i32) == 0 {
    if result.get(i as usize).cloned().expect("IndexError: list index out of range")>{
    let base = & result;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    let tmp: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    result [(i) as usize] = {
    let base = & result;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    result [((i).py_add(1i32)) as usize] = tmp;
   
}
} else {
    if result.get(i as usize).cloned().expect("IndexError: list index out of range")<{
    let base = & result;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    let tmp2: i32 = result.get(i as usize).cloned().expect("IndexError: list index out of range");
    result [(i) as usize] = {
    let base = & result;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") };
    result [((i).py_add(1i32)) as usize] = tmp2;
   
}
} i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
pub fn test_wiggle_sort() -> Result<i32, Box<dyn std::error::Error>>{
    let mut valid: bool = Default::default();
    let mut ok: i32 = Default::default();
    ok = 0;
    let r: Vec<i32>= wiggle_sort(& vec! [3, 5, 2, 1, 6, 4]) ?;
    let _cse_temp_0 = r.len() as i32;
    let n: i32 = _cse_temp_0;
    valid = true;
    let mut i: i32 = 0;
    while i <(n) - (1i32) {
    if(i).py_mod(2i32) == 0 {
    if r.get(i as usize).cloned().expect("IndexError: list index out of range")>{
    let base = & r;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    valid = false;
   
}
} else {
    if r.get(i as usize).cloned().expect("IndexError: list index out of range")<{
    let base = & r;
    let idx: i32  = (i).py_add(1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    valid = false;
   
}
} i  = ((i).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = n == 6;
    let _cse_temp_2  = (valid) &&(_cse_temp_1);
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= wiggle_sort(& vec! []) ?;
    let _cse_temp_3 = r2.is_empty();
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= wiggle_sort(& vec! [1]) ?;
    let _cse_temp_4 = r3 == vec! [1];
    if _cse_temp_4 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Remove duplicates from sorted array, return new array."] pub fn remove_duplicates_sorted(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
    return Ok(vec! []);
   
}
let mut result: Vec<i32>= vec! [arr.get(0usize).cloned().expect("IndexError: list index out of range")];
    let mut i: i32 = 1;
    while i<arr.len() as i32 {
    if arr.get(i as usize).cloned().expect("IndexError: list index out of range") != {
    let base = & arr;
    let idx: i32  = (i) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
}
{
    result.push(arr.get(i as usize).cloned().expect("IndexError: list index out of range"));
   
}
i  = ((i).py_add(1i32)) as i32;
   
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_remove_duplicates_sorted() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= remove_duplicates_sorted(& vec! [1, 1, 2, 2, 3, 4, 4, 5]) ?;
    let _cse_temp_0 = r1 == vec! [1, 2, 3, 4, 5];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= remove_duplicates_sorted(& vec! []) ?;
    let _cse_temp_1 = r2.is_empty();
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= remove_duplicates_sorted(& vec! [7, 7, 7]) ?;
    let _cse_temp_2 = r3 == vec! [7];
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Sort array containing only 0, 1, 2 using single pass."] pub fn sort_colors(arr: & Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let mut result: Vec<i32>= vec! [];
    for val in arr.iter().cloned() {
    result.push(val);
   
}
let _cse_temp_0 = result.len() as i32;
    let n: i32 = _cse_temp_0;
    let mut lo: i32 = 0;
    let mut mid: i32 = 0;
    let mut hi: i32  = ((n) - (1i32)) as i32;
    while mid <= hi {
    if result.get(mid as usize).cloned().expect("IndexError: list index out of range") == 0 {
    let tmp: i32 = result.get(lo as usize).cloned().expect("IndexError: list index out of range");
    result [(lo) as usize] = result.get(mid as usize).cloned().expect("IndexError: list index out of range");
    result [(mid) as usize] = tmp;
    lo  = ((lo).py_add(1i32)) as i32;
    mid  = ((mid).py_add(1i32)) as i32;
   
}
else {
    if result.get(mid as usize).cloned().expect("IndexError: list index out of range") == 1 {
    mid  = ((mid).py_add(1i32)) as i32;
   
}
else {
    let tmp2: i32 = result.get(mid as usize).cloned().expect("IndexError: list index out of range");
    result [(mid) as usize] = result.get(hi as usize).cloned().expect("IndexError: list index out of range");
    result [(hi) as usize] = tmp2;
    hi  = ((hi) - (1i32)) as i32;
   
}
}
}
Ok(result)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_sort_colors() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let r1: Vec<i32>= sort_colors(& vec! [2, 0, 2, 1, 1, 0]) ?;
    let _cse_temp_0 = r1 == vec! [0, 0, 1, 1, 2, 2];
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r2: Vec<i32>= sort_colors(& vec! [2, 0, 1]) ?;
    let _cse_temp_1 = r2 == vec! [0, 1, 2];
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r3: Vec<i32>= sort_colors(& vec! []) ?;
    let _cse_temp_2 = r3.is_empty();
    if _cse_temp_2 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let r4: Vec<i32>= sort_colors(& vec! [0]) ?;
    let _cse_temp_3 = r4 == vec! [0];
    if _cse_temp_3 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = "Find median of two sorted arrays(integer division for simplicity)."] #[doc = " Depyler: proven to terminate"] pub fn median_two_sorted<'a, 'b>(a: & 'a Vec<i32>, b: & 'b Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>{
    let merged: Vec<i32>= merge_two_sorted(& a, & b) ?;
    let _cse_temp_0 = merged.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n == 0;
    if _cse_temp_1 {
    return Ok(0);
   
}
let _cse_temp_2  = ((n).py_mod(2i32)) as i32;
    let _cse_temp_3 = _cse_temp_2 == 1;
    if _cse_temp_3 {
    return Ok({ let base = & merged;
    let idx: i32 = {
    let a = n;
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
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") });
   
}
else {
    return Ok({ let a  = ({ let base = & merged;
    let idx: i32  = ({ let a = n;
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
}) - (1i32);
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") }).py_add({ let base = & merged;
    let idx: i32 = {
    let a = n;
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
    let actual_idx = if idx<0 {
    base.len().saturating_sub(idx.abs() as usize)
}
else {
    idx as usize };
    base.get(actual_idx).cloned().expect("IndexError: list index out of range") });
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
});
   
}
} #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_median_two_sorted() -> Result<i32, Box<dyn std::error::Error>>{
    let mut ok: i32 = Default::default();
    ok = 0;
    let _cse_temp_0 = median_two_sorted(& vec! [1, 3], & vec! [2]) ? == 2;
    if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
if _cse_temp_0 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
let _cse_temp_1 = median_two_sorted(& vec! [], & vec! [1]) ? == 1;
    if _cse_temp_1 {
    ok  = ((ok).py_add(1i32)) as i32;
   
}
Ok(ok)
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn run_all_tests() -> Result<i32, Box<dyn std::error::Error>>{
    let mut total: i32 = 0;
    let _cse_temp_0  = ((total).py_add(test_insertion_sort() ?)) as i32;
    total = _cse_temp_0;
    let _cse_temp_1  = ((total).py_add(test_selection_sort() ?)) as i32;
    total = _cse_temp_1;
    let _cse_temp_2  = ((total).py_add(test_iterative_merge_sort() ?)) as i32;
    total = _cse_temp_2;
    let _cse_temp_3  = ((total).py_add(test_iterative_quicksort() ?)) as i32;
    total = _cse_temp_3;
    let _cse_temp_4  = ((total).py_add(test_dutch_national_flag() ?)) as i32;
    total = _cse_temp_4;
    let _cse_temp_5  = ((total).py_add(test_three_way_partition() ?)) as i32;
    total = _cse_temp_5;
    let _cse_temp_6  = ((total).py_add(test_lower_bound() ?)) as i32;
    total = _cse_temp_6;
    let _cse_temp_7  = ((total).py_add(test_upper_bound() ?)) as i32;
    total = _cse_temp_7;
    let _cse_temp_8  = ((total).py_add(test_search_insert_position() ?)) as i32;
    total = _cse_temp_8;
    let _cse_temp_9  = ((total).py_add(test_interpolation_search() ?)) as i32;
    total = _cse_temp_9;
    let _cse_temp_10  = ((total).py_add(test_exponential_search() ?)) as i32;
    total = _cse_temp_10;
    let _cse_temp_11  = ((total).py_add(test_ternary_search_max() ?)) as i32;
    total = _cse_temp_11;
    let _cse_temp_12  = ((total).py_add(test_quickselect() ?)) as i32;
    total = _cse_temp_12;
    let _cse_temp_13  = ((total).py_add(test_merge_k_sorted() ?)) as i32;
    total = _cse_temp_13;
    let _cse_temp_14  = ((total).py_add(test_chunk_sort() ?)) as i32;
    total = _cse_temp_14;
    let _cse_temp_15  = ((total).py_add(test_counting_sort() ?)) as i32;
    total = _cse_temp_15;
    let _cse_temp_16  = ((total).py_add(test_radix_sort_lsd() ?)) as i32;
    total = _cse_temp_16;
    let _cse_temp_17  = ((total).py_add(test_sift_down() ?)) as i32;
    total = _cse_temp_17;
    let _cse_temp_18  = ((total).py_add(test_sift_up() ?)) as i32;
    total = _cse_temp_18;
    let _cse_temp_19  = ((total).py_add(test_heapify() ?)) as i32;
    total = _cse_temp_19;
    let _cse_temp_20  = ((total).py_add(test_heap_sort() ?)) as i32;
    total = _cse_temp_20;
    let _cse_temp_21  = ((total).py_add(test_topological_sort() ?)) as i32;
    total = _cse_temp_21;
    let _cse_temp_22  = ((total).py_add(test_count_inversions() ?)) as i32;
    total = _cse_temp_22;
    let _cse_temp_23  = ((total).py_add(test_run_length_encode() ?)) as i32;
    total = _cse_temp_23;
    let _cse_temp_24  = ((total).py_add(test_run_length_decode() ?)) as i32;
    total = _cse_temp_24;
    let _cse_temp_25  = ((total).py_add(test_rle_roundtrip() ?)) as i32;
    total = _cse_temp_25;
    let _cse_temp_26  = ((total).py_add(test_naive_pattern_match() ?)) as i32;
    total = _cse_temp_26;
    let _cse_temp_27  = ((total).py_add(test_kmp_failure() ?)) as i32;
    total = _cse_temp_27;
    let _cse_temp_28  = ((total).py_add(test_kmp_search() ?)) as i32;
    total = _cse_temp_28;
    let _cse_temp_29  = ((total).py_add(test_shell_sort() ?)) as i32;
    total = _cse_temp_29;
    let _cse_temp_30  = ((total).py_add(test_comb_sort() ?)) as i32;
    total = _cse_temp_30;
    let _cse_temp_31  = ((total).py_add(test_cocktail_shaker_sort() ?)) as i32;
    total = _cse_temp_31;
    let _cse_temp_32  = ((total).py_add(test_gnome_sort() ?)) as i32;
    total = _cse_temp_32;
    let _cse_temp_33  = ((total).py_add(test_pancake_sort() ?)) as i32;
    total = _cse_temp_33;
    let _cse_temp_34  = ((total).py_add(test_cycle_sort() ?)) as i32;
    total = _cse_temp_34;
    let _cse_temp_35  = ((total).py_add(test_first_last_occurrence() ?)) as i32;
    total = _cse_temp_35;
    let _cse_temp_36  = ((total).py_add(test_count_occurrences() ?)) as i32;
    total = _cse_temp_36;
    let _cse_temp_37  = ((total).py_add(test_search_rotated() ?)) as i32;
    total = _cse_temp_37;
    let _cse_temp_38  = ((total).py_add(test_find_min_rotated() ?)) as i32;
    total = _cse_temp_38;
    let _cse_temp_39  = ((total).py_add(test_find_peak_element() ?)) as i32;
    total = _cse_temp_39;
    let _cse_temp_40  = ((total).py_add(test_two_sum_sorted() ?)) as i32;
    total = _cse_temp_40;
    let _cse_temp_41  = ((total).py_add(test_merge_in_place_sim() ?)) as i32;
    total = _cse_temp_41;
    let _cse_temp_42  = ((total).py_add(test_bitonic_sort() ?)) as i32;
    total = _cse_temp_42;
    let _cse_temp_43  = ((total).py_add(test_patience_sort_lis_length() ?)) as i32;
    total = _cse_temp_43;
    let _cse_temp_44  = ((total).py_add(test_counting_sort_stable() ?)) as i32;
    total = _cse_temp_44;
    let _cse_temp_45  = ((total).py_add(test_next_greater_element() ?)) as i32;
    total = _cse_temp_45;
    let _cse_temp_46  = ((total).py_add(test_inversions_cross_check() ?)) as i32;
    total = _cse_temp_46;
    let _cse_temp_47  = ((total).py_add(test_min_swaps_to_sort() ?)) as i32;
    total = _cse_temp_47;
    let _cse_temp_48  = ((total).py_add(test_sort_by_parity() ?)) as i32;
    total = _cse_temp_48;
    let _cse_temp_49  = ((total).py_add(test_wiggle_sort() ?)) as i32;
    total = _cse_temp_49;
    let _cse_temp_50  = ((total).py_add(test_remove_duplicates_sorted() ?)) as i32;
    total = _cse_temp_50;
    let _cse_temp_51  = ((total).py_add(test_sort_colors() ?)) as i32;
    total = _cse_temp_51;
    let _cse_temp_52  = ((total).py_add(test_median_two_sorted() ?)) as i32;
    total = _cse_temp_52;
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
    #[test] fn quickcheck_insertion_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = insertion_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = insertion_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = insertion_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_insertion_sort_examples() {
    assert_eq!(insertion_sort(vec! []), vec! []);
    assert_eq!(insertion_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_insertion_sort_examples() {
    let _ = test_insertion_sort();
   
}
#[test] fn quickcheck_selection_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = selection_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = selection_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = selection_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_selection_sort_examples() {
    assert_eq!(selection_sort(vec! []), vec! []);
    assert_eq!(selection_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_selection_sort_examples() {
    let _ = test_selection_sort();
   
}
#[test] fn quickcheck_iterative_merge_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = iterative_merge_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = iterative_merge_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = iterative_merge_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_iterative_merge_sort_examples() {
    assert_eq!(iterative_merge_sort(vec! []), vec! []);
    assert_eq!(iterative_merge_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_iterative_merge_sort_examples() {
    let _ = test_iterative_merge_sort();
   
}
#[test] fn quickcheck_iterative_quicksort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = iterative_quicksort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = iterative_quicksort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = iterative_quicksort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_iterative_quicksort_examples() {
    assert_eq!(iterative_quicksort(vec! []), vec! []);
    assert_eq!(iterative_quicksort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_iterative_quicksort_examples() {
    let _ = test_iterative_quicksort();
   
}
#[test] fn test_test_dutch_national_flag_examples() {
    let _ = test_dutch_national_flag();
   
}
#[test] fn test_test_three_way_partition_examples() {
    let _ = test_three_way_partition();
   
}
#[test] fn test_test_lower_bound_examples() {
    let _ = test_lower_bound();
   
}
#[test] fn test_test_upper_bound_examples() {
    let _ = test_upper_bound();
   
}
#[test] fn test_test_search_insert_position_examples() {
    let _ = test_search_insert_position();
   
}
#[test] fn test_test_interpolation_search_examples() {
    let _ = test_interpolation_search();
   
}
#[test] fn test_test_exponential_search_examples() {
    let _ = test_exponential_search();
   
}
#[test] fn test_ternary_search_max_examples() {
    assert_eq!(ternary_search_max(& vec! []), 0);
    assert_eq!(ternary_search_max(& vec! [1]), 1);
    assert_eq!(ternary_search_max(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_ternary_search_max_examples() {
    let _ = test_ternary_search_max();
   
}
#[test] fn test_test_quickselect_examples() {
    let _ = test_quickselect();
   
}
#[test] fn quickcheck_merge_two_sorted() {
    fn prop(a: Vec<i32>, b: Vec<i32>) -> TestResult {
    let result = merge_two_sorted(& a, & b);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = a.clone();
    input_sorted.sort();
    let mut result = merge_two_sorted(& a);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>, Vec<i32>) -> TestResult);
   
}
#[test] fn quickcheck_merge_k_sorted() {
    fn prop(lists: Vec<Vec<i32>>) -> TestResult {
    let input_len = lists.len();
    let result = merge_k_sorted(& lists);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = merge_k_sorted(& lists);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = lists.clone();
    input_sorted.sort();
    let mut result = merge_k_sorted(& lists);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<Vec<i32>>) -> TestResult);
   
}
#[test] fn test_merge_k_sorted_examples() {
    assert_eq!(merge_k_sorted(vec! []), vec! []);
    assert_eq!(merge_k_sorted(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_merge_k_sorted_examples() {
    let _ = test_merge_k_sorted();
   
}
#[test] fn quickcheck_chunk_sort() {
    fn prop(arr: Vec<i32>, chunk_size: i32) -> TestResult {
    let result = chunk_sort(& arr, chunk_size.clone());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = chunk_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>, i32) -> TestResult);
   
}
#[test] fn test_test_chunk_sort_examples() {
    let _ = test_chunk_sort();
   
}
#[test] fn quickcheck_counting_sort() {
    fn prop(arr: Vec<i32>, max_val: i32) -> TestResult {
    let result = counting_sort(& arr, max_val.clone());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = counting_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>, i32) -> TestResult);
   
}
#[test] fn test_test_counting_sort_examples() {
    let _ = test_counting_sort();
   
}
#[test] fn quickcheck_counting_sort_by_digit() {
    fn prop(arr: Vec<i32>, exp: i32) -> TestResult {
    let result = counting_sort_by_digit(& arr, exp.clone());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = counting_sort_by_digit(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>, i32) -> TestResult);
   
}
#[test] fn quickcheck_radix_sort_lsd() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = radix_sort_lsd(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = radix_sort_lsd(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = radix_sort_lsd(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_radix_sort_lsd_examples() {
    assert_eq!(radix_sort_lsd(vec! []), vec! []);
    assert_eq!(radix_sort_lsd(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_radix_sort_lsd_examples() {
    let _ = test_radix_sort_lsd();
   
}
#[test] fn test_test_sift_down_examples() {
    let _ = test_sift_down();
   
}
#[test] fn test_test_sift_up_examples() {
    let _ = test_sift_up();
   
}
#[test] fn test_heapify_examples() {
    assert_eq!(heapify(vec! []), vec! []);
    assert_eq!(heapify(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_heapify_examples() {
    let _ = test_heapify();
   
}
#[test] fn quickcheck_heap_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = heap_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = heap_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = heap_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_heap_sort_examples() {
    assert_eq!(heap_sort(vec! []), vec! []);
    assert_eq!(heap_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_heap_sort_examples() {
    let _ = test_heap_sort();
   
}
#[test] fn quickcheck_topological_sort() {
    fn prop(num_nodes: i32, edges: Vec<Vec<i32>>) -> TestResult {
    let result = topological_sort(num_nodes.clone(), & edges);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = num_nodes.clone();
    input_sorted.sort();
    let mut result = topological_sort(num_nodes.clone());
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(i32, Vec<Vec<i32>>) -> TestResult);
   
}
#[test] fn test_test_topological_sort_examples() {
    let _ = test_topological_sort();
   
}
#[test] fn test_count_inversions_examples() {
    assert_eq!(count_inversions(& vec! []), 0);
    assert_eq!(count_inversions(& vec! [1]), 1);
    assert_eq!(count_inversions(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_count_inversions_examples() {
    let _ = test_count_inversions();
   
}
#[test] fn test_run_length_encode_examples() {
    assert_eq!(run_length_encode(vec! []), vec! []);
    assert_eq!(run_length_encode(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_run_length_encode_examples() {
    let _ = test_run_length_encode();
   
}
#[test] fn test_run_length_decode_examples() {
    assert_eq!(run_length_decode(vec! []), vec! []);
    assert_eq!(run_length_decode(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_run_length_decode_examples() {
    let _ = test_run_length_decode();
   
}
#[test] fn test_test_rle_roundtrip_examples() {
    let _ = test_rle_roundtrip();
   
}
#[test] fn test_test_naive_pattern_match_examples() {
    let _ = test_naive_pattern_match();
   
}
#[test] fn test_kmp_failure_examples() {
    assert_eq!(kmp_failure(vec! []), vec! []);
    assert_eq!(kmp_failure(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_kmp_failure_examples() {
    let _ = test_kmp_failure();
   
}
#[test] fn test_test_kmp_search_examples() {
    let _ = test_kmp_search();
   
}
#[test] fn quickcheck_shell_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = shell_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = shell_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = shell_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_shell_sort_examples() {
    assert_eq!(shell_sort(vec! []), vec! []);
    assert_eq!(shell_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_shell_sort_examples() {
    let _ = test_shell_sort();
   
}
#[test] fn quickcheck_comb_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = comb_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = comb_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = comb_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_comb_sort_examples() {
    assert_eq!(comb_sort(vec! []), vec! []);
    assert_eq!(comb_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_comb_sort_examples() {
    let _ = test_comb_sort();
   
}
#[test] fn quickcheck_cocktail_shaker_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = cocktail_shaker_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = cocktail_shaker_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = cocktail_shaker_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_cocktail_shaker_sort_examples() {
    assert_eq!(cocktail_shaker_sort(vec! []), vec! []);
    assert_eq!(cocktail_shaker_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_cocktail_shaker_sort_examples() {
    let _ = test_cocktail_shaker_sort();
   
}
#[test] fn quickcheck_gnome_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = gnome_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = gnome_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = gnome_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_gnome_sort_examples() {
    assert_eq!(gnome_sort(vec! []), vec! []);
    assert_eq!(gnome_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_gnome_sort_examples() {
    let _ = test_gnome_sort();
   
}
#[test] fn quickcheck_pancake_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = pancake_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = pancake_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = pancake_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_pancake_sort_examples() {
    assert_eq!(pancake_sort(vec! []), vec! []);
    assert_eq!(pancake_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_pancake_sort_examples() {
    let _ = test_pancake_sort();
   
}
#[test] fn quickcheck_cycle_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = cycle_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = cycle_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = cycle_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_cycle_sort_examples() {
    assert_eq!(cycle_sort(vec! []), vec! []);
    assert_eq!(cycle_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_cycle_sort_examples() {
    let _ = test_cycle_sort();
   
}
#[test] fn test_test_first_last_occurrence_examples() {
    let _ = test_first_last_occurrence();
   
}
#[test] fn test_test_count_occurrences_examples() {
    let _ = test_count_occurrences();
   
}
#[test] fn test_test_search_rotated_examples() {
    let _ = test_search_rotated();
   
}
#[test] fn test_find_min_rotated_examples() {
    assert_eq!(find_min_rotated(& vec! []), 0);
    assert_eq!(find_min_rotated(& vec! [1]), 1);
    assert_eq!(find_min_rotated(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_find_min_rotated_examples() {
    let _ = test_find_min_rotated();
   
}
#[test] fn test_find_peak_element_examples() {
    assert_eq!(find_peak_element(& vec! []), 0);
    assert_eq!(find_peak_element(& vec! [1]), 1);
    assert_eq!(find_peak_element(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_find_peak_element_examples() {
    let _ = test_find_peak_element();
   
}
#[test] fn quickcheck_two_sum_sorted() {
    fn prop(arr: Vec<i32>, target: i32) -> TestResult {
    let result = two_sum_sorted(& arr, target.clone());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = two_sum_sorted(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>, i32) -> TestResult);
   
}
#[test] fn test_test_two_sum_sorted_examples() {
    let _ = test_two_sum_sorted();
   
}
#[test] fn test_test_merge_in_place_sim_examples() {
    let _ = test_merge_in_place_sim();
   
}
#[test] fn quickcheck_bitonic_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = bitonic_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = bitonic_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = bitonic_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_bitonic_sort_examples() {
    assert_eq!(bitonic_sort(vec! []), vec! []);
    assert_eq!(bitonic_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_bitonic_sort_examples() {
    let _ = test_bitonic_sort();
   
}
#[test] fn quickcheck_patience_sort_lis_length() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let result = patience_sort_lis_length(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = patience_sort_lis_length(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_patience_sort_lis_length_examples() {
    assert_eq!(patience_sort_lis_length(& vec! []), 0);
    assert_eq!(patience_sort_lis_length(& vec! [1]), 1);
    assert_eq!(patience_sort_lis_length(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_patience_sort_lis_length_examples() {
    let _ = test_patience_sort_lis_length();
   
}
#[test] fn quickcheck_counting_sort_stable() {
    fn prop(arr: Vec<i32>, max_val: i32) -> TestResult {
    let result = counting_sort_stable(& arr, max_val.clone());
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = counting_sort_stable(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>, i32) -> TestResult);
   
}
#[test] fn test_test_counting_sort_stable_examples() {
    let _ = test_counting_sort_stable();
   
}
#[test] fn test_next_greater_element_examples() {
    assert_eq!(next_greater_element(vec! []), vec! []);
    assert_eq!(next_greater_element(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_next_greater_element_examples() {
    let _ = test_next_greater_element();
   
}
#[test] fn test_count_inversions_brute_examples() {
    assert_eq!(count_inversions_brute(& vec! []), 0);
    assert_eq!(count_inversions_brute(& vec! [1]), 1);
    assert_eq!(count_inversions_brute(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_inversions_cross_check_examples() {
    let _ = test_inversions_cross_check();
   
}
#[test] fn quickcheck_min_swaps_to_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let result = min_swaps_to_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = min_swaps_to_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_min_swaps_to_sort_examples() {
    assert_eq!(min_swaps_to_sort(& vec! []), 0);
    assert_eq!(min_swaps_to_sort(& vec! [1]), 1);
    assert_eq!(min_swaps_to_sort(& vec! [1, 2, 3]), 3);
   
}
#[test] fn test_test_min_swaps_to_sort_examples() {
    let _ = test_min_swaps_to_sort();
   
}
#[test] fn quickcheck_sort_by_parity() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = sort_by_parity(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = sort_by_parity(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = sort_by_parity(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_sort_by_parity_examples() {
    assert_eq!(sort_by_parity(vec! []), vec! []);
    assert_eq!(sort_by_parity(vec! [1]), vec! [1]);
   
}
#[test] fn quickcheck_sort_by_parity_inplace() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = sort_by_parity_inplace(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = sort_by_parity_inplace(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = sort_by_parity_inplace(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_sort_by_parity_inplace_examples() {
    assert_eq!(sort_by_parity_inplace(vec! []), vec! []);
    assert_eq!(sort_by_parity_inplace(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_sort_by_parity_examples() {
    let _ = test_sort_by_parity();
   
}
#[test] fn quickcheck_wiggle_sort() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = wiggle_sort(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = wiggle_sort(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = wiggle_sort(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_wiggle_sort_examples() {
    assert_eq!(wiggle_sort(vec! []), vec! []);
    assert_eq!(wiggle_sort(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_wiggle_sort_examples() {
    let _ = test_wiggle_sort();
   
}
#[test] fn quickcheck_remove_duplicates_sorted() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = remove_duplicates_sorted(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = remove_duplicates_sorted(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = remove_duplicates_sorted(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_remove_duplicates_sorted_examples() {
    assert_eq!(remove_duplicates_sorted(vec! []), vec! []);
    assert_eq!(remove_duplicates_sorted(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_remove_duplicates_sorted_examples() {
    let _ = test_remove_duplicates_sorted();
   
}
#[test] fn quickcheck_sort_colors() {
    fn prop(arr: Vec<i32>) -> TestResult {
    let input_len = arr.len();
    let result = sort_colors(& arr);
    if result.len() != input_len {
    return TestResult::failed();
   
}
let result = sort_colors(& arr);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = arr.clone();
    input_sorted.sort();
    let mut result = sort_colors(& arr);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>) -> TestResult);
   
}
#[test] fn test_sort_colors_examples() {
    assert_eq!(sort_colors(vec! []), vec! []);
    assert_eq!(sort_colors(vec! [1]), vec! [1]);
   
}
#[test] fn test_test_sort_colors_examples() {
    let _ = test_sort_colors();
   
}
#[test] fn quickcheck_median_two_sorted() {
    fn prop(a: Vec<i32>, b: Vec<i32>) -> TestResult {
    let result = median_two_sorted(& a, & b);
    for i in 1..result.len() {
    if result [i - 1]>result [i] {
    return TestResult::failed();
   
}
} let mut input_sorted = a.clone();
    input_sorted.sort();
    let mut result = median_two_sorted(& a);
    result.sort();
    if input_sorted != result {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(Vec<i32>, Vec<i32>) -> TestResult);
   
}
#[test] fn test_test_median_two_sorted_examples() {
    let _ = test_median_two_sorted();
   
}
#[test] fn test_run_all_tests_examples() {
    let _ = run_all_tests();
   
}
}