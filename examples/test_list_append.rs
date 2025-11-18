#[derive(Debug, Clone)]
pub struct Logger {
    pub messages: Vec<String>,
}
impl Logger {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    pub fn log(&self, msg: String) -> i32 {
        self.messages.push(msg);
        return self.messages.len();
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_logger() {
    let logger = Logger::new();
    let count = logger.log("Hello".to_string());
    count
}
