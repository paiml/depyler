#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'operator'(tracked in DEPYLER-0424)"]
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
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
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
#[doc = "Test arithmetic operator functions"]
#[doc = " Depyler: proven to terminate"]
pub fn test_arithmetic_operators() -> Result<i32, Box<dyn std::error::Error>> {
    let a: i32 = 10;
    let b: i32 = 5;
    let add_result: i32 = a + b;
    let sub_result: i32 = a - b;
    let _cse_temp_0 = a * b;
    let mul_result: i32 = _cse_temp_0;
    Ok(add_result + sub_result + mul_result)
}
#[doc = "Test comparison operator functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_comparison_operators() -> bool {
    let a: i32 = 10;
    let b: i32 = 5;
    let _cse_temp_0 = a != b;
    let ne: bool = _cse_temp_0;
    let _cse_temp_1 = a > b;
    let gt: bool = _cse_temp_1;
    (gt) && (ne)
}
#[doc = "Test logical operator functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_logical_operators() -> bool {
    let a: bool = true;
    let b: bool = false;
    let _cse_temp_0 = (a) && (b);
    let and_result: bool = _cse_temp_0;
    let _cse_temp_1 = (a) || (b);
    let or_result: bool = _cse_temp_1;
    (or_result) && (!and_result)
}
#[doc = "Test bitwise operator functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bitwise_operators() -> i32 {
    let a: i32 = 12;
    let b: i32 = 10;
    let _cse_temp_0 = a & b;
    let and_result: i32 = _cse_temp_0;
    let _cse_temp_1 = a | b;
    let or_result: i32 = _cse_temp_1;
    and_result + or_result
}
#[doc = "Test itemgetter on list"]
#[doc = " Depyler: proven to terminate"]
pub fn test_itemgetter_list() -> Result<i32, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![10, 20, 30, 40, 50];
    let item: i32 = data
        .get(2usize)
        .cloned()
        .expect("IndexError: list index out of range");
    Ok(item)
}
#[doc = "Test itemgetter on tuple"]
#[doc = " Depyler: proven to terminate"]
pub fn test_itemgetter_tuple() -> Result<String, Box<dyn std::error::Error>> {
    let data: (String, i32, f64) = ("hello".to_string(), 42, 3.14);
    let item: String = data.0;
    Ok(item.to_string())
}
#[doc = "Test itemgetter with multiple indices"]
#[doc = " Depyler: proven to terminate"]
pub fn test_itemgetter_multiple() -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![10, 20, 30, 40, 50];
    let item1: i32 = data
        .get(1usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let item3: i32 = data
        .get(3usize)
        .cloned()
        .expect("IndexError: list index out of range");
    Ok((item1, item3))
}
#[doc = "Sort list of tuples by second element"]
#[doc = " Depyler: proven to terminate"]
pub fn sort_by_second_element(data: &Vec<()>) -> Result<Vec<()>, Box<dyn std::error::Error>> {
    let mut sorted_data: Vec<()> = data.clone();
    for i in 0..(sorted_data.len() as i32) {
        for j in (i + 1)..(sorted_data.len() as i32) {
            if sorted_data
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .1
                < sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
                    .1
            {
                let temp: () = sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                sorted_data.insert(
                    (i) as usize,
                    sorted_data
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range"),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    Ok(sorted_data)
}
#[doc = "Test absolute value operator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_abs_operator() -> i32 {
    let negative: i32 = -42;
    let _cse_temp_0 = (negative).abs();
    let positive: i32 = _cse_temp_0;
    positive
}
#[doc = "Test negation operator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_neg_operator() -> i32 {
    let positive: i32 = 42;
    let negative: i32 = -positive;
    negative
}
#[doc = "Test index/contains operator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_index_operator() -> bool {
    let mut found: bool = Default::default();
    let data: Vec<i32> = vec![10, 20, 30, 40, 50];
    let value: i32 = 30;
    let _cse_temp_0 = data.contains(&value);
    let contains: bool = _cse_temp_0;
    if contains {
        let index: i32 = data
            .iter()
            .position(|x| x == &value)
            .map(|i| i as i32)
            .expect("ValueError: value is not in list");
        let _cse_temp_1 = index >= 0;
        found = _cse_temp_1;
    } else {
        found = false;
    }
    found
}
#[doc = "Test concatenation operator"]
#[doc = " Depyler: verified panic-free"]
pub fn test_concat_operator() -> Vec<i32> {
    let list1: Vec<i32> = vec![1, 2, 3];
    let list2: Vec<i32> = vec![4, 5, 6];
    let mut result: Vec<i32> = vec![];
    for item in list1.iter().cloned() {
        result.push(item);
    }
    for item in list2.iter().cloned() {
        result.push(item);
    }
    result
}
#[doc = "Test repeat operator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_repeat_operator() -> Vec<i32> {
    let base: Vec<i32> = vec![1, 2, 3];
    let times: i32 = 3;
    let mut result: Vec<i32> = vec![];
    for _i in 0..(times) {
        for item in base.iter().cloned() {
            result.push(item);
        }
    }
    result
}
#[doc = "Test getitem operator"]
#[doc = " Depyler: proven to terminate"]
pub fn test_getitem_operator() -> Result<i32, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![10, 20, 30, 40];
    let index: i32 = 2;
    let item: i32 = data
        .get(index as usize)
        .cloned()
        .expect("IndexError: list index out of range");
    Ok(item)
}
#[doc = "Test setitem operator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_setitem_operator() -> Vec<i32> {
    let mut data: Vec<i32> = vec![10, 20, 30, 40];
    let index: i32 = 2;
    let value: i32 = 99;
    data.insert((index) as usize, value);
    data
}
#[doc = "Test delitem operator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_delitem_operator() -> Vec<i32> {
    let data: Vec<i32> = vec![10, 20, 30, 40];
    let mut new_data: Vec<i32> = vec![];
    for i in 0..(data.len() as i32) {
        if i != 2 {
            new_data.push(
                data.get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range"),
            );
        }
    }
    new_data
}
#[doc = "Apply operation based on string"]
#[doc = " Depyler: proven to terminate"]
pub fn apply_operation(a: i32, b: i32, op: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = op == "add";
    if _cse_temp_0 {
        return Ok(a + b);
    } else {
        let _cse_temp_1 = op == "sub";
        if _cse_temp_1 {
            return Ok(a - b);
        } else {
            let _cse_temp_2 = op == "mul";
            if _cse_temp_2 {
                return Ok(a * b);
            } else {
                let _cse_temp_3 = op == "div";
                if _cse_temp_3 {
                    return Ok({
                        let a = a;
                        let b = b;
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
                    });
                } else {
                    return Ok(0);
                }
            }
        }
    }
}
#[doc = "Find max element using key function"]
pub fn max_by_key(data: &Vec<()>) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut max_elem: () = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok((0, 0));
    }
    max_elem = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for elem in data.iter().cloned() {
        if elem.1 > max_elem.1 {
            max_elem = elem;
        }
    }
    Ok(max_elem)
}
#[doc = "Find min element using key function"]
pub fn min_by_key(data: &Vec<()>) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut min_elem: () = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok((0, 0));
    }
    min_elem = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for elem in data.iter().cloned() {
        if elem.1 < min_elem.1 {
            min_elem = elem;
        }
    }
    Ok(min_elem)
}
#[doc = "Test truth value testing"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_truthiness() -> bool {
    let empty_list: Vec<i32> = vec![];
    let _cse_temp_0 = empty_list.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let empty_is_false: bool = _cse_temp_1;
    let full_list: Vec<i32> = vec![1, 2, 3];
    let _cse_temp_2 = full_list.len() as i32;
    let _cse_temp_3 = _cse_temp_2 > 0;
    let full_is_true: bool = _cse_temp_3;
    (empty_is_false) && (full_is_true)
}
#[doc = "Test identity operators"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_identity() -> bool {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    let _cse_temp_0 = a == b;
    let equal: bool = _cse_temp_0;
    let _cse_temp_1 = a != c;
    let different: bool = _cse_temp_1;
    (equal) && (different)
}
#[doc = "Test chained comparisons"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn chain_comparisons(x: i32, low: i32, high: i32) -> bool {
    let _cse_temp_0 = low <= x;
    let _cse_temp_1 = x <= high;
    let _cse_temp_2 = (_cse_temp_0) && (_cse_temp_1);
    let in_range: bool = _cse_temp_2;
    in_range
}
#[doc = "Run all operator module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_operator_features() -> Result<(), Box<dyn std::error::Error>> {
    let arith_result: i32 = test_arithmetic_operators()?;
    let comp_result: bool = test_comparison_operators();
    let logic_result: bool = test_logical_operators();
    let bit_result: i32 = test_bitwise_operators();
    let list_item: i32 = test_itemgetter_list()?;
    let tuple_item: String = test_itemgetter_tuple()?;
    let multi_items: (i32, i32) = test_itemgetter_multiple()?;
    let tuples: Vec<()> = vec![(1, 3), (2, 1), (3, 2)];
    let sorted_tuples: Vec<()> = sort_by_second_element(&tuples)?;
    let abs_val: i32 = test_abs_operator();
    let neg_val: i32 = test_neg_operator();
    let contains: bool = test_index_operator();
    let concatenated: Vec<i32> = test_concat_operator();
    let repeated: Vec<i32> = test_repeat_operator();
    let get_item: i32 = test_getitem_operator()?;
    let set_result: Vec<i32> = test_setitem_operator();
    let del_result: Vec<i32> = test_delitem_operator();
    let op_result: i32 = apply_operation(10, 5, "add")?;
    let data: Vec<()> = vec![(1, 100), (2, 50), (3, 200)];
    let max_elem: (i32, i32) = max_by_key(&data)?;
    let min_elem: (i32, i32) = min_by_key(&data)?;
    let truth: bool = test_truthiness();
    let identity: bool = test_identity();
    let chained: bool = chain_comparisons(5, 1, 10);
    println!("{}", "All operator module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_arithmetic_operators_examples() {
        let _ = test_arithmetic_operators();
    }
    #[test]
    fn test_test_bitwise_operators_examples() {
        let _ = test_bitwise_operators();
    }
    #[test]
    fn test_test_itemgetter_list_examples() {
        let _ = test_itemgetter_list();
    }
    #[test]
    fn quickcheck_sort_by_second_element() {
        fn prop(data: Vec<()>) -> TestResult {
            let input_len = data.len();
            let result = sort_by_second_element(&data);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = sort_by_second_element(&data);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = data.clone();
            input_sorted.sort();
            let mut result = sort_by_second_element(&data);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<()>) -> TestResult);
    }
    #[test]
    fn test_sort_by_second_element_examples() {
        assert_eq!(sort_by_second_element(vec![]), vec![]);
        assert_eq!(sort_by_second_element(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_test_abs_operator() {
        fn prop() -> TestResult {
            let result = test_abs_operator();
            if result < 0 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn() -> TestResult);
    }
    #[test]
    fn test_test_abs_operator_examples() {
        let _ = test_abs_operator();
    }
    #[test]
    fn test_test_neg_operator_examples() {
        let _ = test_neg_operator();
    }
    #[test]
    fn test_test_getitem_operator_examples() {
        let _ = test_getitem_operator();
    }
}
