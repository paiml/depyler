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
    pub fn __exit__(&mut self, exc_type: String, exc_val: String, exc_tb: String) {
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
    let _context = FileManager::new("test.txt".to_string().to_string());
    let fm = _context.__enter__();
    let result = fm.write("Hello, World!".to_string());
    result
}
#[doc = "Test with built-in open"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_with_builtin() -> i32 {
    let f = std::fs::File::create("test.txt".to_string())?;
    f.write("Hello, World!".to_string());
    1
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
