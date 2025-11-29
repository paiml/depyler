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
#[doc = "Calculate fibonacci number recursively."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn fibonacci(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
#[doc = "Find all prime factors of a number."]
pub fn find_prime_factors(mut n: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut factors = vec![];
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n = {
                let a = n;
                let b = d;
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
        }
        d = d + 1;
    }
    let _cse_temp_0 = n > 1;
    if _cse_temp_0 {
        factors.push(n);
    }
    Ok(factors)
}
#[doc = "Perform binary search on sorted array."]
pub fn binary_search(arr: &Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut left = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut right = _cse_temp_0 - 1;
    while left <= right {
        let mid = {
            let a = left + right;
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
        if arr.get(mid as usize).cloned().unwrap_or_default() == target {
            return Ok(mid);
        } else {
            if arr.get(mid as usize).cloned().unwrap_or_default() < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    Ok(-1)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_functions() -> Result<(), Box<dyn std::error::Error>> {
    let result = fibonacci(10);
    println!("{}", format!("Fibonacci(10) = {:?}", result));
    let mut factors = find_prime_factors(60)?;
    println!("{}", format!("Prime factors of 60: {:?}", factors));
    let test_array = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let index = binary_search(&test_array, 7)?;
    println!("{}", format!("Index of 7: {:?}", index));
    Ok(())
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
}
