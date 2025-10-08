use std::collections::HashMap;
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
#[derive(Debug, Clone)] pub struct MathUtils {
    pub precision: i32
}
impl MathUtils {
    pub fn new()  -> Self {
    Self {
    precision: 0
}
} pub fn round_number(& mut self, value: f64)  -> f64 {
    return round(value, self.precision);
   
}
pub fn is_prime(n: i32)  -> bool {
    if n<2 {
    return false
};
    for i in 2..int((n as f64).powf(0.5)) + 1 {
    if n % i == 0 {
    return false
}
};
    return true;
   
}
} #[doc = "Calcul📄 Source: examples/test_ide_integration.py (1380 bytes)
📝 Output: examples/test_ide_integration.rs (2100 bytes)
⏱️  Parse time: 11ms
📊 Throughput: 117.2 KB/s
⏱️  Total time: 11ms
fibonacci(n: i32)  -> i32 {
    let _cse_temp_0 = n <= 1;
    if _cse_temp_0 {
    return n;
   
}
let _cse_temp_1 = calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);
    return _cse_temp_1;
   
}
#[doc = "Process a list of integers and return statistics."] pub fn process_data(items: list<i32>)  -> Result<dict<String, i32>, IndexError>{
    let utils = MathUtils::new();
    let stats = {
    let mut map = HashMap::new();
    map.insert("count", items.len());
    map.insert("sum", sum(items));
    map.insert("primes", 0);
    map };
    for item in items.iter() {
    if utils.is_prime(item) {
    stats.insert("primes", stats.get("primes").cloned().unwrap_or_default() + 1);
   
}
} return Ok(stats);
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_calculate_fibonacci_examples() {
    assert_eq !(calculate_fibonacci(0), 0);
    assert_eq !(calculate_fibonacci(1), 1);
    assert_eq !(calculate_fibonacci(- 1), - 1);
   
}
}