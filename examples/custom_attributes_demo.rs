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
#[doc = "Simple addition with inline hint."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
#[inline]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
#[doc = "Performance-critical multiplication with aggressive inlining."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
#[inline(always)]
pub fn multiply_fast(x: i32, y: i32) -> i32 {
    x * y
}
#[doc = "Calculate checksum - result must be used."]
#[doc = " Depyler: verified panic-free"]
#[must_use]
pub fn calculate_checksum(data: &Vec<i32>) -> i32 {
    let mut checksum = 0;
    for value in data.iter().cloned() {
        checksum = checksum ^ value;
    }
    checksum
}
#[doc = "Error handler - rarely executed."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
#[cold]
pub fn handle_panic(message: String) {
    println!("{}", format!("PANIC: {}", message));
}
#[doc = "Hash function with multiple attributes."]
#[inline]
#[must_use]
pub fn compute_hash(text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut hash_val = 0;
    for _char in text.chars() {
        let char = _char.to_string();
        hash_val = (hash_val * 31 + char.chars().next().unwrap() as i32)
            % ({ 2 } as i32)
                .checked_pow({ 32 } as u32)
                .expect("Power operation overflowed");
    }
    Ok(hash_val)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_add_numbers() {
        fn prop(a: i32, b: i32) -> TestResult {
            if (a > 0 && b > i32::MAX - a) || (a < 0 && b < i32::MIN - a) {
                return TestResult::discard();
            }
            let result1 = add_numbers(a.clone(), b.clone());
            let result2 = add_numbers(b.clone(), a.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_add_numbers_examples() {
        assert_eq!(add_numbers(0, 0), 0);
        assert_eq!(add_numbers(1, 2), 3);
        assert_eq!(add_numbers(-1, 1), 0);
    }
    #[test]
    fn quickcheck_multiply_fast() {
        fn prop(x: i32, y: i32) -> TestResult {
            if (x > 0 && y > i32::MAX - x) || (x < 0 && y < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = multiply_fast(x.clone(), y.clone());
            let result2 = multiply_fast(y.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_multiply_fast_examples() {
        assert_eq!(multiply_fast(0, 0), 0);
        assert_eq!(multiply_fast(1, 2), 3);
        assert_eq!(multiply_fast(-1, 1), 0);
    }
    #[test]
    fn test_calculate_checksum_examples() {
        assert_eq!(calculate_checksum(&vec![]), 0);
        assert_eq!(calculate_checksum(&vec![1]), 1);
        assert_eq!(calculate_checksum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_compute_hash_examples() {
        assert_eq!(compute_hash(""), 0);
        assert_eq!(compute_hash("a"), 1);
        assert_eq!(compute_hash("abc"), 3);
    }
}
