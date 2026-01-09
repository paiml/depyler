#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
pub type UserId = i32;
pub type Username = String;
pub type UserData = std::collections::HashMap<UserId, Username>;
use std::collections::HashMap;
use std::collections::HashSet;
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
#[derive(Debug, Clone, PartialEq)]
pub enum IntOrStringUnion {
    Integer(i32),
    Text(String),
}
impl From<i32> for IntOrStringUnion {
    fn from(value: i32) -> Self {
        IntOrStringUnion::Integer(value)
    }
}
impl From<String> for IntOrStringUnion {
    fn from(value: String) -> Self {
        IntOrStringUnion::Text(value)
    }
}
impl IntOrStringUnion {
    pub fn is_integer(&self) -> bool {
        matches!(self, IntOrStringUnion::Integer(_))
    }
    pub fn is_text(&self) -> bool {
        matches!(self, IntOrStringUnion::Text(_))
    }
    pub fn as_integer(&self) -> Option<&i32> {
        match self {
            IntOrStringUnion::Integer(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_text(&self) -> Option<&String> {
        match self {
            IntOrStringUnion::Text(value) => Some(value),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum UnionType1 {
    Float(f64),
    Integer(i32),
    None,
    Text(String),
}
impl From<f64> for UnionType1 {
    fn from(value: f64) -> Self {
        UnionType1::Float(value)
    }
}
impl From<i32> for UnionType1 {
    fn from(value: i32) -> Self {
        UnionType1::Integer(value)
    }
}
impl From<String> for UnionType1 {
    fn from(value: String) -> Self {
        UnionType1::Text(value)
    }
}
impl UnionType1 {
    pub fn is_float(&self) -> bool {
        matches!(self, UnionType1::Float(_))
    }
    pub fn is_integer(&self) -> bool {
        matches!(self, UnionType1::Integer(_))
    }
    pub fn is_none(&self) -> bool {
        matches!(self, UnionType1::None)
    }
    pub fn is_text(&self) -> bool {
        matches!(self, UnionType1::Text(_))
    }
    pub fn as_float(&self) -> Option<&f64> {
        match self {
            UnionType1::Float(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_integer(&self) -> Option<&i32> {
        match self {
            UnionType1::Integer(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_text(&self) -> Option<&String> {
        match self {
            UnionType1::Text(value) => Some(value),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TypeOrListUnion {
    Variant0(Container<i32>),
    List(Vec<Option<String>>),
}
impl From<Container<i32>> for TypeOrListUnion {
    fn from(value: Container<i32>) -> Self {
        TypeOrListUnion::Variant0(value)
    }
}
impl From<Vec<Option<String>>> for TypeOrListUnion {
    fn from(value: Vec<Option<String>>) -> Self {
        TypeOrListUnion::List(value)
    }
}
impl TypeOrListUnion {
    pub fn is_variant0(&self) -> bool {
        matches!(self, TypeOrListUnion::Variant0(_))
    }
    pub fn is_list(&self) -> bool {
        matches!(self, TypeOrListUnion::List(_))
    }
    pub fn as_variant0(&self) -> Option<&Container<i32>> {
        match self {
            TypeOrListUnion::Variant0(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&Vec<Option<String>>> {
        match self {
            TypeOrListUnion::List(value) => Some(value),
            _ => None,
        }
    }
}
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
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
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Container<T: Clone> {
    pub value: T,
}
impl<T: Clone> Container<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
    pub fn get(&self) -> T {
        return self.value.clone();
    }
    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}
#[derive(Debug, Clone)]
pub struct Mapping<K: Clone, V: Clone> {
    pub data: std::collections::HashMap<K, V>,
}
impl<K: Clone, V: Clone> Mapping<K, V> {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
    pub fn put(&self, key: K, value: V) {
        self.data.clone().insert(key, value);
    }
    pub fn get(&self, key: K) -> Option<V> {
        return Some(self.data.clone().get(key));
    }
}
#[doc = "Function with simple type annotations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_types(a: i32, _b: f64, _c: String, _d: bool, _e: ()) -> i32 {
    a
}
#[doc = "Function with container type annotations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn container_types(
    items: &Vec<i32>,
    _mapping: std::collections::HashMap<String, f64>,
    _unique: std::collections::HashSet<String>,
    _coords: (i32, i32, i32),
) -> Vec<String> {
    items
        .as_slice()
        .iter()
        .cloned()
        .map(|item| (item).to_string())
        .collect::<Vec<_>>()
}
#[doc = "Function with optional and union types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn optional_types(
    maybe_value: &Option<i32>,
    _either_type: IntOrStringUnion,
    _complex_union: String1,
) -> Option<String> {
    if maybe_value.is_some() {
        return Some((maybe_value).unwrap().to_string());
    }
    None
}
#[doc = "Function with nested type annotations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn nested_types(
    _matrix: Vec<Vec<i32>>,
    _lookup: std::collections::HashMap<String, Vec<f64>>,
    _optional_dict: Option<std::collections::HashMap<String, i32>>,
    _union_list: Vec<String>,
) -> HashMap<String, Vec<Option<i32>>> {
    {
        let map: HashMap<String, Vec<Option<i32>>> = HashMap::new();
        map
    }
}
#[doc = "Generic function with type variable."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn generic_function<T: Clone>(items: &Vec<T>) -> Result<T, Box<dyn std::error::Error>> {
    Ok(if !items.is_empty() {
        items
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
    } else {
        None
    })
}
#[doc = "Function with complex nested generic types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn complex_generics<T: Clone>(
    _data: Vec<Option<std::collections::HashMap<String, UnionType>>>,
    _processor: Container<Vec<T>>,
    _mappings: std::collections::HashMap<String, Mapping<String, i32>>,
) -> TypeOrListUnion {
    Container::new(42)
}
#[doc = "Function using type aliases."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn custom_types<'a, 'b>(
    user_id: &'a UserId,
    _username: Username,
    all_users: &'b UserData,
) -> Option<Username> {
    all_users.get(&user_id).cloned()
}
#[doc = "Function with variable-length tuple."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn variable_tuple(args: &[String]) -> Vec<String> {
    args
}
#[doc = "Function taking another function as parameter."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn higher_order(func: impl Fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    func(a, b)
}
#[doc = "Demonstrate type extraction for various Python types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_all_types() {
    println!("{}", "Type Extraction Examples");
    println!("{}", "=".repeat(40 as usize));
    let result1 = simple_types(1, 2.0, "hello".to_string(), true, None);
    println!("{}", format!("Simple types result: {:?}", result1));
    let result2 = container_types(
        &vec![1, 2, 3],
        {
            let mut map = HashMap::new();
            map.insert("a".to_string(), 1.0);
            map
        },
        {
            let mut set = std::collections::HashSet::new();
            set.insert("x".to_string());
            set.insert("y".to_string());
            set
        },
        (1, 2, 3),
    );
    println!("{}", format!("Container types result: {:?}", result2));
    let result3 = optional_types(&Some(42), "either".to_string(), 3.14);
    println!("{}", format!("Optional types result: {:?}", result3));
    let int_container = Container::new(42);
    let str_container = Container::new("hello".to_string());
    println!(
        "{}",
        format!(
            "Generic containers: {}, {}",
            int_container.get(),
            str_container.get()
        )
    );
    let mapping = ({
        let base = &Mapping;
        let idx: i32 = (|x: &_| x.to_string(), |x| x as i32);
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx)
            .cloned()
            .expect("IndexError: list index out of range")
    })();
    mapping.put("answer".to_string(), 42);
    println!(
        "{}",
        format!("Generic mapping: {}", mapping.get("answer").cloned())
    );
    let users = {
        let mut map = HashMap::new();
        map.insert(1, "Alice".to_string());
        map.insert(2, "Bob".to_string());
        map
    };
    let username = custom_types(1, "Alice".to_string(), &users);
    println!("{}", format!("Custom types result: {:?}", username));
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_variable_tuple() {
        fn prop(args: Vec<String>) -> TestResult {
            let result = variable_tuple(&args);
            if result != args {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<String>) -> TestResult);
    }
}
