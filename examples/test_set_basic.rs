use std::collections::HashSet;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_creation() -> std::collections::HashSet<i32> {
    let s2 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    s2
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_with_duplicates() -> std::collections::HashSet<i32> {
    let s = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(2);
        set.insert(3);
        set.insert(3);
        set.insert(3);
        set.insert(4);
        set
    };
    s
}
