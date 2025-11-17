use std::collections::HashMap;
#[doc = "Test dictionary subscript assignment"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_dict_assignment() {
    let mut d = {
        let map = HashMap::new();
        map
    };
    d.insert("key".to_string(), "value");
    d.insert(42, "number key");
    let mut nested = {
        let map = HashMap::new();
        map
    };
    nested.insert("outer".to_string(), {
        let map = HashMap::new();
        map
    });
    d
}
