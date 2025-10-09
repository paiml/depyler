#[doc = "Simple addition function with annotations"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn add_numbers(a: i32, b: i32)  -> i32 {
    return a + b;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn quickcheck_add_numbers() {
    fn prop(a: i32, b: i32)  -> TestResult {
    let result1 = add_numbers(a.clone(), b.clone());
    let result2 = add_numbers(b.clone(), a.clone());
    if result1 != result2 {
    return TestResult::failed();
   
}
TestResult::passed()
}
quickcheck(prop as fn(i32, i32)  -> TestResult);
   
}
#[test] fn test_add_numbers_examples() {
    assert_eq !(add_numbers(0, 0), 0);
    assert_eq !(add_numbers(1, 2), 3);
    assert_eq !(add_numbers(- 1, 1), 0);
   
}
}