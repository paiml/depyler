use std::collections::HashMap;
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
#[doc = "Map transformation over list"]
#[doc = " Depyler: verified panic-free"]
pub fn map_transform(data: &Vec<i32>, multiplier: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        let transformed: i32 = item * multiplier;
        result.push(transformed);
    }
    result
}
#[doc = "Filter data by predicate"]
#[doc = " Depyler: verified panic-free"]
pub fn filter_predicate(data: &Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        if item > threshold {
            result.push(item);
        }
    }
    result
}
#[doc = "Reduce list to sum"]
#[doc = " Depyler: verified panic-free"]
pub fn reduce_sum(data: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for item in data.iter().cloned() {
        total = total + item;
    }
    total
}
#[doc = "Reduce list to product"]
#[doc = " Depyler: verified panic-free"]
pub fn reduce_product(data: &Vec<i32>) -> i32 {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return 0;
    }
    let mut product: i32 = 1;
    for item in data.iter().cloned() {
        product = product * item;
    }
    product
}
#[doc = "Chain multiple operations together"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn chain_operations(data: Vec<i32>) -> i32 {
    let mapped: Vec<i32> = map_transform(&data, 2);
    let filtered: Vec<i32> = filter_predicate(&mapped, 10);
    let mut result: i32 = reduce_sum(&filtered);
    result
}
#[doc = "Zip two lists together"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn zip_lists<'b, 'a>(list1: &'a Vec<i32>, list2: &'b Vec<String>) -> Vec<(i32, String)> {
    let mut result: Vec<(i32, String)> = vec![];
    let _cse_temp_0 = list1.len() as i32;
    let _cse_temp_1 = list2.len() as i32;
    let _cse_temp_2 = std::cmp::min(_cse_temp_0, _cse_temp_1);
    let min_len: i32 = _cse_temp_2;
    for i in 0..min_len {
        let pair: (i32, String) = (
            list1.get(i as usize).cloned().unwrap_or_default(),
            list2.get(i as usize).cloned().unwrap_or_default(),
        );
        result.push(pair);
    }
    result
}
#[doc = "Enumerate list with indices"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn enumerate_list(items: &Vec<String>) -> Vec<(i32, String)> {
    let mut result: Vec<(i32, String)> = vec![];
    for i in 0..items.len() as i32 {
        let pair: (i32, String) = (i, items.get(i as usize).cloned().unwrap_or_default());
        result.push(pair);
    }
    result
}
#[doc = "Group items by property(modulo)"]
pub fn group_by_property(
    items: &Vec<i32>,
    modulo: i32,
) -> Result<HashMap<i32, Vec<i32>>, ZeroDivisionError> {
    let mut groups: HashMap<i32, Vec<i32>> = {
        let map = HashMap::new();
        map
    };
    for item in items.iter().cloned() {
        let key: i32 = item % modulo;
        if !groups.contains_key(&key) {
            groups.insert(key, vec![]);
        }
        groups.get(&key).cloned().unwrap_or_default().push(item);
    }
    Ok(groups)
}
#[doc = "Partition list into two based on predicate"]
#[doc = " Depyler: verified panic-free"]
pub fn partition_by_predicate(items: &Vec<i32>, threshold: i32) -> (Vec<i32>, Vec<i32>) {
    let mut passed: Vec<i32> = vec![];
    let mut failed: Vec<i32> = vec![];
    for item in items.iter().cloned() {
        if item >= threshold {
            passed.push(item);
        } else {
            failed.push(item);
        }
    }
    (passed, failed)
}
#[doc = "Create list of running sums(accumulate pattern)"]
#[doc = " Depyler: verified panic-free"]
pub fn accumulate_running_sum(data: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut total: i32 = 0;
    for item in data.iter().cloned() {
        total = total + item;
        result.push(total);
    }
    result
}
#[doc = "Flatten nested list structure"]
#[doc = " Depyler: verified panic-free"]
pub fn flatten_nested_list(nested: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for sublist in nested.iter().cloned() {
        for item in sublist.iter().cloned() {
            result.push(item);
        }
    }
    result
}
#[doc = "Compute Cartesian product of two lists"]
#[doc = " Depyler: verified panic-free"]
pub fn cartesian_product<'a, 'b>(list1: &'a Vec<i32>, list2: &'b Vec<i32>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    for item1 in list1.iter().cloned() {
        for item2 in list2.iter().cloned() {
            let pair: (i32, i32) = (item1, item2);
            result.push(pair);
        }
    }
    result
}
#[doc = "Take elements while condition is true"]
#[doc = " Depyler: verified panic-free"]
pub fn take_while_condition(data: &Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        if item < threshold {
            result.push(item);
        } else {
            break;
        }
    }
    result
}
#[doc = "Drop elements while condition is true"]
#[doc = " Depyler: verified panic-free"]
pub fn drop_while_condition(data: &Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut dropping: bool = true;
    for item in data.iter().cloned() {
        if (dropping) && (item < threshold) {
            continue;
        }
        dropping = false;
        result.push(item);
    }
    result
}
#[doc = "Iterate over consecutive pairs"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn pairwise_iteration(data: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    for i in 0..(data.len() as i32).saturating_sub(1) {
        let pair: (i32, i32) = (data.get(i as usize).cloned().unwrap_or_default(), {
            let base = &data;
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
#[doc = "Create sliding windows over data"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn sliding_window(data: &Vec<i32>, window_size: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..(data.len() as i32).saturating_sub(window_size) + 1 {
        let mut window: Vec<i32> = vec![];
        for j in 0..window_size {
            window.push({
                let base = &data;
                let idx: i32 = i + j;
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx).cloned().unwrap_or_default()
            });
        }
        result.push(window);
    }
    result
}
#[doc = "Compose two functions(f ∘ g)"]
#[doc = " Depyler: verified panic-free"]
pub fn compose_two_functions(data: Vec<i32>) -> Vec<i32> {
    let step1: Vec<i32> = map_transform(&data, 2);
    let mut step2: Vec<i32> = vec![];
    for item in step1.iter().cloned() {
        step2.push(item + 1);
    }
    step2
}
#[doc = "Apply multiple operations in sequence"]
#[doc = " Depyler: verified panic-free"]
pub fn apply_multiple_operations<'a, 'b>(
    data: &'a Vec<i32>,
    operations: &'b Vec<String>,
) -> Vec<i32> {
    let mut result: Vec<i32> = data.clone();
    for op in operations.iter().cloned() {
        let mut new_result: Vec<i32> = vec![];
        if op == "double" {
            for item in result.iter().cloned() {
                new_result.push(item * 2);
            }
        } else {
            if op == "increment" {
                for item in result.iter().cloned() {
                    new_result.push(item + 1);
                }
            } else {
                if op == "square" {
                    for item in result.iter().cloned() {
                        new_result.push(item * item);
                    }
                } else {
                    new_result = result;
                }
            }
        }
        result = new_result;
    }
    result
}
#[doc = "Classic map-reduce pattern"]
#[doc = " Depyler: verified panic-free"]
pub fn map_reduce_pattern(data: &Vec<i32>) -> i32 {
    let mut mapped: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        mapped.push(item * item);
    }
    let reduced: i32 = reduce_sum(&mapped);
    reduced
}
#[doc = "Filter-Map-Reduce pipeline"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn filter_map_reduce_pattern(data: Vec<i32>, threshold: i32) -> i32 {
    let filtered: Vec<i32> = filter_predicate(&data, threshold);
    let mut mapped: Vec<i32> = map_transform(&filtered, 3);
    let reduced: i32 = reduce_sum(&mapped);
    reduced
}
#[doc = "Get unique elements(set-like operation)"]
#[doc = " Depyler: verified panic-free"]
pub fn unique_elements(data: &Vec<i32>) -> Vec<i32> {
    let mut seen: HashMap<i32, bool> = {
        let map = HashMap::new();
        map
    };
    let mut result: Vec<i32> = vec![];
    for item in data.iter().cloned() {
        if !seen.contains_key(&item) {
            seen.insert(item, true);
            result.push(item);
        }
    }
    result
}
#[doc = "Count occurrences of each value"]
pub fn count_by_value(data: &Vec<i32>) -> Result<HashMap<i32, i32>, IndexError> {
    let mut counts: HashMap<i32, i32> = {
        let map = HashMap::new();
        map
    };
    for item in data.iter().cloned() {
        if counts.contains_key(&item) {
            {
                let _key = item;
                let _old_val = counts.get(&_key).cloned().unwrap_or_default();
                counts.insert(_key, _old_val + 1);
            }
        } else {
            counts.insert(item, 1);
        }
    }
    Ok(counts)
}
#[doc = "Sort list of tuples by second element"]
#[doc = " Depyler: proven to terminate"]
pub fn sorted_by_key(items: &Vec<(String, i32)>) -> Result<Vec<(String, i32)>, IndexError> {
    let mut result: Vec<(String, i32)> = items.clone();
    for i in 0..result.len() as i32 {
        for j in i + 1..result.len() as i32 {
            if result
                .get(j as usize)
                .cloned()
                .unwrap_or_default()
                .get(1usize)
                .cloned()
                .unwrap_or_default()
                < result
                    .get(i as usize)
                    .cloned()
                    .unwrap_or_default()
                    .get(1usize)
                    .cloned()
                    .unwrap_or_default()
            {
                let temp: (String, i32) = result.get(i as usize).cloned().unwrap_or_default();
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
#[doc = "Demonstrate functional programming patterns"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demonstrate_functional_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=== Functional Programming Patterns Demo ===");
    let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", "\n1. Map Pattern");
    let doubled: Vec<i32> = map_transform(&data, 2)?;
    println!(
        "{}",
        format!("   Doubled: {:?} elements", doubled.len() as i32)
    );
    println!("{}", "\n2. Filter Pattern");
    let filtered: Vec<i32> = filter_predicate(&data, 5)?;
    println!(
        "{}",
        format!("   Filtered(>5): {:?} elements", filtered.len() as i32)
    );
    println!("{}", "\n3. Reduce Pattern");
    let mut total: i32 = reduce_sum(&data)?;
    println!("{}", format!("   Sum: {:?}", total));
    println!("{}", "\n4. Chained Operations");
    let chained: i32 = chain_operations(data)?;
    println!("{}", format!("   Result: {:?}", chained));
    println!("{}", "\n5. Zip Pattern");
    let labels: Vec<String> = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
    ];
    let zipped: Vec<(i32, String)> = zip_lists(
        &{
            let base = data;
            let stop = (5).max(0) as usize;
            base[..stop.min(base.len())].to_vec()
        },
        &labels,
    )?;
    println!("{}", format!("   Zipped: {:?} pairs", zipped.len() as i32));
    println!("{}", "\n6. Group By Pattern");
    let mut groups: HashMap<i32, Vec<i32>> = group_by_property(&data, 3)?;
    println!(
        "{}",
        format!("   Groups(mod 3): {:?} groups", groups.len() as i32)
    );
    println!("{}", "\n7. Partition Pattern");
    let parts: (Vec<i32>, Vec<i32>) = partition_by_predicate(&data, 6)?;
    println!(
        "{}",
        format!(
            "   Partition: {:?} passed, {:?} failed",
            parts.get(0usize).cloned().unwrap_or_default().len() as i32,
            parts.get(1usize).cloned().unwrap_or_default().len() as i32
        )
    );
    println!("{}", "\n8. Accumulate Pattern");
    let running_sums: Vec<i32> = accumulate_running_sum(&data)?;
    println!(
        "{}",
        format!("   Running sums: {:?} values", running_sums.len() as i32)
    );
    println!("{}", "\n9. Flatten Pattern");
    let nested: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = flatten_nested_list(&nested)?;
    println!(
        "{}",
        format!("   Flattened: {:?} elements", flattened.len() as i32)
    );
    println!("{}", "\n10. Cartesian Product");
    let list1: Vec<i32> = vec![1, 2, 3];
    let list2: Vec<i32> = vec![10, 20];
    let mut product: Vec<(i32, i32)> = cartesian_product(&list1, &list2)?;
    println!(
        "{}",
        format!("   Product: {:?} combinations", product.len() as i32)
    );
    println!("{}", "\n11. Take While Pattern");
    let taken: Vec<i32> = take_while_condition(&data, 6)?;
    println!(
        "{}",
        format!("   Taken(while <6): {:?} elements", taken.len() as i32)
    );
    println!("{}", "\n12. Pairwise Iteration");
    let pairs: Vec<(i32, i32)> = pairwise_iteration(&data)?;
    println!("{}", format!("   Pairs: {:?} pairs", pairs.len() as i32));
    println!("{}", "\n13. Sliding Window");
    let windows: Vec<Vec<i32>> = sliding_window(&data, 3)?;
    println!(
        "{}",
        format!("   Windows(size 3): {:?} windows", windows.len() as i32)
    );
    println!("{}", "\n14. Function Composition");
    let composed: Vec<i32> = compose_two_functions(vec![1, 2, 3])?;
    println!(
        "{}",
        format!("   Composed result: {:?} elements", composed.len() as i32)
    );
    println!("{}", "\n15. Map-Reduce Pattern");
    let mr_result: i32 = map_reduce_pattern(&vec![1, 2, 3, 4])?;
    println!(
        "{}",
        format!("   Map-Reduce sum of squares: {:?}", mr_result)
    );
    println!("{}", "\n16. Filter-Map-Reduce");
    let fmr_result: i32 = filter_map_reduce_pattern(data, 5)?;
    println!(
        "{}",
        format!("   Filter-Map-Reduce result: {:?}", fmr_result)
    );
    println!("{}", "\n17. Unique Elements");
    let duplicates: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let unique: Vec<i32> = unique_elements(&duplicates)?;
    println!(
        "{}",
        format!("   Unique elements: {:?}", unique.len() as i32)
    );
    println!("{}", "\n=== All Patterns Demonstrated ===");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_reduce_sum_examples() {
        assert_eq!(reduce_sum(&vec![]), 0);
        assert_eq!(reduce_sum(&vec![1]), 1);
        assert_eq!(reduce_sum(&vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_reduce_product_examples() {
        assert_eq!(reduce_product(&vec![]), 0);
        assert_eq!(reduce_product(&vec![1]), 1);
        assert_eq!(reduce_product(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_chain_operations_examples() {
        assert_eq!(chain_operations(&vec![]), 0);
        assert_eq!(chain_operations(&vec![1]), 1);
        assert_eq!(chain_operations(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_enumerate_list_examples() {
        assert_eq!(enumerate_list(vec![]), vec![]);
        assert_eq!(enumerate_list(vec![1]), vec![1]);
    }
    #[test]
    fn test_accumulate_running_sum_examples() {
        assert_eq!(accumulate_running_sum(vec![]), vec![]);
        assert_eq!(accumulate_running_sum(vec![1]), vec![1]);
    }
    #[test]
    fn test_flatten_nested_list_examples() {
        assert_eq!(flatten_nested_list(vec![]), vec![]);
        assert_eq!(flatten_nested_list(vec![1]), vec![1]);
    }
    #[test]
    fn test_pairwise_iteration_examples() {
        assert_eq!(pairwise_iteration(vec![]), vec![]);
        assert_eq!(pairwise_iteration(vec![1]), vec![1]);
    }
    #[test]
    fn test_compose_two_functions_examples() {
        assert_eq!(compose_two_functions(vec![]), vec![]);
        assert_eq!(compose_two_functions(vec![1]), vec![1]);
    }
    #[test]
    fn test_map_reduce_pattern_examples() {
        assert_eq!(map_reduce_pattern(&vec![]), 0);
        assert_eq!(map_reduce_pattern(&vec![1]), 1);
        assert_eq!(map_reduce_pattern(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_unique_elements_examples() {
        assert_eq!(unique_elements(vec![]), vec![]);
        assert_eq!(unique_elements(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_sorted_by_key() {
        fn prop(items: Vec<()>) -> TestResult {
            let input_len = items.len();
            let result = sorted_by_key(&items);
            if result.len() != input_len {
                return TestResult::failed();
            }
            let result = sorted_by_key(&items);
            for i in 1..result.len() {
                if result[i - 1] > result[i] {
                    return TestResult::failed();
                }
            }
            let mut input_sorted = items.clone();
            input_sorted.sort();
            let mut result = sorted_by_key(&items);
            result.sort();
            if input_sorted != result {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<()>) -> TestResult);
    }
    #[test]
    fn test_sorted_by_key_examples() {
        assert_eq!(sorted_by_key(vec![]), vec![]);
        assert_eq!(sorted_by_key(vec![1]), vec![1]);
    }
}
