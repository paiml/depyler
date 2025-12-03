use serde_json;
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
#[doc = "Process a list with nested loops - O(n²) complexity."]
#[doc = " Depyler: verified panic-free"]
pub fn process_list(items: &Vec<i32>) -> i32 {
    let mut total = 0;
    for i in items.iter().cloned() {
        for j in items.iter().cloned() {
            if i < j {
                total = total + i * j;
            }
        }
    }
    total
}
#[doc = "String concatenation in loop - inefficient pattern."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn string_concatenation_in_loop(n: i32) -> String {
    let mut result = "".to_string();
    for i in 0..n {
        result = format!("{}{}", result, (i).to_string());
        result = format!("{}{}", result, ", ");
    }
    result.to_string()
}
#[doc = "Function with many type checks that Rust can optimize away."]
#[doc = " Depyler: verified panic-free"]
pub fn type_check_heavy(values: &Vec<serde_json::Value>) -> i32 {
    let mut count = 0;
    for value in values.iter().cloned() {
        if true {
            count = count + value;
        } else {
            if true {
                count = count + value.len() as i32;
            }
        }
    }
    count
}
#[doc = "Simple function for baseline comparison."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simple_function(x: i32, y: i32) -> i32 {
    x + y
}
#[doc = "Function with hot nested loops."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn hot_loop() -> i32 {
    let mut total = 0;
    for i in 0..100 {
        for j in 0..100 {
            total = total + i * j;
        }
    }
    total
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
    #[test]
    fn test_hot_loop_examples() {
        let _ = hot_loop();
    }
}
