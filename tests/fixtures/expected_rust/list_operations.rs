// Test Case 11: Sum all elements in a list
pub fn sum_list(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for num in numbers {
        total += num;
    }
    total
}

// Test Case 12: Find maximum in list
pub fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }
    let mut max_val = numbers[0];
    for &num in numbers {
        if num > max_val {
            max_val = num;
        }
    }
    Some(max_val)
}

// Test Case 13: Count elements
pub fn count_elements(numbers: &[i32]) -> usize {
    numbers.len()
}

// Test Case 14: Filter positive numbers
pub fn filter_positive(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &num in numbers {
        if num > 0 {
            result.push(num);
        }
    }
    result
}

// Test Case 15: Find element at index
pub fn get_element(numbers: &[i32], index: usize) -> Option<i32> {
    if index < numbers.len() {
        Some(numbers[index])
    } else {
        None
    }
}

// Test Case 16: Reverse list
pub fn reverse_list(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for i in (0..numbers.len()).rev() {
        result.push(numbers[i]);
    }
    result
}

// Test Case 17: List contains element
pub fn contains_element(numbers: &[i32], target: i32) -> bool {
    for &num in numbers {
        if num == target {
            return true;
        }
    }
    false
}

// Test Case 18: First element
pub fn first_element(numbers: &[i32]) -> Option<i32> {
    if !numbers.is_empty() {
        Some(numbers[0])
    } else {
        None
    }
}

// Test Case 19: Last element
pub fn last_element(numbers: &[i32]) -> Option<i32> {
    if !numbers.is_empty() {
        Some(numbers[numbers.len() - 1])
    } else {
        None
    }
}

// Test Case 20: Average of numbers
pub fn average_numbers(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        0.0
    } else {
        let sum: i32 = numbers.iter().sum();
        sum as f64 / numbers.len() as f64
    }
}