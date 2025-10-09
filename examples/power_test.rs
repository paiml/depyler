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
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_integer_power()  -> DynamicType {
    return(a, b, c, d);
   
}
#[doc = " Depyler: proven to terminate"] pub fn test_float_power()  -> Result<DynamicType, ZeroDivisionError>{
    return Ok((a, b, c, d));
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_negative_exponent()  -> DynamicType {
    return(a, b, c);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_large_powers()  -> DynamicType {
    return(a, b, c);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_mixed_operations()  -> DynamicType {
    return(a, b, c, d);
   
}
#[doc = "Test power with function parameters"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn compute_power(base: i32, exp: i32)  -> i32 {
    let _cse_temp_0 = 3.checked_pow(4 as u32).expect("Power operation overflowed");
    return _cse_temp_0;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_compute_power_examples() {
    assert_eq !(compute_power(0, 0), 0);
    assert_eq !(compute_power(1, 2), 3);
    assert_eq !(compute_power(- 1, 1), 0);
   
}
}