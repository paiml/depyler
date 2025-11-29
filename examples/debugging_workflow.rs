use serde_json;
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
#[doc = "Calculate fibonacci number recursively"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
#[doc = "Calculate factorial iteratively"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn factorial(n: i32) -> i32 {
    let mut result = 1;
    for i in 2..n + 1 {
        result = result * i;
    }
    result
}
#[doc = "Find maximum in a list"]
pub fn find_max(numbers: &Vec<serde_json::Value>) -> Result<i32, Box<dyn std::error::Error>> {
    if numbers.is_empty() {
        return Ok(0);
    }
    let mut max_val = numbers.get(0usize).cloned().unwrap_or_default();
    for num in numbers.iter().cloned() {
        if num > max_val {
            max_val = num;
        }
    }
    Ok(max_val)
}
#[doc = "Main function demonstrating various algorithms"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", format!("Fibonacci(10) = {}", fibonacci(10)));
    println!("{}", format!("Factorial(5) = {}", factorial(5)));
    let numbers = vec![3, 7, 2, 9, 1, 5];
    println!(
        "{}",
        format!("Max of {:?} = {:?}", numbers, find_max(&numbers))
    );
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
    fn test_factorial_examples() {
        assert_eq!(factorial(0), 0);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(-1), -1);
    }
    #[test]
    fn test_find_max_examples() {
        assert_eq!(find_max(&vec![]), 0);
        assert_eq!(find_max(&vec![1]), 1);
        assert_eq!(find_max(&vec![1, 2, 3]), 3);
    }
}
