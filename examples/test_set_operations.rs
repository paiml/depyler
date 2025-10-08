use std::collections::HashSet;
    #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn create_sets()  -> (HashSet<i32>, HashSet<i32>) {
    return(s1, s2);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_intersection()  -> HashSet<i32>{
    let s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set };
    let s2 = {
    let mut set = HashSet::new();
    set.insert(4);
    set.insert(5);
    set.insert(6);
    set.insert(7);
    set.insert(8);
    set };
    let _cse_temp_0 = s1 & s2;
    let result = _cse_temp_0;
    return result;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_union()  -> HashSet<i32>{
    let s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set };
    let s2 = {
    let mut set = HashSet::new();
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set };
    let _cse_temp_0 = s1 | s2;
    let result = _cse_temp_0;
    return result;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_difference()  -> HashSet<i32>{
    let s1 = {
    let mut📄 Source: examples/test_set_operations.py (803 bytes)
📝 Output: examples/test_set_operations.rs (2113 bytes)
⏱️  Parse time: 11ms
📊 Throughput: 70.5 KB/s
⏱️  Total time: 11ms
(4);
    set.insert(5);
    set.insert(6);
    set };
    let result = s1 - s2;
    return result;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_symmetric_difference()  -> HashSet<i32>{
    let s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set };
    let s2 = {
    let mut set = HashSet::new();
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set.insert(6);
    set };
    let _cse_temp_0 = s1 ^ s2;
    let result = _cse_temp_0;
    return result
}