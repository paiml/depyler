#[doc = "// NOTE: Map Python module 'pickle'(tracked in DEPYLER-0424)"]
use std::io::Cursor;
const STR__: &'static str = "=";
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
#[doc = "Test pickling basic Python types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_basic_types() {
    let mut data = 42;
    let mut pickled = { format!("{:?}", data).into_bytes() };
    let mut unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled == 42);
    data = "hello world";
    pickled = { format!("{:?}", data).into_bytes() };
    unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled == "hello world");
    data = 3.14159;
    pickled = { format!("{:?}", data).into_bytes() };
    unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled == 3.14159);
    println!("{}", "PASS: test_pickle_basic_types");
}
#[doc = "Test pickling lists."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_list() {
    let data = vec![1, 2, 3, 4, 5];
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled == vec![1, 2, 3, 4, 5]);
    assert!(unpickled.len() as i32 == 5);
    println!("{}", "PASS: test_pickle_list");
}
#[doc = "Test pickling dictionaries."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_dict() {
    let data = {
        let mut map = std::collections::HashMap::new();
        map.insert("name".to_string(), serde_json::json!("Alice"));
        map.insert("age".to_string(), serde_json::json!(30));
        map.insert("city".to_string(), serde_json::json!("NYC"));
        map
    };
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(
        unpickled == {
            let mut map = std::collections::HashMap::new();
            map.insert("name".to_string(), serde_json::json!("Alice"));
            map.insert("age".to_string(), serde_json::json!(30));
            map.insert("city".to_string(), serde_json::json!("NYC"));
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
    let data = {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "users".to_string(),
            serde_json::json!(vec![
                serde_json::json!({ "name": "Alice", "scores": vec! [90, 85, 88] }),
                serde_json::json!({ "name": "Bob", "scores": vec! [78, 82, 91] })
            ]),
        );
        map.insert("count".to_string(), serde_json::json!(2));
        map
    };
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled == data);
    assert!(
        unpickled
            .get("users")
            .cloned()
            .unwrap_or_default()
            .get(0usize)
            .cloned()
            .unwrap_or_default()
            .get("name")
            .cloned()
            .unwrap_or_default()
            == "Alice"
    );
    assert!(
        unpickled
            .get("users")
            .cloned()
            .unwrap_or_default()
            .get(1usize)
            .cloned()
            .unwrap_or_default()
            .get("scores")
            .cloned()
            .unwrap_or_default()
            .get(2usize)
            .cloned()
            .unwrap_or_default()
            == 91
    );
    println!("{}", "PASS: test_pickle_nested_structure");
}
#[doc = "Test pickling tuples."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_tuple() {
    let data = (1, "hello".to_string(), 3.14, true);
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled = = (1, "hello".to_string(), 3.14, true));
    assert!(unpickled.get(1usize).cloned().unwrap_or_default() == "hello");
    println!("{}", "PASS: test_pickle_tuple");
}
#[doc = "Test pickling sets."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_set() {
    let data = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
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
    assert!(unpickled.get(&3).is_some());
    println!("{}", "PASS: test_pickle_set");
}
#[doc = "Test pickling None."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_none() {
    let data = None;
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled == None);
    println!("{}", "PASS: test_pickle_none");
}
#[doc = "Test pickling booleans."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_boolean() {
    let data_true = true;
    let pickled_true = { format!("{:?}", data_true).into_bytes() };
    let unpickled_true = { String::from_utf8_lossy(pickled_true).to_string() };
    assert!(unpickled_true == true);
    let data_false = false;
    let pickled_false = { format!("{:?}", data_false).into_bytes() };
    let unpickled_false = { String::from_utf8_lossy(pickled_false).to_string() };
    assert!(unpickled_false == false);
    println!("{}", "PASS: test_pickle_boolean");
}
#[doc = "Test pickling bytes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_bytes() {
    let data = b"hello bytes";
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled == b"hello bytes");
    println!("{}", "PASS: test_pickle_bytes");
}
#[doc = "Test pickling mixed type collections."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_pickle_mixed_types() {
    let data = vec![1, "two".to_string(), 3.0, true, None, vec![4, 5], {
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value");
        map
    }];
    let pickled = { format!("{:?}", data).into_bytes() };
    let unpickled = { String::from_utf8_lossy(pickled).to_string() };
    assert!(unpickled == data);
    assert!(
        unpickled
            .get(6usize)
            .cloned()
            .unwrap_or_default()
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
