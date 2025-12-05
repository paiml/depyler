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
#[doc = "Process list of strings."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_strings(items: &Vec<String>) -> i32 {
    items.len() as i32 as i32
}
#[doc = "Process list of integers."]
pub fn process_integers(items: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total: i32 = 0;
    for item in items.iter().cloned() {
        total = total + item % 10;
    }
    Ok(total)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_process_strings_examples() {
        assert_eq!(process_strings(&vec![]), 0);
        assert_eq!(process_strings(&vec![1]), 1);
        assert_eq!(process_strings(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_process_integers_examples() {
        assert_eq!(process_integers(&vec![]), 0);
        assert_eq!(process_integers(&vec![1]), 1);
        assert_eq!(process_integers(&vec![1, 2, 3]), 3);
    }
}
