use serde_json;
#[doc = "A simple timing decorator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn timing_decorator(func: serde_json::Value) -> Box<dyn Fn(()) -> ()> {
    let mut wrapper;
    wrapper = |args: ()| {
        let result = func(args);
        return result;
    };
    Box::new(wrapper)
}
#[doc = "A simple logging decorator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn logging_decorator(func: serde_json::Value) -> Box<dyn Fn(()) -> ()> {
    let mut wrapper;
    wrapper = |args: ()| {
        let result = func(args);
        return result;
    };
    Box::new(wrapper)
}
#[doc = "A function that would be slow"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn slow_function(n: i32) -> i32 {
    let mut total = 0;
    for i in 0..n {
        total = total + i;
    }
    total
}
#[doc = "A function with stacked decorators"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn important_calculation(x: i32, y: i32) -> i32 {
    x * y + x + y
}
#[doc = "Decorator that repeats function call"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn repeat(times: i32) -> Box<dyn Fn(()) -> ()> {
    let mut decorator;
    let mut wrapper;
    decorator = |func: ()| {
        wrapper = |args: ()| {
            let result = None;
            for __sanitized in 0..times {
                result = func(args);
            }
            return result;
        };
        return wrapper;
    };
    Box::new(decorator)
}
#[doc = "Function that will be called 3 times"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
#[doc = "Test decorated functions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_decorators() -> i32 {
    let result1 = slow_function(100);
    let result2 = important_calculation(5, 10);
    let _result3 = greet("World".to_string());
    result1 + result2
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_slow_function_examples() {
        assert_eq!(slow_function(0), 0);
        assert_eq!(slow_function(1), 1);
        assert_eq!(slow_function(-1), -1);
    }
    #[test]
    fn test_important_calculation_examples() {
        assert_eq!(important_calculation(0, 0), 0);
        assert_eq!(important_calculation(1, 2), 3);
        assert_eq!(important_calculation(-1, 1), 0);
    }
    #[test]
    fn test_test_decorators_examples() {
        let _ = test_decorators();
    }
}
