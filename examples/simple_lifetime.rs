#[doc = "Get string length"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_len(s: &str) -> i32 {
    s.len() as i32 as i32
}
#[doc = "Return the input unchanged"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn identity(x: String) -> String {
    x
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_get_len_examples() {
        assert_eq!(get_len(""), 0);
        assert_eq!(get_len("a"), 1);
        assert_eq!(get_len("abc"), 3);
    }
    #[test]
    fn quickcheck_identity() {
        fn prop(x: String) -> TestResult {
            let result = identity((&*x).into());
            if result != x {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
}
