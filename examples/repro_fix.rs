use serde_json;
use std::collections::HashMap;
#[doc = "Parse some data into a dictionary."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_data(text: String) -> std::collections::HashMap<String, serde_json::Value> {
    let mut result: std::collections::HashMap<String, serde_json::Value> = {
        let map = HashMap::new();
        map
    };
    result.insert("key".to_string().to_string(), serde_json::json!("value"));
    result
}
