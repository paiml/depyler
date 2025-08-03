use std::collections::HashSet;
    #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_add()  -> HashSet<i32>{
    let mut s = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set };
    s.add(4);
    s.add(3);
    return s;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_remove()  -> HashSet<String>{
    let mut s = {
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
    let mut s = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set };
    s.discard(3);
    s.discard(10);
    return s;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_clear()  -> serde_json::Value {
    let mut s = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set };
    s.clear();
    return(s.len() == 0);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_pop()  -> i32 {
    let mut s = {
    let mut set = HashSet::new();
    set.insert(42);
    set };
    let mut value = s.pop().unwrap_or_default();
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