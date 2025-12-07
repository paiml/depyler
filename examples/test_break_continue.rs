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
#[doc = "Test basic break statement"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_simple_break() {
    for i in 0..10 {
        if i == 5 {
            break;
        }
        println!("{}", i);
    }
}
#[doc = "Test basic continue statement"]
#[doc = " Depyler: proven to terminate"]
pub fn test_simple_continue() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
    Ok(())
}
#[doc = "Test break in nested loops"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nested_break() {
    for i in 0..3 {
        for j in 0..3 {
            if (i == 1) && (j == 1) {
                break;
            }
            println!("{} {}", i, j);
        }
    }
}
#[doc = "Test break in while loop"]
#[doc = " Depyler: verified panic-free"]
pub fn test_while_break() {
    let mut i = 0;
    loop {
        if i >= 5 {
            break;
        }
        println!("{}", i);
        i = i + 1;
    }
}
#[doc = "Test continue in while loop"]
pub fn test_while_continue() -> Result<(), Box<dyn std::error::Error>> {
    let mut i = 0;
    while i < 10 {
        i = i + 1;
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
    Ok(())
}
