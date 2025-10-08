use std::collections::HashMap;
    #[derive(Debug, Clone)] pub struct IndexError {
    message: String ,
}
impl std::fmt::Display for IndexError {
    fn fmt(& self, f: & mut std::fmt::Formatter<'_>)  -> std::fmt::Result {
    write !(f, "index out of range: {}", self.message)
}
} impl std::error::Error for IndexError {
   
}
impl IndexError {
    pub fn new(message: impl Into<String>)  -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = "Calculate sum of numbers."] #[doc = " Depyler: verified panic-free"] pub fn calculate_sum<'a>(numbers: & 'a list<i32>)  -> i32 {
    let mut total = 0;
    for num in numbers.iter() {
    total = total + num;
   
}
return total;
   
}
#[doc = "Find maximum value."] pub fn find_max<'a>(values: & 'a list<i32>)  -> Result<i32, IndexError>{
    if ! values {
    return Ok(0);
   
}
let mut max_val = values.get(0 as usize).copied().unwrap_or_default();
    for val in values.iter() {
    if val>max_val {
    max_val = val;
   
}
} return Ok(max_val);
   
}
#[doc = "Process data and return statistics."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn process_data(data: list<i32>)  -> dict<String, i32>{
    let result = {
    let mut map = HashMap::new();
    map.insert("sum", calculate_sum(data));
    map.insert("max", find_max(data));
    map.insert("count", data.len());
    map };
    return result;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_calculate_sum_examples() {
    assert_eq !(calculate_sum(0), 0);
    assert_eq !(calculate_sum(1), 1);
    assert_eq !(calculate_sum(- 1), - 1);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_find_max_examples() {
    assert_eq !(find_max(0), 0);
    assert_eq !(find_max(1), 1);
    assert_eq !(find_max(- 1), - 1);
   
}
}