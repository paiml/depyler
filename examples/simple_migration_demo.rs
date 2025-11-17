use serde_json;
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
#[doc = "Shows accumulator pattern."]
#[doc = " Depyler: verified panic-free"]
pub fn accumulator_example(items: &serde_json::Value) {
    let mut result = vec![];
    for item in items.iter().cloned() {
        result.push(item * 2);
    }
    result
}
#[doc = "Shows inefficient string building."]
#[doc = " Depyler: verified panic-free"]
pub fn string_concat_example(values: &serde_json::Value) {
    let mut output = "";
    for val in values.iter().cloned() {
        output = output + val.to_string();
    }
    output
}
#[doc = "Shows range(len()) antipattern."]
#[doc = " Depyler: proven to terminate"]
pub fn enumerate_example(data: &serde_json::Value) -> Result<(), IndexError> {
    for i in 0..data.len() as i32 {
        println!(
            "{} {}",
            i,
            data.get(i as usize).cloned().unwrap_or_default()
        );
    }
}
#[doc = "Shows while True pattern."]
#[doc = " Depyler: verified panic-free"]
pub fn while_true_example() {
    let mut count = 0;
    while true {
        count = count + 1;
        if count > 10 {
            break;
        }
    }
    count
}
