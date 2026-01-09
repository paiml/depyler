#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
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
#[doc = "Calculate arithmetic mean"]
#[doc = " Depyler: proven to terminate"]
pub fn mean(numbers: &Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    if numbers.is_empty() {
        return Ok(0.0);
    }
    Ok(((numbers.iter().sum::<f64>()) as f64) / ((numbers.len() as i32) as f64))
}
#[doc = "Calculate median value"]
#[doc = " Depyler: proven to terminate"]
pub fn median(numbers: &Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    if numbers.is_empty() {
        return Ok(0.0);
    }
    let sorted_nums = {
        let mut sorted_vec = numbers.iter().cloned().collect::<Vec<_>>();
        sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        sorted_vec
    };
    let _cse_temp_0 = sorted_nums.len() as i32;
    let n = _cse_temp_0;
    let _cse_temp_1 = n % 2;
    let _cse_temp_2 = _cse_temp_1 == 0;
    if _cse_temp_2 {
        return Ok((({
            let base = &sorted_nums;
            let idx: i32 = {
                let a = n;
                let b = 2;
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
            } - 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        } + {
            let base = &sorted_nums;
            let idx: i32 = {
                let a = n;
                let b = 2;
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
            };
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        }) as f64)
            / ((2.0) as f64));
    } else {
        return Ok({
            let base = &sorted_nums;
            let idx: i32 = {
                let a = n;
                let b = 2;
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
            };
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx)
                .cloned()
                .expect("IndexError: list index out of range")
        });
    }
}
#[doc = "Find the most frequently occurring number"]
pub fn mode(numbers: &Vec<i32>) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let mut mode_value: i32 = Default::default();
    if numbers.is_empty() {
        return Ok(None);
    }
    let mut frequency: std::collections::HashMap<i32, i32> = {
        let map: HashMap<i32, i32> = HashMap::new();
        map
    };
    for num in numbers.iter().cloned() {
        if frequency.get(&num).is_some() {
            {
                let _key = num;
                let _old_val = frequency.get(&_key).cloned().unwrap_or_default();
                frequency.insert(_key, _old_val + 1);
            }
        } else {
            frequency.insert(num.clone(), 1);
        }
    }
    let mut max_count = 0;
    mode_value = numbers
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for (num, count) in frequency
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        if count > max_count {
            max_count = count;
            mode_value = num;
        }
    }
    Ok(Some(mode_value))
}
#[doc = "Calculate sample variance"]
pub fn variance(numbers: &Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    let mut sum_squared_diff: f64 = Default::default();
    let _cse_temp_0 = numbers.len() as i32;
    let _cse_temp_1 = _cse_temp_0 < 2;
    if _cse_temp_1 {
        return Ok(0.0);
    }
    let avg = mean(&numbers)?;
    sum_squared_diff = 0.0;
    for num in numbers.iter().cloned() {
        let diff = num - avg;
        sum_squared_diff = sum_squared_diff + diff * diff;
    }
    Ok(((sum_squared_diff) as f64) / (((numbers.len() as i32).saturating_sub(1)) as f64))
}
#[doc = "Calculate sample standard deviation"]
#[doc = " Depyler: proven to terminate"]
pub fn standard_deviation(numbers: &Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    let mut x: f64 = Default::default();
    let var = variance(&numbers)?;
    let _cse_temp_0 = var == 0.0;
    if _cse_temp_0 {
        return Ok(0.0);
    }
    let _cse_temp_1 = ((var) as f64) / ((2.0) as f64);
    x = _cse_temp_1;
    for __sanitized in 0..(10) {
        x = ((x + ((var) as f64) / ((x) as f64)) as f64) / ((2.0) as f64);
    }
    Ok(x)
}
#[doc = "Calculate Pearson correlation coefficient"]
#[doc = " Depyler: proven to terminate"]
pub fn correlation<'a, 'b>(
    x: &'a Vec<f64>,
    y: &'b Vec<f64>,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut sum_x_squared: f64 = Default::default();
    let mut sum_y_squared: f64 = Default::default();
    let mut numerator: f64 = Default::default();
    let mut denominator: f64 = Default::default();
    let _cse_temp_0 = x.len() as i32;
    let _cse_temp_1 = y.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let _cse_temp_3 = _cse_temp_0 < 2;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    let n = _cse_temp_0;
    let mean_x = mean(&x)?;
    let mean_y = mean(&y)?;
    numerator = 0.0;
    sum_x_squared = 0.0;
    sum_y_squared = 0.0;
    for i in 0..(n) {
        let dx = x
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            - mean_x;
        let dy = y
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            - mean_y;
        numerator = numerator + dx * dy;
        sum_x_squared = sum_x_squared + dx * dx;
        sum_y_squared = sum_y_squared + dy * dy;
    }
    let _cse_temp_5 = sum_x_squared * sum_y_squared;
    let denominator_squared = _cse_temp_5;
    let _cse_temp_6 = denominator_squared == 0.0;
    if _cse_temp_6 {
        return Ok(0.0);
    }
    let _cse_temp_7 = ((denominator_squared) as f64) / ((2.0) as f64);
    denominator = _cse_temp_7;
    for __sanitized in 0..(10) {
        denominator = ((denominator + ((denominator_squared) as f64) / ((denominator) as f64))
            as f64)
            / ((2.0) as f64);
    }
    Ok(((numerator) as f64) / ((denominator) as f64))
}
