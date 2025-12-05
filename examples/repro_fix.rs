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
#[doc = "Typed dict should use native types."]
#[doc = " Depyler: proven to terminate"]
pub fn test_typed_dict() -> Result<i32, Box<dyn std::error::Error>> {
    let mut data: std::collections::HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    data.insert("a".to_string().to_string(), 1);
    data.insert("b".to_string().to_string(), 2);
    Ok(data.get("a").cloned().unwrap_or_default())
}
