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
#[doc = "Calculate fibonacci number recursively."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn fibonacci(n: i32)  -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return n;
   
}
let _cse_temp_1 = fibonacci(n - 1) + fibonacci(n - 2);
    return _cse_temp_1;
   
}
#[doc = "Find all prime factors of a number."] pub fn find_prime_factors(n: i32)  -> Result<list<i32>, ZeroDivisionError>{
    let factors = vec ! [];
    let mut d = 2;
    while d * d <= n {
    while n % d == 0 {
    factors.push(d);
    n = {
    let a = n;
    let b = d;
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
d = d + 1;
   
}
let _cse_temp_0 = n>1;
    if _cse_temp_0 {
    factors.push(n);
   
}
return Ok(factors);
   
}
#[doc = "Perform binary search on sorted array."] pub fn binary_search<'a>(arr: & 'a list<i32>, target: i32)  -> Result<i32, Box<dyn std::error::Error>>{
    let mut left = 0;
    let _cse_temp_0 = arr.len();
    let mut right = _cse_temp_0 - 1;
    while left <= right {
    let mid = {
    let a = left + right;
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
    if arr.get(mid as usize).copied().unwrap_or_default() == target {
    return Ok(mid);
   
}
else {
    if arr.get(mid as usize).copied().unwrap_or_default()<target {
    left = mid + 1;
   
}
else {
    right = mid - 1;
   
}
}
}
return Ok(- 1);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_functions()  -> DynamicType {
    print(format !("Fibonacci(10) = {}", result));
    print(format !("Prime factors of 60: {}", factors));
    let test_array = vec ! [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    print(format !("Index of 7: {}", index));
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_fibonacci_examples() {
    assert_eq !(fibonacci(0), 0);
    assert_eq !(fibonacci(1), 1);
    assert_eq !(fibonacci(- 1), - 1);
   
}
}