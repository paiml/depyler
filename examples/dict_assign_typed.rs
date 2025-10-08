use std::collections::HashMap;
    #[doc = "Test dictionary assignment with string keys"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_string_dict()  -> HashMap<String, String>{
    let d: HashMap<String, String>= {
    let mut map = HashMap::new();
    map };
    d.insert("key1".to_string(), "value1");
    d.insert("key2".to_string(), "value2");
    return d;
   
}
#[doc = "Test dictionary assignment with integer keys"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_int_dict()  -> HashMap<i32, String>{
    let d: HashMap<i32, String>= {
    let mut map = HashMap::new();
    map };
    d.insert(42, "number key");
    d.insert(100, "another number");
    return d;
   
}
#[doc = "Test nested dictionary(but not nested assignm📄 Source: examples/dict_assign_typed.py (668 bytes)
📝 Output: examples/dict_assign_typed.rs (1189 bytes)
⏱️  Parse time: 8ms
📊 Throughput: 74.2 KB/s
⏱️  Total time: 8ms
ring, HashMap<String, String>>= {
    let mut map = HashMap::new();
    map };
    d.insert("outer".to_string(), {
    let mut map = HashMap::new();
    map });
    return d
}