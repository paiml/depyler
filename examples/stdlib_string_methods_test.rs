#[doc = "Test str.upper() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_upper() -> String {
    let result = "hello-world_123".to_string().to_uppercase();
    return result;
}
#[doc = "Test str.lower() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_lower() -> String {
    let result = "hello-world_123".to_string().to_lowercase();
    return result;
}
#[doc = "Test str.strip() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_strip() -> String {
    let result = "hello-world_123".to_string().trim().to_string();
    return result;
}
#[doc = "Test str.startswith() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_startswith() -> bool {
    let result = "hello-world_123".to_string().starts_with("hello");
    return result;
}
#[doc = "Test str.startswith() returns False"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_startswith_false() -> bool {
    let result = "hello-world_123".to_string().starts_with("world");
    return result;
}
#[doc = "Test str.endswith() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_endswith() -> bool {
    let result = "hello-world_123".to_string().ends_with("world");
    return result;
}
#[doc = "Test str.endswith() returns False"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_endswith_false() -> bool {
    let result = "hello-world_123".to_string().ends_with("hello");
    return result;
}
#[doc = "Test str.split() with default whitespace"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_split_whitespace() -> i32 {
    let parts = "hello-world_123"
        .to_string()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _cse_temp_0 = parts.len() as i32;
    return _cse_temp_0;
}
#[doc = "Test str.split(sep) with custom separator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_split_separator() -> i32 {
    let parts = "hello-world_123"
        .to_string()
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _cse_temp_0 = parts.len() as i32;
    return _cse_temp_0;
}
#[doc = "Test str.join () method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_join() -> String {
    let parts = vec!["hello", "world"];
    let result = parts.join(",");
    return result;
}
#[doc = "Test str.join () with space separator"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_join_space() -> String {
    let parts = vec!["hello", "world", "foo"];
    let result = parts.join(" ");
    return result;
}
#[doc = "Test str.find() when substring exists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_find_found() -> i32 {
    let pos = "hello-world_123"
        .to_string()
        .find("world")
        .map(|i| i as i32)
        .unwrap_or(-1);
    return pos;
}
#[doc = "Test str.find() when substring doesn't exist"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_find_not_found() -> i32 {
    let pos = "hello-world_123"
        .to_string()
        .find("xyz")
        .map(|i| i as i32)
        .unwrap_or(-1);
    return pos;
}
#[doc = "Test str.replace() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_replace() -> String {
    let result = "hello-world_123".to_string().replace("world", "rust");
    return result;
}
#[doc = "Test str.replace() with multiple occurrences"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_replace_multiple() -> String {
    let result = "hello-world_123".to_string().replace("hello", "hi");
    return result;
}
#[doc = "Test str.count() method"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count() -> i32 {
    let count = "hello-world_123".to_string().matches("hello").count() as i32;
    return count;
}
#[doc = "Test str.count() with single occurrence"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count_single() -> i32 {
    let count = "hello-world_123".to_string().matches("world").count() as i32;
    return count;
}
#[doc = "Test str.count() with no occurrences"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_count_none() -> i32 {
    let count = "hello-world_123".to_string().matches("xyz").count() as i32;
    return count;
}
#[doc = "Test str.isdigit() returns True for digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isdigit_true() -> bool {
    let result = "hello-world_123"
        .to_string()
        .chars()
        .all(|c| c.is_numeric());
    return result;
}
#[doc = "Test str.isdigit() returns False for non-digits"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isdigit_false() -> bool {
    let result = "hello-world_123"
        .to_string()
        .chars()
        .all(|c| c.is_numeric());
    return result;
}
#[doc = "Test str.isalpha() returns True for letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isalpha_true() -> bool {
    let result = "hello-world_123"
        .to_string()
        .chars()
        .all(|c| c.is_alphabetic());
    return result;
}
#[doc = "Test str.isalpha() returns False for non-letters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_isalpha_false() -> bool {
    let result = "hello-world_123"
        .to_string()
        .chars()
        .all(|c| c.is_alphabetic());
    return result;
}
#[doc = "Test split on empty string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_empty_split() -> i32 {
    let parts = "hello-world_123"
        .to_string()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _cse_temp_0 = parts.len() as i32;
    return _cse_temp_0;
}
#[doc = "Test string methods on single character"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_single_char() -> String {
    let result = "hello-world_123".to_string().to_uppercase();
    return result;
}
#[doc = "Test string methods with special characters"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_str_special_chars() -> bool {
    let result = "hello-world_123".to_string().starts_with("hello");
    return result;
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_split_whitespace_examples() {
        let _ = test_str_split_whitespace();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_split_separator_examples() {
        let _ = test_str_split_separator();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_find_found_examples() {
        let _ = test_str_find_found();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_find_not_found_examples() {
        let _ = test_str_find_not_found();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_count_examples() {
        let _ = test_str_count();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_count_single_examples() {
        let _ = test_str_count_single();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_count_none_examples() {
        let _ = test_str_count_none();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_str_empty_split_examples() {
        let _ = test_str_empty_split();
    }
}
