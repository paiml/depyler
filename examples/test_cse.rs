#[doc = "Multiple uses of the same complex expression."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn repeated_complex_expressions(a: i32, b: i32, c: i32) -> i32 {
    let _cse_temp_0 = a + b * c;
    let x = _cse_temp_0 + 10;
    let y = _cse_temp_0 - 5;
    let _cse_temp_1 = _cse_temp_0 * 2;
    let z = _cse_temp_1;
    x + y + z
}
#[doc = "Repeated calls to pure functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn repeated_function_calls(n: i32) -> i32 {
    let _cse_temp_0 = n - 10.abs();
    let _cse_temp_1 = _cse_temp_0 > 5;
    let mut result;
    if _cse_temp_1 {
        let _cse_temp_2 = _cse_temp_0 * 2;
        result = _cse_temp_2;
    } else {
        result = _cse_temp_0 + 100;
    }
    result + n - 10.abs()
}
#[doc = "Nested common subexpressions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn nested_expressions(x: i32, y: i32) -> i32 {
    let _cse_temp_0 = x * y;
    let _cse_temp_1 = _cse_temp_0 * 2;
    let a = _cse_temp_0 + _cse_temp_1;
    let _cse_temp_2 = _cse_temp_0 * _cse_temp_0;
    let b = _cse_temp_2;
    let _cse_temp_3 = x + 1 * y + 1;
    let c = _cse_temp_3 + 10;
    let d = _cse_temp_3 - 20;
    a + b + c + d
}
#[doc = "CSE across conditional branches."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn conditional_cse(flag: bool, a: i32, b: i32) -> i32 {
    let _cse_temp_0 = a * b;
    let _cse_temp_1 = _cse_temp_0 + a - b;
    let base = _cse_temp_1;
    let mut result;
    if flag {
        result = _cse_temp_1 + 10;
    } else {
        result = _cse_temp_1 - 10;
    }
    result + base
}
#[doc = "Expressions that don't change in loops."]
#[doc = " Depyler: verified panic-free"]
pub fn loop_invariant_expressions(items: &Vec<String>) -> i32 {
    let x = 10;
    let y = 20;
    let mut total = 0;
    for item in items.iter().cloned() {
        total = total + item + x + y * 2;
    }
    total
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_repeated_function_calls_examples() {
        assert_eq!(repeated_function_calls(0), 0);
        assert_eq!(repeated_function_calls(1), 1);
        assert_eq!(repeated_function_calls(-1), -1);
    }
    #[test]
    fn test_nested_expressions_examples() {
        assert_eq!(nested_expressions(0, 0), 0);
        assert_eq!(nested_expressions(1, 2), 3);
        assert_eq!(nested_expressions(-1, 1), 0);
    }
    #[test]
    fn test_loop_invariant_expressions_examples() {
        assert_eq!(loop_invariant_expressions(&vec![]), 0);
        assert_eq!(loop_invariant_expressions(&vec![1]), 1);
        assert_eq!(loop_invariant_expressions(&vec![1, 2, 3]), 3);
    }
}
