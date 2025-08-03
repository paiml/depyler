#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_basic_lambdas()  -> serde_json::Value {
    let mut square = | x |(x * x);
    let mut result1 = square(5);
    let mut add = | x, y |(x + y);
    let mut result2 = add(3, 4);
    let mut get_value = | | 42;
    let mut result3 = get_value();
    return((result1 + result2) + result3)
}