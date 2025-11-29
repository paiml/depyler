use serde_json;
#[derive(Debug, Clone)]
pub struct Calculator {
    pub value: i32,
    pub history: Vec<serde_json::Value>,
}
impl Calculator {
    pub fn new(initial_value: i32) -> Self {
        Self {
            value: 0,
            history: Vec::new(),
        }
    }
    pub fn add(&mut self, x: i32) -> i32 {
        self.value = self.value + x;
        self.history.push(format!("add({})", x));
        return self.value;
    }
    pub fn multiply(&mut self, x: i32) -> i32 {
        self.value = self.value * x;
        self.history.push(format!("multiply({})", x));
        return self.value;
    }
    pub fn square(x: i32) -> i32 {
        return x * x;
    }
    pub fn from_string(s: String) {
        return Self::new(s.parse::<i32>().unwrap_or(0));
    }
    pub fn current(&self) -> i32 {
        return self.value;
    }
    pub fn get_history(&self) -> Vec<String> {
        return self.history.clone();
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_calculator() -> (
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
) {
    let mut calc = Calculator::new(10);
    let result1 = calc.add(5);
    let result2 = calc.multiply(2);
    let squared = Calculator::square(4);
    let current = calc.current;
    let history = calc.get_history();
    (result1, result2, squared, current, history)
}
