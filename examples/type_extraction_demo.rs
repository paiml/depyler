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
#[derive(Debug, Clone)]
pub struct Container {
    pub value: T,
}
impl Container {
    pub fn new(value: T) -> Self {
        Self { value }
    }
    pub fn get(&self) -> T {
        return self.value;
    }
    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}
#[derive(Debug, Clone)]
pub struct Mapping {
    pub data: std::collections::HashMap<K, V>,
}
impl Mapping {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
    pub fn put(&self, key: K, value: V) {
        self.data.insert(key, value);
    }
    pub fn get(&self, key: K) -> Option<V> {
        return self.data.get(key);
    }
}
#[doc = "Function with simple type annotations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_types(a: i32, b: f64, c: String, d: bool, e: ()) -> i32 {
    a
}
#[doc = "Function with container type annotations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn container_types(
    items: &Vec<i32>,
    mapping: std::collections::HashMap<String, f64>,
    unique: std::collections::HashSet<String>,
    coords: (i32, i32, i32),
) -> Vec<String> {
    items
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
    either_type: IntOrStringUnion,
    complex_union: UnionType1,
) -> Option<String> {
    if maybe_value.is_some() {
        return Some((maybe_value).to_string());
    }
    None
}
#[doc = "Function with nested type annotations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn nested_types(
    matrix: Vec<Vec<i32>>,
    lookup: std::collections::HashMap<String, Vec<f64>>,
    optional_dict: Option<std::collections::HashMap<String, i32>>,
    union_list: Vec<UnionType>,
) -> HashMap<String, Vec<Option<i32>>> {
    {
        let map = HashMap::new();
        map
    }
}
#[doc = "Generic function with type variable."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn generic_function<T: Clone>(items: &Vec<T>) -> Result<T, Box<dyn std::error::Error>> {
    Ok(if !items.is_empty() {
        items.get(0usize).cloned().unwrap_or_default()
    } else {
        None
    })
}
#[doc = "Function with complex nested generic types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn complex_generics(
    data: Vec<Option<std::collections::HashMap<String, UnionType>>>,
    processor: Container<Vec<T>>,
    mappings: std::collections::HashMap<String, Mapping<String, i32>>,
) -> TypeOrListUnion {
    Container::new(42)
}
#[doc = "Function using type aliases."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn custom_types<'a, 'b>(
    user_id: &'a UserId,
    username: Username,
    all_users: &'b UserData,
) -> Option<Username> {
    Some(all_users.get(&user_id).cloned())
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
pub fn higher_order(func: Box<dyn Fn(i32, i32) -> i32>, a: i32, b: i32) -> i32 {
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
            let mut set = HashSet::new();
            set.insert("x".to_string());
            set.insert("y".to_string());
            set
        },
        (1, 2, 3),
    );
    println!("{}", format!("Container types result: {:?}", result2));
    let result3 = optional_types(42, "either".to_string(), 3.14);
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
        let idx: i32 = (str, int);
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    })();
    mapping.put("answer".to_string(), 42);
    println!(
        "{}",
        format!("Generic mapping: {}", mapping.get("answer").cloned())
    );
    let users = {
        let mut map = HashMap::new();
        map.insert(1, "Alice");
        map.insert(2, "Bob");
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
