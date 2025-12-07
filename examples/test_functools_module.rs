use std;
use std::iter::Iterator::fold;
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
#[doc = "Test reduce for summing numbers"]
#[doc = " Depyler: verified panic-free"]
pub fn test_reduce_sum(numbers: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for num in numbers.iter().cloned() {
        result = result + num;
    }
    result
}
#[doc = "Test reduce for calculating product"]
#[doc = " Depyler: verified panic-free"]
pub fn test_reduce_product(numbers: &Vec<i32>) -> i32 {
    let _cse_temp_0 = numbers.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return 0;
    }
    let mut result: i32 = 1;
    for num in numbers.iter().cloned() {
        result = result * num;
    }
    result
}
#[doc = "Test reduce to find maximum"]
pub fn test_reduce_max(numbers: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = numbers.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    let mut result: i32 = numbers.get(0usize).cloned().unwrap_or_default();
    for num in numbers.iter().cloned() {
        if num > result {
            result = num;
        }
    }
    Ok(result)
}
#[doc = "Test reduce to find minimum"]
pub fn test_reduce_min(numbers: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = numbers.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    let mut result: i32 = numbers.get(0usize).cloned().unwrap_or_default();
    for num in numbers.iter().cloned() {
        if num < result {
            result = num;
        }
    }
    Ok(result)
}
#[doc = "Test reduce for string concatenation"]
#[doc = " Depyler: verified panic-free"]
pub fn test_reduce_concatenate(strings: &Vec<String>) -> String {
    let mut result: String = "".to_string();
    for s in strings.iter().cloned() {
        result = format!("{}{}", result, s);
    }
    result.to_string()
}
#[doc = "Helper function for partial application"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn multiply_by_two(x: i32) -> i32 {
    x * 2
}
#[doc = "Function for partial application"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn multiply_by(multiplier: i32, x: i32) -> i32 {
    multiplier * x
}
#[doc = "Test partial function application(manual)"]
#[doc = " Depyler: verified panic-free"]
pub fn test_partial_application() -> Vec<i32> {
    let multiplier: i32 = 3;
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut results: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        let result: i32 = multiply_by(multiplier, num);
        results.push(result);
    }
    results
}
#[doc = "Function for testing partial with multiple arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add_three_numbers(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}
#[doc = "Test partial application with multiple arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_partial_multiple_args() -> i32 {
    let fixed_a: i32 = 10;
    let fixed_b: i32 = 20;
    let variable_c: i32 = 5;
    let result: i32 = add_three_numbers(fixed_a, fixed_b, variable_c);
    result
}
#[doc = "Test function composition"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_compose_functions(x: i32) -> i32 {
    let step1: i32 = x + 1;
    let _cse_temp_0 = step1 * 2;
    let step2: i32 = _cse_temp_0;
    let _cse_temp_1 = step2 * step2;
    let step3: i32 = _cse_temp_1;
    step3
}
#[doc = "Test map-reduce pattern"]
#[doc = " Depyler: verified panic-free"]
pub fn test_map_reduce_pattern(numbers: &Vec<i32>) -> i32 {
    let mut squared: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        squared.push(num * num);
    }
    let mut total: i32 = 0;
    for sq in squared.iter().cloned() {
        total = total + sq;
    }
    total
}
#[doc = "Test filter-reduce pattern"]
pub fn test_filter_reduce_pattern(numbers: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut evens: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        if num % 2 == 0 {
            evens.push(num);
        }
    }
    let _cse_temp_0 = evens.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    let mut product: i32 = 1;
    for even in evens.iter().cloned() {
        product = product * even;
    }
    Ok(product)
}
#[doc = "Factorial with manual memoization pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn memoize_factorial(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    let mut result: i32 = 1;
    for i in 2..n + 1 {
        result = result * i;
    }
    result
}
#[doc = "Test currying pattern(manual)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_currying(a: i32, b: i32, c: i32) -> i32 {
    a + b * c
}
#[doc = "Test accumulate pattern with custom function"]
#[doc = " Depyler: verified panic-free"]
pub fn accumulate_with_function(numbers: &Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![];
    let mut acc: i32 = 0;
    for num in numbers.iter().cloned() {
        acc = acc + num;
        results.push(acc);
    }
    results
}
#[doc = "Test reduce with initial value"]
#[doc = " Depyler: verified panic-free"]
pub fn test_reduce_with_initial(numbers: &Vec<i32>, initial: i32) -> i32 {
    let mut result: i32 = initial.clone();
    for num in numbers.iter().cloned() {
        result = result + num;
    }
    result
}
#[doc = "Test reduce for 'all' logic"]
#[doc = " Depyler: verified panic-free"]
pub fn test_reduce_boolean_all(values: &Vec<bool>) -> bool {
    let mut result: bool = true;
    for val in values.iter().cloned() {
        result = (result) && (val);
        if !result {
            break;
        }
    }
    result
}
#[doc = "Test reduce for 'any' logic"]
#[doc = " Depyler: verified panic-free"]
pub fn test_reduce_boolean_any(values: &Vec<bool>) -> bool {
    let mut result: bool = false;
    for val in values.iter().cloned() {
        result = (result) || (val);
        if result {
            break;
        }
    }
    result
}
#[doc = "Test reduce to flatten nested lists"]
#[doc = " Depyler: verified panic-free"]
pub fn test_reduce_flatten(nested: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for sublist in nested.iter().cloned() {
        for item in sublist.iter().cloned() {
            result.push(item);
        }
    }
    result
}
#[doc = "Test reduce to group by parity"]
pub fn test_reduce_group_by(items: &Vec<i32>) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let mut evens: Vec<i32> = vec![];
    let mut odds: Vec<i32> = vec![];
    for item in items.iter().cloned() {
        if item % 2 == 0 {
            evens.push(item);
        } else {
            odds.push(item);
        }
    }
    let result: Vec<Vec<i32>> = vec![evens, odds];
    Ok(result)
}
#[doc = "Test function pipeline pattern"]
#[doc = " Depyler: verified panic-free"]
pub fn pipeline(value: i32, operations: &Vec<String>) -> i32 {
    let mut result: i32 = value.clone();
    for op in operations.iter().cloned() {
        if op == "double" {
            result = result * 2;
        } else {
            if op == "increment" {
                result = result + 1;
            } else {
                if op == "square" {
                    result = result * result;
                }
            }
        }
    }
    result
}
#[doc = "Test memoization with Fibonacci"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_memoization_fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    let mut prev: i32 = 0;
    let mut curr: i32 = 1;
    for _i in 2..n + 1 {
        let next_val: i32 = prev + curr;
        prev = curr;
        curr = next_val;
    }
    curr
}
#[doc = "Run all functools module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_functools_features() -> Result<(), Box<dyn std::error::Error>> {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let _sum_result: i32 = test_reduce_sum(&numbers);
    let _product_result: i32 = test_reduce_product(&numbers);
    let _max_result: i32 = test_reduce_max(&numbers)?;
    let _min_result: i32 = test_reduce_min(&numbers)?;
    let strings: Vec<String> = vec![
        "Hello".to_string(),
        " ".to_string(),
        "World".to_string(),
        "!".to_string(),
    ];
    let _concat_result: String = test_reduce_concatenate(&strings);
    let _partial_result: Vec<i32> = test_partial_application();
    let _partial_multi: i32 = test_partial_multiple_args();
    let _composed: i32 = test_compose_functions(5);
    let _map_reduce: i32 = test_map_reduce_pattern(&vec![1, 2, 3, 4, 5]);
    let _filter_reduce: i32 = test_filter_reduce_pattern(&vec![1, 2, 3, 4, 5, 6])?;
    let _fact: i32 = memoize_factorial(5);
    let _fib: i32 = test_memoization_fibonacci(10);
    let _curried: i32 = test_currying(1, 2, 3);
    let _accumulated: Vec<i32> = accumulate_with_function(&vec![1, 2, 3, 4, 5]);
    let _with_initial: i32 = test_reduce_with_initial(&vec![1, 2, 3], 10);
    let _all_true: bool = test_reduce_boolean_all(&vec![true, true, true]);
    let _all_false: bool = test_reduce_boolean_all(&vec![true, false, true]);
    let _any_true: bool = test_reduce_boolean_any(&vec![false, true, false]);
    let _any_false: bool = test_reduce_boolean_any(&vec![false, false, false]);
    let nested: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let _flattened: Vec<i32> = test_reduce_flatten(&nested);
    let items: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let _grouped: Vec<Vec<i32>> = test_reduce_group_by(&items)?;
    let ops: Vec<String> = vec![
        "double".to_string(),
        "increment".to_string(),
        "square".to_string(),
    ];
    let _piped: i32 = pipeline(3, &ops);
    println!("{}", "All functools module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_reduce_sum_examples() {
        assert_eq!(test_reduce_sum(&vec![]), 0);
        assert_eq!(test_reduce_sum(&vec![1]), 1);
        assert_eq!(test_reduce_sum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_test_reduce_product_examples() {
        assert_eq!(test_reduce_product(&vec![]), 0);
        assert_eq!(test_reduce_product(&vec![1]), 1);
        assert_eq!(test_reduce_product(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_test_reduce_max_examples() {
        assert_eq!(test_reduce_max(&vec![]), 0);
        assert_eq!(test_reduce_max(&vec![1]), 1);
        assert_eq!(test_reduce_max(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_test_reduce_min_examples() {
        assert_eq!(test_reduce_min(&vec![]), 0);
        assert_eq!(test_reduce_min(&vec![1]), 1);
        assert_eq!(test_reduce_min(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_multiply_by_two_examples() {
        assert_eq!(multiply_by_two(0), 0);
        assert_eq!(multiply_by_two(1), 1);
        assert_eq!(multiply_by_two(-1), -1);
    }
    #[test]
    fn quickcheck_multiply_by() {
        fn prop(multiplier: i32, x: i32) -> TestResult {
            if (multiplier > 0 && x > i32::MAX - multiplier)
                || (multiplier < 0 && x < i32::MIN - multiplier)
            {
                return TestResult::discard();
            }
            let result1 = multiply_by(multiplier.clone(), x.clone());
            let result2 = multiply_by(x.clone(), multiplier.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_multiply_by_examples() {
        assert_eq!(multiply_by(0, 0), 0);
        assert_eq!(multiply_by(1, 2), 3);
        assert_eq!(multiply_by(-1, 1), 0);
    }
    #[test]
    fn test_test_partial_multiple_args_examples() {
        let _ = test_partial_multiple_args();
    }
    #[test]
    fn test_test_compose_functions_examples() {
        assert_eq!(test_compose_functions(0), 0);
        assert_eq!(test_compose_functions(1), 1);
        assert_eq!(test_compose_functions(-1), -1);
    }
    #[test]
    fn test_test_map_reduce_pattern_examples() {
        assert_eq!(test_map_reduce_pattern(&vec![]), 0);
        assert_eq!(test_map_reduce_pattern(&vec![1]), 1);
        assert_eq!(test_map_reduce_pattern(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_test_filter_reduce_pattern_examples() {
        assert_eq!(test_filter_reduce_pattern(&vec![]), 0);
        assert_eq!(test_filter_reduce_pattern(&vec![1]), 1);
        assert_eq!(test_filter_reduce_pattern(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_memoize_factorial_examples() {
        assert_eq!(memoize_factorial(0), 0);
        assert_eq!(memoize_factorial(1), 1);
        assert_eq!(memoize_factorial(-1), -1);
    }
    #[test]
    fn test_accumulate_with_function_examples() {
        assert_eq!(accumulate_with_function(vec![]), vec![]);
        assert_eq!(accumulate_with_function(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_reduce_boolean_all_examples() {
        let _ = test_reduce_boolean_all(Default::default());
    }
    #[test]
    fn test_test_reduce_boolean_any_examples() {
        let _ = test_reduce_boolean_any(Default::default());
    }
    #[test]
    fn test_test_reduce_flatten_examples() {
        assert_eq!(test_reduce_flatten(vec![]), vec![]);
        assert_eq!(test_reduce_flatten(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_reduce_group_by_examples() {
        assert_eq!(test_reduce_group_by(vec![]), vec![]);
        assert_eq!(test_reduce_group_by(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_memoization_fibonacci_examples() {
        assert_eq!(test_memoization_fibonacci(0), 0);
        assert_eq!(test_memoization_fibonacci(1), 1);
        assert_eq!(test_memoization_fibonacci(-1), -1);
    }
}
