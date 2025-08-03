use std::collections::HashSet;
    #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn create_sets()  -> tuple<HashSet<i32>, HashSet<i32>>{
    let mut s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set };
    let mut s2 = {
    let mut set = HashSet::new();
    set.insert(4);
    set.insert(5);
    set.insert(6);
    set.insert(7);
    set.insert(8);
    set };
    return(s1, s2);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_intersection()  -> HashSet<i32>{
    let mut s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set };
    let mut s2 = {
    let mut set = HashSet::new();
    set.insert(4);
    set.insert(5);
    set.insert(6);
    set.insert(7);
    set.insert(8);
    set };
    let mut result  = (s1 & s2);
    return result;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_union()  -> HashSet<i32>{
    let mut s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set };
    let mut s2 = {
    let mut set = HashSet::new();
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set };
    let mut result  = (s1 | s2);
    return result;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_difference()  -> HashSet<i32>{
    let mut s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set };
    let mut s2 = {
    let mut set = HashSet::new();
    set.insert(4);
    set.insert(5);
    set.insert(6);
    set };
    let mut result  = (s1 - s2);
    return result;
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_set_symmetric_difference()  -> HashSet<i32>{
    let mut s1 = {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set };
    let mut s2 = {
    let mut set = HashSet::new();
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set.insert(6);
    set };
    let mut result  = (s1 ^ s2);
    return result
}