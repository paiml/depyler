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
#[doc = "Binary search implementation - returns index or -1"]
pub fn binary_search(arr: &Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut left = 0;
    let _cse_temp_0 = arr.len() as i32;
    let mut right = _cse_temp_0 - 1;
    while left <= right {
        let mid = {
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
        if arr.get(mid as usize).cloned().unwrap_or_default() == target {
            return Ok(mid);
        } else {
            if arr.get(mid as usize).cloned().unwrap_or_default() < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    Ok(-1)
}
#[doc = "Linear search for comparison"]
#[doc = " Depyler: proven to terminate"]
pub fn linear_search(arr: &Vec<i32>, target: i32) -> Result<i32, Box<dyn std::error::Error>> {
    for i in 0..arr.len() as i32 {
        if arr.get(i as usize).cloned().unwrap_or_default() == target {
            return Ok(i);
        }
    }
    Ok(-1)
}
#[doc = "Find maximum element in array"]
pub fn find_maximum(arr: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    if arr.is_empty() {
        return Ok(0);
    }
    let mut max_val = arr.get(0usize).cloned().unwrap_or_default();
    for val in arr.iter().cloned() {
        if val > max_val {
            max_val = val;
        }
    }
    Ok(max_val)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_find_maximum_examples() {
        assert_eq!(find_maximum(&vec![]), 0);
        assert_eq!(find_maximum(&vec![1]), 1);
        assert_eq!(find_maximum(&vec![1, 2, 3]), 3);
    }
}
