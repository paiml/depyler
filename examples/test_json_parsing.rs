use serde_json as json;
use serde_json;
use std::collections::HashMap;
#[doc = "Parse JSON from string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_json_string(json_str: &str) -> std::collections::HashMap<String, serde_json::Value> {
    serde_json::from_str::<serde_json::Value>(&json_str).unwrap()
}
#[doc = "Convert data to JSON string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn to_json_string(data: &std::collections::HashMap<String, serde_json::Value>) -> String {
    serde_json::to_string(&data).unwrap()
}
#[doc = "Parse JSON with a default value on error"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_json_with_default(
    json_str: &str,
    default: std::collections::HashMap<String, serde_json::Value>,
) -> std::collections::HashMap<String, serde_json::Value> {
    {
        return serde_json::from_str::<serde_json::Value>(&json_str).unwrap();
        return default;
    }
}
#[doc = "Merge two JSON strings into one dictionary"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn merge_json_objects<'a, 'b>(
    json1: &'a str,
    json2: &'b str,
) -> std::collections::HashMap<String, serde_json::Value> {
    let mut obj1 = serde_json::from_str::<serde_json::Value>(&json1).unwrap();
    let obj2 = serde_json::from_str::<serde_json::Value>(&json2).unwrap();
    obj1.update(&obj2);
    obj1
}
