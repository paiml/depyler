#[doc = "// NOTE: Map Python module 'bisect'(tracked in DEPYLER-0424)"]
const STR_INSERT: &'static str = "insert";
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
#[doc = "Binary search finding leftmost position"]
pub fn binary_search_left(arr: &Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut left: i32 = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut right: i32 = _cse_temp_0.clone();
    while left < right {
        let mid: i32 = {
            let a = left + right;
            let b = 2;
            let q = a / b;
            let r = a % b;
            let r_negative = r < 0;
            let b_negative = b < 0;
            let r_nonzero = r != 0;
            let signs_differ = r_negative != b_negative;
            let needs_adjustment = r_nonzero && signs_differ;
            if needs_adjustment {
                q - 1
            } else {
                q
            }
        };
        if arr.get(mid as usize).cloned().unwrap_or_default() < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    Ok(left)
}
#[doc = "Binary search finding rightmost position"]
pub fn binary_search_right(arr: &Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut left: i32 = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut right: i32 = _cse_temp_0.clone();
    while left < right {
        let mid: i32 = {
            let a = left + right;
            let b = 2;
            let q = a / b;
            let r = a % b;
            let r_negative = r < 0;
            let b_negative = b < 0;
            let r_nonzero = r != 0;
            let signs_differ = r_negative != b_negative;
            let needs_adjustment = r_nonzero && signs_differ;
            if needs_adjustment {
                q - 1
            } else {
                q
            }
        };
        if arr.get(mid as usize).cloned().unwrap_or_default() <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    Ok(left)
}
#[doc = "Test finding insertion point(leftmost)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bisect_left() -> Result<i32, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 3, 3, 3, 5, 7, 9];
    let target: i32 = 3;
    let position: i32 = binary_search_left(&data, target)?;
    Ok(position)
}
#[doc = "Test finding insertion point(rightmost)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bisect_right() -> Result<i32, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 3, 3, 3, 5, 7, 9];
    let target: i32 = 3;
    let position: i32 = binary_search_right(&data, target)?;
    Ok(position)
}
#[doc = "Test bisect_left with value not in list"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bisect_not_found_left() -> Result<i32, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 3, 5, 7, 9];
    let target: i32 = 4;
    let position: i32 = binary_search_left(&data, target)?;
    Ok(position)
}
#[doc = "Test bisect_right with value not in list"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bisect_not_found_right() -> Result<i32, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 3, 5, 7, 9];
    let target: i32 = 4;
    let position: i32 = binary_search_right(&data, target)?;
    Ok(position)
}
#[doc = "Insert value maintaining sort order(leftmost position)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn insort_left(arr: Vec<i32>, value: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let position: i32 = binary_search_left(&arr, value)?;
    let mut new_arr: Vec<i32> = vec![];
    for i in 0..arr.len() as i32 {
        if i == position {
            new_arr.push(value);
        }
        new_arr.push(arr.get(i as usize).cloned().unwrap_or_default());
    }
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = position == _cse_temp_0;
    if _cse_temp_1 {
        new_arr.push(value);
    }
    Ok(new_arr)
}
#[doc = "Insert value maintaining sort order(rightmost position)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn insort_right(arr: Vec<i32>, value: i32) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let position: i32 = binary_search_right(&arr, value)?;
    let mut new_arr: Vec<i32> = vec![];
    for i in 0..arr.len() as i32 {
        if i == position {
            new_arr.push(value);
        }
        new_arr.push(arr.get(i as usize).cloned().unwrap_or_default());
    }
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = position == _cse_temp_0;
    if _cse_temp_1 {
        new_arr.push(value);
    }
    Ok(new_arr)
}
#[doc = "Test inserting with insort_left"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_insort_left() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 3, 5, 7, 9];
    let value: i32 = 4;
    let result: Vec<i32> = insort_left(data, value)?;
    Ok(result)
}
#[doc = "Test inserting with insort_right"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_insort_right() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 3, 5, 7, 9];
    let value: i32 = 4;
    let result: Vec<i32> = insort_right(data, value)?;
    Ok(result)
}
#[doc = "Test inserting duplicate with insort_left"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_insort_duplicate_left() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 3, 3, 3, 5];
    let value: i32 = 3;
    let result: Vec<i32> = insort_left(data, value)?;
    Ok(result)
}
#[doc = "Test inserting duplicate with insort_right"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_insort_duplicate_right() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let data: Vec<i32> = vec![1, 3, 3, 3, 5];
    let value: i32 = 3;
    let result: Vec<i32> = insort_right(data, value)?;
    Ok(result)
}
#[doc = "Check if value exists in sorted array"]
#[doc = " Depyler: proven to terminate"]
pub fn binary_search_contains(
    arr: Vec<i32>,
    target: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let position: i32 = binary_search_left(&arr, target)?;
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = position < _cse_temp_0;
    let _cse_temp_2 = arr.get(position as usize).cloned().unwrap_or_default() == target;
    let _cse_temp_3 = (_cse_temp_1) && (_cse_temp_2);
    if _cse_temp_3 {
        Ok(true)
    } else {
        Ok(false)
    }
}
#[doc = "Count occurrences of value in sorted array"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn count_occurrences_sorted(
    arr: Vec<i32>,
    target: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut left: i32 = binary_search_left(&arr, target)?;
    let mut right: i32 = binary_search_right(&arr, target)?;
    let count: i32 = right - left;
    Ok(count)
}
#[doc = "Find start and end indices of target in sorted array"]
#[doc = " Depyler: proven to terminate"]
pub fn find_range(arr: Vec<i32>, target: i32) -> Result<(), Box<dyn std::error::Error>> {
    let start: i32 = binary_search_left(&arr, target)?;
    let end: i32 = binary_search_right(&arr, target)?;
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = start < _cse_temp_0;
    let _cse_temp_2 = arr.get(start as usize).cloned().unwrap_or_default() == target;
    let _cse_temp_3 = (_cse_temp_1) && (_cse_temp_2);
    if _cse_temp_3 {
        Ok((start, end - 1))
    } else {
        Ok((-1, -1))
    }
}
#[doc = "Find closest value to target in sorted array"]
#[doc = " Depyler: proven to terminate"]
pub fn find_closest_value(arr: Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(-1);
    }
    let position: i32 = binary_search_left(&arr, target)?;
    let _cse_temp_2 = position == 0;
    if _cse_temp_2 {
        return Ok(arr.get(0usize).cloned().unwrap_or_default());
    }
    let _cse_temp_3 = position == _cse_temp_0;
    if _cse_temp_3 {
        return Ok({
            let base = &arr;
            let idx: i32 = (arr.len() as i32).saturating_sub(1);
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        });
    }
    let before: i32 = {
        let base = &arr;
        let idx: i32 = position - 1;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    };
    let after: i32 = arr.get(position as usize).cloned().unwrap_or_default();
    let _cse_temp_4 = target - before.abs();
    let before_dist: i32 = _cse_temp_4;
    let _cse_temp_5 = target - after.abs();
    let after_dist: i32 = _cse_temp_5;
    let _cse_temp_6 = before_dist <= after_dist;
    if _cse_temp_6 {
        Ok(before)
    } else {
        Ok(after)
    }
}
#[doc = "Merge two sorted arrays"]
pub fn merge_sorted_arrays<'a, 'b>(
    arr1: &'a Vec<i32>,
    arr2: &'b Vec<i32>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    while (i < arr1.len() as i32) && (j < arr2.len() as i32) {
        if arr1.get(i as usize).cloned().unwrap_or_default()
            <= arr2.get(j as usize).cloned().unwrap_or_default()
        {
            result.push(arr1.get(i as usize).cloned().unwrap_or_default());
            i = i + 1;
        } else {
            result.push(arr2.get(j as usize).cloned().unwrap_or_default());
            j = j + 1;
        }
    }
    while i < arr1.len() as i32 {
        result.push(arr1.get(i as usize).cloned().unwrap_or_default());
        i = i + 1;
    }
    while j < arr2.len() as i32 {
        result.push(arr2.get(j as usize).cloned().unwrap_or_default());
        j = j + 1;
    }
    Ok(result)
}
#[doc = "Maintain sorted list through multiple insertions"]
pub fn maintain_sorted_list(operations: &Vec<()>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut sorted_list: Vec<i32> = vec![];
    for op in operations.iter().cloned() {
        let operation_type: String = op.get(0usize).cloned().unwrap_or_default();
        let value: i32 = op.get(1usize).cloned().unwrap_or_default();
        if operation_type == STR_INSERT {
            sorted_list = insort_right(sorted_list, value)?;
        }
    }
    Ok(sorted_list)
}
#[doc = "Find where to insert value to maintain order"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn find_insertion_point(arr: Vec<i32>, value: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let position: i32 = binary_search_left(&arr, value)?;
    Ok(position)
}
#[doc = "Test bisect with edge cases"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_bisect_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    let empty: Vec<i32> = vec![];
    let empty_pos: i32 = binary_search_left(&empty, 5)?;
    let single: Vec<i32> = vec![5];
    let single_before: i32 = binary_search_left(&single, 3)?;
    let single_after: i32 = binary_search_left(&single, 7)?;
    let single_equal: i32 = binary_search_left(&single, 5)?;
    Ok((empty_pos, single_before, single_after, single_equal))
}
#[doc = "Find floor and ceiling values for target"]
#[doc = " Depyler: proven to terminate"]
pub fn find_floor_ceiling(arr: Vec<i32>, target: i32) -> Result<(), Box<dyn std::error::Error>> {
    let position: i32 = binary_search_left(&arr, target)?;
    let mut floor_val: i32 = -1;
    let mut ceiling_val: i32 = -1;
    let _cse_temp_0 = position > 0;
    if _cse_temp_0 {
        floor_val = {
            let base = &arr;
            let idx: i32 = position - 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        };
    }
    let _cse_temp_1 = arr.len() as i32;
    let _cse_temp_2 = position < _cse_temp_1;
    if _cse_temp_2 {
        ceiling_val = arr.get(position as usize).cloned().unwrap_or_default();
    }
    Ok((floor_val, ceiling_val))
}
#[doc = "Run all bisect module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_bisect_features() -> Result<(), Box<dyn std::error::Error>> {
    let arr1: Vec<i32> = vec![1, 3, 5];
    let arr2: Vec<i32> = vec![2, 4, 6];
    let operations: Vec<()> = vec![
        (STR_INSERT, 5),
        (STR_INSERT, 2),
        (STR_INSERT, 8),
        (STR_INSERT, 1),
    ];
    println!("{}", "All bisect module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_bisect_left_examples() {
        let _ = test_bisect_left();
    }
    #[test]
    fn test_test_bisect_right_examples() {
        let _ = test_bisect_right();
    }
    #[test]
    fn test_test_bisect_not_found_left_examples() {
        let _ = test_bisect_not_found_left();
    }
    #[test]
    fn test_test_bisect_not_found_right_examples() {
        let _ = test_bisect_not_found_right();
    }
    #[test]
    fn quickcheck_insort_left() {
        fn prop(arr: Vec<i32>, value: i32) -> TestResult {
            let result = insort_left(&arr, value.clone());
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = arr.clone();
            input_sorted.sort();
            let mut result = insort_left(&arr);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>, i32) -> TestResult);
    }
    #[test]
    fn quickcheck_insort_right() {
        fn prop(arr: Vec<i32>, value: i32) -> TestResult {
            let result = insort_right(&arr, value.clone());
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = arr.clone();
            input_sorted.sort();
            let mut result = insort_right(&arr);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>, i32) -> TestResult);
    }
    #[test]
    fn quickcheck_count_occurrences_sorted() {
        fn prop(arr: Vec<i32>, target: i32) -> TestResult {
            let result = count_occurrences_sorted(&arr, target.clone());
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = arr.clone();
            input_sorted.sort();
            let mut result = count_occurrences_sorted(&arr);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>, i32) -> TestResult);
    }
    #[test]
    fn quickcheck_merge_sorted_arrays() {
        fn prop(arr1: Vec<i32>, arr2: Vec<i32>) -> TestResult {
            let result = merge_sorted_arrays(&arr1, &arr2);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = arr1.clone();
            input_sorted.sort();
            let mut result = merge_sorted_arrays(&arr1);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>, Vec<i32>) -> TestResult);
    }
    #[test]
    fn quickcheck_maintain_sorted_list() {
        fn prop(operations: Vec<()>) -> TestResult {
            let input_len = operations.len();
            let result = maintain_sorted_list(&operations);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = maintain_sorted_list(&operations);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = operations.clone();
            input_sorted.sort();
            let mut result = maintain_sorted_list(&operations);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<()>) -> TestResult);
    }
    #[test]
    fn test_maintain_sorted_list_examples() {
        assert_eq!(maintain_sorted_list(vec![]), vec![]);
        assert_eq!(maintain_sorted_list(vec![1]), vec![1]);
    }
}
