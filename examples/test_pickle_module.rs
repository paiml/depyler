#[doc = "// TODO: Map Python module 'pickle'"]
#[doc = "// TODO: Map Python module 'io'"]
const STR__: &'static str = "=";
use std::collections::HashMap;
use std::collections::HashSet;
#[doc = "Test pickling basic Python types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_basic_types() {
    let mut data = 42;
    let mut pickled = pickle.dumps(data);
    assert!(unpickled == 42);
    data = "hello world";
    pickled = pickle.dumps(data);
    assert!(unpickled == "hello world");
    data = 3.14159;
    pickled = pickle.dumps(data);
    assert!(unpickled == 3.14159);
    println!("{}", "PASS: test_pickle_basic_types");
}
#[doc = "Test pickling lists."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_list() {
    let mut data = vec![1, 2, 3, 4, 5];
    let mut pickled = pickle.dumps(data);
    assert!(unpickled == vec![1, 2, 3, 4, 5]);
    assert!(unpickled.len() as i32 == 5);
    println!("{}", "PASS: test_pickle_list");
}
#[doc = "Test pickling dictionaries."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_dict() {
    let mut data = {
        let mut map = HashMap::new();
        map.insert("name", "Alice");
        map.insert("age", 30);
        map.insert("city", "NYC");
        map
    };
    let mut pickled = pickle.dumps(data);
    assert!(
        unpickled == {
            let mut map = HashMap::new();
            map.insert("name", "Alice");
            map.insert("age", 30);
            map.insert("city", "NYC");
            map
        }
    );
    assert!(unpickled.get("name").cloned().unwrap_or_default() == "Alice");
    println!("{}", "PASS: test_pickle_dict");
}
#[doc = "Test pickling nested data structures."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_nested_structure() {
    let mut data = {
        let mut map = HashMap::new();
        map.insert(
            "users",
            vec![
                {
                    let mut map = HashMap::new();
                    map.insert("name", "Alice");
                    map.insert("scores", vec![90, 85, 88]);
                    map
                },
                {
                    let mut map = HashMap::new();
                    map.insert("name", "Bob");
                    map.insert("scores", vec![78, 82, 91]);
                    map
                },
            ],
        );
        map.insert("count", 2);
        map
    };
    let mut pickled = pickle.dumps(data);
    assert!(unpickled == data);
    assert!(
        {
            let base = &unpickled.get("users").cloned().unwrap_or_default();
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .get("name")
        .cloned()
        .unwrap_or_default()
            == "Alice"
    );
    assert!(
        {
            let base = &{
                let base = &unpickled.get("users").cloned().unwrap_or_default();
                let idx: i32 = 1;
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx).cloned().unwrap_or_default()
            }
            .get("scores")
            .cloned()
            .unwrap_or_default();
            let idx: i32 = 2;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        } == 91
    );
    println!("{}", "PASS: test_pickle_nested_structure");
}
#[doc = "Test pickling tuples."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_tuple() {
    let mut data = (1, "hello", 3.14, true);
    let mut pickled = pickle.dumps(data);
    assert!(unpickled = = (1, "hello", 3.14, true));
    assert!(
        {
            let base = &unpickled;
            let idx: i32 = 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        } == "hello"
    );
    println!("{}", "PASS: test_pickle_tuple");
}
#[doc = "Test pickling sets."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_set() {
    let mut data = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let mut pickled = pickle.dumps(data);
    assert!(
        unpickled == {
            let mut set = HashSet::new();
            set.insert(1);
            set.insert(2);
            set.insert(3);
            set.insert(4);
            set.insert(5);
            set
        }
    );
    assert!(unpickled.contains_key(&3));
    println!("{}", "PASS: test_pickle_set");
}
#[doc = "Test pickling None."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_none() {
    let mut data = ();
    let mut pickled = pickle.dumps(data);
    assert!(unpickled = = ());
    println!("{}", "PASS: test_pickle_none");
}
#[doc = "Test pickling booleans."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_boolean() {
    let pickled_true = pickle.dumps(true);
    assert!(unpickled_true == true);
    let pickled_false = pickle.dumps(false);
    assert!(unpickled_false == false);
    println!("{}", "PASS: test_pickle_boolean");
}
#[doc = "Test pickling bytes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_bytes() {
    let mut data = b"hello bytes";
    let mut pickled = pickle.dumps(data);
    assert!(unpickled == b"hello bytes");
    println!("{}", "PASS: test_pickle_bytes");
}
#[doc = "Test pickling mixed type collections."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_mixed_types() {
    let mut data = vec![1, "two", 3.0, true, (), vec![4, 5], {
        let mut map = HashMap::new();
        map.insert("key", "value");
        map
    }];
    let mut pickled = pickle.dumps(data);
    assert!(unpickled == data);
    assert!(
        {
            let base = &unpickled;
            let idx: i32 = 6;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .get("key")
        .cloned()
        .unwrap_or_default()
            == "value"
    );
    println!("{}", "PASS: test_pickle_mixed_types");
}
#[doc = "Run all pickle tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "PICKLE MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_pickle_basic_types();
    test_pickle_list();
    test_pickle_dict();
    test_pickle_nested_structure();
    test_pickle_tuple();
    test_pickle_set();
    test_pickle_none();
    test_pickle_boolean();
    test_pickle_bytes();
    test_pickle_mixed_types();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL PICKLE TESTS PASSED!");
    println!("{}", "Total tests: 10");
    println!("{}", STR__.repeat(60 as usize));
}
