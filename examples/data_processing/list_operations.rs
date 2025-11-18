#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Filter out even numbers from list"]
pub fn filter_even_numbers(numbers: &Vec<i32>) -> Result<Vec<i32>, ZeroDivisionError> {
    let mut result: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        if num % 2 == 0 {
            result.push(num);
        }
    }
    Ok(result)
}
#[doc = "Find duplicate numbers in list"]
#[doc = " Depyler: verified panic-free"]
pub fn find_duplicates(numbers: &Vec<i32>) -> Vec<i32> {
    let mut seen: Vec<i32> = vec![];
    let mut duplicates: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        if seen.contains_key(&num) {
            if !duplicates.contains_key(&num) {
                duplicates.push(num);
            }
        } else {
            seen.push(num);
        }
    }
    duplicates
}
#[doc = "Merge two sorted lists into one sorted list"]
pub fn merge_sorted_lists<'a, 'b>(
    list1: &'a Vec<i32>,
    list2: &'b Vec<i32>,
) -> Result<Vec<i32>, IndexError> {
    let mut result: Vec<i32> = vec![];
    let mut i = 0;
    let mut j = 0;
    while (i < list1.len() as i32) && (j < list2.len() as i32) {
        if list1.get(i as usize).cloned().unwrap_or_default()
            <= list2.get(j as usize).cloned().unwrap_or_default()
        {
            result.push(list1.get(i as usize).cloned().unwrap_or_default());
            i = i + 1;
        } else {
            result.push(list2.get(j as usize).cloned().unwrap_or_default());
            j = j + 1;
        }
    }
    while i < list1.len() as i32 {
        result.push(list1.get(i as usize).cloned().unwrap_or_default());
        i = i + 1;
    }
    while j < list2.len() as i32 {
        result.push(list2.get(j as usize).cloned().unwrap_or_default());
        j = j + 1;
    }
    Ok(result)
}
#[doc = "Calculate running sum of list"]
#[doc = " Depyler: verified panic-free"]
pub fn calculate_running_sum(numbers: &Vec<i32>) -> Vec<i32> {
    if numbers.is_empty() {
        return vec![];
    }
    let mut result: Vec<i32> = vec![];
    let mut running_total = 0;
    for num in numbers.iter().cloned() {
        running_total = running_total + num;
        result.push(running_total);
    }
    result
}
#[doc = "Rotate list left by specified positions"]
#[doc = " Depyler: proven to terminate"]
pub fn rotate_list_left(
    numbers: Vec<i32>,
    mut positions: i32,
) -> Result<Vec<i32>, ZeroDivisionError> {
    let _cse_temp_0 = positions <= 0;
    let _cse_temp_1 = (numbers.is_empty()) || (_cse_temp_0);
    if _cse_temp_1 {
        return Ok(numbers);
    }
    let _cse_temp_2 = numbers.len() as i32;
    let length = _cse_temp_2;
    let _cse_temp_3 = positions % length;
    positions = _cse_temp_3;
    let mut result: Vec<i32> = vec![];
    for i in positions..length {
        result.push(numbers.get(i as usize).cloned().unwrap_or_default());
    }
    for i in 0..positions {
        result.push(numbers.get(i as usize).cloned().unwrap_or_default());
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_filter_even_numbers_examples() {
        assert_eq!(filter_even_numbers(vec![]), vec![]);
        assert_eq!(filter_even_numbers(vec![1]), vec![1]);
    }
    #[test]
    fn test_find_duplicates_examples() {
        assert_eq!(find_duplicates(vec![]), vec![]);
        assert_eq!(find_duplicates(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_merge_sorted_lists() {
        fn prop(list1: Vec<i32>, list2: Vec<i32>) -> TestResult {
            let result = merge_sorted_lists(&list1, &list2);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = list1.clone();
            input_sorted.sort();
            let mut result = merge_sorted_lists(&list1);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>, Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_calculate_running_sum_examples() {
        assert_eq!(calculate_running_sum(vec![]), vec![]);
        assert_eq!(calculate_running_sum(vec![1]), vec![1]);
    }
}
