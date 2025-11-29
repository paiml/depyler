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
#[doc = "Calculate factorial using iteration"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn factorial(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    let mut result = 1;
    for i in 2..n + 1 {
        result = result * i;
    }
    result
}
#[doc = "Greatest common divisor using Euclidean algorithm"]
pub fn gcd(mut a: i32, mut b: i32) -> Result<i32, Box<dyn std::error::Error>> {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    Ok(a)
}
#[doc = "Check if number is prime"]
pub fn is_prime(n: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let _cse_temp_0 = n < 2;
    if _cse_temp_0 {
        return Ok(false);
    }
    let _cse_temp_1 = n == 2;
    if _cse_temp_1 {
        return Ok(true);
    }
    let _cse_temp_2 = n % 2;
    let _cse_temp_3 = _cse_temp_2 == 0;
    if _cse_temp_3 {
        return Ok(false);
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return Ok(false);
        }
        i = i + 2;
    }
    Ok(true)
}
#[doc = "Calculate sum of squares"]
#[doc = " Depyler: verified panic-free"]
pub fn sum_of_squares(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers.iter().cloned() {
        total = total + num * num;
    }
    total
}
#[doc = "Calculate power using exponentiation by squaring"]
pub fn power(mut base: i32, mut exponent: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = exponent == 0;
    if _cse_temp_0 {
        return Ok(1);
    }
    let _cse_temp_1 = exponent < 0;
    if _cse_temp_1 {
        return Ok(0);
    }
    let mut result = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = result * base;
        }
        base = base * base;
        exponent = {
            let a = exponent;
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
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_factorial_examples() {
        assert_eq!(factorial(0), 0);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(-1), -1);
    }
    #[test]
    fn test_gcd_examples() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(1, 2), 3);
        assert_eq!(gcd(-1, 1), 0);
    }
    #[test]
    fn test_is_prime_examples() {
        let _ = is_prime(Default::default());
    }
    #[test]
    fn test_sum_of_squares_examples() {
        assert_eq!(sum_of_squares(&vec![]), 0);
        assert_eq!(sum_of_squares(&vec![1]), 1);
        assert_eq!(sum_of_squares(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_power_examples() {
        assert_eq!(power(0, 0), 0);
        assert_eq!(power(1, 2), 3);
        assert_eq!(power(-1, 1), 0);
    }
}
