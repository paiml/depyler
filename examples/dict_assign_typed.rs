use std::collections::HashMap;
#[doc = "Test dictionary assignment with string keys"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_string_dict() -> HashMap<String, String> {
    let mut d: HashMap<String, String> = {
        let map = HashMap::new();
        map
    };
    d.insert("key1".to_string().to_string(), "value1".to_string());
    d.insert("key2".to_string().to_string(), "value2".to_string());
    d
}
#[doc = "Test dictionary assignment with integer keys"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_int_dict() -> HashMap<i32, String> {
    let mut d: HashMap<i32, String> = {
        let map = HashMap::new();
        map
    };
    d.insert(42, "number key".to_string());
    d.insert(100, "another number".to_string());
    d
}
#[doc = "Test nested dictionary(but not nested assignment yet)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nested_dict() -> HashMap<String, HashMap<String, String>> {
    let mut d: HashMap<String, HashMap<String, String>> = {
        let map = HashMap::new();
        map
    };
    d.insert("outer".to_string().to_string(), {
        let map = HashMap::new();
        map
    });
    d
}
