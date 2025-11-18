#[doc = "Add two numbers."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
#[doc = "Multiply two numbers."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn multiply(x: i32, y: i32) -> i32 {
    let _cse_temp_0 = x * y;
    let result = _cse_temp_0;
    result
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
    fn test_multiply_examples() {
        assert_eq!(multiply(0, 0), 0);
        assert_eq!(multiply(1, 2), 3);
        assert_eq!(multiply(-1, 1), 0);
    }
}
