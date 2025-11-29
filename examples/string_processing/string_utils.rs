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
#[doc = "Reverse a string"]
#[doc = " Depyler: proven to terminate"]
pub fn reverse_string(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = "".to_string().to_string();
    for i in {
        let step = (-1 as i32).abs() as usize;
        if step == 0 {
            panic!("range() arg 3 must not be zero");
        }
        (-1..(s.len() as i32).saturating_sub(1))
            .rev()
            .step_by(step.max(1))
    } {
        result = format!("{}{}", result, {
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
        });
    }
    Ok(result.to_string())
}
#[doc = "Count vowels in string"]
#[doc = " Depyler: verified panic-free"]
pub fn count_vowels(s: &str) -> i32 {
    let vowels = "aeiouAEIOU";
    let mut count = 0;
    for _char in s.chars() {
        let char = _char.to_string();
        if vowels.get(&char).is_some() {
            count = count + 1;
        }
    }
    count
}
#[doc = "Check if string is palindrome"]
pub fn is_palindrome_simple(s: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut cleaned = "".to_string().to_string();
    for _char in s.chars() {
        let char = _char.to_string();
        if char.chars().all(|c| c.is_alphabetic()) {
            cleaned = cleaned + char.to_lowercase();
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
#[doc = "Count words in text"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn count_words(text: &str) -> i32 {
    if text.is_empty() {
        return 0;
    }
    let words = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    words.len() as i32 as i32
}
#[doc = "Capitalize first letter of each word"]
#[doc = " Depyler: verified panic-free"]
pub fn capitalize_words(text: &str) -> String {
    if text.is_empty() {
        return "".to_string();
    }
    let words = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut result_words = vec![];
    for word in words.iter().cloned() {
        if !word.is_empty() {
            let capitalized = format!(
                "{}{}",
                {
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
                .to_uppercase(),
                {
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
                .to_lowercase()
            );
            result_words.push(capitalized);
        }
    }
    result_words.join(" ")
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_count_vowels_examples() {
        assert_eq!(count_vowels(""), 0);
        assert_eq!(count_vowels("a"), 1);
        assert_eq!(count_vowels("abc"), 3);
    }
    #[test]
    fn test_is_palindrome_simple_examples() {
        let _ = is_palindrome_simple(Default::default());
    }
    #[test]
    fn test_count_words_examples() {
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("a"), 1);
        assert_eq!(count_words("abc"), 3);
    }
}
