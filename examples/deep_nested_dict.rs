use std::collections::HashMap;
    #[doc = "Test deeply nested dictionary assignment"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_deep_nested()  -> HashMap<String, HashMap<String, HashMap<String, String>>>{
    let d: HashMap<String, HashMap<String, HashMap<String, String>>>= {
    let mut map = HashMap::new();
    map };
    d.insert("level1".to_string(), {
    let mut map = HashMap::new();
    map });
    d.get_mut(& "level1".to_string()).unwrap().insert("level2".to_string(), {
    let mut map = HashMap::new();
    map });
📄 Source: examples/deep_nested_dict.py (349 bytes)
📝 Output: examples/deep_nested_dict.rs (728 bytes)
⏱️  Parse time: 7ms
📊 Throughput: 44.8 KB/s
⏱️  Total time: 7ms
