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
#[doc = "Classic quicksort algorithm implementation"]
#[doc = " Depyler: proven to terminate"]
pub fn quicksort(arr: Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = arr.len() as i32;
    let _cse_temp_1 = _cse_temp_0 <= 1;
    if _cse_temp_1 {
        return Ok(arr);
    }
    let pivot = {
        let base = &arr;
        let idx: i32 = {
            let a = arr.len() as i32;
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
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    };
    let left = arr
        .as_slice()
        .iter()
        .copied()
        .filter(|&x| x < pivot)
        .map(|x| x)
        .collect::<Vec<_>>();
    let middle = arr
        .as_slice()
        .iter()
        .copied()
        .filter(|&x| x == pivot)
        .map(|x| x)
        .collect::<Vec<_>>();
    let right = arr
        .as_slice()
        .iter()
        .copied()
        .filter(|&x| x > pivot)
        .map(|x| x)
        .collect::<Vec<_>>();
    Ok(quicksort(left)?
        .iter()
        .chain(middle.iter())
        .cloned()
        .collect::<Vec<_>>()
        + quicksort(right)?)
}
#[doc = "In-place partition for quicksort"]
#[doc = " Depyler: proven to terminate"]
pub fn partition(arr: &Vec<i32>, low: i32, high: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let pivot = arr.get(high as usize).cloned().unwrap_or_default();
    let mut i = low - 1;
    for j in low..high {
        if arr.get(j as usize).cloned().unwrap_or_default() <= pivot {
            i = i + 1;
            let _swap_temp = (
                arr.get(j as usize).cloned().unwrap_or_default(),
                arr.get(i as usize).cloned().unwrap_or_default(),
            );
            arr.insert(i, _swap_temp.0);
            arr.insert(j, _swap_temp.1);
        }
    }
    let _swap_temp = (arr.get(high as usize).cloned().unwrap_or_default(), {
        let base = &arr;
        let idx: i32 = i + 1;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    });
    arr.insert(i + 1, _swap_temp.0);
    arr.insert(high, _swap_temp.1);
    Ok(i + 1)
}
#[doc = "In-place quicksort implementation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn quicksort_inplace(
    arr: Vec<i32>,
    low: i32,
    high: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let _cse_temp_0 = low < high;
    if _cse_temp_0 {
        let pi = partition(&arr, low, high)?;
        quicksort_inplace(arr, low, pi - 1);
        quicksort_inplace(arr, pi + 1, high);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn quickcheck_quicksort() {
        fn prop(arr: Vec<i32>) -> TestResult {
            let input_len = arr.len();
            let result = quicksort(&arr);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = quicksort(&arr);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = arr.clone();
            input_sorted.sort();
            let mut result = quicksort(&arr);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<i32>) -> TestResult);
    }
    #[test]
    fn test_quicksort_examples() {
        assert_eq!(quicksort(vec![]), vec![]);
        assert_eq!(quicksort(vec![1]), vec![1]);
    }
}
