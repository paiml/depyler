#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn apply_func(x: i32, f: serde_json::Value)  -> i32 {
    return f(x);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_lambda_passing()  -> serde_json::Value {
    let mut result1 = apply_func(10, | x |(x * 2));
    let mut result2 = apply_func(5, | x |(x + 10));
    let mut triple = | x |(x * 3);
    let mut result3 = apply_func(7, triple);
    return((result1 + result2) + result3)
}