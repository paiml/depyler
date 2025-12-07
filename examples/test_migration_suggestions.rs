use serde_json;
#[doc = "Pattern: accumulator - should suggest iterator methods."]
#[doc = " Depyler: verified panic-free"]
pub fn accumulator_pattern(items: &Vec<i32>) -> Vec<serde_json::Value> {
    let mut result = vec![];
    for item in items.iter().cloned() {
        if item > 0 {
            result.push(item * 2);
        }
    }
    result
}
#[doc = "Pattern: returning None for errors - should suggest Result."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn error_with_none(value: String) -> Option<serde_json::Value> {
    if !validate(&value) {
        return None;
    }
    let processed = process_data(&value);
    if processed.is_none() {
        return None;
    }
    Some(processed)
}
#[doc = "Pattern: mutating parameters - should suggest ownership patterns."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn mutating_parameter(data: &mut Vec<serde_json::Value>) -> Vec<String> {
    data.push(42);
    data.sort();
    data
}
#[doc = "Pattern: runtime type checking - should suggest enums."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn type_checking_pattern(value: &str) -> String {
    if true {
        value.to_uppercase()
    } else {
        if true {
            value.repeat(2 as usize)
        } else {
            (value).to_string()
        }
    }
}
#[doc = "Pattern: string concatenation - should suggest efficient methods."]
#[doc = " Depyler: verified panic-free"]
pub fn inefficient_string_building(items: &serde_json::Value) -> String {
    let mut result = "".to_string();
    for item in items.iter().cloned() {
        result = format!("{}{}", format!("{}{}", result, (item).to_string()), ", ");
    }
    result.to_string()
}
#[doc = "Pattern: range(len()) - should suggest enumerate."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn enumerate_pattern(items: &Vec<serde_json::Value>) {
    for i in 0..items.len() as i32 {
        println!(
            "{}",
            format!(
                "{:?}: {}",
                i,
                items.get(i as usize).cloned().unwrap_or_default()
            )
        );
    }
}
#[doc = "Pattern: filter + map in loop - should suggest filter_map."]
#[doc = " Depyler: verified panic-free"]
pub fn filter_map_pattern(data: &Vec<i32>) -> Vec<serde_json::Value> {
    let mut output = vec![];
    for x in data.iter().cloned() {
        if x > 0 {
            output.push(x * x);
        }
    }
    output
}
#[doc = "Pattern: while True - should suggest loop."]
#[doc = " Depyler: verified panic-free"]
pub fn while_true_pattern() -> i32 {
    let mut counter = 0;
    loop {
        counter = counter + 1;
        if counter > 10 {
            break;
        }
    }
    counter
}
#[doc = "Pattern: None checking - should suggest pattern matching."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn none_checking_pattern(optional_value: serde_json::Value) {
    if optional_value.is_some() {
        process(optional_value)
    } else {
        default_value()
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate(x: &str) -> bool {
    (x).as_str() > 0
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_data(x: i32) -> i32 {
    x * 2
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process(x: serde_json::Value) {
    let _ = x;
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn default_value() -> i32 {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_accumulator_pattern_examples() {
        assert_eq!(accumulator_pattern(vec![]), vec![]);
        assert_eq!(accumulator_pattern(vec![1]), vec![1]);
    }
    #[test]
    fn test_mutating_parameter_examples() {
        assert_eq!(mutating_parameter(vec![]), vec![]);
        assert_eq!(mutating_parameter(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_filter_map_pattern() {
        fn prop(data: Vec<i32>) -> TestResult {
            let input_len = data.len();
            let result = filter_map_pattern(&data);
            if result.len() != input_len {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_filter_map_pattern_examples() {
        assert_eq!(filter_map_pattern(vec![]), vec![]);
        assert_eq!(filter_map_pattern(vec![1]), vec![1]);
    }
    #[test]
    fn test_while_true_pattern_examples() {
        let _ = while_true_pattern();
    }
    #[test]
    fn quickcheck_process() {
        fn prop(x: ()) -> TestResult {
            let result = process(x.clone());
            if result != x {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(()) -> TestResult);
    }
    #[test]
    fn test_default_value_examples() {
        let _ = default_value();
    }
}
