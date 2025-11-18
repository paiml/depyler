use serde_json as json;
use serde_json;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let data = serde_json::json!({ "name": "test", "count": 42, "enabled": true, "rate": 3.14, "items": vec! [1, 2, 3] });
    println!("{}", serde_json::to_string(&data).unwrap());
}
