#[doc = "Numeric operations suggest int/float types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_numbers(a: i32, b: i32) -> i32 {
    let mut result = a + b;
    let _cse_temp_0 = result * 2;
    result = _cse_temp_0;
    result
}
#[doc = "String methods suggest str type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn handle_text(message: &str) -> String {
    let formatted = message.to_uppercase();
    formatted.to_string()
}
#[doc = "List operations suggest list type."]
#[doc = " Depyler: verified panic-free"]
pub fn work_with_list(data: &mut Vec<i32>) -> i32 {
    data.push(100);
    let mut total = 0;
    for item in data.iter().cloned() {
        total = total + item;
    }
    total
}
#[doc = "Boolean context suggests bool type."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn check_condition(flag: bool) -> i32 {
    if flag {
        1
    } else {
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_process_numbers_examples() {
        assert_eq!(process_numbers(0, 0), 0);
        assert_eq!(process_numbers(1, 2), 3);
        assert_eq!(process_numbers(-1, 1), 0);
    }
    #[test]
    fn test_work_with_list_examples() {
        assert_eq!(work_with_list(&vec![]), 0);
        assert_eq!(work_with_list(&vec![1]), 1);
        assert_eq!(work_with_list(&vec![1, 2, 3]), 3);
    }
}
