use serde_json;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_lambda_with_operations() -> (serde_json::Value, serde_json::Value, serde_json::Value) {
    let calc = |x, y, z| x + y * z;
    let result1 = calc(((2) as i64), ((3) as i64), ((4) as i64));
    let negate = |x| -x;
    let result2 = negate(((5) as i64));
    let multiply = |a, b| a * b;
    let result3 = multiply(((10) as i64), ((20) as i64));
    (result1, result2, result3)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_lambda_as_argument() -> (serde_json::Value, serde_json::Value) {
    let numbers = vec![1, 2, 3, 4, 5];
    fn apply_to_list(lst: Vec<i64>, func: ()) -> Vec<i64> {
        let result = vec![];
        for item in lst.iter().cloned() {
            result.push(func(item));
        }
        return result;
    }
    let doubled = apply_to_list(&numbers, |x| x * 2);
    let squared = apply_to_list(&numbers, |x| x * x);
    (doubled, squared)
}
