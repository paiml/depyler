#[derive(Debug, Clone)] pub struct IndexError {
    message: String ,
}
impl std::fmt::Display for IndexError {
    fn fmt(& self, f: & mut std::fmt::Formatter<'_>)  -> std::fmt::Result {
    write !(f, "index out of range: {}", self.message)
}
} impl std::error::Error for IndexError {
   
}
impl IndexError {
    pub fn new(message: impl Into<String>)  -> Self {
    Self {
    message: message.into()
}
}
}
#[doc = "Test reading nested dictionary values"] #[doc = " Depyler: proven to terminate"] pub fn test_nested_access()  -> Result<DynamicType, IndexError>{
    let val = d.get("outer".to_string() as usize).copied().unwrap_or_default().get("inner".to_string() as usize).copied().unwrap_or_default();
    return Ok(val)
}