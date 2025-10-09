use std::collections::HashMap;
    #[doc = "Test dictionary subscript assignment"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_dict_assignment()  -> DynamicType {
    let d = {
    let mut map = HashMap::new();
    map };
    d.insert("key".to_string(), "value");
    d.insert(42, "number key");
    nested.insert("outer".to_string(), {
    let mut map = HashMap::new();
    map });
    return d
}