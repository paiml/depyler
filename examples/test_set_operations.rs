use std::collections::HashSet;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn create_sets() -> (
    std::collections::HashSet<i32>,
    std::collections::HashSet<i32>,
) {
    let s1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let s2 = {
        let mut set = HashSet::new();
        set.insert(4);
        set.insert(5);
        set.insert(6);
        set.insert(7);
        set.insert(8);
        set
    };
    (s1, s2)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_intersection() -> std::collections::HashSet<i32> {
    let s1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let s2 = {
        let mut set = HashSet::new();
        set.insert(4);
        set.insert(5);
        set.insert(6);
        set.insert(7);
        set.insert(8);
        set
    };
    let _cse_temp_0 = s1
        .intersection(&s2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let result = _cse_temp_0;
    result
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_union() -> std::collections::HashSet<i32> {
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
pub fn test_set_difference() -> std::collections::HashSet<i32> {
    let s1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let s2 = {
        let mut set = HashSet::new();
        set.insert(4);
        set.insert(5);
        set.insert(6);
        set
    };
    let result = s1
        .difference(&s2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    result
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_set_symmetric_difference() -> std::collections::HashSet<i32> {
    let s1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set
    };
    let s2 = {
        let mut set = HashSet::new();
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set.insert(6);
        set
    };
    let _cse_temp_0 = s1
        .symmetric_difference(&s2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let result = _cse_temp_0;
    result
}
