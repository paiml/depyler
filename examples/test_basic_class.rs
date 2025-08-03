#[derive(Debug, Clone, PartialEq)] pub struct Counter {
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
    let mut c = Counter();
    c.increment();
    c.increment();
    let mut val = c.get_value();
    let mut c2 = Counter.create_with_value(10);
    return(val, c2.value)
}