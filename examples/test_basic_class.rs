use serde_json;
#[doc = "// NOTE: Map Python module 'dataclasses'(tracked in DEPYLER-0424)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub value: i32,
}
impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
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
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn dataclass<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_counter() -> (serde_json::Value, serde_json::Value) {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    let val = c.get_value();
    let c2 = Counter::create_with_value(10);
    (val, c2.value)
}
