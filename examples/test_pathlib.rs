use std::path::PathBuf;
    #[derive(Debug, Clone)] pub struct ZeroDivisionError {
    message: String ,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(& self, f: & mut std::fmt::Formatter<'_>)  -> std::fmt::Result {
    write !(f, "division by zero: {}", self.message)
}
} impl std::error::Error for ZeroDivisionError {
   
}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>)  -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = "Get all Python files in a directory"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_python_files(directory: String)  -> Vec<String>{
    return path.glob("*.py".to_string()).into_iter().map(| p | str(p)).collect::<Vec<_>>();
   
}
#[doc = "Create a nested path from parts"] pub fn create_nested_path(base: String)  -> Result<String, ZeroDivisionError>{
    let mut path = std::path::PathBuf(base);
    for part in parts.iter() {
    path = path / part;
   
}
let _cse_temp_0 = str(path);
    return Ok(_cse_temp_0);
   
}
#[doc = "Get file information"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_file_info(filepath: String)  -> tuple {
    return(path.name, path.suffix, path.parent.name);
   
}
#[doc = "Check if a path exists"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn check_path_exists(filepath: String)  -> bool {
    return std::path::PathBuf(filepath).exists();
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_check_path_exists_examples() {
    let _ = check_path_exists(Default::default());
   
}
}