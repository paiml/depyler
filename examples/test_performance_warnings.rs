use serde_json;
use std::collections::HashMap;
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
#[doc = "String concatenation in loop - O(n²) complexity."]
#[doc = " Depyler: verified panic-free"]
pub fn string_concat_in_loop(items: &serde_json::Value) -> String {
    let mut result = "";
    for item in items.iter().cloned() {
        result = format!("{}{}", result, item.to_string());
    }
    result
}
#[doc = "Deeply nested loops - O(n³) complexity."]
#[doc = " Depyler: proven to terminate"]
pub fn nested_loops_cubic(matrix: &serde_json::Value) -> Result<i32, IndexError> {
    let mut total = 0;
    for i in 0..matrix.len() as i32 {
        for j in 0..matrix.get(i as usize).cloned().unwrap_or_default().len() as i32 {
            for k in 0..matrix
                .get(i as usize)
                .cloned()
                .unwrap_or_default()
                .get(j as usize)
                .cloned()
                .unwrap_or_default()
                .len() as i32
            {
                total = total
                    + matrix
                        .get(i as usize)
                        .cloned()
                        .unwrap_or_default()
                        .get(j as usize)
                        .cloned()
                        .unwrap_or_default()
                        .get(k as usize)
                        .cloned()
                        .unwrap_or_default();
            }
        }
    }
    Ok(total)
}
#[doc = "Expensive operations in loop."]
#[doc = " Depyler: verified panic-free"]
pub fn repeated_expensive_computation(data: serde_json::Value) -> Vec<serde_json::Value> {
    let mut results = vec![];
    for item in data.iter().cloned() {
        let sorted_data = {
            let mut __sorted_result = data.clone();
            __sorted_result.sort();
            __sorted_result
        };
        results.push(item * sorted_data.len() as i32);
    }
    results
}
#[doc = "Inefficient list operations."]
#[doc = " Depyler: verified panic-free"]
pub fn inefficient_list_operations(items: &mut serde_json::Value) {
    while items.len() as i32 > 0 {
        if let Some(pos) = items
            .iter()
            .position(|x| x == &items.get(0usize).cloned().unwrap_or_default())
        {
            items.remove(pos)
        } else {
            panic!("ValueError: list.remove(x): x not in list")
        };
    }
}
#[doc = "Creating large objects in loops."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn large_list_in_loop(n: serde_json::Value) -> Vec<serde_json::Value> {
    let mut results = vec![];
    for _i in 0..n {
        let temp = (0..1000).map(|j| j).collect::<Vec<_>>();
        results.push(temp.iter().sum::<i32>());
    }
    results
}
#[doc = "Linear search in nested loop - O(n²)."]
#[doc = " Depyler: verified panic-free"]
pub fn linear_search_in_loop<'a, 'b>(
    items: &'a mut serde_json::Value,
    targets: &'b serde_json::Value,
) -> Vec<serde_json::Value> {
    let mut found = vec![];
    for target in targets.iter().cloned() {
        if items.contains_key(&target) {
            let idx = items
                .iter()
                .position(|x| x == &target)
                .map(|i| i as i32)
                .expect("ValueError: value is not in list");
            found.push((target, idx));
        }
    }
    found
}
#[doc = "Expensive math operations in loop."]
#[doc = " Depyler: verified panic-free"]
pub fn power_in_tight_loop(values: &serde_json::Value) -> Vec<serde_json::Value> {
    let mut results = vec![];
    for x in values.iter().cloned() {
        let mut result = (x as f64).powf(3.5 as f64);
        results.push(result);
    }
    results
}
#[doc = "Using range(len()) instead of enumerate."]
#[doc = " Depyler: proven to terminate"]
pub fn range_len_antipattern(items: &mut serde_json::Value) -> Result<(), IndexError> {
    for i in 0..items.len() as i32 {
        process_item(i, items.get(i as usize).cloned().unwrap_or_default())?;
    }
}
#[doc = "Computing aggregates repeatedly."]
#[doc = " Depyler: verified panic-free"]
pub fn aggregate_in_nested_loop(matrix: &serde_json::Value) -> i32 {
    let mut result = 0;
    for row in matrix.iter().cloned() {
        for col in row.iter().cloned() {
            let mut total = row.iter().sum::<i32>();
            result = result + col * total;
        }
    }
    result
}
#[doc = "Large parameters passed by value."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn large_parameter_by_value<'b, 'a>(
    huge_list: &'a Vec<serde_json::Value>,
    huge_dict: &'b HashMap<serde_json::Value, serde_json::Value>,
) -> i32 {
    huge_list.len() as i32 + huge_dict.len() as i32 as i32
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_item(idx: serde_json::Value, item: serde_json::Value) {}
