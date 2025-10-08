#[derive(Debug, Clone)] pub struct ZeroDivisionError {
    message: String ,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(& self, f: & mut std::fmt::Formatter<'_>)  -> std::fmt::Result {
    write !(f, "division by zero: {}", self.message)
}
} impl std::error::Error for ZeroDivisionError {
   
}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>)  -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = "Demonstrates constant propagation and dead code elimination"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn compute_constants()  -> i32 {
    let z = 15;
    let _cse_temp_0 = z * 2;
    let result = _cse_temp_0;
    return result;
   
}
#[doc = "Example that won't be fully optimized(recursive)"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn fibonacci(n: i32)  -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return n;
   
}
let _cse_temp_1 = fibonacci(n - 1) + fibonacci(n - 2);
    return _cse_temp_1;
   
}
#[doc = "More constant folding examples"] #[doc = " Depyler: proven to terminate"] pub fn simple_math()  -> Result<f64, ZeroDivisionError>{
    let c = 6.28;
    let _cse_temp_0 = c / 2;
    let d = _cse_temp_0;
    return Ok(d);
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_compute_constants_examples() {
    let _ = compute_constants();
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_fibonacci_examples() {
    assert_eq !(fibonacci(0), 0);
    assert_eq !(fibonacci(1), 1);
    assert_eq !(fibonacci(- 1), - 1);
   
}
}