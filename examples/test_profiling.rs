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
#[doc = "Recursive Fibonacci - will be identified as hot path."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_recursive(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}
#[doc = "Iterative Fibonacci - more efficient."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci_iterative(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    let (mut a, mut b) = (0, 1);
    for __ in 2..n + 1 {
        (a, b) = (b, a + b);
    }
    b
}
#[doc = "Process a list with nested loops - O(n²) complexity."]
#[doc = " Depyler: proven to terminate"]
pub fn process_list(items: &Vec<i32>) -> Result<i32, IndexError> {
    let mut total = 0;
    for i in 0..items.len() as i32 {
        for j in i..items.len() as i32 {
            if items.get(i as usize).cloned().unwrap_or_default()
                < items.get(j as usize).cloned().unwrap_or_default()
            {
                total = total
                    + items.get(i as usize).cloned().unwrap_or_default()
                        * items.get(j as usize).cloned().unwrap_or_default();
            }
        }
    }
    Ok(total)
}
#[doc = "String concatenation in loop - inefficient pattern."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn string_concatenation_in_loop(n: i32) -> String {
    let mut result = "";
    for _i in 0..n {
        result = format!("{}{}", result, format!("Item {:?}, ", i));
    }
    result
}
#[doc = "Function with many allocations."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn allocate_many_lists(n: i32) -> Vec<Vec<i32>> {
    let mut results = vec![];
    for i in 0..n {
        let mut inner_list = vec![];
        for j in 0..10 {
            inner_list.push(i * j);
        }
        results.push(inner_list);
    }
    results
}
#[doc = "Function with many type checks that Rust can optimize away."]
#[doc = " Depyler: verified panic-free"]
pub fn type_check_heavy(values: &Vec<object>) -> i32 {
    let mut count = 0;
    for value in values.iter().cloned() {
        let mut count;
        if true {
            count = count + value;
        } else {
            let mut count;
            if true {
                count = count + value.len() as i32;
            } else {
                if true {
                    count = count + value.len() as i32;
                }
            }
        }
    }
    count
}
#[doc = "Matrix multiplication - triple nested loop."]
#[doc = " Depyler: proven to terminate"]
pub fn matrix_multiply<'b, 'a>(
    a: &'a mut Vec<Vec<f64>>,
    b: &'b mut Vec<Vec<f64>>,
) -> Result<Vec<Vec<f64>>, IndexError> {
    let _cse_temp_0 = a.len() as i32;
    let rows_a = _cse_temp_0;
    let cols_a = if !a.is_empty() {
        a.get(0usize).cloned().unwrap_or_default().len() as i32
    } else {
        0
    };
    let cols_b = if !b.is_empty() {
        b.get(0usize).cloned().unwrap_or_default().len() as i32
    } else {
        0
    };
    let mut result = (0..rows_a)
        .map(|_| (0..cols_b).map(|_| 0.0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
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
                            .get(k as usize)
                            .cloned()
                            .unwrap_or_default()
                            * b.get(k as usize)
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
#[doc = "Simple function for baseline comparison."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_function(x: i32, y: i32) -> i32 {
    x + y
}
#[doc = "Main entry point with various function calls."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..5 {
        fibonacci_recursive(i)?;
    }
    fibonacci_iterative(30)?;
    let test_list = (0..100).collect::<Vec<_>>();
    process_list(test_list)?;
    string_concatenation_in_loop(100)?;
    allocate_many_lists(50)?;
    let mixed_values = vec![
        1,
        "hello".to_string(),
        vec![1, 2, 3],
        42,
        "world".to_string(),
    ];
    type_check_heavy(&mixed_values)?;
    let mat_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let mat_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    matrix_multiply(&mat_a, &mat_b)?;
    simple_function(10, 20)?;
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_fibonacci_recursive_examples() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(-1), -1);
    }
    #[test]
    fn test_fibonacci_iterative_examples() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(-1), -1);
    }
    #[test]
    fn test_process_list_examples() {
        assert_eq!(process_list(&vec![]), 0);
        assert_eq!(process_list(&vec![1]), 1);
        assert_eq!(process_list(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_type_check_heavy_examples() {
        assert_eq!(type_check_heavy(&vec![]), 0);
        assert_eq!(type_check_heavy(&vec![1]), 1);
        assert_eq!(type_check_heavy(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn quickcheck_simple_function() {
        fn prop(x: i32, y: i32) -> TestResult {
            if (x > 0 && y > i32::MAX - x) || (x < 0 && y < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = simple_function(x.clone(), y.clone());
            let result2 = simple_function(y.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_simple_function_examples() {
        assert_eq!(simple_function(0, 0), 0);
        assert_eq!(simple_function(1, 2), 3);
        assert_eq!(simple_function(-1, 1), 0);
    }
}
