#[derive(Debug, Clone)]
pub struct Calculator {
    pub result: i32,
}
impl Calculator {
    pub fn new() -> Self {
        Self { result: 0 }
    }
    pub fn compute_sum(&mut self, x: i32, y: i32) -> i32 {
        self.result = x + y;
        return self.result;
    }
}
#[doc = "Add two numbers together.\n    \n    Args:\n        x: First number\n        y: Second number\n        \n    Returns:\n        Sum of x and y\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
#[doc = "Multiply two numbers.\n    \n    Args:\n        x: First number\n        y: Second number\n        \n    Returns:\n        Product of x and y\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_add() {
        fn prop(x: i32, y: i32) -> TestResult {
            if (x > 0 && y > i32::MAX - x) || (x < 0 && y < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = add(x.clone(), y.clone());
            let result2 = add(y.clone(), x.clone());
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
    fn quickcheck_multiply() {
        fn prop(x: i32, y: i32) -> TestResult {
            if (x > 0 && y > i32::MAX - x) || (x < 0 && y < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = multiply(x.clone(), y.clone());
            let result2 = multiply(y.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_multiply_examples() {
        assert_eq!(multiply(0, 0), 0);
        assert_eq!(multiply(1, 2), 3);
        assert_eq!(multiply(-1, 1), 0);
    }
}
