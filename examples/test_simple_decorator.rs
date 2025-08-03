#[doc = "A simple function without decorators"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn simple_function(x: i32)  -> i32 {
    return(x * 2);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_no_decorator()  -> DynamicType {
    let mut result = simple_function(5);
    return result;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_simple_function_examples() {
    assert_eq !(simple_function(0), 0);
    assert_eq !(simple_function(1), 1);
    assert_eq !(simple_function(- 1), - 1);
   
}
}