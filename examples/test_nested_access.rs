use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Test reading nested dictionary values"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nested_access() -> Result<String, Box<dyn std::error::Error>> {
    let d = {
        let mut map = HashMap::new();
        map.insert("outer".to_string(), {
            let mut map = HashMap::new();
            map.insert("inner".to_string(), "value");
            map
        });
        map
    };
    let val = d
        .get("outer")
        .cloned()
        .unwrap_or_default()
        .get("inner")
        .cloned()
        .unwrap_or_default();
    Ok(val.to_string())
}
