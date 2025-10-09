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
#[doc = "Calculate fibonacci number recursively"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn fibonacci(n: i32)  -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return n;
   
}
let _cse_temp_1 = fibonacci(n - 1) + fibonacci(n - 2);
    return _cse_temp_1;
   
}
#[doc = "Calculate factorial iteratively"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn factorial(n: i32)  -> i32 {
    let mut result = 1;
    for i in 2..n + 1 {
    result = result * i;
   
}
return result;
   
}
#[doc = "Find maximum in a list"] pub fn find_max<'a>(numbers: & 'a [DynamicType;
    1])  -> Result<i32, IndexError>{
    if ! numbers {
    return Ok(0);
   
}
let mut max_val = numbers.get(0 as usize).copied().unwrap_or_default();
    for num in numbers.iter() {
    if num>max_val {
    max_val = num;
   
}
} return Ok(max_val);
   
}
#[doc = "Main function demonstrating various algorithms"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn main ()  -> DynamicType {
    print(format !("Fibonacci(10) = {}", fibonacci(10)));
    print(format !("Factorial(5) = {}", factorial(5)));
    print(format !("Max of {} = {}", numbers, find_max(numbers)));
   
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
} #[cfg(test)] mod tests {
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
    #[test] fn test_find_max_examples() {
    assert_eq !(find_max(0), 0);
    assert_eq !(find_max(1), 1);
    assert_eq !(find_max(- 1), - 1);
   
}
}