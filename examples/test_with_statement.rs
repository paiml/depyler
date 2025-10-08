#[derive(Debug, Clone)] pub struct FileManager {
    pub filename: String, pub file :()
}
impl FileManager {
    pub fn new(filename: String)  -> Self {
    Self {
    filename, file: Default::default()
}
} pub fn __enter__(& mut self) {
    self.file = self.filename;
    return self;
   
}
pub fn __exit__(& mut self, exc_type: DynamicType, exc_val: DynamicType, exc_tb: DynamicType) {
    self.file  = ();
    return false;
   
}
pub fn write(& mut self, data: String) {
    return data.len();
   
}
} #[doc = "Test basic with statement"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_simple_with()  -> DynamicType {
    { let mut fm = FileManager::new("test.txt".to_string());
    let result = fm.write("Hello, World!".to_string());
   
}
return result;
   
}
#[doc = "Test with built-in open"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_with_builtin ()  -> DynamicType {
    { let mut f = open("test.txt".to_string(), "w".to_string());
    f.write("Hello, World!".to_string());
   
}
return 1
}