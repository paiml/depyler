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
#[doc = "Demonstrate power operator with different cases"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn power_examples(base: i32, exponent: i32)  -> i32 {
    let _cse_temp_0 = {
    let a = 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let _cse_temp_1 = {
    let a = - 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let _cse_temp_2 = {
    let a = 17;
    let b = - 5;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let _cse_temp_3 = _cse_temp_0 + _cse_temp_1 + _cse_temp_2;
    return _cse_temp_3;
   
}
#[doc = "Demonstrate floor division with Python semantics"] #[doc = " Depyler: proven to terminate"] pub fn floor_division_examples(dividend: i32, divisor: i32)  -> Result<i32, ZeroDivisionError>{
    let _cse_temp_0 = {
    let a = dividend;
    let b = divisor;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let result5 = _cse_temp_0;
    let _cse_temp_1 = {
    let a = 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let _cse_temp_2 = {
    let a = - 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let _cse_temp_3 = {
    let a = 17;
    let b = - 5;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
 📄 Source: examples/power_and_floor_division.py (1726 bytes)
📝 Output: examples/power_and_floor_division.rs (7209 bytes)
⏱️  Parse time: 12ms
📊 Throughput: 132.5 KB/s
⏱️  Total time: 12ms
r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let _cse_temp_6 = _cse_temp_4 + _cse_temp_5 + result5;
    return Ok(_cse_temp_6);
   
}
#[doc = "Combine power and floor division"] #[doc = " Depyler: proven to terminate"] pub fn combined_operations(a: i32, b: i32)  -> Result<i32, ZeroDivisionError>{
    let _cse_temp_0 = {
    if 2>= 0 && 2 <= u32::MAX as i64 {
    a.checked_pow(2 as u32).expect("Power operation overflowed")
}
else {
   (a as f64).powf(2 as f64) as i64
}
};
    let step1 = _cse_temp_0;
    let _cse_temp_1 = {
    let a = a;
    let b = b;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let step2 = _cse_temp_1;
    let _cse_temp_2 = {
    let a = 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    let _cse_temp_3 = {
    let a = - 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    return Ok(_cse_temp_2 + _cse_temp_3);
   
}
#[doc = "Calculate sum of squares using both operators"] pub fn mathematical_sequence(n: i32)  -> Result<i32, ZeroDivisionError>{
    let mut total = 0;
    let mut i = 1;
    while i <= n {
    let square = {
    if 2>= 0 && 2 <= u32::MAX as i64 {
    i.checked_pow(2 as u32).expect("Power operation overflowed")
}
else {
   (i as f64).powf(2 as f64) as i64
}
};
    let contribution = {
    let a = square;
    let b = 1;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
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
    total = total + contribution;
    i = i + 1;
   
}
return Ok(total);
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_power_examples_examples() {
    assert_eq !(power_examples(0, 0), 0);
    assert_eq !(power_examples(1, 2), 3);
    assert_eq !(power_examples(- 1, 1), 0);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_floor_division_examples_examples() {
    assert_eq !(floor_division_examples(0, 0), 0);
    assert_eq !(floor_division_examples(1, 2), 3);
    assert_eq !(floor_division_examples(- 1, 1), 0);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_combined_operations_examples() {
    assert_eq !(combined_operations(0, 0), 0);
    assert_eq !(combined_operations(1, 2), 3);
    assert_eq !(combined_operations(- 1, 1), 0);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_mathematical_sequence_examples() {
    assert_eq !(mathematical_sequence(0), 0);
    assert_eq !(mathematical_sequence(1), 1);
    assert_eq !(mathematical_sequence(- 1), - 1);
   
}
}