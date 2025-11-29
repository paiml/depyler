use serde_json;
#[derive(Debug, Clone)]
pub struct ResourceManager {
    pub name: String,
    pub is_open: bool,
}
impl ResourceManager {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_open: false,
        }
    }
    pub fn __enter__(&mut self) -> &Self {
        self.is_open = true;
        return self;
    }
    pub fn __exit__(
        &mut self,
        exc_type: serde_json::Value,
        exc_val: serde_json::Value,
        exc_tb: serde_json::Value,
    ) -> bool {
        self.is_open = false;
        return false;
    }
    pub fn use_resource(&self) -> i32 {
        if self.is_open {
            return 42;
        };
        return 0;
    }
}
#[derive(Debug, Clone)]
pub struct Counter {
    pub max_count: i32,
    pub count: i32,
}
impl Counter {
    pub fn new(max_count: i32) -> Self {
        Self {
            max_count,
            count: 0,
        }
    }
    pub fn __iter__(&self) -> &Self {
        return self;
    }
    pub fn __next__(&mut self) -> i32 {
        if self.count < self.max_count {
            self.count = self.count + 1;
            return self.count;
        };
        return -1;
    }
}
#[doc = "Demonstrate with statement support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demo_with_statement() {
    let _context = ResourceManager::new("test".to_string().to_string());
    let rm = _context.__enter__();
    let result = rm.use_resource();
    result
}
#[doc = "Demonstrate iterator protocol"]
#[doc = " Depyler: verified panic-free"]
pub fn demo_iterator() -> i32 {
    let mut counter = Counter::new(3);
    let mut total = 0;
    let mut val = counter.__next__();
    while val != -1 {
        total = total + val;
        val = counter.__next__();
    }
    total
}
#[doc = "Run all v1.3.0 feature demos"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let with_result = demo_with_statement();
    let iter_result = demo_iterator();
    with_result + iter_result
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_demo_iterator_examples() {
        let _ = demo_iterator();
    }
}
