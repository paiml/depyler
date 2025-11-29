use serde_json as json;
use serde_json;
use std::collections::HashMap;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let data = {
        let mut map = std::collections::HashMap::new();
        map.insert("name".to_string(), serde_json::json!("test"));
        map.insert("count".to_string(), serde_json::json!(42));
        map.insert("enabled".to_string(), serde_json::json!(true));
        map.insert("rate".to_string(), serde_json::json!(3.14));
        map.insert("items".to_string(), serde_json::json!(vec![1, 2, 3]));
        map
    };
    println!("{}", serde_json::to_string(&data).unwrap());
}
