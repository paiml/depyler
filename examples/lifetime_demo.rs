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
#[doc = "Get the length of a string without consuming it"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_length<'a>(s: & 'a str)  -> i32 {
    let _cse_temp_0 = s.len();
    return _cse_temp_0;
   
}
#[doc = "Extract the first word from a string"] #[doc = " Depyler: proven to terminate"] pub fn first_word<'a>(s: & 'a str)  -> Result<String, IndexError>{
    let words = s.split_whitespace().map(| s | s.to_string()).collect::<Vec<String>>();
    if words {
    return Ok(words.get(0 as usize).copied().unwrap_or_default());
   
}
return Ok("".to_string());
   
}
#[doc = "Append an exclamation mark to a string"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn append_exclamation(mut s: String)  -> String {
    s = format !("{}{}", s, "!");
    return s;
   
}
#[doc = "Return the longest of two strings"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn longest<'static>(x: Cow<'static, str>, y: Cow<'static, str>)  -> Cow<'static, str>{
    let _cse_temp_0 = x.len();
    let _cse_temp_1 = y.len();
    let _cse_temp_2 = _cse_temp_0>_cse_temp_1;
    if _cse_temp_2 {
    return x;
   
}
else {
    return y;
   
}
} #[doc = "Modify a string in place"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn modify_string(mut s: String)  -> DynamicType {
    s = format !("{}{}", s, " modified");
    return();
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_get_length_examples() {
    assert_eq !(get_length(0), 0);
    assert_eq !(get_length(1), 1);
    assert_eq !(get_length(- 1), - 1);
   
}
}