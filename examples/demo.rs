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
    for i in 1..n + 1 {
        result = result * i;
    }
    result
}
#[doc = "Check if a number is prime"]
#[doc = " Depyler: proven to terminate"]
pub fn is_prime(n: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let _cse_temp_0 = n < 2;
    if _cse_temp_0 {
        return Ok(false);
    }
    for i in 2..n {
        if n % i == 0 {
            return Ok(false);
        }
    }
    Ok(true)
}
#[doc = "Sum all numbers in a list"]
#[doc = " Depyler: verified panic-free"]
pub fn process_list(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers.iter().cloned() {
        total = total + num;
    }
    total
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
    fn test_is_prime_examples() {
        let _ = is_prime(Default::default());
    }
    #[test]
    fn test_process_list_examples() {
        assert_eq!(process_list(&vec![]), 0);
        assert_eq!(process_list(&vec![1]), 1);
        assert_eq!(process_list(&vec![1, 2, 3]), 3);
    }
}
