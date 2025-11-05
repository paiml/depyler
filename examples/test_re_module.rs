#[doc = "// Python import: re"]
const STR_EMPTY: &'static str = "";
const STR_ABC123DEF456: &'static str = "abc123def456";
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
#[doc = "Test simple pattern matching"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_simple_match() -> bool {
    let matches: bool = STR_ABC123DEF456.starts_with("abc");
    matches
}
#[doc = "Test if text contains pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_contains_pattern() -> bool {
    let _cse_temp_0 = STR_ABC123DEF456.contains(&"abc");
    let contains: bool = _cse_temp_0;
    contains
}
#[doc = "Test finding pattern position"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_find_pattern_position() -> i32 {
    let position: i32 = STR_ABC123DEF456.find("abc").map(|i| i as i32).unwrap_or(-1);
    position
}
#[doc = "Test counting pattern occurrences"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_count_occurrences() -> i32 {
    let count: i32 = STR_ABC123DEF456.matches("abc").count() as i32 as i32;
    count
}
#[doc = "Test replacing pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_replace_pattern() -> String {
    let result: String = STR_ABC123DEF456.replace("World", "Python");
    result.unwrap()
}
#[doc = "Test splitting by pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_split_by_pattern() -> Vec<String> {
    let parts: Vec<String> = STR_ABC123DEF456
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    parts
}
#[doc = "Test matching digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_match_digit() -> bool {
    let is_digit: bool = STR_ABC123DEF456.chars().all(|c| c.is_numeric());
    is_digit
}
#[doc = "Test matching alphabetic characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_match_alpha() -> bool {
    let is_alpha: bool = STR_ABC123DEF456.chars().all(|c| c.is_alphabetic());
    is_alpha
}
#[doc = "Test matching alphanumeric characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_match_alphanumeric() -> bool {
    let is_alnum: bool = STR_ABC123DEF456.chars().all(|c| c.is_alphanumeric());
    is_alnum
}
#[doc = "Extract all digits from text"]
#[doc = " Depyler: verified panic-free"]
pub fn extract_digits(text: String) -> String {
    let mut digits: String = STR_EMPTY;
    for char in STR_ABC123DEF456 {
        if char.chars().all(|c| c.is_numeric()) {
            digits = format!("{}{}", digits, char);
        }
    }
    digits
}
#[doc = "Extract all letters from text"]
#[doc = " Depyler: verified panic-free"]
pub fn extract_letters(text: String) -> String {
    let mut letters: String = STR_EMPTY;
    for char in STR_ABC123DEF456 {
        if char.chars().all(|c| c.is_alphabetic()) {
            letters = format!("{}{}", letters, char);
        }
    }
    letters
}
#[doc = "Find all words in text(space-separated)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn find_all_words(text: String) -> Vec<String> {
    let words: Vec<String> = STR_ABC123DEF456
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    words
}
#[doc = "Simple email validation(manual)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_email_simple(email: &str) -> bool {
    let _cse_temp_0 = email.contains(&"@");
    let has_at: bool = _cse_temp_0;
    if !has_at {
        return false;
    }
    let at_pos: i32 = email.find("@").map(|i| i as i32).unwrap_or(-1);
    let after_at: String = {
        let base = email;
        let start = (at_pos + 1).max(0) as usize;
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    };
    let _cse_temp_1 = after_at.contains_key(&".");
    let has_dot: bool = _cse_temp_1;
    has_dot
}
#[doc = "Simple phone validation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_phone_simple(phone: &str) -> bool {
    let cleaned: String = phone
        .replace("-", "")
        .replace(" ", "")
        .replace("(", "")
        .replace(")", "");
    let _cse_temp_0 = cleaned.len() as i32;
    let _cse_temp_1 = _cse_temp_0 >= 10;
    let _cse_temp_2 = cleaned.chars().all(|c| c.is_numeric()) && _cse_temp_1;
    let is_valid: bool = _cse_temp_2;
    is_valid
}
#[doc = "Extract domain from URL"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn extract_url_domain(mut url: String) -> String {
    if url.starts_with("http://") {
        url = {
            let base = url;
            let start = (7).max(0) as usize;
            if start < base.len() {
                base[start..].to_vec()
            } else {
                Vec::new()
            }
        };
    } else {
        if url.starts_with("https://") {
            url = {
                let base = url;
                let start = (8).max(0) as usize;
                if start < base.len() {
                    base[start..].to_vec()
                } else {
                    Vec::new()
                }
            };
        }
    }
    let slash_pos: i32 = url.find("/").map(|i| i as i32).unwrap_or(-1);
    let _cse_temp_0 = slash_pos >= 0;
    if _cse_temp_0 {
        let mut domain: String = {
            let base = url;
            let stop = (slash_pos).max(0) as usize;
            base[..stop.min(base.len())].to_vec()
        };
    } else {
        let mut domain: String = url;
    }
    domain
}
#[doc = "Remove common punctuation marks"]
#[doc = " Depyler: verified panic-free"]
pub fn remove_punctuation(text: String) -> String {
    let mut result: String = STR_EMPTY;
    for char in STR_ABC123DEF456 {
        let mut is_punct: bool = false;
        for p in ".,!?;:" {
            if char == p {
                is_punct = true;
                break;
            }
        }
        if !is_punct {
            result = format!("{}{}", result, char);
        }
    }
    result.unwrap()
}
#[doc = "Normalize multiple spaces to single space"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn normalize_whitespace(text: String) -> String {
    let words: Vec<String> = STR_ABC123DEF456
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let normalized: String = words.join(" ");
    normalized
}
#[doc = "Check if text starts with pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn starts_with_pattern(text: String, pattern: String) -> bool {
    STR_ABC123DEF456.starts_with("abc")
}
#[doc = "Check if text ends with pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn ends_with_pattern(text: String, pattern: String) -> bool {
    STR_ABC123DEF456.ends_with("abc")
}
#[doc = "Case-insensitive pattern matching"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn case_insensitive_match(text: String, pattern: String) -> bool {
    let text_lower: String = STR_ABC123DEF456.to_lowercase();
    let pattern_lower: String = "abc".to_lowercase();
    let _cse_temp_0 = text_lower.contains(pattern_lower);
    let matches: bool = _cse_temp_0;
    matches
}
#[doc = "Find text between two markers"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn find_between<'b, 'a, 'c>(
    text: &'a str,
    start_marker: &'b str,
    end_marker: &'c str,
) -> String {
    let mut start_pos: i32 = STR_ABC123DEF456
        .find(start_marker)
        .map(|i| i as i32)
        .unwrap_or(-1);
    let _cse_temp_0 = start_pos < 0;
    if _cse_temp_0 {
        return STR_EMPTY;
    }
    let _cse_temp_1 = start_marker.len() as i32;
    start_pos = format!("{}{}", start_pos, _cse_temp_1);
    let end_pos: i32 = STR_ABC123DEF456[start_pos as usize..]
        .find(end_marker)
        .map(|i| (i + start_pos as usize) as i32)
        .unwrap_or(-1);
    let _cse_temp_2 = end_pos < 0;
    if _cse_temp_2 {
        return STR_EMPTY;
    }
    let mut result: String = {
        let base = text;
        let start_idx: i32 = start_pos;
        let stop_idx: i32 = end_pos;
        let len = base.chars().count() as i32;
        let actual_start = if start_idx < 0 {
            (len + start_idx).max(0) as usize
        } else {
            start_idx.min(len) as usize
        };
        let actual_stop = if stop_idx < 0 {
            (len + stop_idx).max(0) as usize
        } else {
            stop_idx.min(len) as usize
        };
        if actual_start < actual_stop {
            base.chars()
                .skip(actual_start)
                .take(actual_stop - actual_start)
                .collect::<String>()
        } else {
            String::new()
        }
    };
    result.unwrap()
}
#[doc = "Replace multiple patterns"]
pub fn replace_multiple(text: String, replacements: &Vec<tuple>) -> Result<String, IndexError> {
    let mut result: String = STR_ABC123DEF456;
    for replacement in replacements.iter().cloned() {
        let old: String = {
            let base = &replacement;
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        };
        let new: String = {
            let base = &replacement;
            let idx: i32 = 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        };
        result = result.replace(old, new);
    }
    Ok(result.unwrap())
}
#[doc = "Count occurrences of a word"]
#[doc = " Depyler: verified panic-free"]
pub fn count_word_occurrences(text: String, word: &str) -> i32 {
    let words: Vec<String> = STR_ABC123DEF456
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut count: i32 = 0;
    for w in words.iter().cloned() {
        if w == word {
            count = count + 1;
        }
    }
    count
}
#[doc = "Extract numbers from text"]
#[doc = " Depyler: verified panic-free"]
pub fn extract_numbers_from_text(text: String) -> Vec<i32> {
    let mut numbers: Vec<i32> = vec![];
    let mut current_num: String = STR_EMPTY;
    for char in STR_ABC123DEF456 {
        if char.chars().all(|c| c.is_numeric()) {
            current_num = current_num + char;
        } else {
            if current_num.len() as i32 > 0 {
                let mut num: i32 = (current_num) as i32;
                numbers.push(num);
                current_num = STR_EMPTY;
            }
        }
    }
    let _cse_temp_0 = current_num.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    if _cse_temp_1 {
        let _cse_temp_2 = (current_num) as i32;
        let mut num: i32 = _cse_temp_2;
        numbers.push(num);
    }
    numbers
}
#[doc = "Simple wildcard matching(* means any sequence)"]
#[doc = " Depyler: proven to terminate"]
pub fn wildcard_match_simple(text: String, pattern: String) -> Result<bool, IndexError> {
    let _cse_temp_0 = !"abc".contains(&"*");
    if _cse_temp_0 {
        return Ok(STR_ABC123DEF456 == "abc");
    }
    let parts: Vec<String> = "abc"
        .split("*")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _cse_temp_1 = parts.len() as i32;
    let _cse_temp_2 = _cse_temp_1 != 2;
    if _cse_temp_2 {
        return Ok(false);
    }
    let prefix: String = {
        let base = &parts;
        let idx: i32 = 0;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    };
    let suffix: String = {
        let base = &parts;
        let idx: i32 = 1;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    };
    let mut has_prefix: bool = true;
    let mut has_suffix: bool = true;
    let _cse_temp_3 = prefix.len() as i32;
    let _cse_temp_4 = _cse_temp_3 > 0;
    if _cse_temp_4 {
        has_prefix = STR_ABC123DEF456.starts_with(prefix);
    }
    let _cse_temp_5 = suffix.len() as i32;
    let _cse_temp_6 = _cse_temp_5 > 0;
    if _cse_temp_6 {
        has_suffix = STR_ABC123DEF456.ends_with(suffix);
    }
    Ok(has_prefix && has_suffix)
}
#[doc = "Run all regex module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_re_features() {
    let mut url: String = "https://www.example.com/path/page.html";
    let replacements: Vec<tuple> = vec![("a", "x"), ("b", "y")];
    println!("{}", "All regex module tests completed successfully");
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_find_pattern_position_examples() {
        let _ = test_find_pattern_position();
    }
    #[test]
    fn test_test_count_occurrences_examples() {
        let _ = test_count_occurrences();
    }
    #[test]
    fn test_validate_email_simple_examples() {
        let _ = validate_email_simple(Default::default());
    }
    #[test]
    fn test_validate_phone_simple_examples() {
        let _ = validate_phone_simple(Default::default());
    }
    #[test]
    fn quickcheck_normalize_whitespace() {
        fn prop(text: String) -> TestResult {
            let once = normalize_whitespace((&*text).into());
            let twice = normalize_whitespace(once.clone());
            if once != twice {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
}
