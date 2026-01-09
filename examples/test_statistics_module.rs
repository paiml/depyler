#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::f64 as math;
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
#[doc = "Test calculating arithmetic mean"]
pub fn test_mean() -> Result<f64, Box<dyn std::error::Error>> {
    let mut total: f64 = Default::default();
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    total = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = (_cse_temp_0) as f64;
    let _cse_temp_2 = ((total) as f64) / ((_cse_temp_1) as f64);
    let mean: f64 = _cse_temp_2;
    Ok(mean)
}
#[doc = "Test median with odd number of elements"]
#[doc = " Depyler: proven to terminate"]
pub fn test_median_odd() -> Result<f64, Box<dyn std::error::Error>> {
    let data: Vec<f64> = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    let mut sorted_data: Vec<f64> = data.clone();
    for i in 0..(sorted_data.len() as i32) {
        for j in (i + 1)..(sorted_data.len() as i32) {
            if sorted_data
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                let temp: f64 = sorted_data
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
    let _cse_temp_0 = sorted_data.len() as i32;
    let _cse_temp_1 = {
        let a = _cse_temp_0;
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
    let mid: i32 = _cse_temp_1;
    let median: f64 = sorted_data
        .get(mid as usize)
        .cloned()
        .expect("IndexError: list index out of range");
    Ok(median)
}
#[doc = "Test median with even number of elements"]
#[doc = " Depyler: proven to terminate"]
pub fn test_median_even() -> Result<f64, Box<dyn std::error::Error>> {
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
    let mut sorted_data: Vec<f64> = data.clone();
    for i in 0..(sorted_data.len() as i32) {
        for j in (i + 1)..(sorted_data.len() as i32) {
            if sorted_data
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                let temp: f64 = sorted_data
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
    let _cse_temp_0 = sorted_data.len() as i32;
    let _cse_temp_1 = {
        let a = _cse_temp_0;
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
    let mid: i32 = _cse_temp_1;
    let _cse_temp_2 = {
        let base = &sorted_data;
        let idx: i32 = mid - 1;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx)
            .cloned()
            .expect("IndexError: list index out of range")
    } + sorted_data
        .get(mid as usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let _cse_temp_3 = ((_cse_temp_2) as f64) / ((2.0) as f64);
    let median: f64 = _cse_temp_3;
    Ok(median)
}
#[doc = "Test finding mode(most common value)"]
#[doc = " Depyler: proven to terminate"]
pub fn test_mode() -> Result<i32, Box<dyn std::error::Error>> {
    let mut mode_value: i32 = Default::default();
    let data: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4];
    let mut max_count: i32 = 0;
    mode_value = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for i in 0..(data.len() as i32) {
        let mut count: i32 = 0;
        for j in 0..(data.len() as i32) {
            if data
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                == data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                count = count + 1;
            }
        }
        if count > max_count {
            max_count = count;
            mode_value = data
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
        }
    }
    Ok(mode_value)
}
#[doc = "Test calculating variance"]
pub fn test_variance() -> Result<f64, Box<dyn std::error::Error>> {
    let mut variance_sum: f64 = Default::default();
    let mut total: f64 = Default::default();
    let data: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    total = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = (_cse_temp_0) as f64;
    let _cse_temp_2 = ((total) as f64) / ((_cse_temp_1) as f64);
    let mean: f64 = _cse_temp_2;
    variance_sum = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_3 = ((variance_sum) as f64) / ((_cse_temp_1) as f64);
    let variance: f64 = _cse_temp_3;
    Ok(variance)
}
#[doc = "Test calculating standard deviation"]
pub fn test_stdev() -> Result<f64, Box<dyn std::error::Error>> {
    let mut total: f64 = Default::default();
    let mut variance_sum: f64 = Default::default();
    let data: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    total = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = (_cse_temp_0) as f64;
    let _cse_temp_2 = ((total) as f64) / ((_cse_temp_1) as f64);
    let mean: f64 = _cse_temp_2;
    variance_sum = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_3 = ((variance_sum) as f64) / ((_cse_temp_1) as f64);
    let variance: f64 = _cse_temp_3;
    let stdev: f64 = (variance as f64).sqrt();
    Ok(stdev)
}
#[doc = "Test finding min and max"]
pub fn test_min_max() -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let mut max_val: f64 = Default::default();
    let mut min_val: f64 = Default::default();
    let data: Vec<f64> = vec![3.5, 1.2, 7.8, 2.4, 9.1];
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok((0.0, 0.0));
    }
    min_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    max_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for value in data.iter().cloned() {
        if value < min_val {
            min_val = value;
        }
        if value > max_val {
            max_val = value;
        }
    }
    Ok((min_val, max_val))
}
#[doc = "Test calculating range(max - min)"]
pub fn test_range() -> Result<f64, Box<dyn std::error::Error>> {
    let mut min_val: f64 = Default::default();
    let mut max_val: f64 = Default::default();
    let data: Vec<f64> = vec![1.0, 5.0, 3.0, 9.0, 2.0];
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0.0);
    }
    min_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    max_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for value in data.iter().cloned() {
        if value < min_val {
            min_val = value;
        }
        if value > max_val {
            max_val = value;
        }
    }
    let data_range: f64 = max_val - min_val;
    Ok(data_range)
}
#[doc = "Test sum calculation"]
#[doc = " Depyler: verified panic-free"]
pub fn test_sum() -> f64 {
    let mut total: f64 = Default::default();
    let data: Vec<f64> = vec![1.5, 2.5, 3.5, 4.5];
    total = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    total
}
#[doc = "Calculate percentile(simplified)"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_percentile(
    data: &Vec<f64>,
    percentile: i32,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut index: i32 = Default::default();
    let mut sorted_data: Vec<f64> = data.clone();
    for i in 0..(sorted_data.len() as i32) {
        for j in (i + 1)..(sorted_data.len() as i32) {
            if sorted_data
                .get(j as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                < sorted_data
                    .get(i as usize)
                    .cloned()
                    .expect("IndexError: list index out of range")
            {
                let temp: f64 = sorted_data
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
    let _cse_temp_0 = sorted_data.len() as i32;
    let _cse_temp_1 = percentile * _cse_temp_0;
    let _cse_temp_2 = {
        let a = _cse_temp_1;
        let b = 100;
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
    index = _cse_temp_2;
    let _cse_temp_3 = index >= _cse_temp_0;
    if _cse_temp_3 {
        index = _cse_temp_0 - 1;
    }
    Ok(sorted_data
        .get(index as usize)
        .cloned()
        .expect("IndexError: list index out of range"))
}
#[doc = "Calculate Q1, Q2(median), Q3"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_quartiles(data: &Vec<f64>) -> Result<(f64, f64, f64), Box<dyn std::error::Error>> {
    let q1: f64 = calculate_percentile(&data, 25)?;
    let q2: f64 = calculate_percentile(&data, 50)?;
    let q3: f64 = calculate_percentile(&data, 75)?;
    Ok((q1, q2, q3))
}
#[doc = "Calculate interquartile range(IQR)"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_iqr(data: &Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    let quartiles: (f64, f64, f64) = calculate_quartiles(&data)?;
    let q1: f64 = quartiles.0;
    let q3: f64 = quartiles.2;
    let iqr: f64 = q3 - q1;
    Ok(iqr)
}
#[doc = "Detect outliers using IQR method"]
pub fn detect_outliers(data: &Vec<f64>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let quartiles: (f64, f64, f64) = calculate_quartiles(&data)?;
    let q1: f64 = quartiles.0;
    let q3: f64 = quartiles.2;
    let iqr: f64 = q3 - q1;
    let _cse_temp_0 = 1.5 * iqr;
    let lower_bound: f64 = q1 - _cse_temp_0;
    let upper_bound: f64 = q3 + _cse_temp_0;
    let mut outliers: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        if (value < lower_bound) || (value > upper_bound) {
            outliers.push(value);
        }
    }
    Ok(outliers)
}
#[doc = "Normalize data to 0-1 range"]
pub fn normalize_data(data: Vec<f64>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut min_val: f64 = Default::default();
    let mut max_val: f64 = Default::default();
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    min_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    max_val = data
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for value in data.iter().cloned() {
        if value < min_val {
            min_val = value;
        }
        if value > max_val {
            max_val = value;
        }
    }
    let data_range: f64 = max_val - min_val;
    let _cse_temp_2 = data_range == 0.0;
    if _cse_temp_2 {
        return Ok(data);
    }
    let mut normalized: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        let norm_value: f64 = ((value - min_val) as f64) / ((data_range) as f64);
        normalized.push(norm_value);
    }
    Ok(normalized)
}
#[doc = "Standardize data(z-score)"]
pub fn standardize_data(data: Vec<f64>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut variance_sum: f64 = Default::default();
    let mut total: f64 = Default::default();
    total = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = (_cse_temp_0) as f64;
    let _cse_temp_2 = ((total) as f64) / ((_cse_temp_1) as f64);
    let mean: f64 = _cse_temp_2;
    variance_sum = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_3 = ((variance_sum) as f64) / ((_cse_temp_1) as f64);
    let variance: f64 = _cse_temp_3;
    let stdev: f64 = (variance as f64).sqrt();
    let _cse_temp_4 = stdev == 0.0;
    if _cse_temp_4 {
        return Ok(data);
    }
    let mut standardized: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        let z_score: f64 = ((value - mean) as f64) / ((stdev) as f64);
        standardized.push(z_score);
    }
    Ok(standardized)
}
#[doc = "Calculate covariance between two datasets"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_covariance<'b, 'a>(
    x: &'a Vec<f64>,
    y: &'b Vec<f64>,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut y_total: f64 = Default::default();
    let mut x_total: f64 = Default::default();
    let mut cov_sum: f64 = Default::default();
    let _cse_temp_0 = x.len() as i32;
    let _cse_temp_1 = y.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let _cse_temp_3 = _cse_temp_0 == 0;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    x_total = 0.0;
    y_total = 0.0;
    for i in 0..(x.len() as i32) {
        x_total = x_total
            + x.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
        y_total = y_total
            + y.get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range");
    }
    let _cse_temp_5 = (_cse_temp_0) as f64;
    let _cse_temp_6 = ((x_total) as f64) / ((_cse_temp_5) as f64);
    let x_mean: f64 = _cse_temp_6;
    let _cse_temp_7 = (_cse_temp_1) as f64;
    let _cse_temp_8 = ((y_total) as f64) / ((_cse_temp_7) as f64);
    let y_mean: f64 = _cse_temp_8;
    cov_sum = 0.0;
    for i in 0..(x.len() as i32) {
        let x_diff: f64 = x
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            - x_mean;
        let y_diff: f64 = y
            .get(i as usize)
            .cloned()
            .expect("IndexError: list index out of range")
            - y_mean;
        cov_sum = cov_sum + x_diff * y_diff;
    }
    let _cse_temp_9 = ((cov_sum) as f64) / ((_cse_temp_5) as f64);
    let covariance: f64 = _cse_temp_9;
    Ok(covariance)
}
#[doc = "Calculate Pearson correlation coefficient"]
pub fn calculate_correlation<'b, 'a>(
    x: &'a Vec<f64>,
    y: &'b Vec<f64>,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut x_var_sum: f64 = Default::default();
    let mut x_total: f64 = Default::default();
    let mut diff: f64 = Default::default();
    let mut y_total: f64 = Default::default();
    let mut y_var_sum: f64 = Default::default();
    let _cse_temp_0 = x.len() as i32;
    let _cse_temp_1 = y.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let _cse_temp_3 = _cse_temp_0 == 0;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    let cov: f64 = calculate_covariance(&x, &y)?;
    x_total = 0.0;
    for val in x.iter().cloned() {
        x_total = x_total + val;
    }
    let _cse_temp_5 = (_cse_temp_0) as f64;
    let _cse_temp_6 = ((x_total) as f64) / ((_cse_temp_5) as f64);
    let x_mean: f64 = _cse_temp_6;
    x_var_sum = 0.0;
    for val in x.iter().cloned() {
        diff = val - x_mean;
        x_var_sum = x_var_sum + diff * diff;
    }
    let x_stdev: f64 = (((x_var_sum) as f64) / (((x.len() as i32) as f64) as f64) as f64).sqrt();
    y_total = 0.0;
    for val in y.iter().cloned() {
        y_total = y_total + val;
    }
    let _cse_temp_7 = (_cse_temp_1) as f64;
    let _cse_temp_8 = ((y_total) as f64) / ((_cse_temp_7) as f64);
    let y_mean: f64 = _cse_temp_8;
    y_var_sum = 0.0;
    for val in y.iter().cloned() {
        diff = val - y_mean;
        y_var_sum = y_var_sum + diff * diff;
    }
    let y_stdev: f64 = (((y_var_sum) as f64) / (((y.len() as i32) as f64) as f64) as f64).sqrt();
    let _cse_temp_9 = x_stdev == 0.0;
    let _cse_temp_10 = y_stdev == 0.0;
    let _cse_temp_11 = (_cse_temp_9) || (_cse_temp_10);
    if _cse_temp_11 {
        return Ok(0.0);
    }
    let _cse_temp_12 = x_stdev.mul(&y_stdev).unwrap();
    let _cse_temp_13 = ((cov) as f64) / ((_cse_temp_12) as f64);
    let correlation: f64 = _cse_temp_13;
    Ok(correlation)
}
#[doc = "Run all statistics module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_statistics_features() -> Result<(), Box<dyn std::error::Error>> {
    let mean: f64 = test_mean()?;
    let median_odd: f64 = test_median_odd()?;
    let median_even: f64 = test_median_even()?;
    let mode: i32 = test_mode()?;
    let variance: f64 = test_variance()?;
    let stdev: f64 = test_stdev()?;
    let minmax: (f64, f64) = test_min_max()?;
    let data_range: f64 = test_range()?;
    let total: f64 = test_sum();
    let sample: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let p50: f64 = calculate_percentile(&sample, 50)?;
    let quartiles: (f64, f64, f64) = calculate_quartiles(&sample)?;
    let iqr: f64 = calculate_iqr(&sample)?;
    let outlier_data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0];
    let outliers: Vec<f64> = detect_outliers(&outlier_data)?;
    let normalized: Vec<f64> = normalize_data(sample)?;
    let standardized: Vec<f64> = standardize_data(sample)?;
    let x_data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_data: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let cov: f64 = calculate_covariance(&x_data, &y_data)?;
    let corr: f64 = calculate_correlation(&x_data, &y_data)?;
    println!("{}", "All statistics module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_mode_examples() {
        let _ = test_mode();
    }
    #[test]
    fn test_detect_outliers_examples() {
        assert_eq!(detect_outliers(vec![]), vec![]);
        assert_eq!(detect_outliers(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_normalize_data() {
        fn prop(data: Vec<f64>) -> TestResult {
            let once = normalize_data(&data);
            let twice = normalize_data(once.clone());
            if once != twice {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<f64>) -> TestResult);
    }
    #[test]
    fn test_normalize_data_examples() {
        assert_eq!(normalize_data(vec![]), vec![]);
        assert_eq!(normalize_data(vec![1]), vec![1]);
    }
    #[test]
    fn test_standardize_data_examples() {
        assert_eq!(standardize_data(vec![]), vec![]);
        assert_eq!(standardize_data(vec![1]), vec![1]);
    }
}
