use std::collections::HashSet;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_creation() {
    let s1 = HashSet::new();
    let s2 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let s3 = {
        let mut set = HashSet::new();
        set.insert("apple".to_string());
        set.insert("banana".to_string());
        set.insert("cherry".to_string());
        set
    };
    let s4 = vec![1, 2, 3, 3, 4, 4, 5]
        .into_iter()
        .collect::<HashSet<_>>();
    s2
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_with_duplicates() {
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
