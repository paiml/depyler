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
#[derive(Debug, Clone)]
pub struct CSVParser {
    pub delimiter: String,
    pub quote_char: String,
}
impl CSVParser {
    pub fn new(delimiter: String, quote_char: String) -> Self {
        Self {
            delimiter,
            quote_char,
        }
    }
    pub fn parse_line(&self, line: String) -> Vec<String> {
        let mut fields = vec![];
        let mut current_field = "".to_string();
        let mut in_quotes = false;
        let mut i = 0;
        while i < (line.len() as i32) {
            let char = line[i as usize];
            if char == self.quote_char.clone() {
                if in_quotes
                    && i + 1 < (line.len() as i32)
                    && line[i + 1 as usize] == self.quote_char.clone()
                {
                    let current_field = current_field + self.quote_char.clone();
                    let i = i + 2;
                } else {
                    let in_quotes = !in_quotes;
                    let i = i + 1;
                };
            } else {
                if char == self.delimiter.clone() && !in_quotes {
                    fields.push(current_field);
                    let current_field = "".to_string();
                    let i = i + 1;
                } else {
                    let current_field = current_field + char;
                    let i = i + 1;
                };
            };
        }
        fields.push(current_field);
        return fields;
    }
    pub fn parse_string(&self, csv_content: String) -> Vec<Vec<String>> {
        let lines = csv_content
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut rows = vec![];
        for line in lines {
            let stripped = line.trim().to_string();
            if stripped {
                let fields = self.parse_line(stripped);
                rows.push(fields);
            };
        }
        return rows;
    }
    pub fn to_dict_list(
        &self,
        csv_content: String,
    ) -> Vec<std::collections::HashMap<String, String>> {
        let rows = self.parse_string(csv_content);
        if !rows {
            return vec![];
        };
        let headers = rows[0 as usize];
        let mut result = vec![];
        for row in {
            let s = &rows;
            let len = s.chars().count() as isize;
            let start_idx = (1) as isize;
            let start = if start_idx < 0 {
                (len + start_idx).max(0) as usize
            } else {
                start_idx as usize
            };
            s.chars().skip(start).collect::<String>()
        } {
            let mut row_dict = {
                let mut map = std::collections::HashMap::new();
                map
            };
            for (i, value) in row.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) {
                if i < (headers.len() as i32) {
                    row_dict.insert(headers[i as usize], value);
                } else {
                    row_dict.insert(format!("column_{}", i), value);
                };
            }
            result.push(row_dict);
        }
        return result;
    }
}
#[doc = "Calculate basic statistics for a numeric column in CSV"]
pub fn calculate_column_stats<'a, 'b>(
    csv_content: &'a str,
    column_name: &'b str,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let parser = CSVParser::new();
    let dict_rows = parser.to_dict_list(csv_content);
    let _cse_temp_0 = !dict_rows
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range")
        .contains(&*column_name);
    let _cse_temp_1 = (!dict_rows) || (_cse_temp_0);
    if _cse_temp_1 {
        return Ok({
            let mut map = HashMap::new();
            map.insert("count".to_string(), 0.0);
            map.insert("sum".to_string(), 0.0);
            map.insert("mean".to_string(), 0.0);
            map.insert("min".to_string(), 0.0);
            map.insert("max".to_string(), 0.0);
            map
        });
    }
    let mut values: Vec<f64> = vec![];
    for row in dict_rows.iter().cloned() {
        let mut value: f64 = Default::default();
        match (|| -> Result<(), Box<dyn std::error::Error>> {
            value = row
                .get(column_name)
                .cloned()
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap();
            values.push(value);
            Ok(())
        })() {
            Ok(()) => {}
            Err(_) => {
                continue;
            }
        }
    }
    if values.is_empty() {
        return Ok({
            let mut map = HashMap::new();
            map.insert("count".to_string(), 0.0);
            map.insert("sum".to_string(), 0.0);
            map.insert("mean".to_string(), 0.0);
            map.insert("min".to_string(), 0.0);
            map.insert("max".to_string(), 0.0);
            map
        });
    }
    let _cse_temp_2 = values.iter().sum::<i32>();
    let total = _cse_temp_2;
    let _cse_temp_3 = values.len() as i32;
    let count = _cse_temp_3;
    let _cse_temp_4 = total / count;
    let mean_val = _cse_temp_4;
    let _cse_temp_5 = *values.iter().min().unwrap();
    let min_val = _cse_temp_5;
    let _cse_temp_6 = *values.iter().max().unwrap();
    let max_val = _cse_temp_6;
    Ok({
        let mut map = HashMap::new();
        map.insert(
            "count".to_string(),
            DepylerValue::Str(format!("{:?}", (count) as f64)),
        );
        map.insert("sum".to_string(), DepylerValue::Int(total as i64));
        map.insert("mean".to_string(), DepylerValue::Int(mean_val as i64));
        map.insert("min".to_string(), DepylerValue::Int(min_val as i64));
        map.insert("max".to_string(), DepylerValue::Int(max_val as i64));
        map
    })
}
#[doc = "Filter CSV rows where column equals condition_value"]
pub fn filter_csv_rows<'a, 'b>(
    csv_content: String,
    column_name: &'a str,
    condition_value: &'b str,
) -> Result<String, Box<dyn std::error::Error>> {
    let parser = CSVParser::new();
    let rows = parser.parse_string(csv_content);
    if !rows {
        return Ok("".to_string());
    }
    let headers = rows
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    let _cse_temp_0 = !headers.contains(&*column_name);
    if _cse_temp_0 {
        return Ok(csv_content.to_string());
    }
    let column_index = headers
        .iter()
        .position(|x| x == &column_name)
        .map(|i| i as i32)
        .expect("ValueError: value is not in list");
    let mut filtered_rows = vec![headers];
    for row in {
        let base = &rows;
        let start_idx = 1 as isize;
        let start = if start_idx < 0 {
            (base.len() as isize + start_idx).max(0) as usize
        } else {
            start_idx as usize
        };
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    } {
        if (column_index < row.len() as i32)
            && ([row.0, row.1][column_index as usize] == condition_value)
        {
            filtered_rows.push(row);
        }
    }
    let mut result_lines: Vec<String> = vec![];
    for row in filtered_rows.iter().cloned() {
        let line = row.join(",");
        result_lines.push(line);
    }
    Ok(result_lines.join("\n"))
}
#[doc = "Group CSV rows by values in specified column"]
pub fn group_by_column<'b, 'a>(
    csv_content: &'a str,
    group_column: &'b str,
) -> Result<HashMap<String, Vec<HashMap<String, String>>>, Box<dyn std::error::Error>> {
    let parser = CSVParser::new();
    let dict_rows = parser.to_dict_list(csv_content);
    let mut groups: std::collections::HashMap<
        String,
        Vec<std::collections::HashMap<String, String>>,
    > = {
        let map: HashMap<String, Vec<std::collections::HashMap<String, String>>> = HashMap::new();
        map
    };
    for row in dict_rows.iter().cloned() {
        if row.contains(&*group_column) {
            let group_key = row.get(group_column).cloned().unwrap_or_default();
            if groups.get(&group_key).is_none() {
                groups.insert(group_key.to_string().clone(), vec![]);
            }
            groups
                .get(&group_key)
                .cloned()
                .unwrap_or_default()
                .push(row);
        }
    }
    Ok(groups)
}
