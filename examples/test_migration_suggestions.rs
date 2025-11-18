use serde_json;
#[doc = "Pattern: accumulator - should suggest iterator methods."]
#[doc = " Depyler: verified panic-free"]
pub fn accumulator_pattern(items: &serde_json::Value) -> Vec<serde_json::Value> {
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
pub fn error_with_none(value: serde_json::Value) {
    if !validate(value) {
        return;
    }
    let processed = process_data(value);
    if processed.is_none() {
        return;
    }
    ()
}
#[doc = "Pattern: mutating parameters - should suggest ownership patterns."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn mutating_parameter(mut data: serde_json::Value) -> i32 {
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
            value * 2
        } else {
            value.to_string()
        }
    }
}
#[doc = "Pattern: string concatenation - should suggest efficient methods."]
#[doc = " Depyler: verified panic-free"]
pub fn inefficient_string_building(items: &serde_json::Value) -> String {
    let mut result = "";
    for item in items.iter().cloned() {
        result = format!("{}{}", format!("{}{}", result, item.to_string()), ", ");
    }
    result
}
#[doc = "Pattern: range(len()) - should suggest enumerate."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn enumerate_pattern(items: &serde_json::Value) {
    for _i in 0..items.len() as i32 {
        println!(
            "{}",
            format!(
                "{:?}: {:?}",
                i,
                items.get(i as usize).cloned().unwrap_or_default()
            )
        );
    }
}
#[doc = "Pattern: filter + map in loop - should suggest filter_map."]
#[doc = " Depyler: verified panic-free"]
pub fn filter_map_pattern(data: &mut serde_json::Value) -> Vec<serde_json::Value> {
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
    while true {
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
pub fn none_checking_pattern(optional_value: serde_json::Value) -> i32 {
    if optional_value.is_some() {
        process(optional_value)
    } else {
        default_value()
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate(x: &serde_json::Value) -> bool {
    x > 0
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_data(x: &serde_json::Value) -> i32 {
    x * 2
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process(x: serde_json::Value) -> i32 {
    x
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
