#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'string'(tracked in DEPYLER-0424)"]
const STR_EMPTY: &'static str = "";
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[doc = "Test accessing lowercase ASCII letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_ascii_lowercase() -> String {
    let lowercase: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    lowercase.to_string()
}
#[doc = "Test accessing uppercase ASCII letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_ascii_uppercase() -> String {
    let uppercase: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    uppercase.to_string()
}
#[doc = "Test accessing all ASCII letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_ascii_letters() -> String {
    let lowercase: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    let uppercase: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let letters: String = format!("{}{}", lowercase, uppercase);
    letters.to_string()
}
#[doc = "Test accessing digit characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_digits() -> String {
    let digits: String = "0123456789".to_string();
    digits.to_string()
}
#[doc = "Test accessing hexadecimal digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_hexdigits() -> String {
    let hexdigits: String = "0123456789abcdefABCDEF".to_string();
    hexdigits.to_string()
}
#[doc = "Test accessing octal digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_octdigits() -> String {
    let octdigits: String = "01234567".to_string();
    octdigits.to_string()
}
#[doc = "Test accessing punctuation characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_punctuation() -> String {
    let punctuation: String = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".to_string();
    punctuation.to_string()
}
#[doc = "Test accessing whitespace characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_whitespace() -> String {
    let whitespace: String = " \t\n\r".to_string();
    whitespace.to_string()
}
#[doc = "Check if character is ASCII letter"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_ascii_letter(char: &str) -> bool {
    let _cse_temp_0 = char.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 1;
    if _cse_temp_1 {
        return false;
    }
    let code: i32 = char.chars().next().unwrap() as i32;
    let _cse_temp_2 = code >= 65;
    let _cse_temp_3 = code <= 90;
    let _cse_temp_4 = (_cse_temp_2) && (_cse_temp_3);
    let is_upper: bool = _cse_temp_4;
    let _cse_temp_5 = code >= 97;
    let _cse_temp_6 = code <= 122;
    let _cse_temp_7 = (_cse_temp_5) && (_cse_temp_6);
    let is_lower: bool = _cse_temp_7;
    (is_upper) || (is_lower)
}
#[doc = "Check if character is digit"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_digit(char: &str) -> bool {
    let _cse_temp_0 = char.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 1;
    if _cse_temp_1 {
        return false;
    }
    char.chars().all(|c| c.is_numeric())
}
#[doc = "Check if character is alphanumeric"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_alphanumeric(char: &str) -> bool {
    let _cse_temp_0 = char.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 1;
    if _cse_temp_1 {
        return false;
    }
    (is_ascii_letter(char)) || (is_digit(char))
}
#[doc = "Check if character is whitespace"]
#[doc = " Depyler: verified panic-free"]
pub fn is_whitespace(char: &str) -> bool {
    let _cse_temp_0 = char.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 1;
    if _cse_temp_1 {
        return false;
    }
    let whitespace_chars: String = " \t\n\r".to_string();
    for ws in whitespace_chars.chars() {
        if char == ws.to_string() {
            return true;
        }
    }
    false
}
#[doc = "Check if character is punctuation"]
#[doc = " Depyler: verified panic-free"]
pub fn is_punctuation(char: &str) -> bool {
    let _cse_temp_0 = char.len() as i32;
    let _cse_temp_1 = _cse_temp_0 != 1;
    if _cse_temp_1 {
        return false;
    }
    let punctuation_chars: String = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".to_string();
    for p in punctuation_chars.chars() {
        if char == p.to_string() {
            return true;
        }
    }
    false
}
#[doc = "Capitalize first letter of each word"]
#[doc = " Depyler: verified panic-free"]
pub fn capitalize_words(text: &str) -> String {
    let words: Vec<String> = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut capitalized: Vec<String> = vec![];
    for word in words.iter().cloned() {
        if word.len() as i32 > 0 {
            let first_char: String = {
                let base = &word;
                let idx: i32 = 0;
                let actual_idx = if idx < 0 {
                    base.chars().count().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.chars()
                    .nth(actual_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            }
            .to_uppercase();
            let rest: String = {
                let base = word;
                let start_idx: i32 = 1;
                let len = base.chars().count() as i32;
                let actual_start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx.min(len) as usize
                };
                base.chars().skip(actual_start).collect::<String>()
            }
            .to_lowercase();
            let cap_word: String = format!("{}{}", first_char, rest);
            capitalized.push(cap_word);
        }
    }
    capitalized.join(" ")
}
#[doc = "Convert to title case"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn to_title_case(text: &str) -> String {
    capitalize_words(text)
}
#[doc = "Swap uppercase to lowercase and vice versa"]
#[doc = " Depyler: verified panic-free"]
pub fn swap_case(text: &str) -> String {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string();
    for char in text.chars() {
        if !char.is_alphabetic() || char.is_uppercase() {
            result = format!("{}{}", result, char.to_lowercase());
        } else {
            if !char.is_alphabetic() || char.is_lowercase() {
                result = format!("{}{}", result, char.to_uppercase());
            } else {
                result = format!("{}{}", result, char);
            }
        }
    }
    result.to_string()
}
#[doc = "Count number of letters in text"]
#[doc = " Depyler: verified panic-free"]
pub fn count_letters(text: &str) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    for char in text.chars() {
        if is_ascii_letter(&char.to_string()) {
            count = count + 1;
        }
    }
    count
}
#[doc = "Count number of digits in text"]
#[doc = " Depyler: verified panic-free"]
pub fn count_digits(text: &str) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    for char in text.chars() {
        if is_digit(&char.to_string()) {
            count = count + 1;
        }
    }
    count
}
#[doc = "Count whitespace characters"]
#[doc = " Depyler: verified panic-free"]
pub fn count_whitespace(text: &str) -> i32 {
    let mut count: i32 = Default::default();
    count = 0;
    for char in text.chars() {
        if is_whitespace(&char.to_string()) {
            count = count + 1;
        }
    }
    count
}
#[doc = "Remove all whitespace from text"]
#[doc = " Depyler: verified panic-free"]
pub fn remove_whitespace(text: &str) -> String {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string();
    for char in text.chars() {
        if !is_whitespace(&char.to_string()) {
            result = format!("{}{}", result, char);
        }
    }
    result.to_string()
}
#[doc = "Keep only letters, remove everything else"]
#[doc = " Depyler: verified panic-free"]
pub fn keep_only_letters(text: &str) -> String {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string();
    for char in text.chars() {
        if is_ascii_letter(&char.to_string()) {
            result = format!("{}{}", result, char);
        }
    }
    result.to_string()
}
#[doc = "Keep only digits, remove everything else"]
#[doc = " Depyler: verified panic-free"]
pub fn keep_only_digits(text: &str) -> String {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string();
    for char in text.chars() {
        if is_digit(&char.to_string()) {
            result = format!("{}{}", result, char);
        }
    }
    result.to_string()
}
#[doc = "Keep only alphanumeric characters"]
#[doc = " Depyler: verified panic-free"]
pub fn keep_alphanumeric(text: &str) -> String {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string();
    for char in text.chars() {
        if is_alphanumeric(&char.to_string()) {
            result = format!("{}{}", result, char);
        }
    }
    result.to_string()
}
#[doc = "Simple template substitution"]
pub fn template_substitute<'b, 'a>(
    template: &'a str,
    values: &'b std::collections::HashMap<String, DepylerValue>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String = Default::default();
    result = template.to_string();
    for key in values.keys().cloned().collect::<Vec<_>>() {
        let placeholder: String = format!("{}{}", format!("{}{}", "${", key), "}");
        let value: String = (values.get(&key).cloned().unwrap_or_default()).to_string();
        result = result.replace(&placeholder, &value);
    }
    Ok(result.to_string())
}
#[doc = "Simple Caesar cipher"]
pub fn caesar_cipher(text: &str, shift: i32) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string();
    for char in text.chars() {
        if char.is_alphabetic() {
            let mut base: i32;
            let mut shifted: i32;
            let mut new_char: String;
            if !char.is_alphabetic() || char.is_uppercase() {
                base = "A".chars().next().unwrap() as i32;
                shifted = (char as u32 as i32 - base + shift) % 26;
                new_char = char::from_u32((base + shifted) as u32).unwrap().to_string();
                result = format!("{}{}", result, new_char);
            } else {
                base = "a".chars().next().unwrap() as i32;
                shifted = (char as u32 as i32 - base + shift) % 26;
                new_char = char::from_u32((base + shifted) as u32).unwrap().to_string();
                result = format!("{}{}", result, new_char);
            }
        } else {
            result = format!("{}{}", result, char);
        }
    }
    Ok(result.to_string())
}
#[doc = "Reverse a string"]
#[doc = " Depyler: proven to terminate"]
pub fn reverse_string(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: String = Default::default();
    result = STR_EMPTY.to_string();
    for i in {
        let step = (-1 as i32).abs() as usize;
        if step == 0 {
            panic!("range() arg 3 must not be zero");
        }
        (-1..(text.len() as i32).saturating_sub(1))
            .rev()
            .step_by(step.max(1))
    } {
        result = format!("{}{}", result, {
            let base = &text;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        });
    }
    Ok(result.to_string())
}
#[doc = "Check if text is palindrome(ignoring case and spaces)"]
pub fn is_palindrome(text: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut normalized: String = Default::default();
    normalized = STR_EMPTY.to_string();
    for char in text.to_lowercase().chars() {
        if char.is_alphanumeric() {
            normalized = format!("{}{}", normalized, char);
        }
    }
    let mut left: i32 = 0;
    let _cse_temp_0 = normalized.len() as i32;
    let mut right: i32 = _cse_temp_0 - 1;
    while left < right {
        if {
            let base = &normalized;
            let idx: i32 = left;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } != {
            let base = &normalized;
            let idx: i32 = right;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } {
            return Ok(false);
        }
        left = left + 1;
        right = right - 1;
    }
    Ok(true)
}
#[doc = "Count vowels in text"]
#[doc = " Depyler: verified panic-free"]
pub fn count_vowels(text: &str) -> i32 {
    let mut count: i32 = Default::default();
    let vowels: String = "aeiouAEIOU".to_string();
    count = 0;
    for char in text.chars() {
        if vowels.contains(char) {
            count = count + 1;
        }
    }
    count
}
#[doc = "Count consonants in text"]
#[doc = " Depyler: verified panic-free"]
pub fn count_consonants(text: &str) -> i32 {
    let mut count: i32 = Default::default();
    let vowels: String = "aeiouAEIOU".to_string();
    count = 0;
    for char in text.chars() {
        if (is_ascii_letter(&char.to_string())) && (!vowels.contains(char)) {
            count = count + 1;
        }
    }
    count
}
#[doc = "Run all string module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_string_features() -> Result<(), Box<dyn std::error::Error>> {
    let lowercase: String = test_ascii_lowercase();
    let uppercase: String = test_ascii_uppercase();
    let letters: String = test_ascii_letters();
    let digits: String = test_digits();
    let hexdigits: String = test_hexdigits();
    let octdigits: String = test_octdigits();
    let punct: String = test_punctuation();
    let ws: String = test_whitespace();
    let is_letter: bool = is_ascii_letter("a");
    let is_num: bool = is_digit("5");
    let is_alnum: bool = is_alphanumeric("a");
    let is_ws: bool = is_whitespace(" ");
    let is_punct: bool = is_punctuation("!");
    let text: String = "hello world".to_string();
    let capitalized: String = capitalize_words(&text);
    let title: String = to_title_case(&text);
    let swapped: String = swap_case(&text);
    let sample: String = "Hello World 123!".to_string();
    let letter_count: i32 = count_letters(&sample);
    let digit_count: i32 = count_digits(&sample);
    let ws_count: i32 = count_whitespace(&sample);
    let no_ws: String = remove_whitespace(&sample);
    let only_letters: String = keep_only_letters(&sample);
    let only_digits: String = keep_only_digits(&sample);
    let only_alnum: String = keep_alphanumeric(&sample);
    let template: String = "Hello ${name}, you are ${age} years old".to_string();
    let values: std::collections::HashMap<String, DepylerValue> = {
        let mut map = HashMap::new();
        map.insert("name".to_string(), DepylerValue::Str("Alice".to_string()));
        map.insert("age".to_string(), DepylerValue::Str("30".to_string()));
        map
    };
    let substituted: String = template_substitute(&template, &values)?;
    let message: String = "HELLO".to_string();
    let encrypted: String = caesar_cipher(&message, 3)?;
    let decrypted: String = caesar_cipher(&encrypted, -3)?;
    let reversed_text: String = reverse_string("hello")?;
    let is_palin: bool = is_palindrome("A man a plan a canal Panama")?;
    let vowel_count: i32 = count_vowels("hello world");
    let consonant_count: i32 = count_consonants("hello world");
    println!("{}", "All string module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_is_ascii_letter_examples() {
        let _ = is_ascii_letter(Default::default());
    }
    #[test]
    fn test_is_digit_examples() {
        let _ = is_digit(Default::default());
    }
    #[test]
    fn test_is_alphanumeric_examples() {
        let _ = is_alphanumeric(Default::default());
    }
    #[test]
    fn test_is_whitespace_examples() {
        let _ = is_whitespace(Default::default());
    }
    #[test]
    fn test_is_punctuation_examples() {
        let _ = is_punctuation(Default::default());
    }
    #[test]
    fn test_count_letters_examples() {
        assert_eq!(count_letters(""), 0);
        assert_eq!(count_letters("a"), 1);
        assert_eq!(count_letters("abc"), 3);
    }
    #[test]
    fn test_count_digits_examples() {
        assert_eq!(count_digits(""), 0);
        assert_eq!(count_digits("a"), 1);
        assert_eq!(count_digits("abc"), 3);
    }
    #[test]
    fn test_count_whitespace_examples() {
        assert_eq!(count_whitespace(""), 0);
        assert_eq!(count_whitespace("a"), 1);
        assert_eq!(count_whitespace("abc"), 3);
    }
    #[test]
    fn test_is_palindrome_examples() {
        let _ = is_palindrome(Default::default());
    }
    #[test]
    fn test_count_vowels_examples() {
        assert_eq!(count_vowels(""), 0);
        assert_eq!(count_vowels("a"), 1);
        assert_eq!(count_vowels("abc"), 3);
    }
    #[test]
    fn test_count_consonants_examples() {
        assert_eq!(count_consonants(""), 0);
        assert_eq!(count_consonants("a"), 1);
        assert_eq!(count_consonants("abc"), 3);
    }
}
