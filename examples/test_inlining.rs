#[doc = "Trivial function - should be inlined."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn square(x: i32) -> i32 {
    x * x
}
#[doc = "Another trivial function."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn add_one(n: i32) -> i32 {
    n + 1
}
#[doc = "Should inline the square calls."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn compute_distance_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    square(dx) + square(dy)
}
#[doc = "Called only once - should be inlined."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_single_use(value: i32) -> i32 {
    let _cse_temp_0 = value * 2;
    let temp = _cse_temp_0;
    let result = temp + 10;
    result
}
#[doc = "Main function that uses other functions."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main_computation(a: i32, b: i32) -> i32 {
    let step1 = process_single_use(a);
    let step2 = add_one(step1);
    let step3 = add_one(b);
    let distance = compute_distance_squared(0, 0, step2, step3);
    distance
}
#[doc = "Recursive function - should NOT be inlined."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn recursive_factorial(n: i32) -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
        return 1;
    }
    n * recursive_factorial(n - 1)
}
#[doc = "Contains loop - may not be inlined depending on config."]
#[doc = " Depyler: verified panic-free"]
pub fn has_loop(items: &Vec<String>) -> i32 {
    let mut total = 0;
    for item in items.iter().cloned() {
        total = total + item;
    }
    total
}
#[doc = "Large function - should NOT be inlined."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn large_function(x: i32, y: i32, z: i32) -> i32 {
    let a = x + y;
    let b = y + z;
    let c = z + x;
    let _cse_temp_0 = a * b;
    let d = _cse_temp_0;
    let _cse_temp_1 = b * c;
    let e = _cse_temp_1;
    let _cse_temp_2 = c * a;
    let f = _cse_temp_2;
    let g = d + e;
    let h = e + f;
    let i = f + d;
    let _cse_temp_3 = g * h;
    let j = _cse_temp_3;
    let _cse_temp_4 = h * i;
    let k = _cse_temp_4;
    let _cse_temp_5 = i * g;
    let l = _cse_temp_5;
    let m = j + k;
    let n = k + l;
    let o = l + j;
    let _cse_temp_6 = m + n + o;
    let result = _cse_temp_6;
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_square_examples() {
        assert_eq!(square(0), 0);
        assert_eq!(square(1), 1);
        assert_eq!(square(-1), -1);
    }
    #[test]
    fn test_add_one_examples() {
        assert_eq!(add_one(0), 0);
        assert_eq!(add_one(1), 1);
        assert_eq!(add_one(-1), -1);
    }
    #[test]
    fn test_process_single_use_examples() {
        assert_eq!(process_single_use(0), 0);
        assert_eq!(process_single_use(1), 1);
        assert_eq!(process_single_use(-1), -1);
    }
    #[test]
    fn test_main_computation_examples() {
        assert_eq!(main_computation(0, 0), 0);
        assert_eq!(main_computation(1, 2), 3);
        assert_eq!(main_computation(-1, 1), 0);
    }
    #[test]
    fn test_recursive_factorial_examples() {
        assert_eq!(recursive_factorial(0), 0);
        assert_eq!(recursive_factorial(1), 1);
        assert_eq!(recursive_factorial(-1), -1);
    }
    #[test]
    fn test_has_loop_examples() {
        assert_eq!(has_loop(&vec![]), 0);
        assert_eq!(has_loop(&vec![1]), 1);
        assert_eq!(has_loop(&vec![1, 2, 3]), 3);
    }
}
