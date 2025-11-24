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
#[doc = " Depyler: proven to terminate"]
pub fn test_arrays() -> Result<i32, IndexError> {
    let arr1 = vec![1, 2, 3, 4, 5];
    let _cse_temp_0 = arr1.get(0usize).cloned().unwrap_or_default()
        + arr1.get(1usize).cloned().unwrap_or_default();
    let sum_val = _cse_temp_0;
    Ok(sum_val)
}
