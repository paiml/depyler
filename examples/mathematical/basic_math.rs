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
#[doc = "Calculate factorial using iteration"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn factorial(n: i32)  -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return 1;
   
}
let mut result = 1;
    for i in 2..n + 1 {
    result = result * i;
   
}
return result;
   
}
#[doc = "Greatest common divisor using Euclidean algorithm"] pub fn gcd(a: i32, b: i32)  -> Result<i32, ZeroDivisionError>{
    while b != 0 {
    let temp = b;
    b = a % b;
    a = temp;
   
}
return Ok(a);
   
}
#[doc = "Check if number is prime"] pub fn is_prime(n: i32)  -> Result<bool, ZeroDivisionError>{
    let _cse_temp_0 = n<2;
    if _cse_temp_0 {
    return Ok(false);
   
}
let _cse_temp_1 = n == 2;
    if _cse_temp_1 {
    return Ok(true);
   
}
let _cse_temp_2 = n % 2;
    let _cse_temp_3 = _cse_temp_2 == 0;
    if _cse_temp_3 {
    return Ok(false);
   
}
let mut i = 3;
    while i * i <= n {
    if n % i == 0 {
    return Ok(false);
   
}
i = i + 2;
   
}
return Ok(true);
   
}
#[doc = "Calculate sum of squares"] #[doc = " Depyler: verified panic-free"] pub fn sum_of_squares<'a>(numbers: & 'a Vec<i32>)  -> i32 {
    let mut total = 0;
    for num in numbers.iter() {
    total = total + num * num;
   
}
return total;
   
}
#[doc = "Calculate power using exponentiation by squaring"] pub fn power(base: i32, exponent: i32)  -> Result<i32, ZeroDivisionError>{
    let _cse_temp_0 = exponent == 0;
    if _cse_temp_0 {
    return Ok(1);
   
}
let _cse_temp_1 = exponent<0;
    if _cse_temp_1 {
    return Ok(0);
   
}
let mut result = 1;
    while exponent>0 {
    if exponent % 2 == 1 {
    result = result * base;
   
}
base = base * base;
    exponent = {
    let a = exponent;
    let b = 2;
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
   
}
return Ok(result);
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_factorial_examples() {
    assert_eq !(factorial(0), 0);
    assert_eq !(factorial(1), 1);
    assert_eq !(factorial(- 1), - 1);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_gcd_examples() {
    assert_eq !(gcd(0, 0), 0);
    assert_eq !(gcd(1, 2), 3);
    assert_eq !(gcd(- 1, 1), 0);
   
}
} #[cfg(📄 Source: examples/mathematical/basic_math.py (1360 bytes)
📝 Output: examples/mathematical/basic_math.rs (3685 bytes)
⏱️  Parse time: 11ms
📊 Throughput: 118.0 KB/s
⏱️  Total time: 11ms
s {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_sum_of_squares_examples() {
    assert_eq !(sum_of_squares(0), 0);
    assert_eq !(sum_of_squares(1), 1);
    assert_eq !(sum_of_squares(- 1), - 1);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_power_examples() {
    assert_eq !(power(0, 0), 0);
    assert_eq !(power(1, 2), 3);
    assert_eq !(power(- 1, 1), 0);
   
}
}