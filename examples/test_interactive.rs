use std::collections::HashMap;
    #[doc = "Compute the sum of a list of numbers."] #[doc = " Depyler: verified panic-free"] pub fn compute_sum<'a>(numbers: & 'a Vec<i32>)  -> i32 {
    let mut total = 0;
    for num in numbers.iter() {
    total = total + num;
   
}
return total;
   
}
#[doc = "Binary search with nested loops and array access."] pub fn binary_search<'a>(arr: & 'a Vec<i32>, target: i32)  -> Result<i32, Box<dyn std::error::Error>>{
    let mut left = 0;
    let _cse_temp_0 = arr.len();
    let mut right = _cse_temp_0 - 1;
    while left <= right {
    let mid = {
    let a = left + right;
    let b = 2;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
    if needs_adjustment {
    q - 1
}
else {
    q
}
};
    if arr.get(mid as usize).copied().unwrap_or_default() == target {
    return Ok(mid);
   
}
else {
    if arr.get(mid as usize).copied().unwrap_or_default()<target {
    left = mid + 1;
   
}
else {
    right = mid - 1;
   
}
}
}
return Ok(- 1);
   
}
#[doc = "Process strings with concatenation."] #[doc = " Depyler: verified panic-free"] pub fn process_strings<'a>(strings: & 'a Vec<String>)  -> String {
    let mut result = "";
    for s in strings.iter() {
    result = result + format !("{}{}", s, " ");
   
}
return result.trim().to_string();
   
}
#[doc = "Function with frequent dictionary lookups."] #[doc = " Depyler: verified panic-free"] pub fn lookup_values<'a, 'b>(data: & 'a HashMap<String, i32>, keys: & 'b Vec<String>)  -> Vec<i32>{
    let results = vec ! [];
    for key in keys.iter() {
    if data.contains_key(& key) {
    results.push(data.get(key as usize).copied().unwrap_or_default());
   
}
else {
    results.push(0);
   
}
} return results;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_compute_sum_examples() {
    assert_eq !(compute_sum(0), 0);
    assert_eq !(compute_sum(1), 1);
    assert_eq !(compute_sum(- 1), - 1);
   
}
}