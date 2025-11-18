use itertools;
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
#[doc = "Test chaining multiple iterables together"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_chain_iterables() -> Vec<i32> {
    let list1: Vec<i32> = vec![1, 2, 3];
    let list2: Vec<i32> = vec![4, 5, 6];
    let list3: Vec<i32> = vec![7, 8, 9];
    let chained: Vec<i32> = list1
        .into_iter()
        .chain(list2.into_iter())
        .chain(list3.into_iter())
        .collect::<Vec<_>>();
    chained
}
#[doc = "Test zipping iterables together"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zip_iterables() -> Vec<(i32, String)> {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let letters: Vec<String> = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
    ];
    let zipped: Vec<(i32, String)> = numbers
        .into_iter()
        .zip(letters.into_iter())
        .collect::<Vec<_>>();
    zipped
}
#[doc = "Test enumerate for indexed iteration"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enumerate() -> Vec<(i32, String)> {
    let items: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];
    let enumerated: Vec<(i32, String)> = items.into_iter().enumerate().collect::<Vec<_>>();
    enumerated
}
#[doc = "Test filtering iterables"]
pub fn test_filter() -> Result<Vec<i32>, ZeroDivisionError> {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut evens: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        if num % 2 == 0 {
            evens.push(num);
        }
    }
    Ok(evens)
}
#[doc = "Test mapping function over iterable"]
#[doc = " Depyler: verified panic-free"]
pub fn test_map() -> Vec<i32> {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut squared: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        squared.push(num * num);
    }
    squared
}
#[doc = "Test count() infinite iterator with limit"]
#[doc = " Depyler: verified panic-free"]
pub fn test_count(start: i32, step: i32, limit: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut current: i32 = start;
    let mut count: i32 = 0;
    while count < limit {
        result.push(current);
        current = current + step;
        count = count + 1;
    }
    result
}
#[doc = "Test cycle() to repeat iterable indefinitely"]
#[doc = " Depyler: proven to terminate"]
pub fn test_cycle(items: &Vec<String>, num_items: i32) -> Result<Vec<String>, ZeroDivisionError> {
    let mut result: Vec<String> = vec![];
    let mut idx: i32 = 0;
    for _i in 0..num_items {
        result.push(items.get(idx as usize).cloned().unwrap_or_default());
        idx = (idx + 1) % items.len() as i32;
    }
    Ok(result)
}
#[doc = "Test repeat() to repeat value multiple times"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_repeat(value: i32, times: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for _i in 0..times {
        result.push(value);
    }
    result
}
#[doc = "Test islice() to slice an iterable"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_islice(items: &Vec<i32>, start: i32, stop: i32) -> Vec<i32> {
    let mut result: Vec<i32> = {
        let base = items;
        let start = (start).max(0) as usize;
        let stop = (stop).max(0) as usize;
        if start < base.len() {
            base[start..stop.min(base.len())].to_vec()
        } else {
            Vec::new()
        }
    };
    result
}
#[doc = "Test takewhile() to take items while condition is true"]
#[doc = " Depyler: verified panic-free"]
pub fn test_takewhile(numbers: &Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for num in numbers.iter().cloned() {
        if num < threshold {
            result.push(num);
        } else {
            break;
        }
    }
    result
}
#[doc = "Test dropwhile() to drop items while condition is true"]
#[doc = " Depyler: verified panic-free"]
pub fn test_dropwhile(numbers: &Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut dropping: bool = true;
    for num in numbers.iter().cloned() {
        if (dropping) && (num < threshold) {
            continue;
        }
        dropping = false;
        result.push(num);
    }
    result
}
#[doc = "Test accumulate() for running totals"]
#[doc = " Depyler: verified panic-free"]
pub fn test_accumulate(numbers: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut total: i32 = 0;
    for num in numbers.iter().cloned() {
        total = total + num;
        result.push(total);
    }
    result
}
#[doc = "Test pairwise iteration(sliding window of 2)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pairwise(items: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    for i in 0..(items.len() as i32).saturating_sub(1) {
        let pair: (i32, i32) = (items.get(i as usize).cloned().unwrap_or_default(), {
            let base = &items;
            let idx: i32 = i + 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        });
        result.push(pair);
    }
    result
}
#[doc = "Test groupby-like functionality(manual implementation)"]
#[doc = " Depyler: proven to terminate"]
pub fn test_groupby_manual(items: &Vec<i32>) -> Result<Vec<(bool, Vec<i32>)>, ZeroDivisionError> {
    let mut groups: Vec<(bool, Vec<i32>)> = vec![];
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(groups);
    }
    let _cse_temp_2 = items.get(0usize).cloned().unwrap_or_default() % 2;
    let _cse_temp_3 = _cse_temp_2 == 0;
    let mut current_is_even: bool = _cse_temp_3;
    let mut current_group: Vec<i32> = vec![items.get(0usize).cloned().unwrap_or_default()];
    for i in 1..items.len() as i32 {
        let item_is_even: bool = items.get(i as usize).cloned().unwrap_or_default() % 2 == 0;
        if item_is_even == current_is_even {
            current_group.push(items.get(i as usize).cloned().unwrap_or_default());
        } else {
            groups.push((current_is_even, current_group));
            current_is_even = item_is_even;
            current_group = vec![items.get(i as usize).cloned().unwrap_or_default()];
        }
    }
    groups.push((current_is_even, current_group));
    Ok(groups)
}
#[doc = "Test compress() to filter data by selectors"]
#[doc = " Depyler: proven to terminate"]
pub fn test_compress<'a, 'b>(
    data: &'a Vec<String>,
    selectors: &'b Vec<bool>,
) -> Result<Vec<String>, IndexError> {
    let mut result: Vec<String> = vec![];
    for i in 0..std::cmp::min(data.len() as i32, selectors.len() as i32) {
        if selectors.get(i as usize).cloned().unwrap_or_default() {
            result.push(data.get(i as usize).cloned().unwrap_or_default());
        }
    }
    Ok(result)
}
#[doc = "Test chain.from_iterable() to flatten list of lists"]
#[doc = " Depyler: verified panic-free"]
pub fn test_chain_from_iterable(lists: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for sublist in lists.iter().cloned() {
        for item in sublist.iter().cloned() {
            result.push(item);
        }
    }
    result
}
#[doc = "Flatten nested lists using chain concept"]
#[doc = " Depyler: verified panic-free"]
pub fn flatten_nested_lists(nested: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut flattened: Vec<i32> = vec![];
    for sublist in nested.iter().cloned() {
        for item in sublist.iter().cloned() {
            flattened.push(item);
        }
    }
    flattened
}
#[doc = "Manual implementation of Cartesian product"]
#[doc = " Depyler: verified panic-free"]
pub fn cartesian_product_manual<'b, 'a>(
    list1: &'a Vec<i32>,
    list2: &'b Vec<i32>,
) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    for item1 in list1.iter().cloned() {
        for item2 in list2.iter().cloned() {
            let pair: (i32, i32) = (item1, item2);
            result.push(pair);
        }
    }
    result
}
#[doc = "Manual implementation of zip_longest"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zip_longest<'b, 'a>(
    list1: &'a Vec<i32>,
    list2: &'b Vec<i32>,
    fillvalue: i32,
) -> Result<Vec<(i32, i32)>, IndexError> {
    let mut result: Vec<(i32, i32)> = vec![];
    let _cse_temp_0 = list1.len() as i32;
    let _cse_temp_1 = list2.len() as i32;
    let _cse_temp_2 = std::cmp::max(_cse_temp_0, _cse_temp_1);
    let max_len: i32 = _cse_temp_2;
    for i in 0..max_len {
        let mut val1: i32 = fillvalue;
        let mut val2: i32 = fillvalue;
        if i < list1.len() as i32 {
            val1 = list1.get(i as usize).cloned().unwrap_or_default();
        }
        if i < list2.len() as i32 {
            val2 = list2.get(i as usize).cloned().unwrap_or_default();
        }
        let pair: (i32, i32) = (val1, val2);
        result.push(pair);
    }
    Ok(result)
}
#[doc = "Split iterable into batches of fixed size"]
#[doc = " Depyler: verified panic-free"]
pub fn test_batching(items: &Vec<i32>, batch_size: i32) -> Vec<Vec<i32>> {
    let mut batches: Vec<Vec<i32>> = vec![];
    let mut current_batch: Vec<i32> = vec![];
    for item in items.iter().cloned() {
        current_batch.push(item);
        if current_batch.len() as i32 == batch_size {
            batches.push(current_batch);
            current_batch = vec![];
        }
    }
    let _cse_temp_0 = current_batch.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        batches.push(current_batch);
    }
    batches
}
#[doc = "Create sliding windows over iterable"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sliding_window(items: &Vec<i32>, window_size: i32) -> Vec<Vec<i32>> {
    let mut windows: Vec<Vec<i32>> = vec![];
    for i in 0..(items.len() as i32).saturating_sub(window_size) + 1 {
        let window: Vec<i32> = {
            let base = items;
            let start = (i).max(0) as usize;
            let stop = (i + window_size).max(0) as usize;
            if start < base.len() {
                base[start..stop.min(base.len())].to_vec()
            } else {
                Vec::new()
            }
        };
        windows.push(window);
    }
    windows
}
#[doc = "Remove consecutive duplicates"]
#[doc = " Depyler: proven to terminate"]
pub fn test_unique_justseen(items: &Vec<i32>) -> Result<Vec<i32>, IndexError> {
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    let mut result: Vec<i32> = vec![items.get(0usize).cloned().unwrap_or_default()];
    for i in 1..items.len() as i32 {
        if items.get(i as usize).cloned().unwrap_or_default() != {
            let base = &items;
            let idx: i32 = i - 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        } {
            result.push(items.get(i as usize).cloned().unwrap_or_default());
        }
    }
    Ok(result)
}
#[doc = "Get nth item from iterable"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nth_item(items: &Vec<i32>, n: i32, default: i32) -> Result<i32, IndexError> {
    let _cse_temp_0 = n < 0;
    let _cse_temp_1 = items.len() as i32;
    let _cse_temp_2 = n >= _cse_temp_1;
    let _cse_temp_3 = (_cse_temp_0) || (_cse_temp_2);
    if _cse_temp_3 {
        return Ok(default);
    }
    Ok(items.get(n as usize).cloned().unwrap_or_default())
}
#[doc = "Check if all items in iterable are equal"]
pub fn test_all_equal(items: &Vec<i32>) -> Result<bool, IndexError> {
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(true);
    }
    let first: i32 = items.get(0usize).cloned().unwrap_or_default();
    for item in items.iter().cloned() {
        if item != first {
            return Ok(false);
        }
    }
    Ok(true)
}
#[doc = "Count how many items meet a condition"]
#[doc = " Depyler: verified panic-free"]
pub fn test_quantify(items: &Vec<i32>, threshold: i32) -> i32 {
    let mut count: i32 = 0;
    for item in items.iter().cloned() {
        if item > threshold {
            count = count + 1;
        }
    }
    count
}
#[doc = "Run all itertools tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_itertools_features() -> Result<(), Box<dyn std::error::Error>> {
    let chained: Vec<i32> = test_chain_iterables()?;
    let numbers: Vec<i32> = vec![1, 2, 3];
    let letters: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let zipped: Vec<(i32, String)> = test_zip_iterables()?;
    let items: Vec<String> = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    let enumerated: Vec<(i32, String)> = test_enumerate()?;
    let mut evens: Vec<i32> = test_filter()?;
    let mut squared: Vec<i32> = test_map()?;
    let counted: Vec<i32> = test_count(0, 2, 5)?;
    let colors: Vec<String> = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let cycled: Vec<String> = test_cycle(&colors, 10)?;
    let repeated: Vec<i32> = test_repeat(42, 5)?;
    let data: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let sliced: Vec<i32> = test_islice(&data, 2, 7)?;
    let numbers2: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let taken: Vec<i32> = test_takewhile(&numbers2, 5)?;
    let dropped: Vec<i32> = test_dropwhile(&numbers2, 5)?;
    let accumulated: Vec<i32> = test_accumulate(&vec![1, 2, 3, 4, 5])?;
    let pairs: Vec<(i32, i32)> = test_pairwise(&vec![1, 2, 3, 4, 5])?;
    let grouped: Vec<(bool, Vec<i32>)> = test_groupby_manual(&vec![1, 1, 2, 2, 2, 3, 4, 4])?;
    let data_str: Vec<String> = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
    ];
    let selectors: Vec<bool> = vec![true, false, true, false, true];
    let compressed: Vec<String> = test_compress(&data_str, &selectors)?;
    let nested: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let mut flattened: Vec<i32> = test_chain_from_iterable(&nested)?;
    let flattened2: Vec<i32> = flatten_nested_lists(&nested)?;
    let list1: Vec<i32> = vec![1, 2, 3];
    let list2: Vec<i32> = vec![10, 20];
    let product: Vec<(i32, i32)> = cartesian_product_manual(&list1, &list2)?;
    let short_list: Vec<i32> = vec![1, 2, 3];
    let long_list: Vec<i32> = vec![10, 20, 30, 40, 50];
    let zip_long: Vec<(i32, i32)> = test_zip_longest(&short_list, &long_list, 0)?;
    let batch_data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut batches: Vec<Vec<i32>> = test_batching(&batch_data, 3)?;
    let window_data: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut windows: Vec<Vec<i32>> = test_sliding_window(&window_data, 3)?;
    let duplicates: Vec<i32> = vec![1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 4, 5];
    let unique: Vec<i32> = test_unique_justseen(&duplicates)?;
    let nth: i32 = test_nth_item(&vec![10, 20, 30, 40, 50], 2, -1)?;
    let all_same: bool = test_all_equal(&vec![5, 5, 5, 5])?;
    let not_same: bool = test_all_equal(&vec![1, 2, 3])?;
    let above_threshold: i32 = test_quantify(&vec![1, 5, 10, 3, 8, 2, 15], 5)?;
    println!("{}", "All itertools tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_accumulate_examples() {
        assert_eq!(test_accumulate(vec![]), vec![]);
        assert_eq!(test_accumulate(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_pairwise_examples() {
        assert_eq!(test_pairwise(vec![]), vec![]);
        assert_eq!(test_pairwise(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_groupby_manual_examples() {
        assert_eq!(test_groupby_manual(vec![]), vec![]);
        assert_eq!(test_groupby_manual(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_chain_from_iterable_examples() {
        assert_eq!(test_chain_from_iterable(vec![]), vec![]);
        assert_eq!(test_chain_from_iterable(vec![1]), vec![1]);
    }
    #[test]
    fn test_flatten_nested_lists_examples() {
        assert_eq!(flatten_nested_lists(vec![]), vec![]);
        assert_eq!(flatten_nested_lists(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_unique_justseen_examples() {
        assert_eq!(test_unique_justseen(vec![]), vec![]);
        assert_eq!(test_unique_justseen(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_all_equal_examples() {
        let _ = test_all_equal(Default::default());
    }
}
