const STR_HELLO_WORLD: &'static str = "hello world";
const STR_HELLO: &'static str = "hello";
#[doc = "Test str.upper() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_upper() -> String {
    let text = STR_HELLO_WORLD;
    let result = text.to_uppercase();
    result.to_string()
}
#[doc = "Test str.lower() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_lower() -> String {
    let text = "HELLO WORLD";
    let result = text.to_lowercase();
    result.to_string()
}
#[doc = "Test str.strip() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_strip() -> String {
    let text = "  hello world  ";
    let result = text.trim().to_string();
    result.to_string()
}
#[doc = "Test str.startswith() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_startswith() -> bool {
    let text = STR_HELLO_WORLD;
    let result = text.starts_with("hello");
    result
}
#[doc = "Test str.startswith() returns False"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_startswith_false() -> bool {
    let text = STR_HELLO_WORLD;
    let result = text.starts_with("world");
    result
}
#[doc = "Test str.endswith() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_endswith() -> bool {
    let text = STR_HELLO_WORLD;
    let result = text.ends_with("world");
    result
}
#[doc = "Test str.endswith() returns False"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_endswith_false() -> bool {
    let text = STR_HELLO_WORLD;
    let result = text.ends_with("hello");
    result
}
#[doc = "Test str.split() with default whitespace"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_split_whitespace() -> i32 {
    let text = "hello world foo bar";
    let parts = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    parts.len() as i32 as i32
}
#[doc = "Test str.split(sep) with custom separator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_split_separator() -> i32 {
    let text = "hello,world,foo,bar";
    let parts = text
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    parts.len() as i32 as i32
}
#[doc = "Test str.join () method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_join() -> String {
    let parts = vec![STR_HELLO.to_string(), "world".to_string()];
    let result = parts.join(",");
    result.to_string()
}
#[doc = "Test str.join () with space separator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_join_space() -> String {
    let parts = vec![
        STR_HELLO.to_string(),
        "world".to_string(),
        "foo".to_string(),
    ];
    let result = parts.join(" ");
    result.to_string()
}
#[doc = "Test str.find() when substring exists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_find_found() -> i32 {
    let text = STR_HELLO_WORLD;
    let pos = text.find("world").map(|i| i as i32).unwrap_or(-1);
    pos
}
#[doc = "Test str.find() when substring doesn't exist"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_find_not_found() -> i32 {
    let text = STR_HELLO_WORLD;
    let pos = text.find("xyz").map(|i| i as i32).unwrap_or(-1);
    pos
}
#[doc = "Test str.replace() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_replace() -> String {
    let text = STR_HELLO_WORLD;
    let result = text.replace("world", "rust");
    result.to_string()
}
#[doc = "Test str.replace() with multiple occurrences"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_replace_multiple() -> String {
    let text = "hello hello hello";
    let result = text.replace("hello", "hi");
    result.to_string()
}
#[doc = "Test str.count() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count() -> i32 {
    let text = "hello hello world";
    let count = text.matches("hello").count() as i32;
    count
}
#[doc = "Test str.count() with single occurrence"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count_single() -> i32 {
    let text = STR_HELLO_WORLD;
    let count = text.matches("world").count() as i32;
    count
}
#[doc = "Test str.count() with no occurrences"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count_none() -> i32 {
    let text = STR_HELLO_WORLD;
    let count = text.matches("xyz").count() as i32;
    count
}
#[doc = "Test str.isdigit() returns True for digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isdigit_true() -> bool {
    let text = "12345";
    let result = text.chars().all(|c| c.is_numeric());
    result
}
#[doc = "Test str.isdigit() returns False for non-digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isdigit_false() -> bool {
    let text = STR_HELLO;
    let result = text.chars().all(|c| c.is_numeric());
    result
}
#[doc = "Test str.isalpha() returns True for letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isalpha_true() -> bool {
    let text = STR_HELLO;
    let result = text.chars().all(|c| c.is_alphabetic());
    result
}
#[doc = "Test str.isalpha() returns False for non-letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isalpha_false() -> bool {
    let text = "hello123";
    let result = text.chars().all(|c| c.is_alphabetic());
    result
}
#[doc = "Test split on empty string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_empty_split() -> i32 {
    let text = "";
    let parts = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    parts.len() as i32 as i32
}
#[doc = "Test string methods on single character"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_single_char() -> String {
    let text = "a";
    let result = text.to_uppercase();
    result.to_string()
}
#[doc = "Test string methods with special characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_special_chars() -> bool {
    let text = "hello-world_123";
    let result = text.starts_with("hello");
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_split_whitespace_examples() {
        let _ = test_str_split_whitespace();
    }
    #[test]
    fn test_test_str_split_separator_examples() {
        let _ = test_str_split_separator();
    }
    #[test]
    fn test_test_str_find_found_examples() {
        let _ = test_str_find_found();
    }
    #[test]
    fn test_test_str_find_not_found_examples() {
        let _ = test_str_find_not_found();
    }
    #[test]
    fn test_test_str_count_examples() {
        let _ = test_str_count();
    }
    #[test]
    fn test_test_str_count_single_examples() {
        let _ = test_str_count_single();
    }
    #[test]
    fn test_test_str_count_none_examples() {
        let _ = test_str_count_none();
    }
    #[test]
    fn test_test_str_empty_split_examples() {
        let _ = test_str_empty_split();
    }
}
