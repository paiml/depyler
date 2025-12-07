use std::collections::HashMap;
#[doc = "Test nested dictionary assignment"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nested_assignment() -> HashMap<String, HashMap<String, String>> {
    let mut d: std::collections::HashMap<String, std::collections::HashMap<String, String>> = {
        let map = HashMap::new();
        map
    };
    d.insert("outer".to_string().to_string(), {
        let map = HashMap::new();
        map
    });
    d.get_mut(&"outer".to_string())
        .unwrap()
        .insert("inner".to_string().to_string(), "value".to_string());
    d.get_mut(&"outer".to_string())
        .unwrap()
        .insert("another".to_string().to_string(), "value2".to_string());
    d
}
