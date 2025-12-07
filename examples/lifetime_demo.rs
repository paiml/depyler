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
#[doc = "Get the length of a string without consuming it"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_length(s: &str) -> i32 {
    s.len() as i32 as i32
}
#[doc = "Extract the first word from a string"]
#[doc = " Depyler: proven to terminate"]
pub fn first_word(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let words = s
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if !words.is_empty() {
        return Ok(words.get(0usize).cloned().unwrap_or_default());
    }
    Ok("".to_string())
}
#[doc = "Append an exclamation mark to a string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn append_exclamation(mut s: String) -> String {
    s = format!("{}{}", s, "!");
    s.to_string()
}
#[doc = "Return the longest of two strings"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn longest(x: String, y: String) -> String {
    let _cse_temp_0 = x.len() as i32;
    let _cse_temp_1 = y.len() as i32;
    let _cse_temp_2 = _cse_temp_0 > _cse_temp_1;
    if _cse_temp_2 {
        x.to_string()
    } else {
        y.to_string()
    }
}
#[doc = "Modify a string in place"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn modify_string(mut s: String) {
    s = format!("{}{}", s, " modified");
    let _ = ();
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_get_length_examples() {
        assert_eq!(get_length(""), 0);
        assert_eq!(get_length("a"), 1);
        assert_eq!(get_length("abc"), 3);
    }
}
