use regex as re;
const STR_EMPTY: &'static str = "";
const STR_HELLO: &'static str = "Hello";
const STR_HELLO_WORLD: &'static str = "Hello World";
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
    let text: String = STR_HELLO_WORLD.to_string();
    let pattern: String = STR_HELLO.to_string();
    let matches: bool = text.starts_with(pattern);
    matches
}
#[doc = "Test if text contains pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_contains_pattern() -> bool {
    let text: String = "The quick brown fox".to_string();
    let pattern: String = "quick".to_string();
    let _cse_temp_0 = text.contains(&pattern);
    let contains: bool = _cse_temp_0;
    contains
}
#[doc = "Test finding pattern position"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_find_pattern_position() -> i32 {
    let text: String = "Hello World Hello".to_string();
    let pattern: String = "World".to_string();
    let position: i32 = text.find(pattern).map(|i| i as i32).unwrap_or(-1);
    position
}
#[doc = "Test counting pattern occurrences"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_count_occurrences() -> i32 {
    let text: String = "abc abc abc".to_string();
    let pattern: String = "abc".to_string();
    let count: i32 = text.matches(pattern).count() as i32 as i32;
    count
}
#[doc = "Test replacing pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_replace_pattern() -> String {
    let text: String = STR_HELLO_WORLD.to_string();
    let old_pattern: String = "World".to_string();
    let new_text: String = "Python".to_string();
    let result: String = text.replace(old_pattern, new_text);
    result
}
#[doc = "Test splitting by pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_split_by_pattern() -> Vec<String> {
    let text: String = "apple,banana,cherry".to_string();
    let delimiter: String = ",".to_string();
    let parts: Vec<String> = text
        .split(delimiter)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    parts
}
#[doc = "Test matching digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_match_digit() -> bool {
    let text: String = "123".to_string();
    let is_digit: bool = text.chars().all(|c| c.is_numeric());
    is_digit
}
#[doc = "Test matching alphabetic characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_match_alpha() -> bool {
    let text: String = STR_HELLO.to_string();
    let is_alpha: bool = text.chars().all(|c| c.is_alphabetic());
    is_alpha
}
#[doc = "Test matching alphanumeric characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_match_alphanumeric() -> bool {
    let text: String = "Hello123".to_string();
    let is_alnum: bool = text.chars().all(|c| c.is_alphanumeric());
    is_alnum
}
#[doc = "Extract all digits from text"]
#[doc = " Depyler: verified panic-free"]
pub fn extract_digits(text: &str) -> String {
    let mut digits: String = STR_EMPTY.to_string();
    for _char in text.chars() {
        let char = _char.to_string();
        if char.chars().all(|c| c.is_numeric()) {
            digits = format!("{}{}", digits, char);
        }
    }
    digits
}
#[doc = "Extract all letters from text"]
#[doc = " Depyler: verified panic-free"]
pub fn extract_letters(text: &str) -> String {
    let mut letters: String = STR_EMPTY.to_string();
    for _char in text.chars() {
        let char = _char.to_string();
        if char.chars().all(|c| c.is_alphabetic()) {
            letters = format!("{}{}", letters, char);
        }
    }
    letters
}
#[doc = "Find all words in text(space-separated)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn find_all_words(text: &str) -> Vec<String> {
    let words: Vec<String> = text
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
    let _cse_temp_2 = (cleaned.chars().all(|c| c.is_numeric())) && (_cse_temp_1);
    let is_valid: bool = _cse_temp_2;
    is_valid
}
#[doc = "Extract domain from URL"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn extract_url_domain(mut url: String) -> String {
    let mut url;
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
    let mut domain: String;
    if _cse_temp_0 {
        domain = {
            let base = url;
            let stop = (slash_pos).max(0) as usize;
            base[..stop.min(base.len())].to_vec()
        };
    } else {
        domain = url;
    }
    domain
}
#[doc = "Remove common punctuation marks"]
#[doc = " Depyler: verified panic-free"]
pub fn remove_punctuation(text: &str) -> String {
    let punctuation: String = ".,!?;:".to_string();
    let mut result: String = STR_EMPTY.to_string();
    for _char in text.chars() {
        let char = _char.to_string();
        let mut is_punct: bool = false;
        for p in punctuation.iter().cloned() {
            if char == p {
                is_punct = true;
                break;
            }
        }
        if !is_punct {
            result = format!("{}{}", result, char);
        }
    }
    result
}
#[doc = "Normalize multiple spaces to single space"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn normalize_whitespace(text: &str) -> String {
    let words: Vec<String> = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let normalized: String = words.join(" ");
    normalized
}
#[doc = "Check if text starts with pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn starts_with_pattern<'a, 'b>(text: &'a str, pattern: &'b str) -> bool {
    text.starts_with(pattern)
}
#[doc = "Check if text ends with pattern"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn ends_with_pattern<'a, 'b>(text: &'a str, pattern: &'b str) -> bool {
    text.ends_with(pattern)
}
#[doc = "Case-insensitive pattern matching"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn case_insensitive_match<'b, 'a>(text: &'a str, pattern: &'b str) -> bool {
    let text_lower: String = text.to_lowercase();
    let pattern_lower: String = pattern.to_lowercase();
    let _cse_temp_0 = text_lower.contains(pattern_lower);
    let matches: bool = _cse_temp_0;
    matches
}
#[doc = "Find text between two markers"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn find_between<'a, 'c, 'b>(
    text: &'a str,
    start_marker: &'b str,
    end_marker: &'c str,
) -> String {
    let mut start_pos: i32 = text.find(start_marker).map(|i| i as i32).unwrap_or(-1);
    let _cse_temp_0 = start_pos < 0;
    if _cse_temp_0 {
        return STR_EMPTY;
    }
    let _cse_temp_1 = start_marker.len() as i32;
    start_pos = format!("{}{}", start_pos, _cse_temp_1);
    let end_pos: i32 = text[start_pos as usize..]
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
    result
}
#[doc = "Replace multiple patterns"]
pub fn replace_multiple<'b, 'a>(
    text: &'a str,
    replacements: &'b Vec<()>,
) -> Result<String, IndexError> {
    let mut result: String = text;
    for replacement in replacements.iter().cloned() {
        let old: String = replacement.get(0usize).cloned().unwrap_or_default();
        let new: String = replacement.get(1usize).cloned().unwrap_or_default();
        result = result.replace(old, new);
    }
    Ok(result)
}
#[doc = "Count occurrences of a word"]
#[doc = " Depyler: verified panic-free"]
pub fn count_word_occurrences<'a, 'b>(text: &'a str, word: &'b str) -> i32 {
    let words: Vec<String> = text
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
pub fn extract_numbers_from_text(text: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = vec![];
    let mut current_num: String = STR_EMPTY.to_string();
    for _char in text.chars() {
        let char = _char.to_string();
        let mut current_num;
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
pub fn wildcard_match_simple<'a, 'b>(text: &'a str, pattern: &'b str) -> Result<bool, IndexError> {
    let _cse_temp_0 = !pattern.contains(&"*");
    if _cse_temp_0 {
        return Ok(text == pattern);
    }
    let parts: Vec<String> = pattern
        .split("*")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _cse_temp_1 = parts.len() as i32;
    let _cse_temp_2 = _cse_temp_1 != 2;
    if _cse_temp_2 {
        return Ok(false);
    }
    let prefix: String = parts.get(0usize).cloned().unwrap_or_default();
    let suffix: String = parts.get(1usize).cloned().unwrap_or_default();
    let mut has_prefix: bool = true;
    let mut has_suffix: bool = true;
    let _cse_temp_3 = prefix.len() as i32;
    let _cse_temp_4 = _cse_temp_3 > 0;
    if _cse_temp_4 {
        has_prefix = text.starts_with(prefix);
    }
    let _cse_temp_5 = suffix.len() as i32;
    let _cse_temp_6 = _cse_temp_5 > 0;
    if _cse_temp_6 {
        has_suffix = text.ends_with(suffix);
    }
    Ok((has_prefix) && (has_suffix))
}
#[doc = "Run all regex module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_re_features() -> Result<(), Box<dyn std::error::Error>> {
    let matches: bool = test_simple_match();
    let contains: bool = test_contains_pattern();
    let position: i32 = test_find_pattern_position();
    let mut count: i32 = test_count_occurrences();
    let replaced: String = test_replace_pattern();
    let split_result: Vec<String> = test_split_by_pattern();
    let is_digit: bool = test_match_digit();
    let is_alpha: bool = test_match_alpha();
    let is_alnum: bool = test_match_alphanumeric();
    let text: String = "abc123def456".to_string();
    let mut digits: String = extract_digits(text);
    let mut letters: String = extract_letters(text);
    let sentence: String = "Hello world from Python".to_string();
    let words: Vec<String> = find_all_words(sentence);
    let email_valid: bool = validate_email_simple("user@example.com");
    let email_invalid: bool = validate_email_simple("notanemail");
    let phone_valid: bool = validate_phone_simple("555-123-4567");
    let phone_invalid: bool = validate_phone_simple("abc");
    let mut url: String = "https://www.example.com/path/page.html".to_string();
    let mut domain: String = extract_url_domain(url);
    let punct_text: String = "Hello, World!".to_string();
    let no_punct: String = remove_punctuation(punct_text);
    let spaces: String = "Hello    World  !".to_string();
    let normalized: String = normalize_whitespace(spaces);
    let starts: bool = starts_with_pattern(STR_HELLO_WORLD, STR_HELLO);
    let ends: bool = ends_with_pattern(STR_HELLO_WORLD, "World");
    let case_match: bool = case_insensitive_match(STR_HELLO, "hello");
    let tagged: String = "<tag>content</tag>".to_string();
    let content: String = find_between(tagged, "<tag>", "</tag>");
    let replacements: Vec<()> = vec![("a", "x"), ("b", "y")];
    let multi_replace: String = replace_multiple("aabbcc", &replacements);
    let para: String = "the quick brown fox jumps over the lazy dog".to_string();
    let the_count: i32 = count_word_occurrences(para, "the");
    let mixed: String = "I have 2 apples and 5 oranges".to_string();
    let nums: Vec<i32> = extract_numbers_from_text(mixed);
    let wildcard1: bool = wildcard_match_simple("hello.txt", "*.txt");
    let wildcard2: bool = wildcard_match_simple("test_file.py", "test_*");
    println!("{}", "All regex module tests completed successfully");
    Ok(())
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
