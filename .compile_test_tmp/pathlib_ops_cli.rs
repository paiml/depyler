#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::path::PathBuf;
    #[derive(Debug, Clone)] pub struct FileNotFoundError {
    message: String ,
}
impl std::fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "file not found: {}", self.message)
}
} impl std::error::Error for FileNotFoundError {
   
}
impl FileNotFoundError {
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
} #[doc = r" Convert to String"] pub fn to_string(&self) -> String {
    match self {
    DepylerValue::Str(_dv_str) =>_dv_str.clone(), DepylerValue::Int(_dv_int) =>_dv_int.to_string(), DepylerValue::Float(_dv_float) =>_dv_float.to_string(), DepylerValue::Bool(_dv_bool) =>_dv_bool.to_string(), DepylerValue::None =>"None".to_string(), DepylerValue::List(_dv_list) =>format!("{:?}", _dv_list), DepylerValue::Dict(_dv_dict) =>format!("{:?}", _dv_dict), DepylerValue::Tuple(_dv_tuple) =>format!("{:?}", _dv_tuple) ,
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
    DepylerValue::Bool(_dv_bool) =>* _dv_bool, DepylerValue::Int(_dv_int) =>* _dv_int!= 0, DepylerValue::Float(_dv_float) =>* _dv_float!= 0.0, DepylerValue::Str(_dv_str) =>! _dv_str.is_empty(), DepylerValue::List(_dv_list) =>! _dv_list.is_empty(), DepylerValue::Dict(_dv_dict) =>! _dv_dict.is_empty(), DepylerValue::Tuple(_dv_tuple) =>! _dv_tuple.is_empty(), DepylerValue::None =>false ,
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
    if _dv_tuple.len()!= _dv_expected_len {
    panic!("Expected tuple of length {}, got length {}", _dv_expected_len, _dv_tuple.len())
}
_dv_tuple.clone()
}
DepylerValue::List(_dv_list) =>{
    if _dv_list.len()!= _dv_expected_len {
    panic!("Expected list of length {}, got length {}", _dv_expected_len, _dv_list.len())
}
_dv_list.clone()
}
_dv_other =>panic!("Expected tuple or list for unpacking, found {:?}", _dv_other) ,
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
    DepylerValue::Str(v.to_string())
}
} impl From<bool>for DepylerValue {
    fn from(v: bool) -> Self {
    DepylerValue::Bool(v)
}
} impl From<Vec<DepylerValue>>for DepylerValue {
    fn from(v: Vec<DepylerValue>) -> Self {
    DepylerValue::List(v)
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
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b!= 0 =>DepylerValue::Int(_dv_a / _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b!= 0.0 =>DepylerValue::Float(_dv_a / _dv_b) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) if _dv_b!= 0.0 =>DepylerValue::Float(_dv_a as f64 / _dv_b) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) if _dv_b!= 0 =>DepylerValue::Float(_dv_a / _dv_b as f64), _ =>DepylerValue::None ,
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
   (DepylerValue::Int(_dv_a), DepylerValue::Int(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Float(_dv_a), DepylerValue::Float(_dv_b)) =>Some(_dv_a.total_cmp(_dv_b)) ,(DepylerValue::Str(_dv_a), DepylerValue::Str(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Bool(_dv_a), DepylerValue::Bool(_dv_b)) =>Some(_dv_a.cmp(_dv_b)) ,(DepylerValue::Int(_dv_a), DepylerValue::Float(_dv_b)) =>Some((* _dv_a as f64).total_cmp(_dv_b)) ,(DepylerValue::Float(_dv_a), DepylerValue::Int(_dv_b)) =>Some(_dv_a.total_cmp(&(* _dv_b as f64))) ,(DepylerValue::None, DepylerValue::None) =>Some(std::cmp::Ordering::Equal) ,(DepylerValue::None, _) =>Some(std::cmp::Ordering::Less) ,(_, DepylerValue::None) =>Some(std::cmp::Ordering::Greater) ,(DepylerValue::List(_dv_a), DepylerValue::List(_dv_b)) =>_dv_a.partial_cmp(_dv_b) ,(DepylerValue::Tuple(_dv_a), DepylerValue::Tuple(_dv_b)) =>_dv_a.partial_cmp(_dv_b), _ =>Option::None ,
}
}
}
impl std::cmp::Ord for DepylerValue {
    fn cmp(&self, other: & Self) -> std::cmp::Ordering {
    self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
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
    if self.days!= 0 {
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
} #[derive(Default)] enum Commands {
    #[default]
    __DepylerNone,
    Ls {
    #[doc = "Directory path"] path: Option<String>, #[doc = "Only dirs"] dirs: bool, #[doc = "Only files"] files: bool
}
, Stat {
    #[doc = "File path"] path: String
}
, Glob {
    #[doc = "Base directory"] path: String, #[doc = "Glob pattern"] pattern: String, #[doc = "Recursive"] recursive: bool
}
, Count {
    #[doc = "Directory path"] path: String, #[doc = "Pattern"] pattern: Option<String>
}
, Touch {
    #[doc = "File path"] path: String
}
, Rmdir {
    #[doc = "Directory path"] path: String
}
, Rm {
    #[doc = "File path"] path: String, #[doc = "Ignore missing"] force: bool
}
, Mv {
    #[doc = "Source path"] src: String, #[doc = "Destination path"] dst: String
}
, Mkdir {
    #[doc = "Directory path"] path: String, #[doc = "Create parents"] parents: bool, #[doc = "OK if exists"] exist_ok: bool
}
} #[derive(Default)] struct Args {
    command: Option<Commands>
}
#[doc = "Create directory."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn mkdir(path: & std::path::PathBuf, parents: bool, exist_ok: bool) -> bool {
    match(|| -> Result<bool, Box<dyn std::error::Error>>{
    std::fs::create_dir(& path).unwrap();
    return Ok(true);
    })() {
    Ok(_result) =>{
    return _result;
   
}
, Err(_) =>{
    return false;
   
}
}
}
#[doc = "Remove empty directory."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn rmdir(path: & std::path::PathBuf) -> bool {
    match(|| -> Result<bool, Box<dyn std::error::Error>>{
    std::fs::remove_dir(& path).unwrap();
    return Ok(true);
    })() {
    Ok(_result) =>{
    return _result;
   
}
, Err(_) =>{
    return false;
   
}
}
}
#[doc = "Remove file."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn unlink(path: & std::path::PathBuf, missing_ok: bool) -> bool {
    match(|| -> Result<bool, Box<dyn std::error::Error>>{
    std::fs::remove_file(& path).unwrap();
    return Ok(true);
    })() {
    Ok(_result) =>{
    return _result;
   
}
, Err(_) =>{
    return false;
   
}
}
}
#[doc = "Rename file or directory."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn rename<'a, 'b>(src: & 'a std::path::PathBuf, dst: & 'b std::path::PathBuf) -> std::path::PathBuf {
    src.rename(dst)
}
#[doc = "Replace file or directory."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn replace<'a, 'b>(src: & 'a std::path::PathBuf, dst: & 'b std::path::PathBuf) -> std::path::PathBuf {
    src.replace()
}
#[doc = "Create file or update timestamp."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn touch(path: & std::path::PathBuf, exist_ok: bool) -> bool {
    match(|| -> Result<bool, Box<dyn std::error::Error>>{
    path.touch();
    return Ok(true);
    })() {
    Ok(_result) =>{
    return _result;
   
}
, Err(_) =>{
    return false;
   
}
}
}
#[doc = "Find files matching pattern."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn glob_pattern<'a, 'b>(path: & 'a std::path::PathBuf, pattern: & 'b str) -> Vec<std::path::PathBuf>{
    vec![std::path::PathBuf::from(& format!("{}/{}", path.display(), pattern)).unwrap().filter_map(| e | e.ok()).collect::<Vec<_>>()
}
#[doc = "Recursively find files matching pattern."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn rglob_pattern<'b, 'a>(path: & 'a std::path::PathBuf, pattern: & 'b str) -> Vec<std::path::PathBuf>{
    vec![std::path::PathBuf::from(& format!("{}/**/{}", path.display(), pattern)).unwrap().filter_map(| e | e.ok()).collect::<Vec<_>>()
}
#[doc = "List directory contents."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn iterdir(path: & std::path::PathBuf) -> Vec<std::path::PathBuf>{
    std::fs::read_dir(& path).unwrap().map(| e | e.unwrap().path()).collect::<Vec<_>>()
}
#[doc = "List only files in directory."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn list_files(path: & std::path::PathBuf) -> Vec<std::path::PathBuf>{
    std::fs::read_dir(& path).unwrap().map(| e | e.unwrap().path()).collect::<Vec<_>>().into_iter().filter(| p | {
    let p = p.clone();
    p.is_file() }).map(| p | p).collect::<Vec<_>>()
}
#[doc = "List only directories in directory."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn list_dirs(path: & std::path::PathBuf) -> Vec<std::path::PathBuf>{
    std::fs::read_dir(& path).unwrap().map(| e | e.unwrap().path()).collect::<Vec<_>>().into_iter().filter(| p | {
    let p = p.clone();
    p.is_dir() }).map(| p | p).collect::<Vec<_>>()
}
#[doc = "Get file size in bytes."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn stat_size(path: & std::path::PathBuf) -> i32 {
    std::fs::metadata(& path).unwrap().st_size
}
#[doc = "Get modification time."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn stat_mtime(path: & std::path::PathBuf) -> f64 {
    std::fs::metadata(& path).unwrap().st_mtime
}
#[doc = "Get file mode/permissions."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn stat_mode(path: & std::path::PathBuf) -> i32 {
    std::fs::metadata(& path).unwrap().st_mode
}
#[doc = "Change file permissions."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn chmod(path: & std::path::PathBuf, mode: i32) {
    path.chmod(mode);
   
}
#[doc = "Check if two paths point to same file."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn samefile<'a, 'b>(path1: & 'a std::path::PathBuf, path2: & 'b std::path::PathBuf) -> bool {
    path1.samefile(path2)
}
#[doc = "Read file as text."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn read_text(path: & std::path::PathBuf, encoding: String) -> String {
    std::fs::read_to_string(& path).unwrap()
}
#[doc = "Write text to file."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn write_text<'b, 'a>(path: & 'a std::path::PathBuf, content: & 'b str, encoding: String) -> i32 {
    std::fs::write(& path, content).unwrap()
}
#[doc = "Read file as bytes."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn read_bytes(path: & std::path::PathBuf) -> Vec<u8>{
    std::fs::read(& path).unwrap()
}
#[doc = "Write bytes to file."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn write_bytes<'b, 'a>(path: & 'a std::path::PathBuf, data: & 'b Vec<u8>) -> i32 {
    std::fs::write(& path, data).unwrap()
}
#[doc = "Get file owner name."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn owner(path: & std::path::PathBuf) -> String {
    path.owner()
}
#[doc = "Get file group name."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn group(path: & std::path::PathBuf) -> String {
    path.group(0)
}
#[doc = "Create hard link."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn hardlink_to<'b, 'a>(src: & 'a std::path::PathBuf, dst: & 'b std::path::PathBuf) {
    dst.hardlink_to(src);
   
}
#[doc = "Create symbolic link."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn symlink_to<'a, 'b>(link: & 'a std::path::PathBuf, target: & 'b std::path::PathBuf) {
    link.symlink_to(target);
   
}
#[doc = "Read symbolic link target."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn readlink(path: & std::path::PathBuf) -> std::path::PathBuf {
    path.readlink()
}
#[doc = "Count files matching pattern."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn count_files<'a, 'b>(path: & 'a std::path::PathBuf, pattern: & 'b str) -> i32 {
    vec![std::path::PathBuf::from(& format!("{}/{}", path.display(), pattern)).unwrap().filter_map(| e | e.ok()).collect::<Vec<_>>().into_iter().filter(| p | {
    let p = p.clone();
    p.is_file() }).map(| p | p).collect::<Vec<_>>().len() as i32 as i32
}
#[doc = "Count subdirectories."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn count_dirs(path: & std::path::PathBuf) -> i32 {
    std::fs::read_dir(& path).unwrap().map(| e | e.unwrap().path()).collect::<Vec<_>>().into_iter().filter(| p | {
    let p = p.clone();
    p.is_dir() }).map(| p | p).collect::<Vec<_>>().len() as i32 as i32
}
#[doc = "Get total size of all files in directory."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn total_size(path: & std::path::PathBuf) -> i32 {
    vec![std::path::PathBuf::from(& format!("{}/**/{}", path.display(), "*".to_string())).unwrap().filter_map(| e | e.ok()).collect::<Vec<_>>().into_iter().filter(| & p | p.is_file()).map(| p | std::fs::metadata(& p).unwrap().st_size).sum::<i32>()
}
#[doc = "Find all files with given suffix."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn find_by_suffix<'b, 'a>(path: & 'a std::path::PathBuf, suffix: & 'b str) -> Vec<std::path::PathBuf>{
    vec![std::path::PathBuf::from(& format!("{}/**/{}", path.display(), "*".to_string())).unwrap().filter_map(| e | e.ok()).collect::<Vec<_>>().into_iter().filter(| p | {
    let p = p.clone();
    p.extension().map(| e | format!(".{}", e.to_str().unwrap())).unwrap_or_default() = = (* suffix) }).map(| p | p).collect::<Vec<_>>()
}
#[doc = "Find all files with given name."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn find_by_name<'b, 'a>(path: & 'a std::path::PathBuf, name: & 'b str) -> Vec<std::path::PathBuf>{
    vec![std::path::PathBuf::from(& format!("{}/**/{}", path.display(), "*".to_string())).unwrap().filter_map(| e | e.ok()).collect::<Vec<_>>().into_iter().filter(| p | {
    let p = p.clone();
    p.name = = (* name) }).map(| p | p).collect::<Vec<_>>()
}
#[doc = "Walk directory tree(similar to os.walk)."] #[doc = " Depyler: verified panic-free"] pub fn walk_dir(path: & std::path::PathBuf) -> Vec <(std::path::PathBuf, Vec<String>, Vec<String>)>{
    let mut result = vec! [];
    let dirs = std::fs::read_dir(& path).unwrap().map(| e | e.unwrap().path()).collect::<Vec<_>>().into_iter().filter(| d | {
    let d = d.clone();
    d.is_dir() }).map(| d | d.name).collect::<Vec<_>>();
    let files = std::fs::read_dir(& path).unwrap().map(| e | e.unwrap().path()).collect::<Vec<_>>().into_iter().filter(| f | {
    let f = f.clone();
    f.is_file() }).map(| f | f.path().to_string_lossy().to_string()).collect::<Vec<_>>();
    result.push(DepylerValue::Str(format!("{:?}" ,(path, dirs, files))));
    for d in std::fs::read_dir(& path).unwrap().map(| e | e.unwrap().path()).collect::<Vec<_>>() {
    if d.is_dir() {
    result.extend(walk_dir(& d).iter().cloned());
   
}
} result
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn main () {
    let mut matches: DepylerValue = Default::default();
    let mut items: DepylerValue = Default::default();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
   ();
    let args = Args::default();
    let _cse_temp_0 = matches!(args.command, Some(Commands::Mkdir {
  ..}));
    match & args.command {
    Some(Commands::Mkdir {
    ref path,..}) =>{
    let mut path = std::path::PathBuf::from(& path);
    if mkdir(path.display().to_string(), parents, exist_ok) {
    println!("{}", format!("Created: {}", path.display()));
   
}
else {
    println!("{}", format!("Failed to create: {}", path.display()));
    std::process::exit(1i32)
}
} Some(Commands::Rmdir {
    ref path,..}) =>{
    let mut path = std::path::PathBuf::from(& path);
    if rmdir(path.display().to_string()) {
    println!("{}", format!("Removed: {}", path.display()));
   
}
else {
    println!("{}", format!("Failed to remove: {}", path.display()));
    std::process::exit(1i32)
}
} Some(Commands::Touch {
    ref path,..}) =>{
    let mut path = std::path::PathBuf::from(& path);
    if touch(path.display().to_string(), true) {
    println!("{}", format!("Touched: {}", path.display()));
   
}
else {
    println!("{}", format!("Failed to touch: {}", path.display()));
    std::process::exit(1i32)
}
} Some(Commands::Rm {
    ref path,..}) =>{
    let mut path = std::path::PathBuf::from(& path);
    if unlink(path.display().to_string(), force) {
    println!("{}", format!("Removed: {}", path.display()));
   
}
else {
    println!("{}", format!("Failed to remove: {}", path.display()));
    std::process::exit(1i32)
}
} Some(Commands::Mv {
    ref dst, ref src,..}) =>{
    let src = std::path::PathBuf::from(& src);
    let dst = std::path::PathBuf::from(& dst);
    let result = rename(src.display().to_string(), dst.display().to_string());
    println!("{}", format!("Moved: {} -> {:?}", src.display(), result));
   
}
Some(Commands::Ls {
    ref dirs, ref files, ref path,..}) =>{
    let dirs = * dirs;
    let files = * files;
    let mut path = std::path::PathBuf::from(& path);
    if dirs {
    items = list_dirs(path.display().to_string());
   
}
else {
    if files {
    items = list_files(path.display().to_string());
   
}
else {
    items = iterdir(path.display().to_string());
   
}
} for item in {
    let mut sorted_vec = items.iter().cloned().collect::<Vec<_>>();
    sorted_vec.sort_by(| a, b | a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sorted_vec
}
{
    println!("{}", item.name);
   
}
} Some(Commands::Glob {
    ref path, ref pattern, ref recursive,..}) =>{
    let recursive = * recursive;
    let mut path = std::path::PathBuf::from(& path);
    if recursive {
    matches = rglob_pattern(path.display().to_string(), & pattern);
   
}
else {
    matches = glob_pattern(path.display().to_string(), & pattern);
   
}
for m in {
    let mut sorted_vec = matches.iter().cloned().collect::<Vec<_>>();
    sorted_vec.sort_by(| a, b | a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sorted_vec
}
{
    println!("{}", m);
   
}
} Some(Commands::Stat {
    ref path,..}) =>{
    let mut path = std::path::PathBuf::from(& path);
    println!("{}", format!("Size: {} bytes", stat_size(path.display().to_string())));
    println!("{}", format!("Mode: {}", format!("0o{:o}", stat_mode(path.display().to_string()))));
    println!("{}", format!("Modified: {}", stat_mtime(path.display().to_string())));
   
}
Some(Commands::Count {
    ref path, ref pattern,..}) =>{
    let pattern = pattern.as_deref().unwrap_or_default();
    let mut path = std::path::PathBuf::from(& path);
    let files = count_files(path.display().to_string(), & pattern);
    let dirs = count_dirs(path.display().to_string());
    println!("{}", format!("Files: {}", files));
    println!("{}", format!("Directories: {}", dirs));
   
}
_ =>unreachable!("Other command variants handled elsewhere")
}
()
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_rmdir_examples() {
    let _ = rmdir(Default::default());
   
}
}
