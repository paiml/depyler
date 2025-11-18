use serde_json;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn apply_func(x: i32, f: serde_json::Value) -> i32 {
    f(x)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_lambda_passing() -> i32 {
    let result1 = apply_func(10, |x| x * 2);
    let result2 = apply_func(5, |x| x + 10);
    let triple = |x| x * 3;
    let result3 = apply_func(7, triple);
    result1 + result2 + result3
}
