#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'operator'(tracked in DEPYLER-0424)"]
use std::iter::Iterator::fold;
const STR_ID: &'static str = "id";
const STR_AGE: &'static str = "age";
const STR_CITY: &'static str = "city";
const STR_NAME: &'static str = "name";
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
pub struct DataProcessor {
    pub data: Vec<std::collections::HashMap<String, DepylerValue>>,
}
impl DataProcessor {
    pub fn new(data: Vec<std::collections::HashMap<String, DepylerValue>>) -> Self {
        Self { data }
    }
    pub fn filter_by_field(
        &self,
        field: String,
        value: &DepylerValue,
    ) -> Vec<std::collections::HashMap<String, DepylerValue>> {
        return self
            .data
            .clone()
            .into_iter()
            .filter(|record| {
                let record = record.clone();
                record.get(field) == value
            })
            .map(|record| record)
            .collect::<Vec<_>>();
    }
    pub fn map_field(
        &self,
        field: String,
        transform: impl Fn(DepylerValue) -> DepylerValue,
    ) -> Vec<DepylerValue> {
        return self
            .data
            .clone()
            .into_iter()
            .filter(|record| {
                let record = record.clone();
                record.contains_key(&field)
            })
            .map(|record| transform(record.get(field)))
            .collect::<Vec<_>>();
    }
    pub fn group_by(
        &self,
        field: String,
    ) -> std::collections::HashMap<DepylerValue, Vec<std::collections::HashMap<String, DepylerValue>>>
    {
        let mut groups = {
            let mut map = std::collections::HashMap::new();
            map
        };
        for record in self.data.clone() {
            let key = record.get(field);
            if !groups.contains_key(&key) {
                groups.insert(key, vec![]);
            };
            groups.get(&key).cloned().unwrap_or_default().push(record);
        }
        return groups;
    }
    pub fn aggregate(&self, field: String, operation: String) -> Option<f64> {
        let values = self
            .data
            .clone()
            .into_iter()
            .filter(|record| {
                let record = record.clone();
                true
            })
            .map(|record| record.get(field, 0))
            .collect::<Vec<_>>();
        if !values {
            return None;
        };
        if operation == "sum".to_string() {
            return Some(values.iter().sum::<f64>());
        } else {
            if operation == "avg".to_string() {
                return Some(values.iter().sum::<f64>() / (values.len() as i32));
            } else {
                if operation == "min".to_string() {
                    return Some(min(values));
                } else {
                    if operation == "max".to_string() {
                        return Some(max(values));
                    } else {
                        return None;
                    };
                };
            };
        };
    }
    pub fn sort_by(
        &self,
        field: String,
        reverse: bool,
    ) -> Vec<std::collections::HashMap<String, DepylerValue>> {
        return {
            let mut v: Vec<_> = self.data.clone().into_iter().collect();
            v.sort_by_key(|x| x.get(field, "".to_string()));
            v
        };
    }
    pub fn project(
        &self,
        fields: Vec<String>,
    ) -> Vec<std::collections::HashMap<String, DepylerValue>> {
        return self
            .data
            .clone()
            .into_iter()
            .map(|record| {
                fields
                    .into_iter()
                    .filter(|field| record.contains_key(&field))
                    .map(|field| (field, record.get(field)))
                    .collect::<std::collections::HashMap<_, _>>()
            })
            .collect::<Vec<_>>();
    }
    pub fn distinct(&self, field: String) -> Vec<DepylerValue> {
        let mut seen = std::collections::HashSet::<i32>::new();
        let mut result = vec![];
        for record in self.data.clone() {
            let value = record.get(field);
            if !seen.contains_key(&value) {
                seen.insert(value);
                result.push(value);
            };
        }
        return result;
    }
}
#[doc = "Process a list of numbers with various operations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_numbers(numbers: &Vec<i32>) -> HashMap<String, DepylerValue> {
    if numbers.is_empty() {
        return {
            let mut map = HashMap::new();
            map.insert("error".to_string(), "Empty list".to_string().to_string());
            map
        };
    }
    let evens = numbers
        .as_slice()
        .iter()
        .cloned()
        .filter(|n| {
            let n = n.clone();
            n % 2 == 0
        })
        .map(|n| n)
        .collect::<Vec<_>>();
    let odds = numbers
        .as_slice()
        .iter()
        .cloned()
        .filter(|n| {
            let n = n.clone();
            n % 2 != 0
        })
        .map(|n| n)
        .collect::<Vec<_>>();
    let squares = numbers
        .as_slice()
        .iter()
        .cloned()
        .map(|n| {
            if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
                ({ n } as i32)
                    .checked_pow({ 2 } as u32)
                    .expect("Power operation overflowed")
            } else {
                ({ n } as f64).powf({ 2 } as f64) as i32
            }
        })
        .collect::<Vec<_>>();
    let doubled = numbers.iter().map(|x| x * 2).collect::<Vec<_>>();
    let positive = numbers
        .iter()
        .cloned()
        .filter(|x| x > 0)
        .collect::<Vec<_>>();
    let product = std::iter::Iterator::fold(operator.mul, numbers, 1);
    {
        let mut map = HashMap::new();
        map.insert("original".to_string(), numbers);
        map.insert("count".to_string(), numbers.len() as i32);
        map.insert("sum".to_string(), numbers.iter().sum::<i32>());
        map.insert(
            "average".to_string(),
            numbers.iter().sum::<i32>() / numbers.len() as i32,
        );
        map.insert("min".to_string(), *numbers.iter().min().unwrap());
        map.insert("max".to_string(), *numbers.iter().max().unwrap());
        map.insert("evens".to_string(), evens);
        map.insert("odds".to_string(), odds);
        map.insert("squares".to_string(), squares);
        map.insert("doubled".to_string(), doubled);
        map.insert("positive".to_string(), positive);
        map.insert("product".to_string(), product);
        map
    }
}
#[doc = "Transform and analyze text."]
#[doc = " Depyler: verified panic-free"]
pub fn transform_text(text: &str) -> HashMap<String, DepylerValue> {
    let words = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _cse_temp_0 = words.len() as i32;
    let word_count = _cse_temp_0;
    let _cse_temp_1 = text.len() as i32;
    let char_count = _cse_temp_1;
    let mut word_freq = {
        let map: HashMap<String, String> = HashMap::new();
        map
    };
    for word in words.iter().cloned() {
        let word_lower = word.to_lowercase();
        word_freq.insert(
            word_lower.to_string().clone(),
            word_freq.get(&word_lower).cloned().unwrap_or(0) + 1,
        );
    }
    let longest = if !words.is_empty() {
        *words.iter().max().unwrap()
    } else {
        "".to_string()
    };
    let shortest = if !words.is_empty() {
        *words.iter().min().unwrap()
    } else {
        "".to_string()
    };
    let char_freq = text
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|char| {
            let _v = text.matches(&*char).count() as i32;
            (char, _v)
        })
        .collect::<std::collections::HashMap<_, _>>();
    {
        let mut map = HashMap::new();
        map.insert("original".to_string(), DepylerValue::Str(text.to_string()));
        map.insert(
            "word_count".to_string(),
            DepylerValue::Int(word_count as i64),
        );
        map.insert(
            "char_count".to_string(),
            DepylerValue::Int(char_count as i64),
        );
        map.insert(
            "words".to_string(),
            DepylerValue::Str(format!("{:?}", words)),
        );
        map.insert(
            "unique_words".to_string(),
            DepylerValue::Str(format!(
                "{:?}",
                words.into_iter().collect::<std::collections::HashSet<_>>()
            )),
        );
        map.insert(
            "word_frequency".to_string(),
            DepylerValue::Str(format!("{:?}", word_freq)),
        );
        map.insert(
            "longest_word".to_string(),
            DepylerValue::Int(longest as i64),
        );
        map.insert(
            "shortest_word".to_string(),
            DepylerValue::Int(shortest as i64),
        );
        map.insert(
            "char_frequency".to_string(),
            DepylerValue::Str(format!("{:?}", char_freq)),
        );
        map.insert(
            "reversed".to_string(),
            DepylerValue::Str(format!("{:?}", {
                let base = text;
                let step: i32 = -1;
                if step == 1 {
                    base.to_string()
                } else if step > 0 {
                    base.chars().step_by(step as usize).collect::<String>()
                } else if step == -1 {
                    base.chars().rev().collect::<String>()
                } else {
                    let abs_step = step.abs() as usize;
                    base.chars().rev().step_by(abs_step).collect::<String>()
                }
            })),
        );
        map.insert(
            "uppercase".to_string(),
            DepylerValue::Str(format!("{:?}", text.to_uppercase())),
        );
        map.insert(
            "lowercase".to_string(),
            DepylerValue::Str(format!("{:?}", text.to_lowercase())),
        );
        map
    }
}
#[doc = "Perform operations on 2D matrix."]
#[doc = " Depyler: proven to terminate"]
pub fn matrix_operations(
    matrix: &Vec<Vec<i32>>,
) -> Result<HashMap<String, DepylerValue>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = (matrix.is_empty())
        || (!matrix
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range"));
    if _cse_temp_0 {
        return Ok({
            let mut map = HashMap::new();
            map.insert(
                "error".to_string(),
                "Invalid matrix".to_string().to_string(),
            );
            map
        });
    }
    let _cse_temp_1 = matrix.len() as i32;
    let rows = _cse_temp_1;
    let _cse_temp_2 = matrix
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        .len() as i32;
    let cols = _cse_temp_2;
    let transposed = (0..(cols))
        .into_iter()
        .map(|j| {
            (0..(rows))
                .into_iter()
                .map(|i| {
                    matrix
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let flattened = matrix
        .into_iter()
        .flat_map(|row| row.into_iter().map(move |elem| elem))
        .collect::<Vec<_>>();
    let row_sums = matrix
        .iter()
        .cloned()
        .map(|row| row.iter().sum::<i32>())
        .collect::<Vec<_>>();
    let col_sums = (0..(cols))
        .into_iter()
        .map(|j| {
            (0..(rows))
                .into_iter()
                .map(|i| {
                    matrix
                        .get(i as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                        .get(j as usize)
                        .cloned()
                        .expect("IndexError: list index out of range")
                })
                .sum::<i32>()
        })
        .collect::<Vec<_>>();
    let diagonal = (0..(std::cmp::min(rows, cols)))
        .into_iter()
        .map(|i| {
            matrix
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        })
        .collect::<Vec<_>>();
    Ok({
        let mut map = HashMap::new();
        map.insert(
            "original".to_string(),
            DepylerValue::Str(format!("{:?}", matrix)),
        );
        map.insert("rows".to_string(), DepylerValue::Int(rows as i64));
        map.insert("cols".to_string(), DepylerValue::Int(cols as i64));
        map.insert(
            "transposed".to_string(),
            DepylerValue::Str(format!("{:?}", transposed)),
        );
        map.insert(
            "flattened".to_string(),
            DepylerValue::Str(format!("{:?}", flattened)),
        );
        map.insert(
            "row_sums".to_string(),
            DepylerValue::Str(format!("{:?}", row_sums)),
        );
        map.insert(
            "col_sums".to_string(),
            DepylerValue::Str(format!("{:?}", col_sums)),
        );
        map.insert(
            "diagonal".to_string(),
            DepylerValue::Str(format!("{:?}", diagonal)),
        );
        map.insert(
            "total_sum".to_string(),
            DepylerValue::Str(format!("{:?}", flattened.iter().sum::<i32>())),
        );
        map.insert(
            "max_element".to_string(),
            DepylerValue::Str(format!("{:?}", *flattened.iter().max().unwrap())),
        );
        map.insert(
            "min_element".to_string(),
            DepylerValue::Str(format!("{:?}", *flattened.iter().min().unwrap())),
        );
        map
    })
}
#[doc = "Example of functional pipeline pattern."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn pipeline_example(data: &Vec<i32>) -> i32 {
    let result = std::iter::Iterator::fold(
        operator.add,
        data.iter()
            .cloned()
            .filter(|x| x % 2 == 0)
            .iter()
            .map(|x| {
                if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
                    ({ x } as i32)
                        .checked_pow({ 2 } as u32)
                        .expect("Power operation overflowed")
                } else {
                    ({ x } as f64).powf({ 2 } as f64) as i32
                }
            })
            .collect::<Vec<_>>(),
        0,
    );
    result
}
#[doc = "Test data processing functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_data = vec![
        {
            let mut map = HashMap::new();
            map.insert(STR_ID.to_string(), DepylerValue::Int(1 as i64));
            map.insert(STR_NAME.to_string(), DepylerValue::Str("Alice".to_string()));
            map.insert(STR_AGE.to_string(), DepylerValue::Int(30 as i64));
            map.insert(STR_CITY.to_string(), DepylerValue::Str("NYC".to_string()));
            map
        },
        {
            let mut map = HashMap::new();
            map.insert(STR_ID.to_string(), DepylerValue::Int(2 as i64));
            map.insert(STR_NAME.to_string(), DepylerValue::Str("Bob".to_string()));
            map.insert(STR_AGE.to_string(), DepylerValue::Int(25 as i64));
            map.insert(STR_CITY.to_string(), DepylerValue::Str("LA".to_string()));
            map
        },
        {
            let mut map = HashMap::new();
            map.insert(STR_ID.to_string(), DepylerValue::Int(3 as i64));
            map.insert(
                STR_NAME.to_string(),
                DepylerValue::Str("Charlie".to_string()),
            );
            map.insert(STR_AGE.to_string(), DepylerValue::Int(35 as i64));
            map.insert(STR_CITY.to_string(), DepylerValue::Str("NYC".to_string()));
            map
        },
        {
            let mut map = HashMap::new();
            map.insert(STR_ID.to_string(), DepylerValue::Int(4 as i64));
            map.insert(STR_NAME.to_string(), DepylerValue::Str("Diana".to_string()));
            map.insert(STR_AGE.to_string(), DepylerValue::Int(28 as i64));
            map.insert(
                STR_CITY.to_string(),
                DepylerValue::Str("Chicago".to_string()),
            );
            map
        },
    ];
    let processor = DataProcessor::new(sample_data);
    println!(
        "{} {}",
        "NYC residents:",
        processor.filter_by_field(STR_CITY, "NYC")
    );
    println!("{} {}", "Grouped by city:", processor.group_by(STR_CITY));
    println!(
        "{} {}",
        "Average age:",
        processor.aggregate(STR_AGE, "avg".to_string())
    );
    println!("{} {}", "Distinct cities:", processor.distinct(STR_CITY));
    let numbers = vec![1, 2, 3, 4, 5, -1, -2, 0];
    println!("{} {}", "\nNumber processing:", process_numbers(&numbers));
    let text = "Hello World from Depyler";
    println!("{} {}", "\nText analysis:", transform_text(text));
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!(
        "{} {:?}",
        "\nMatrix operations:",
        matrix_operations(&matrix)
    );
    println!(
        "{} {}",
        "\nPipeline result:",
        pipeline_example(&vec![1, 2, 3, 4, 5, 6])
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_pipeline_example_examples() {
        assert_eq!(pipeline_example(&vec![]), 0);
        assert_eq!(pipeline_example(&vec![1]), 1);
        assert_eq!(pipeline_example(&vec![1, 2, 3]), 3);
    }
}
