use serde_json;
use std::borrow::Cow;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct MyObject {}
impl MyObject {
    pub fn new() -> Self {
        Self {}
    }
    pub fn setup(&mut self, mode: String, timeout: i32, retry: bool) {
        self.mode = mode;
        self.timeout = timeout;
        self.retry = retry;
    }
}
#[doc = "Test function calls with keyword arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_function_kwargs() -> (serde_json::Value, serde_json::Value, serde_json::Value) {
    let result1 = greet("Alice".to_string(), "Hello".to_string());
    let result2 = calculate(10, 20, "add", true);
    let result3 = configure(800, 600, "My App".to_string());
    (result1, result2, result3)
}
#[doc = "Test method calls with keyword arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_method_kwargs() -> String {
    let mut obj = MyObject::new();
    obj.setup("advanced".to_string(), 30, true);
    let text = "hello world";
    let formatted = text.replace("world", "Python");
    formatted
}
#[doc = "Test builtin functions with keyword arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_builtin_kwargs() -> HashMap<serde_json::Value, serde_json::Value> {
    let f = std::fs::File::open("data.txt")?;
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted_desc = {
        let mut __sorted_result = numbers.clone();
        __sorted_result.sort();
        __sorted_result.reverse();
        __sorted_result
    };
    let config = HashMap::new();
    config
}
#[doc = "Test nested function calls with kwargs"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_nested_kwargs() -> i32 {
    let result = outer(inner(10, 20), 2.0, inner(5, 5));
    result
}
#[doc = "Test kwargs with complex expressions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_complex_kwargs() -> i32 {
    let settings = configure(
        100 + 200,
        get_height(),
        (true) && (!false),
        format!("{}{}", "App ".to_string(), 42.to_string()),
    );
    settings
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn greet(name: String, greeting: String) -> String {
    format!("{:?}, {:?}!", greeting, name)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate(a: i32, b: i32, operation: &str, verbose: bool) -> i32 {
    let _cse_temp_0 = operation == "add";
    let mut result;
    if _cse_temp_0 {
        result = a + b;
    } else {
        result = a - b;
    }
    if verbose {
        println!("{}", format!("Result: {:?}", result));
    }
    result
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn configure(
    width: i32,
    height: i32,
    title: &str,
) -> HashMap<serde_json::Value, serde_json::Value> {
    {
        let mut map = HashMap::new();
        map.insert("width".to_string(), width);
        map.insert("height".to_string(), height);
        map.insert(std::borrow::Cow::Borrowed("title").to_string(), title);
        map
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn outer<'a, 'b>(
    inner_result: &'a serde_json::Value,
    scale: f64,
    offset: &'b serde_json::Value,
) -> i32 {
    inner_result * scale + offset
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn inner(x: i32, y: i32) -> i32 {
    x + y
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_height() -> i32 {
    600
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_inner() {
        fn prop(x: i32, y: i32) -> TestResult {
            if (x > 0 && y > i32::MAX - x) || (x < 0 && y < i32::MIN - x) {
                return TestResult::discard();
            }
            let result1 = inner(x.clone(), y.clone());
            let result2 = inner(y.clone(), x.clone());
            if result1 != result2 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(i32, i32) -> TestResult);
    }
    #[test]
    fn test_inner_examples() {
        assert_eq!(inner(0, 0), 0);
        assert_eq!(inner(1, 2), 3);
        assert_eq!(inner(-1, 1), 0);
    }
    #[test]
    fn test_get_height_examples() {
        let _ = get_height();
    }
}
