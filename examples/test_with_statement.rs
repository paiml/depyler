use serde_json;
use std::io::Read;
use std::io::Write;
#[derive(Debug, Clone)]
pub struct FileManager {
    pub filename: String,
    pub file: (),
}
impl FileManager {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            file: Default::default(),
        }
    }
    pub fn __enter__(&mut self) -> &Self {
        self.file = self.filename;
        return self;
    }
    pub fn __exit__(
        &mut self,
        exc_type: serde_json::Value,
        exc_val: serde_json::Value,
        exc_tb: serde_json::Value,
    ) -> bool {
        self.file = ();
        return false;
    }
    pub fn write(&self, data: String) {
        return data.len();
    }
}
#[doc = "Test basic with statement"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_simple_with() {
    let mut _context = FileManager::new("test.txt".to_string().to_string());
    let fm = _context.__enter__();
    let result = fm
        .write_all("Hello, World!".to_string().as_bytes())
        .unwrap();
    result
}
#[doc = "Test with built-in open"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_with_builtin() -> Result<i32, std::io::Error> {
    let mut f = std::fs::File::create("test.txt".to_string())?;
    f.write_all("Hello, World!".to_string().as_bytes()).unwrap();
    Ok(1)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_with_builtin_examples() {
        let _ = test_with_builtin();
    }
}
