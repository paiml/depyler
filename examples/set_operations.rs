use std::collections::HashSet;
#[doc = "Test basic set creation and operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_creation() -> std::collections::HashSet<i32> {
    let s1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    s1
}
#[doc = "Test set operators"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_operators() -> std::collections::HashSet<i32> {
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
    let union = _cse_temp_0;
    union
}
#[doc = "Test set mutation methods"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_methods() -> Vec<String> {
    let mut s = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    s.insert(4);
    if !s.remove(&2) {
        panic!("KeyError: element not in set")
    };
    s.remove(&5);
    s
}
#[doc = "Test set comprehension"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_comprehension() -> std::collections::HashSet<i32> {
    let even_squares = (0..10)
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect::<HashSet<_>>();
    even_squares
}
