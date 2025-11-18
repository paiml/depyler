#[doc = "Calculate fibonacci with annotations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
#[doc = "Process list with performance hints"]
#[doc = " Depyler: verified panic-free"]
pub fn process_list(items: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for item in items.iter().cloned() {
        if item > 0 {
            result.push(item * 2);
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_fibonacci_examples() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(-1), -1);
    }
    #[test]
    fn test_process_list_examples() {
        assert_eq!(process_list(vec![]), vec![]);
        assert_eq!(process_list(vec![1]), vec![1]);
    }
}
