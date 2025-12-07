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
pub fn test_array_functions() -> Result<i32, Box<dyn std::error::Error>> {
    let z1 = vec![0; 5];
    let o1 = vec![1; 10];
    let f1 = vec![42; 8];
    let _z2 = vec![0; 100 as usize];
    Ok(z1.get(0usize).cloned().unwrap_or_default()
        + o1.get(0usize).cloned().unwrap_or_default()
        + f1.get(0usize).cloned().unwrap_or_default())
}
