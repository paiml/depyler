#[doc = "// TODO: Map Python module 'dataclasses'"] #[derive(Debug, Clone, PartialEq)] pub struct Counter {
    pub value: i32
}
impl Counter {
    pub fn new(value: i32)  -> Self {
    Self {
    value
}
} pub fn increment(& mut self) {
    self.value = self.value + 1;
   
}
pub fn get_value(& mut self)  -> i32 {
    return self.value;
   
}
pub fn create_with_value(val: i32) {
    return Counter::new(val);
   
}
} #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_counter()  -> DynamicType {
    let c = Counter::new(0);
    c.i📄 Source: examples/test_basic_class.py (758 bytes)
📝 Output: examples/test_basic_class.rs (636 bytes)
⏱️  Parse time: 8ms
📊 Throughput: 91.7 KB/s
⏱️  Total time: 8ms
