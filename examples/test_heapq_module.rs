#[doc = "// TODO: Map Python module 'heapq'"]
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
#[doc = "Test basic heap push and pop operations"]
pub fn test_heap_push_pop() -> Result<Vec<i32>, IndexError> {
    let mut heap: Vec<i32> = vec![];
    let values: Vec<i32> = vec![5, 3, 7, 1, 9, 2];
    for val in values.iter().cloned() {
        heap.push(val);
    }
    for i in 0..heap.len() as i32 {
        for j in i + 1..heap.len() as i32 {
            if heap.get(j as usize).cloned().unwrap_or_default()
                < heap.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: i32 = heap.get(i as usize).cloned().unwrap_or_default();
                heap.insert(
                    (i) as usize,
                    heap.get(j as usize).cloned().unwrap_or_default(),
                );
                heap.insert((j) as usize, temp);
            }
        }
    }
    Ok(heap)
}
#[doc = "Test converting list to heap"]
#[doc = " Depyler: proven to terminate"]
pub fn test_heapify() -> Result<Vec<i32>, IndexError> {
    let data: Vec<i32> = vec![5, 3, 7, 1, 9, 2, 4];
    let mut heap: Vec<i32> = data.clone();
    for i in 0..heap.len() as i32 {
        for j in i + 1..heap.len() as i32 {
            if heap.get(j as usize).cloned().unwrap_or_default()
                < heap.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: i32 = heap.get(i as usize).cloned().unwrap_or_default();
                heap.insert(
                    (i) as usize,
                    heap.get(j as usize).cloned().unwrap_or_default(),
                );
                heap.insert((j) as usize, temp);
            }
        }
    }
    Ok(heap)
}
#[doc = "Test popping minimum element"]
#[doc = " Depyler: proven to terminate"]
pub fn test_heap_pop_min() -> Result<i32, IndexError> {
    let mut heap: Vec<i32> = vec![1, 2, 3, 4, 5];
    let _cse_temp_0 = heap.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        let min_val: i32 = heap.get(0usize).cloned().unwrap_or_default();
        let mut new_heap: Vec<i32> = vec![];
        for i in 1..heap.len() as i32 {
            new_heap.push(heap.get(i as usize).cloned().unwrap_or_default());
        }
        Ok(min_val)
    } else {
        Ok(-1)
    }
}
#[doc = "Test peeking at minimum without removing"]
#[doc = " Depyler: proven to terminate"]
pub fn test_heap_peek() -> Result<i32, IndexError> {
    let mut heap: Vec<i32> = vec![1, 2, 3, 4, 5];
    let _cse_temp_0 = heap.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        Ok(heap.get(0usize).cloned().unwrap_or_default())
    } else {
        Ok(-1)
    }
}
#[doc = "Test finding n smallest elements"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nsmallest(data: &Vec<i32>, n: i32) -> Result<Vec<i32>, IndexError> {
    let mut sorted_data: Vec<i32> = data.clone();
    for i in 0..sorted_data.len() as i32 {
        for j in i + 1..sorted_data.len() as i32 {
            if sorted_data.get(j as usize).cloned().unwrap_or_default()
                < sorted_data.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: i32 = sorted_data.get(i as usize).cloned().unwrap_or_default();
                sorted_data.insert(
                    (i) as usize,
                    sorted_data.get(j as usize).cloned().unwrap_or_default(),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    let mut result: Vec<i32> = vec![];
    for i in 0..std::cmp::min(n, sorted_data.len() as i32) {
        result.push(sorted_data.get(i as usize).cloned().unwrap_or_default());
    }
    Ok(result)
}
#[doc = "Test finding n largest elements"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nlargest(data: &Vec<i32>, n: i32) -> Result<Vec<i32>, IndexError> {
    let mut sorted_data: Vec<i32> = data.clone();
    for i in 0..sorted_data.len() as i32 {
        for j in i + 1..sorted_data.len() as i32 {
            if sorted_data.get(j as usize).cloned().unwrap_or_default()
                > sorted_data.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: i32 = sorted_data.get(i as usize).cloned().unwrap_or_default();
                sorted_data.insert(
                    (i) as usize,
                    sorted_data.get(j as usize).cloned().unwrap_or_default(),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    let mut result: Vec<i32> = vec![];
    for i in 0..std::cmp::min(n, sorted_data.len() as i32) {
        result.push(sorted_data.get(i as usize).cloned().unwrap_or_default());
    }
    Ok(result)
}
#[doc = "Manual heap insert operation"]
pub fn manual_heap_insert(
    heap: &mut Vec<i32>,
    value: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut new_heap: Vec<i32> = heap.clone();
    new_heap.push(value);
    let _cse_temp_0 = new_heap.len() as i32;
    let mut index: i32 = _cse_temp_0 - 1;
    while index > 0 {
        let parent: i32 = {
            let a = index - 1;
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
        if new_heap.get(index as usize).cloned().unwrap_or_default()
            < new_heap.get(parent as usize).cloned().unwrap_or_default()
        {
            let temp: i32 = new_heap.get(index as usize).cloned().unwrap_or_default();
            new_heap.insert(
                (index) as usize,
                new_heap.get(parent as usize).cloned().unwrap_or_default(),
            );
            new_heap.insert((parent) as usize, temp);
            index = parent;
        } else {
            break;
        }
    }
    Ok(new_heap)
}
#[doc = "Manual heap extract minimum"]
pub fn manual_heap_extract_min(heap: &mut Vec<i32>) -> Result<(), IndexError> {
    let _cse_temp_0 = heap.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok((-1, vec![]));
    }
    let min_val: i32 = heap.get(0usize).cloned().unwrap_or_default();
    let _cse_temp_2 = _cse_temp_0 == 1;
    if _cse_temp_2 {
        return Ok((min_val, vec![]));
    }
    let mut new_heap: Vec<i32> = vec![{
        let base = &heap;
        let idx: i32 = (heap.len() as i32).saturating_sub(1);
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    }];
    for i in 1..(heap.len() as i32).saturating_sub(1) {
        new_heap.push(heap.get(i as usize).cloned().unwrap_or_default());
    }
    let mut index: i32 = 0;
    while true {
        let left: i32 = 2 * index + 1;
        let right: i32 = 2 * index + 2;
        let mut smallest: i32 = index;
        if (left < new_heap.len() as i32)
            && (new_heap.get(left as usize).cloned().unwrap_or_default()
                < new_heap.get(smallest as usize).cloned().unwrap_or_default())
        {
            smallest = left;
        }
        if (right < new_heap.len() as i32)
            && (new_heap.get(right as usize).cloned().unwrap_or_default()
                < new_heap.get(smallest as usize).cloned().unwrap_or_default())
        {
            smallest = right;
        }
        if smallest != index {
            let temp: i32 = new_heap.get(index as usize).cloned().unwrap_or_default();
            new_heap.insert(
                (index) as usize,
                new_heap.get(smallest as usize).cloned().unwrap_or_default(),
            );
            new_heap.insert((smallest) as usize, temp);
            index = smallest;
        } else {
            break;
        }
    }
    Ok((min_val, new_heap))
}
#[doc = "Simulate priority queue using heap"]
pub fn priority_queue_simulation() -> Result<Vec<i32>, IndexError> {
    let tasks: Vec<()> = vec![(3, "low"), (1, "high"), (2, "medium")];
    let mut sorted_tasks: Vec<()> = tasks.clone();
    for i in 0..sorted_tasks.len() as i32 {
        for j in i + 1..sorted_tasks.len() as i32 {
            if sorted_tasks
                .get(j as usize)
                .cloned()
                .unwrap_or_default()
                .get(0usize)
                .cloned()
                .unwrap_or_default()
                < sorted_tasks
                    .get(i as usize)
                    .cloned()
                    .unwrap_or_default()
                    .get(0usize)
                    .cloned()
                    .unwrap_or_default()
            {
                let temp: () = sorted_tasks.get(i as usize).cloned().unwrap_or_default();
                sorted_tasks.insert(
                    (i) as usize,
                    sorted_tasks.get(j as usize).cloned().unwrap_or_default(),
                );
                sorted_tasks.insert((j) as usize, temp);
            }
        }
    }
    let mut priorities: Vec<i32> = vec![];
    for task in sorted_tasks.iter().cloned() {
        priorities.push(task.get(0usize).cloned().unwrap_or_default());
    }
    Ok(priorities)
}
#[doc = "Merge multiple sorted lists using heap concept"]
pub fn merge_sorted_lists(lists: &Vec<Vec<i32>>) -> Result<Vec<i32>, IndexError> {
    let mut result: Vec<i32> = vec![];
    for lst in lists.iter().cloned() {
        for item in lst.iter().cloned() {
            result.push(item);
        }
    }
    for i in 0..result.len() as i32 {
        for j in i + 1..result.len() as i32 {
            if result.get(j as usize).cloned().unwrap_or_default()
                < result.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: i32 = result.get(i as usize).cloned().unwrap_or_default();
                result.insert(
                    (i) as usize,
                    result.get(j as usize).cloned().unwrap_or_default(),
                );
                result.insert((j) as usize, temp);
            }
        }
    }
    Ok(result)
}
#[doc = "Find kth smallest element"]
#[doc = " Depyler: proven to terminate"]
pub fn find_kth_smallest(data: &Vec<i32>, k: i32) -> Result<i32, IndexError> {
    let mut sorted_data: Vec<i32> = data.clone();
    for i in 0..sorted_data.len() as i32 {
        for j in i + 1..sorted_data.len() as i32 {
            if sorted_data.get(j as usize).cloned().unwrap_or_default()
                < sorted_data.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: i32 = sorted_data.get(i as usize).cloned().unwrap_or_default();
                sorted_data.insert(
                    (i) as usize,
                    sorted_data.get(j as usize).cloned().unwrap_or_default(),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    let _cse_temp_0 = k > 0;
    let _cse_temp_1 = sorted_data.len() as i32;
    let _cse_temp_2 = k <= _cse_temp_1;
    let _cse_temp_3 = (_cse_temp_0) && (_cse_temp_2);
    if _cse_temp_3 {
        Ok({
            let base = &sorted_data;
            let idx: i32 = k - 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        })
    } else {
        Ok(-1)
    }
}
#[doc = "Find median using two heaps concept"]
#[doc = " Depyler: proven to terminate"]
pub fn find_median_using_heaps(data: &Vec<i32>) -> Result<f64, Box<dyn std::error::Error>> {
    let mut sorted_data: Vec<i32> = data.clone();
    for i in 0..sorted_data.len() as i32 {
        for j in i + 1..sorted_data.len() as i32 {
            if sorted_data.get(j as usize).cloned().unwrap_or_default()
                < sorted_data.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: i32 = sorted_data.get(i as usize).cloned().unwrap_or_default();
                sorted_data.insert(
                    (i) as usize,
                    sorted_data.get(j as usize).cloned().unwrap_or_default(),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    let _cse_temp_0 = sorted_data.len() as i32;
    let n: i32 = _cse_temp_0;
    let _cse_temp_1 = n % 2;
    let _cse_temp_2 = _cse_temp_1 == 1;
    let mut median: f64;
    if _cse_temp_2 {
        let _cse_temp_3 = ({
            let base = &sorted_data;
            let idx: i32 = {
                let a = n;
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
        }) as f64;
        median = _cse_temp_3;
    } else {
        let mid1: i32 = {
            let base = &sorted_data;
            let idx: i32 = {
                let a = n;
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
            } - 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        };
        let mid2: i32 = {
            let base = &sorted_data;
            let idx: i32 = {
                let a = n;
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
        let _cse_temp_4 = (mid1 + mid2) as f64;
        let _cse_temp_5 = (_cse_temp_4 as f64) / (2.0 as f64);
        median = _cse_temp_5;
    }
    Ok(median)
}
#[doc = "Run all heapq module tests"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_heapq_features() -> Result<(), IndexError> {
    let mut heap: Vec<i32> = test_heap_push_pop();
    let heapified: Vec<i32> = test_heapify();
    let min_val: i32 = test_heap_pop_min();
    let peek_val: i32 = test_heap_peek();
    let data: Vec<i32> = vec![5, 2, 8, 1, 9, 3, 7];
    let smallest_3: Vec<i32> = test_nsmallest(&data, 3);
    let largest_3: Vec<i32> = test_nlargest(&data, 3);
    let mut h: Vec<i32> = vec![];
    h = manual_heap_insert(&h, 5);
    h = manual_heap_insert(&h, 3);
    h = manual_heap_insert(&h, 7);
    let extract_result: () = manual_heap_extract_min(&h);
    let extracted: i32 = extract_result.get(0usize).cloned().unwrap_or_default();
    let remaining: Vec<i32> = extract_result.get(1usize).cloned().unwrap_or_default();
    let mut priorities: Vec<i32> = priority_queue_simulation();
    let lists: Vec<Vec<i32>> = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
    let merged: Vec<i32> = merge_sorted_lists(&lists);
    let sample: Vec<i32> = vec![7, 10, 4, 3, 20, 15];
    let kth: i32 = find_kth_smallest(&sample, 3);
    let median_data: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut median: f64 = find_median_using_heaps(&median_data);
    println!("{}", "All heapq module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_heap_pop_min_examples() {
        let _ = test_heap_pop_min();
    }
    #[test]
    fn test_test_heap_peek_examples() {
        let _ = test_heap_peek();
    }
    #[test]
    fn quickcheck_merge_sorted_lists() {
        fn prop(lists: Vec<Vec<i32>>) -> TestResult {
            let input_len = lists.len();
            let result = merge_sorted_lists(&lists);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = merge_sorted_lists(&lists);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = lists.clone();
            input_sorted.sort();
            let mut result = merge_sorted_lists(&lists);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<Vec<i32>>) -> TestResult);
    }
    #[test]
    fn test_merge_sorted_lists_examples() {
        assert_eq!(merge_sorted_lists(vec![]), vec![]);
        assert_eq!(merge_sorted_lists(vec![1]), vec![1]);
    }
}
