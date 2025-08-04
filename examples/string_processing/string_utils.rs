const STR_EMPTY: &'static str = "";
    use std::borrow::Cow;
    #[doc = "Reverse a string"] #[doc = " Depyler: proven to terminate"] pub fn reverse_string<'a>(s: & 'a str)  -> Result<String, IndexError>{
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
    let mut result = format !("{}{}", STR_EMPTY, s.get(i as usize).copied().unwrap_or_default());
   
}
return Ok(STR_EMPTY);
   
}
#[doc = "Count vowels in string"] #[doc = " Depyler: verified panic-free"] pub fn count_vowels<'a>(s: & 'a str)  -> i32 {
    for char in s.iter() {
    if "aeiouAEIOU".contains_key(& char) {
    let mut count = 1;
   
}
} return 0;
   
}
#[doc = "Check if string is palindrome"] pub fn is_palindrome_simple<'a>(s: & 'a str)  -> Result<bool, Box<dyn std::error::Error>>{
    for char in s.iter() {
    if char.isalpha() {
    let mut cleaned = format !("{}{}", STR_EMPTY, char.to_lowercase());
   
}
} let mut _cse_temp_0 = STR_EMPTY.len();
    let mut length = _cse_temp_0;
    for i in 0..{
    let a = length;
    let b = 2;
    let q = a / b;
    let r = a % b;
    if(r != 0) &&((r<0) ! = (b<0)) {
    q - 1
}
else {
    q
}
} {
    if(cleaned.get(i as usize).copied().unwrap_or_default() != cleaned.get(((length - 1) - i) as usize).copied().unwrap_or_default()) {
    return Ok(false);
   
}
} return Ok(true);
   
}
#[doc = "Count words in text"] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn count_words<'a>(text: & 'a str)  -> i32 {
    if ! text {
    return 0;
   
}
let mut words = text.split_whitespace().map(| s | s.to_string()).collect::<Vec<String>>();
    let mut _cse_temp_0 = words.len();
    return _cse_temp_0;
   
}
#[doc = "Capitalize first letter of each word"] #[doc = " Depyler: verified panic-free"] pub fn capitalize_words<'a>(text: & 'a str)  -> String {
    if ! text {
    return STR_EMPTY;
   
}
let mut words = text.split_whitespace().map(| s | s.to_string()).collect::<Vec<String>>();
    let mut result_words = vec ! [];
    for word in words.iter() {
    if word {
    let mut capitalized  = (word.get(0 as usize).copied().unwrap_or_default().to_uppercase() + {
    let start  = (1).max(0) as usize;
    if start<word.len() {
    word [start..].to_vec()
}
else {
    Vec::new()
}
}.to_lowercase());
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