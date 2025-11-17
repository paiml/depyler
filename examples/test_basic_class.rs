#[doc = "// TODO: Map Python module 'dataclasses'"]
#[derive(Debug, Clone, PartialEq)]
pub struct Counter {}
impl Counter {
    pub const value: i32 = 0;
    pub fn new() -> Self {
        Self {}
    }
    pub fn increment(&mut self) {
        self.value = self.value + 1;
    }
    pub fn get_value(&self) -> i32 {
        return self.value;
    }
    pub fn create_with_value(val: i32) {
        return Counter::new(val);
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_counter() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    let val = c.get_value();
    let c2 = Counter.create_with_value(10);
    (val, c2.value)
}
