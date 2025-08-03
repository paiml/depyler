#[doc = "Calculate fibonacci number recursively"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn fibonacci(n: i32)  -> i32 {
    if(n <= 1) {
    return n;
   
}
return(fibonacci((n - 1)) + fibonacci((n - 2)));
   
}
#[doc = "Calculate factorial iteratively"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn factorial(n: i32)  -> i32 {
    let mut result = 1;
    for i in 1..(n + 1) {
    result  = (result * i);
   
}
return result;
   
}
#[doc = "Check if a number is prime"] #[doc = " Depyler: proven to terminate"] pub fn is_prime(n: i32)  -> Result<bool, ZeroDivisionError>{
    if(n<2) {
    return Ok(false);
   
}
for i in 2..n {
    if((n % i) == 0) {
    return Ok(false);
   
}
} return Ok(true);
   
}
#[doc = "Sum all numbers in a list"] #[doc = " Depyler: verified panic-free"] pub fn process_list<'a>(numbers: & 'a Vec<i32>)  -> i32 {
    let mut total = 0;
    for num in numbers.iter() {
    total  = (total + num);
   
}
return total;
   
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
    #[test] fn test_is_prime_examples() {
    let _ = is_prime(Default::default());
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_process_list_examples() {
    assert_eq !(process_list(0), 0);
    assert_eq !(process_list(1), 1);
    assert_eq !(process_list(- 1), - 1);
   
}
}