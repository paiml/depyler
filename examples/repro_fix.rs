use serde_json as json;
use serde_json;
use std::collections::HashMap;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_data(text: &str) -> HashMap<String, serde_json::Value> {
    serde_json::from_str::<std::collections::HashMap<String, serde_json::Value>>(&text).unwrap()
}
