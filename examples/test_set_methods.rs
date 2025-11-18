use std::collections::HashSet;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_add() -> HashSet<i32> {
    let mut s = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    s.insert(4);
    s.insert(3);
    s
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_remove() -> HashSet<String> {
    let mut s = {
        let mut set = HashSet::new();
        set.insert("apple".to_string());
        set.insert("banana".to_string());
        set.insert("cherry".to_string());
        set
    };
    if !s.remove(&"banana".to_string()) {
        panic!("KeyError: element not in set")
    };
    s
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_discard() -> HashSet<i32> {
    let mut s = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    s.remove(&3);
    s.remove(&10);
    s
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_clear() -> bool {
    let mut s = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    s.clear();
    s.len() as i32 == 0
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_pop() -> i32 {
    let mut s = {
        let mut set = HashSet::new();
        set.insert(42);
        set
    };
    let value = s
        .iter()
        .next()
        .cloned()
        .map(|x| {
            s.remove(&x);
            x
        })
        .expect("pop from empty set");
    value
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_set_pop_examples() {
        let _ = test_set_pop();
    }
}
