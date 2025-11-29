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
        while i < line.len() {
            let mut char = line[i as usize];
            if char == self.quote_char {
                if in_quotes && i + 1 < line.len() && line[i + 1 as usize] == self.quote_char {
                    let mut current_field = current_field + self.quote_char;
                    let mut i = i + 2;
                } else {
                    let mut in_quotes = !in_quotes;
                    let mut i = i + 1;
                };
            } else {
                if char == self.delimiter && !in_quotes {
                    fields.push(current_field);
                    let mut current_field = "".to_string();
                    let mut i = i + 1;
                } else {
                    let mut current_field = current_field + char;
                    let mut i = i + 1;
                };
            };
        }
        fields.push(current_field);
        return fields;
    }
    pub fn parse_string(&self, csv_content: String) -> Vec<Vec<String>> {
        let mut lines = csv_content
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut rows = vec![];
        for line in lines {
            let mut stripped = line.trim().to_string();
            if stripped {
                let mut fields = self.parse_line(stripped);
                rows.push(fields);
            };
        }
        return rows;
    }
    pub fn to_dict_list(&self, csv_content: String) -> Vec<HashMap<String, String>> {
        let mut rows = self.parse_string(csv_content);
        if !rows {
            return vec![];
        };
        let mut headers = rows[0 as usize];
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
                let mut map = HashMap::new();
                map
            };
            for (i, value) in enumerate(row) {
                if i < headers.len() {
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
pub fn calculate_column_stats<'b, 'a>(
    csv_content: &'a str,
    column_name: &'b str,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let parser = CSVParser::new();
    let dict_rows = parser.to_dict_list(csv_content);
    let _cse_temp_0 = !dict_rows
        .get(0usize)
        .cloned()
        .unwrap_or_default()
        .get(column_name)
        .is_some();
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
        let value = (row.get(column_name).cloned().unwrap_or_default()) as f64;
        values.push(value);
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
        map.insert("count".to_string(), (count) as f64);
        map.insert("sum".to_string(), total);
        map.insert("mean".to_string(), mean_val);
        map.insert("min".to_string(), min_val);
        map.insert("max".to_string(), max_val);
        map
    })
}
#[doc = "Filter CSV rows where column equals condition_value"]
pub fn filter_csv_rows<'b, 'a>(
    csv_content: String,
    column_name: &'a str,
    condition_value: &'b str,
) -> Result<String, Box<dyn std::error::Error>> {
    let parser = CSVParser::new();
    let rows = parser.parse_string(csv_content);
    if !rows {
        return Ok("".to_string());
    }
    let headers = rows.get(0usize).cloned().unwrap_or_default();
    let _cse_temp_0 = !headers.get(column_name).is_some();
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
            && (row.get(column_index as usize).cloned().unwrap_or_default() == condition_value)
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
pub fn group_by_column<'a, 'b>(
    csv_content: &'a str,
    group_column: &'b str,
) -> Result<HashMap<String, Vec<HashMap<String, String>>>, Box<dyn std::error::Error>> {
    let parser = CSVParser::new();
    let dict_rows = parser.to_dict_list(csv_content);
    let mut groups: HashMap<String, Vec<HashMap<String, String>>> = {
        let map = HashMap::new();
        map
    };
    for row in dict_rows.iter().cloned() {
        if row.get(group_column).is_some() {
            let group_key = row.get(group_column).cloned().unwrap_or_default();
            if !groups.get(&group_key).is_some() {
                groups.insert(group_key, vec![]);
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
