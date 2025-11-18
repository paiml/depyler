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
#[doc = "Count word frequencies in text"]
pub fn word_frequency(text: &str) -> Result<HashMap<String, i32>, IndexError> {
    let words = text
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut frequency: HashMap<String, i32> = {
        let map = HashMap::new();
        map
    };
    for word in words.iter().cloned() {
        let mut clean_word = STR_EMPTY;
        for _char in word.chars() {
            let char = _char.to_string();
            if char.chars().all(|c| c.is_alphabetic()) {
                clean_word = clean_word + char;
            }
        }
        if clean_word {
            if frequency.contains_key(&clean_word) {
                {
                    let _key = clean_word;
                    let _old_val = frequency.get(&_key).cloned().unwrap_or_default();
                    frequency.insert(_key, _old_val + 1);
                }
            } else {
                frequency.insert(clean_word, 1);
            }
        }
    }
    Ok(frequency)
}
#[doc = "Group words that are anagrams of each other"]
#[doc = " Depyler: verified panic-free"]
pub fn find_anagrams(words: &Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = {
        let map = HashMap::new();
        map
    };
    for word in words.iter().cloned() {
        let sorted_chars = {
            let mut __sorted_result = word.to_lowercase().clone();
            __sorted_result.sort();
            __sorted_result
        }
        .join("");
        if groups.contains_key(&sorted_chars) {
            groups
                .get(sorted_chars as usize)
                .cloned()
                .unwrap_or_default()
                .push(word);
        } else {
            groups.insert(sorted_chars, vec![word]);
        }
    }
    let mut result: Vec<Vec<String>> = vec![];
    for group in groups.values().cloned().collect::<Vec<_>>() {
        if group.len() as i32 > 1 {
            result.push(group);
        }
    }
    result
}
#[doc = "Find the longest common prefix among strings"]
pub fn longest_common_prefix(strings: &Vec<String>) -> Result<String, IndexError> {
    if strings.is_empty() {
        return Ok(STR_EMPTY);
    }
    let _cse_temp_0 = strings.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 1;
    if _cse_temp_1 {
        return Ok(strings.get(0usize).cloned().unwrap_or_default());
    }
    let _cse_temp_2 = strings.get(0usize).cloned().unwrap_or_default().len() as i32;
    let mut min_length = _cse_temp_2;
    for s in {
        let base = strings;
        let start = (1).max(0) as usize;
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    } {
        if (s.len() as i32) < min_length {
            min_length = s.len() as i32;
        }
    }
    let mut prefix = STR_EMPTY;
    for i in 0..min_length {
        let char = strings
            .get(0usize)
            .cloned()
            .unwrap_or_default()
            .get(i as usize)
            .cloned()
            .unwrap_or_default();
        let mut all_match = true;
        for s in {
            let base = strings;
            let start = (1).max(0) as usize;
            if start < base.len() {
                base[start..].to_vec()
            } else {
                Vec::new()
            }
        } {
            if {
                let base = &s;
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
            } != char
            {
                all_match = false;
                break;
            }
        }
        if all_match {
            prefix = format!("{}{}", prefix, char);
        } else {
            break;
        }
    }
    Ok(prefix)
}
#[doc = "Check if string is a palindrome(ignoring case and non-alphanumeric)"]
pub fn is_palindrome(s: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut cleaned = STR_EMPTY;
    for char in s.to_lowercase() {
        if char.chars().all(|c| c.is_alphanumeric()) {
            cleaned = cleaned + char;
        }
    }
    let _cse_temp_0 = cleaned.len() as i32;
    let length = _cse_temp_0;
    for i in 0.. {
        let a = length;
        let b = 2;
        let q = a / b;
        let r = a % b;
        let r_negative = r < 0;
        let b_negative = b < 0;
        let r_nonzero = r != 0;
        let signs_differ = r_negative != b_negative;
        let needs_adjustment = r_nonzero && signs_differ;
        if needs_adjustment {
            q - 1
        } else {
            q
        }
    }
    {
        if cleaned.get(i as usize).cloned().unwrap_or_default() != {
            let base = &cleaned;
            let idx: i32 = length - 1 - i;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        } {
            return Ok(false);
        }
    }
    Ok(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_find_anagrams_examples() {
        assert_eq!(find_anagrams(vec![]), vec![]);
        assert_eq!(find_anagrams(vec![1]), vec![1]);
    }
    #[test]
    fn test_is_palindrome_examples() {
        let _ = is_palindrome(Default::default());
    }
}
