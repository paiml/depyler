use std::f64 as math;
const STR_A: &'static str = "A";
const STR_C: &'static str = "C";
const STR_B: &'static str = "B";
const STR_D: &'static str = "D";
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
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
pub struct DataBuffer {
    pub data: serde_json::Value,
    pub position: i32,
}
impl DataBuffer {
    pub fn new(_size: i32) -> Self {
        Self {
            data: Default::default(),
            position: 0,
        }
    }
    pub fn write(&mut self, values: Vec<i32>) {
        for value in values {
            if self.position < self.data.len() as i32 {
                self.data.insert(self.position, value);
                self.position = self.position + 1;
            };
        }
    }
    pub fn read(&self, count: i32) -> Vec<i32> {
        let start = (0).max(self.position - count);
        return {
            let s = &self.data;
            let len = s.chars().count() as isize;
            let start_idx = (start) as isize;
            let stop_idx = (self.position) as isize;
            let start = if start_idx < 0 {
                (len + start_idx).max(0) as usize
            } else {
                start_idx as usize
            };
            let stop = if stop_idx < 0 {
                (len + stop_idx).max(0) as usize
            } else {
                stop_idx as usize
            };
            if stop > start {
                s.chars().skip(start).take(stop - start).collect::<String>()
            } else {
                String::new()
            }
        };
    }
}
#[doc = "\n    Matrix multiplication with nested loops.\n    \n    Interactive mode will suggest:\n    - Aggressive optimization for nested loops\n    - Potential SIMD vectorization\n    - Loop unrolling opportunities\n    "]
#[doc = " Depyler: proven to terminate"]
pub fn matrix_multiply<'b, 'a>(
    a: &'a Vec<Vec<f64>>,
    b: &'b Vec<Vec<f64>>,
) -> Result<Vec<Vec<f64>>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = a.len() as i32;
    let n = _cse_temp_0;
    let _cse_temp_1 = b.get(0usize).cloned().unwrap_or_default().len() as i32;
    let m = _cse_temp_1;
    let _cse_temp_2 = b.len() as i32;
    let k = _cse_temp_2;
    let result = (0..n)
        .into_iter()
        .map(|_| (0..m).into_iter().map(|_| 0.0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..n {
        for j in 0..m {
            for p in 0..k {
                result.get_mut(&i).unwrap().insert(
                    (j) as usize,
                    result
                        .get(i as usize)
                        .cloned()
                        .unwrap_or_default()
                        .get(j as usize)
                        .cloned()
                        .unwrap_or_default()
                        + a.get(i as usize)
                            .cloned()
                            .unwrap_or_default()
                            .get(p as usize)
                            .cloned()
                            .unwrap_or_default()
                            * b.get(p as usize)
                                .cloned()
                                .unwrap_or_default()
                                .get(j as usize)
                                .cloned()
                                .unwrap_or_default(),
                );
            }
        }
    }
    Ok(result)
}
#[doc = "\n    Process text data to count keyword occurrences.\n    \n    Interactive mode will suggest:\n    - String ownership strategy(borrowed vs owned)\n    - Potential zero-copy optimizations\n    "]
pub fn process_text_data<'b, 'a>(
    texts: &'a Vec<String>,
    keywords: &'b Vec<String>,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut keyword_counts = keywords
        .iter()
        .cloned()
        .map(|kw| (kw, 0))
        .collect::<std::collections::HashMap<_, _>>();
    for text in texts.iter().cloned() {
        let normalized = text.to_lowercase().trim().to_string();
        for keyword in keywords.iter().cloned() {
            if normalized.contains(&*keyword) {
                {
                    let _key = keyword;
                    let _old_val = keyword_counts.get(&_key).cloned().unwrap_or_default();
                    keyword_counts.insert(_key, _old_val + 1);
                }
            }
        }
    }
    Ok(keyword_counts)
}
#[doc = "\n    Recursive quicksort implementation.\n    \n    Interactive mode will suggest:\n    - Stack depth limits\n    - Tail recursion optimization\n    - Memory allocation strategy\n    "]
#[doc = " Depyler: proven to terminate"]
pub fn quicksort(arr: Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 <= 1;
    if _cse_temp_1 {
        return Ok(arr);
    }
    let pivot = {
        let base = &arr;
        let idx: i32 = {
            let a = arr.len() as i32;
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
        base.get(actual_idx).cloned().unwrap_or_default()
    };
    let left = arr
        .as_slice()
        .iter()
        .copied()
        .filter(|&x| x < pivot)
        .map(|x| x)
        .collect::<Vec<_>>();
    let middle = arr
        .as_slice()
        .iter()
        .copied()
        .filter(|&x| x == pivot)
        .map(|x| x)
        .collect::<Vec<_>>();
    let right = arr
        .as_slice()
        .iter()
        .copied()
        .filter(|&x| x > pivot)
        .map(|x| x)
        .collect::<Vec<_>>();
    Ok(quicksort(left)?
        .iter()
        .chain(middle.iter())
        .cloned()
        .collect::<Vec<_>>()
        + quicksort(right)?)
}
#[doc = "\n    Safe division with error handling.\n    \n    Interactive mode will suggest:\n    - Error handling strategy\n    - Result type usage\n    - Panic-free guarantees\n    "]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide<'a, 'b>(
    numbers: &'a Vec<f64>,
    divisors: &'b Vec<f64>,
) -> Result<Vec<Option<f64>>, Box<dyn std::error::Error>> {
    let mut results = vec![];
    for i in 0..std::cmp::min(numbers.len() as i32, divisors.len() as i32) {
        if divisors.get(i as usize).cloned().unwrap_or_default() != 0 {
            results.push(
                (numbers.get(i as usize).cloned().unwrap_or_default() as f64)
                    / (divisors.get(i as usize).cloned().unwrap_or_default() as f64),
            );
        } else {
            results.push(None);
        }
    }
    Ok(results)
}
#[doc = "\n    Function that could benefit from parallelization.\n    \n    Interactive mode will suggest:\n    - Thread safety requirements\n    - Parallelization strategy\n    - Send/Sync trait bounds\n    "]
pub fn parallel_map(
    func: serde_json::Value,
    data: &Vec<i32>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut results = vec![];
    for item in data.iter().cloned() {
        let mut result = func(item);
        for __sanitized in 0..1000 {
            result = (result * 7 + 13) % 1000000;
        }
        results.push(result);
    }
    Ok(results)
}
#[doc = "\n    Route optimization using dynamic programming.\n    \n    Interactive mode will suggest multiple annotations:\n    - Algorithm complexity hints\n    - Memory vs speed tradeoffs\n    - Caching strategy\n    - Error handling approach\n    "]
pub fn optimize_route<'b, 'c, 'a>(
    distances: &'a std::collections::HashMap<String, std::collections::HashMap<String, f64>>,
    start: &'b str,
    end: &'c str,
) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
    let mut visited = HashSet::<i32>::new();
    let mut distances_from_start = {
        let mut map = HashMap::new();
        map.insert(start, 0);
        map
    };
    let mut previous = {
        let map = HashMap::new();
        map
    };
    while (visited.len() as i32) < distances.len() as i32 {
        let mut current_distance = "inf".parse::<f64>().unwrap();
        for node in distances.iter().cloned() {
            if (!visited.contains(&node)) && (distances_from_start.get(&node).is_some()) {
                if distances_from_start.get(&node).cloned().unwrap_or_default() < current_distance {
                    let mut current = node.clone();
                    current_distance = distances_from_start.get(&node).cloned().unwrap_or_default();
                }
            }
        }
        if current.is_none() {
            break;
        }
        visited.insert(current);
        for neighbor in distances.get(&current).cloned().unwrap_or_default() {
            if !visited.contains(&neighbor) {
                let new_distance = distances_from_start
                    .get(&current)
                    .cloned()
                    .unwrap_or_default()
                    + distances
                        .get(&current)
                        .cloned()
                        .unwrap_or_default()
                        .get(neighbor as usize)
                        .cloned()
                        .unwrap_or_default();
                if (!distances_from_start.get(&neighbor).is_some())
                    || (new_distance
                        < distances_from_start
                            .get(&neighbor)
                            .cloned()
                            .unwrap_or_default())
                {
                    distances_from_start.insert(neighbor.clone(), new_distance);
                    previous.insert(neighbor.clone(), serde_json::json!(current));
                }
            }
        }
    }
    let _cse_temp_0 = !previous.get(end).is_some();
    if _cse_temp_0 {
        return Ok(None);
    }
    let mut path = vec![];
    let mut current = end.to_string();
    while current != start {
        path.push(current);
        current = previous.get(&current).cloned().unwrap_or_default();
    }
    path.push(start);
    path.reverse();
    Ok(Some(path))
}
#[doc = "\n    Demonstration of functions that benefit from interactive annotation.\n    \n    When run with --interactive --annotate, Depyler will:\n    1. Analyze each function's characteristics\n    2. Suggest appropriate annotations\n    3. Guide you through the annotation process\n    4. Show before/after transpilation results\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Interactive Annotation Examples");
    println!("{}", "=".repeat(40 as usize));
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];
    let result = matrix_multiply(&a, &b)?;
    println!("{}", format!("Matrix multiplication result: {:?}", result));
    let texts = vec![
        "Hello world".to_string(),
        "Hello Python".to_string(),
        "Rust is fast".to_string(),
    ];
    let keywords = vec![
        "hello".to_string(),
        "rust".to_string(),
        "python".to_string(),
    ];
    let counts = process_text_data(&texts, &keywords)?;
    println!("{}", format!("Keyword counts: {:?}", counts));
    let numbers = vec![64, 34, 25, 12, 22, 11, 90];
    let sorted_nums = quicksort(numbers)?;
    println!("{}", format!("Sorted: {:?}", sorted_nums));
    let nums = vec![10.0, 20.0, 30.0];
    let divs = vec![2.0, 0.0, 5.0];
    let results = safe_divide(&nums, &divs)?;
    println!("{}", format!("Division results: {:?}", results));
    let data = (0..10).collect::<Vec<_>>();
    let mapped = parallel_map(|x| x * x, &data)?;
    println!("{}", format!("Mapped data: {:?}", mapped));
    let mut buffer = DataBuffer::new(100);
    buffer.write_all(vec![1, 2, 3, 4, 5].as_bytes()).unwrap();
    let recent = {
        let mut _read_buf = vec![0u8; 3];
        let _n = buffer.read(&mut _read_buf).unwrap_or(0);
        _read_buf.truncate(_n);
        _read_buf
    };
    println!("{}", format!("Recent buffer data: {}", recent));
    let graph = {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "A".to_string(),
            serde_json::json!(serde_json::json!({ "B": 1, "C": 4 })),
        );
        map.insert(
            "B".to_string(),
            serde_json::json!(serde_json::json!({ "A": 1, "C": 2, "D": 5 })),
        );
        map.insert(
            "C".to_string(),
            serde_json::json!(serde_json::json!({ "A": 4, "B": 2, "D": 1 })),
        );
        map.insert(
            "D".to_string(),
            serde_json::json!(serde_json::json!({ "B": 5, "C": 1 })),
        );
        map
    };
    let route = optimize_route(&graph, STR_A, STR_D)?;
    println!("{}", format!("Optimal route: {:?}", route));
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_quicksort() {
        fn prop(arr: Vec<i32>) -> TestResult {
            let input_len = arr.len();
            let result = quicksort(&arr);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = quicksort(&arr);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = arr.clone();
            input_sorted.sort();
            let mut result = quicksort(&arr);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_quicksort_examples() {
        assert_eq!(quicksort(vec![]), vec![]);
        assert_eq!(quicksort(vec![1]), vec![1]);
    }
}
