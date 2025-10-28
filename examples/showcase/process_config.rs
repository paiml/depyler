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
#[doc = "Process configuration dictionary and return debug value if present."]
#[doc = " Depyler: proven to terminate"]
pub fn process_config(config: &HashMap<String, String>) -> Result<Option<String>, IndexError> {
    let _cse_temp_0 = config.contains_key("debug");
    if _cse_temp_0 {
        return Ok(Some(config.get("debug").cloned().unwrap_or_default()));
    }
    Ok(None)
}
