#[doc = "Calculate fibonacci with annotations"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn fibonacci(n: i32)  -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return n;
   
}
let _cse_temp_1 = fibonacci(n - 1) + fibonacci(n - 2);
    return _cse_temp_1;
   
}
#[doc = "Process list with performance hints"] #[doc = " Depyler: verified panic-free"] pub fn process_list<'a>(items: & 'a list<i32>)  -> list<i32>{
    let result = vec ! [];
    for item in items.iter() {
    if item>0 {
    result.push(item * 2);
   
}
} return result;
   
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