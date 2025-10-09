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
#[doc = " Depyler: proven to terminate"] pub fn test_array_functions()  -> Result<DynamicType, IndexError>{
    let _cse_temp_0 = z1.get(0 as usize).copied().unwrap_or_default() + o1.get(0 as usize).copied().unwrap_or_default();
    let _cse_temp_1 = _cse_temp_0 + f1.get(0 as usize).copied().unwrap_or_default();
    return Ok(_cse_temp_1)
}