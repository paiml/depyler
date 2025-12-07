use serde_json;
use std::collections::HashMap;
#[doc = "Test dictionary subscript assignment"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_assignment() -> HashMap<serde_json::Value, serde_json::Value> {
    let mut d = {
        let mut map = std::collections::HashMap::new();
        map
    };
    d.insert("key".to_string().to_string(), serde_json::json!("value"));
    d.insert(42, serde_json::json!("number key"));
    let mut nested = {
        let mut map = std::collections::HashMap::new();
        map
    };
    nested.insert(
        "outer".to_string().to_string(),
        serde_json::to_value({
            let mut map = std::collections::HashMap::new();
            map
        })
        .unwrap(),
    );
    d
}
