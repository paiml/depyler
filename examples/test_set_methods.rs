use std::collections::HashSet;
    #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_add()  -> HashSet<i32>{
    let s = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set };
    s.insert(4);
    s.insert(3);
    return s;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_remove()  -> HashSet<String>{
    let s = {
    let mut set = HashSet::new();
    set.insert("apple".to_string());
    set.insert("banana".to_string());
    set.insert("cherry".to_string());
    set };
    if let Some(pos) = s.iter().position(| x | x == & "banana".to_string()) {
    s.remove(pos)
}
else {
    panic !("ValueError: list.remove(x): x not in list") };
    return s;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_discard()  -> HashSet<i32>{
    let s = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set };
    s.remove(& 3);
    s.remove(& 10);
    return s;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_clear()  -> DynamicType {
    let s = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.i📄 Source: examples/test_set_methods.py (707 bytes)
📝 Output: examples/test_set_methods.rs (1945 bytes)
⏱️  Parse time: 9ms
📊 Throughput: 73.2 KB/s
⏱️  Total time: 9ms
= " Depyler: proven to terminate"] pub fn test_set_pop()  -> i32 {
    let s = {
    let mut set = HashSet::new();
    set.insert(42);
    set };
    let value = s.pop().unwrap_or_default();
    return value;
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_test_set_pop_examples() {
    let _ = test_set_pop();
   
}
}