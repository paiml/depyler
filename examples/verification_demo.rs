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
pub struct VerifiedStack {
    pub capacity: i32,
}
impl VerifiedStack {
    pub fn new(capacity: i32) -> Self {
        Self { capacity }
    }
    pub fn push(&self, item: i32) {
        if self.items.len() < self.capacity {
            self.items.push(item);
        };
    }
    pub fn pop(&self) -> i32 {
        if self.items {
            return self.items.pop().unwrap_or_default();
        };
        return 0;
    }
    pub fn is_empty(&self) -> bool {
        return self.items.len() == 0;
    }
    pub fn is_full(&self) -> bool {
        return self.items.len() >= self.capacity;
    }
    pub fn size(&self) -> i32 {
        return self.items.len();
    }
}
#[doc = "Pure function - no side effects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[doc = "Safe array access with bounds checking"]
#[doc = " Depyler: proven to terminate"]
pub fn safe_access(items: &Vec<i32>, index: i32) -> Result<Option<i32>, IndexError> {
    let _cse_temp_0 = 0 <= index;
    let _cse_temp_1 = items.len() as i32;
    let _cse_temp_2 = index < _cse_temp_1;
    let _cse_temp_3 = (_cse_temp_0) && (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(Some(items.get(index as usize).cloned().unwrap_or_default()));
    }
    Ok(None)
}
#[doc = "Thread-safe counter increment"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn concurrent_counter(current: i32, increment: i32) -> i32 {
    current + increment
}
#[doc = "Guaranteed to terminate for non-negative inputs"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn factorial(n: i32) -> i32 {
    let _cse_temp_0 = n <= 0;
    if _cse_temp_0 {
        return 1;
    }
    let mut result = 1;
    for i in 2..n + 1 {
        result = result * i;
    }
    result
}
#[doc = "Division that never panics"]
#[doc = " Depyler: proven to terminate"]
pub fn safe_divide(a: i32, b: i32) -> Result<Option<f64>, ZeroDivisionError> {
    let _cse_temp_0 = b == 0;
    if _cse_temp_0 {
        return Ok(None);
    }
    Ok(Some((a as f64) / (b as f64)))
}
#[doc = "Returns reference to max value with proper lifetime"]
pub fn find_max(numbers: &Vec<i32>) -> Result<Option<i32>, IndexError> {
    if numbers.is_empty() {
        return Ok(None);
    }
    let mut max_val = numbers.get(0usize).cloned().unwrap_or_default();
    for num in {
        let base = numbers;
        let start = (1).max(0) as usize;
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    } {
        if num > max_val {
            max_val = num;
        }
    }
    Ok(Some(max_val))
}
#[doc = "Fibonacci with formal contracts"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
#[doc = "Demonstrate verified functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", "=== Verification Demo ===");
    let mut result = add(5, 3);
    println!("{}", format!("Pure add: {:?}", result));
    let items = vec![10, 20, 30];
    println!("{}", format!("Safe access: {:?}", safe_access(&items, 1)));
    println!(
        "{}",
        format!("Safe access OOB: {:?}", safe_access(&items, 10))
    );
    println!(
        "{}",
        format!("Concurrent: {:?}", concurrent_counter(100, 5))
    );
    println!("{}", format!("Factorial(5): {:?}", factorial(5)));
    println!("{}", format!("Safe divide: {:?}", safe_divide(10, 2)));
    println!(
        "{}",
        format!("Safe divide by zero: {:?}", safe_divide(10, 0))
    );
    let stack = VerifiedStack::new(3);
    stack.push(1);
    stack.push(2);
    println!("{}", format!("Stack size: {:?}", stack.size()));
    println!("{}", format!("Stack pop: {:?}", stack.pop()));
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_add() {
        fn prop(a: i32, b: i32) -> TestResult {
            if (a > 0 && b > i32::MAX - a) || (a < 0 && b < i32::MIN - a) {
                return TestResult::discard();
            }
            let result1 = add(a.clone(), b.clone());
            let result2 = add(b.clone(), a.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_add_examples() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
    }
    #[test]
    fn quickcheck_concurrent_counter() {
        fn prop(current: i32, increment: i32) -> TestResult {
            if (current > 0 && increment > i32::MAX - current)
                || (current < 0 && increment < i32::MIN - current)
            {
                return TestResult::discard();
            }
            let result1 = concurrent_counter(current.clone(), increment.clone());
            let result2 = concurrent_counter(increment.clone(), current.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_concurrent_counter_examples() {
        assert_eq!(concurrent_counter(0, 0), 0);
        assert_eq!(concurrent_counter(1, 2), 3);
        assert_eq!(concurrent_counter(-1, 1), 0);
    }
    #[test]
    fn test_factorial_examples() {
        assert_eq!(factorial(0), 0);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(-1), -1);
    }
    #[test]
    fn test_fibonacci_examples() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(-1), -1);
    }
}
