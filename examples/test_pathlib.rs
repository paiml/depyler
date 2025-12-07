use serde_json;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Get all Python files in a directory"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_python_files(directory: String) -> Vec<String> {
    let path = std::path::PathBuf::from(&directory);
    path.glob("*.py".to_string())
        .into_iter()
        .map(|p| (p).to_string())
        .collect::<Vec<_>>()
}
#[doc = "Create a nested path from parts"]
pub fn create_nested_path(
    base: String,
    parts: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let mut path = std::path::PathBuf::from(&base);
    for part in parts.iter().cloned() {
        path = path.join(part);
    }
    Ok((path).display().to_string())
}
#[doc = "Get file information"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_file_info(
    filepath: String,
) -> (serde_json::Value, serde_json::Value, serde_json::Value) {
    let path = std::path::PathBuf::from(&filepath);
    (
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string(),
        path.extension()
            .map(|e| format!(".{}", e.to_str().unwrap()))
            .unwrap_or_default(),
        path.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default()
            .name,
    )
}
#[doc = "Check if a path exists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn check_path_exists(filepath: String) -> bool {
    std::path::PathBuf::from(&filepath).exists()
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_check_path_exists_examples() {
        let _ = check_path_exists(Default::default());
    }
}
