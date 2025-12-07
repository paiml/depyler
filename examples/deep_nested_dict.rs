use std::collections::HashMap;
#[doc = "Test deeply nested dictionary assignment"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_deep_nested() -> HashMap<String, HashMap<String, HashMap<String, String>>> {
    let mut d: std::collections::HashMap<
        String,
        std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    > = {
        let map = HashMap::new();
        map
    };
    d.insert("level1".to_string().to_string(), {
        let map = HashMap::new();
        map
    });
    d.get_mut(&"level1".to_string())
        .unwrap()
        .insert("level2".to_string().to_string(), {
            let map = HashMap::new();
            map
        });
    d.get_mut(&"level1".to_string())
        .unwrap()
        .get_mut(&"level2".to_string())
        .unwrap()
        .insert("level3".to_string().to_string(), "deep value".to_string());
    d
}
