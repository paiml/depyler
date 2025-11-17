#[derive(Debug, Clone)]
pub struct Counter {
    pub value: i32,
}
impl Counter {
    pub fn new(initial: i32) -> Self {
        Self { value: 0 }
    }
    pub fn increment(&mut self, amount: i32) {
        self.value = self.value + amount;
        return self.value;
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_counter() {
    let mut c = Counter::new(10);
    let result = c.increment(5);
    result
}
