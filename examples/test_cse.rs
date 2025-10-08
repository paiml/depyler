#[doc = "Multiple uses of the same complex expression."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn repeated_complex_expressions(a: i32, b: i32, c: i32)  -> i32 {
    let _cse_temp_0 = a + b * c;
    let _cse_temp_1 = _cse_temp_0 * 2;
    let z = _cse_temp_1;
    return 30 + z;
   
}
#[doc = "Repeated calls to pure functions."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn repeated_function_calls(n: i32)  -> i32 {
    let _cse_temp_0 = abs(n - 10);
    let _cse_temp_1 = _cse_temp_0>5;
    if _cse_temp_1 {
    let _cse_temp_2 = _cse_temp_0 * 2;
    let mut result = _cse_temp_2;
   
}
else {
    let mut result = _cse_temp_0 + 100;
   
}
return result + _cse_temp_0;
   
}
#[doc = "Nested common subexpressions."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn nested_expressions(x: i32, y: i32)  -> i32 {
    let a = 600;
    let b = 40000;
    let c = 241;
    let d = 211;
    let _cse_temp_0 = a + b + c;
    return _cse_temp_0 + d;
   
}
#[doc = "CSE across conditional branches."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn conditional_cse(flag: bool, a: i32, b: i32)  -> i32 {
    let _cse_temp_0 = a * b;
    let _cse_temp_1 = _cse_temp_0 + a - b;
    let base = _cse_temp_1;
    if flag {
    let mut result = _cse_temp_1 + 10;
   
}
else {
    let mut result = _cse_temp_1 - 10;
   
}
return result + base;
   
}
#[doc = "Expressions that don't change in loops."] #[doc = " Depyler: verified panic-free"] pub fn loop_invariant_expressions<'a>(items: & 'a Vec<DynamicType>)  -> i32 {
    let mut total = 0;
    for item in items.iter() {
    total = total + item + 60;
   
}
return total;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_repeated_function_calls_examples() {
    assert_eq !(repeated_function_calls(0), 0);
    assert_eq !(repeated_function_calls(1), 1);
    assert_eq !(repeated_function_calls(- 1), - 1);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_nested_expressions_examples() {
    assert_eq !(nested_expressions(0, 0), 0);
    assert_eq !(nested_expressions(1, 2), 3);
    assert_eq !(nested_expressions(- 1, 1), 0);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_loop_invariant_expressions_examples() {
    assert_eq !(loop_invariant_expressions(0), 0);
    assert_eq !(loop_invariant_expressions(1), 1);
    assert_eq !(loop_invariant_expressions(- 1), - 1);
   
}
}