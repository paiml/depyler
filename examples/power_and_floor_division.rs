#[doc = "Demonstrate power operator with different cases"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn power_examples(base: i32, exponent: i32)  -> i32 {
    let mut _cse_temp_0 = {
    let a = 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut _cse_temp_1 = {
    let a = - 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut _cse_temp_2 = {
    let a = 17;
    let b = - 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut _cse_temp_3  = ((_cse_temp_0 + _cse_temp_1) + _cse_temp_2);
    return _cse_temp_3;
   
}
#[doc = "Demonstrate floor division with Python semantics"] #[doc = " Depyler: proven to terminate"] pub fn floor_division_examples(dividend: i32, divisor: i32)  -> Result<i32, ZeroDivisionError>{
    let mut _cse_temp_0 = {
    let a = dividend;
    let b = divisor;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut result5 = _cse_temp_0;
    let mut _cse_temp_1 = {
    let a = 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut _cse_temp_2 = {
    let a = - 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut _cse_temp_3 = {
    let a = 17;
    let b = - 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut _cse_temp_4  = ((_cse_temp_1 + _cse_temp_2) + _cse_temp_3);
    let mut _cse_temp_5 = {
    let a = - 17;
    let b = - 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut _cse_temp_6  = ((_cse_temp_4 + _cse_temp_5) + result5);
    return Ok(_cse_temp_6);
   
}
#[doc = "Combine power and floor division"] #[doc = " Depyler: proven to terminate"] pub fn combined_operations(a: i32, b: i32)  -> Result<i32, ZeroDivisionError>{
    let mut _cse_temp_0 = {
    if 2>= 0 && 2 <= u32::MAX as i64 {
    a.checked_pow(2 as u32).expect("Power operation overflowed")
}
else {
   (a as f64).powf(2 as f64) as i64
}
};
    let mut step1 = _cse_temp_0;
    let mut _cse_temp_1 = {
    let a = a;
    let b = b;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut step2 = _cse_temp_1;
    let mut _cse_temp_2 = {
    let a = 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut _cse_temp_3 = {
    let a = - 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    return Ok((_cse_temp_2 + _cse_temp_3));
   
}
#[doc = "Calculate sum of squares using both operators"] pub fn mathematical_sequence(n: i32)  -> Result<i32, ZeroDivisionError>{
    while(1 <= n) {
    let mut square = 1.checked_pow(2 as u32).expect("Power operation overflowed");
    let mut contribution = {
    let a = square;
    let b = 1;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
};
    let mut total  = (0 + contribution);
    let mut i = 2;
   
}
return Ok(0);
   
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