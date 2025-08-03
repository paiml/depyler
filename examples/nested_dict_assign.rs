use std::collections::HashMap;
    #[doc = "Test nested dictionary assignment"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_nested_assignment()  -> HashMap<String, HashMap<String, String>>{
    let mut d = {
    let mut map = HashMap::new();
    map };
    d.insert("outer".to_string(), {
    let mut map = HashMap::new();
    map });
    d.get_mut(& "outer".to_string()).unwrap().insert("inner".to_string(), "value");
    d.get_mut(& "outer".to_string()).unwrap().insert("another".to_string(), "value2");
    return d
}