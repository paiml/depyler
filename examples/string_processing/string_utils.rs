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
#[doc = "Reverse a string"] #[doc = " Depyler: proven to terminate"] pub fn reverse_string<'a>(s: & 'a str)  -> Result<String, IndexError>{
    let mut result = "".to_string();
    for i in {
    let step  = (- 1).abs() as usize;
    if step == 0 {
    panic !("range() arg 3 must not be zero");
   
}
if step == 1 {
   (- 1..s.len().saturating_sub(1)).rev()
}
else {
   (- 1..s.len().saturating_sub(1)).rev().step_by(step)
}
} {
    result = result + s.get(i as usize).copied().unwrap_or_default();
   
}
return Ok(result);
   
}
#[doc = "Count vowels in string"] #[doc = " Depyler: verified panic-free"] pub fn count_vowels<'a>(s: & 'a str)  -> i32 {
    let mut count = 0;
    for char in s.iter() {
    if "aeiouAEIOU".contains_key(& char) {
    count = count + 1;
   
}
} return count;
   
}
#[doc = "Check if string is palindrome"] pub fn is_palindrome_simple<'a>(s: & 'a str)  -> Result<bool, Box<dyn std::error::Error>>{
    let mut cleaned = std::borrow::Cow::Borrowed("");
    for char in s.iter() {
    if char.isalpha() {
    cleaned = cleaned + char.to_lowercase();
   
}
} let _cse_temp_0 = cleaned.len();
    let length = _cse_temp_0;
    for i in 0..{
    let a = length;
    let b = 2;
    let q = a / b;
    let r = a % b;
    let r_negative = r<0;
    let b_negative = b<0;
    let r_nonzero = r != 0;
    let signs_differ = r_negative != b_negative;
    let needs_adjustment = r_nonzero && signs_differ;
    if needs_adjustment {
    q - 1
}
else {
    q
}
} {
    if cleaned.get(i as usize).copied().unwrap_or_default() != cleaned.get(length - 1 - i as usize).copied().unwrap_or_default() {
    return Ok(false);
   
}
} return Ok(true);
   
}
#[doc = "Count words in text"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn count_words<'a>(text: & 'a str)  -> i32 {
    if ! text {
    return 0;
   
}
let words = text.split_whitespace().map(| s | s.to_string()).collect::<Vec<String>>();
    let _cse_temp_0 = words.len();
    return _cse_temp_0;
   
}
#[doc = "Capitalize first letter of each word"] #[doc = " Depyler: verified panic-free"] pub fn capitalize_words<'a>(text: & 'a str)  -> String {
    if ! text {
    return "".to_string();
   
}
let words = text.split_whitespace().map(| s | s.to_string()).collect::<Vec<String>>();
    let result_words = vec ! [];
    for word in words.iter() {
    if word {
    let capitalized = word.get(0 as usize).copied().unwrap_or_default().to_uppercase() + {
    let start  = (1).max(0) as usize;
    if start<word.len() {
    word [start..].to_vec()
}
else {
    Vec::new()
}
}.to_lowercase();
    result_words.push(capitalized);
   
}
} return result_words.join (" ".to_string());
   
}
#[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_count_vowels_examples() {
    assert_eq !(count_vowels(0), 0);
    assert_eq !(count_vowels(1), 1);
    assert_eq !(count_vowels(- 1), - 1);
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_is_palindrome_simple_examples() {
    let _ = is_palindrome_simple(Default::default());
   
}
} #[cfg(test)] mod tests {
    use super::*;
    use quickcheck::{
    quickcheck, TestResult };
    #[test] fn test_count_words_examples() {
    assert_eq !(count_words(0), 0);
    assert_eq !(count_words(1), 1);
    assert_eq !(count_words(- 1), - 1);
   
}
}