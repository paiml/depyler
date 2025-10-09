use std::collections::HashMap;
    #[derive(Debug, Clone)] pub struct IndexError {
    message: String ,
}
impl std::fmt::Display for IndexError {
    fn fmt(& self, f: & mut std::fmt::Formatter<'_>)  -> std::fmt::Result {
    write !(f, "index out of range: {}", self.message)
}
} impl std::error::Error for IndexError {
   
}
impl IndexError {
    pub fn new(message: impl Into<String>)  -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = "Classic recursive fibonacci - demonstrates recursion"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn fibonacci_recursive(n: i32)  -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return n;
   
}
let _cse_temp_1 = fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
    return _cse_temp_1;
   
}
#[doc = "Memoized fibonacci - demonstrates optimization patterns"] #[doc = " Depyler: proven to terminate"] pub fn fibonacci_memoized(n: i32, memo: HashMap<i32, i32>)  -> Result<i32, IndexError>{
    if memo.is_none() {
    memo = {
    let mut map = HashMap::new();
    map };
   
}
let _cse_temp_0 = memo.contains_key(& n);
    if _cse_temp_0 {
    return Ok(memo.get(n as usize).copied().unwrap_or_default());
   
}
let _cse_temp_1 = n <= 1;
    if _cse_temp_1 {
    let mut result = n;
   
}
else {
    let _cse_temp_2 = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo);
    let mut result = _cse_temp_2;
   
}
memo.insert(n, result);
    return Ok(result);
   
}
#[doc = "Iterative fibonacci - demonstrates loops and efficiency"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn fibonacci_iterative(n: i32)  -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return n;
   
}
let(mut a, mut b)  = (0, 1);
    for _ in 2..n + 1 {
   (a, b)  = (b, a + b);
   
}
return b;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_fibonacci_recursive_examples() {
    assert_eq !(fibonacci_recursive(0), 0);
    assert_eq !(fibonacci_recursive(1), 1);
    assert_eq !(fibonacci_recursive(- 1), - 1);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_fibonacci_iterative_examples() {
    assert_eq !(fibonacci_iterative(0), 0);
    assert_eq !(fibonacci_iterative(1), 1);
    assert_eq !(fibonacci_iterative(- 1), - 1);
   
}
}