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
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_power(x: i32)  -> i32 {
    let _cse_temp_0 = {
    if 2>= 0 && 2 <= u32::MAX as i64 {
    x.checked_pow(2 as u32).expect("Power operation overflowed")
}
else {
   (x as f64).powf(2 as f64) as i64
}
};
    return _cse_temp_0;
   
}
#[doc = " Depyler: proven to terminate"] pub fn test_floor_div(📄 Source: examples/simple_power_floor.py (110 bytes)
📝 Output: examples/simple_power_floor.rs (1776 bytes)
⏱️  Parse time: 7ms
📊 Throughput: 14.1 KB/s
⏱️  Total time: 7ms
 = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
    if needs_adjustment {
    q - 1
}
else {
    q
}
};
    return Ok(_cse_temp_0);
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_test_power_examples() {
    assert_eq !(test_power(0), 0);
    assert_eq !(test_power(1), 1);
    assert_eq !(test_power(- 1), - 1);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_test_floor_div_examples() {
    assert_eq !(test_floor_div(0, 0), 0);
    assert_eq !(test_floor_div(1, 2), 3);
    assert_eq !(test_floor_div(- 1, 1), 0);
   
}
}