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
#[doc = " Depyler: proven to terminate"] pub fn test_arrays()  -> Result<DynamicType, IndexError>{
    let _cse_temp_0 = arr1.get(0 as usize).copied().unwrap_or_default() + arr1.get(1 as usize).copied().unwrap_or_default();
    let sum_val = _cse_temp_0;
    return Ok(sum_val)
}