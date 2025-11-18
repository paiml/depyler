use std::collections::HashSet;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_simple_set() -> HashSet<i32> {
    let s1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    let s2 = {
        let mut set = HashSet::new();
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let _cse_temp_0 = s1
        .union(&s2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let result = _cse_temp_0;
    result
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_method() -> HashSet<String> {
    let mut fruits = {
        let mut set = HashSet::new();
        set.insert("apple".to_string());
        set.insert("banana".to_string());
        set
    };
    fruits.insert("cherry".to_string());
    fruits
}
